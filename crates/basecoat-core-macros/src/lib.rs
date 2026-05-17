use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
    Data, DeriveInput, Field, Fields, Ident, LitStr, Meta, Type, parse_macro_input,
    punctuated::Punctuated, token::Comma,
};

/// `#[derive(BasecoatProps)]` generates a builder for component prop structs.
///
/// # Field attributes
///
/// - `#[prop(default)]`          — field uses `Default::default()` if not provided in builder.
/// - `#[prop(default = expr)]`   — field uses `expr` if not provided.
/// - `#[prop(into)]`             — builder setter accepts `impl Into<FieldType>`.
/// - `#[prop(optional)]`         — field is `Option<T>`; missing → `None`.
/// - `#[prop(extend)]`           — marks the `AttrMap` catch-all field (at most one per struct).
///
/// # Builder design choice
///
/// We use a **runtime-panic builder** for v0.1. A typestate builder would require one
/// phantom-type parameter per required field and substantially more generated code.
/// For a UI component library where every field is either optional or has a default,
/// the extra complexity of typestate adds no practical safety benefit. If a required
/// field (no `default`/`optional`) is omitted, `build()` panics with a clear message.
///
/// # Hydration helper
///
/// A `const __BASECOAT_EXTEND_FIELD: Option<&'static str>` associated constant is
/// emitted on the props struct so the `rsx!` macro (phase 2c) can discover at compile
/// time where to push unknown HTML attributes.
#[proc_macro_derive(BasecoatProps, attributes(prop))]
pub fn derive_basecoat_props(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match derive_impl(input) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

// ── per-field parsed metadata ────────────────────────────────────────────────

struct PropField {
    ident: Ident,
    ty: Type,
    default_expr: Option<TokenStream2>, // None = required, Some(expr) = has default
    into: bool,
    extend: bool,
}

impl PropField {
    fn from_field(field: &Field) -> syn::Result<Self> {
        let ident = field
            .ident
            .clone()
            .ok_or_else(|| syn::Error::new_spanned(field, "BasecoatProps requires named fields"))?;
        let ty = field.ty.clone();

        let mut default_expr: Option<TokenStream2> = None;
        let mut into = false;
        let mut extend = false;
        let mut has_default_marker = false;

        for attr in &field.attrs {
            if !attr.path().is_ident("prop") {
                continue;
            }
            let nested = attr.parse_args_with(Punctuated::<Meta, Comma>::parse_terminated)?;
            for meta in nested {
                match &meta {
                    Meta::Path(p) if p.is_ident("default") => {
                        has_default_marker = true;
                        default_expr = Some(quote! { ::core::default::Default::default() });
                    }
                    Meta::NameValue(nv) if nv.path.is_ident("default") => {
                        has_default_marker = true;
                        let val = &nv.value;
                        default_expr = Some(quote! { #val });
                    }
                    Meta::Path(p) if p.is_ident("into") => {
                        into = true;
                    }
                    Meta::Path(p) if p.is_ident("optional") => {
                        if !has_default_marker {
                            default_expr = Some(quote! { ::core::option::Option::None });
                        }
                    }
                    Meta::Path(p) if p.is_ident("extend") => {
                        extend = true;
                        if !has_default_marker {
                            default_expr = Some(quote! { ::core::default::Default::default() });
                        }
                    }
                    other => {
                        return Err(syn::Error::new_spanned(
                            other,
                            "unknown prop attribute; expected default, into, optional, extend",
                        ));
                    }
                }
            }
        }

        Ok(PropField {
            ident,
            ty,
            default_expr,
            into,
            extend,
        })
    }
}

// ── code generation ──────────────────────────────────────────────────────────

fn derive_impl(input: DeriveInput) -> syn::Result<TokenStream2> {
    let struct_ident = &input.ident;
    let builder_ident = format_ident!("{}Builder", struct_ident);

    let fields = match &input.data {
        Data::Struct(ds) => match &ds.fields {
            Fields::Named(f) => &f.named,
            _ => {
                return Err(syn::Error::new_spanned(
                    &input.ident,
                    "BasecoatProps only supports structs with named fields",
                ));
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                &input.ident,
                "BasecoatProps can only be derived on structs",
            ));
        }
    };

    let prop_fields: Vec<PropField> = fields
        .iter()
        .map(PropField::from_field)
        .collect::<syn::Result<_>>()?;

    // Validate: at most one #[prop(extend)] field
    let extend_fields: Vec<_> = prop_fields.iter().filter(|f| f.extend).collect();
    if extend_fields.len() > 1 {
        return Err(syn::Error::new_spanned(
            struct_ident,
            "at most one #[prop(extend)] field is allowed per prop struct",
        ));
    }
    let extend_field_name: Option<LitStr> = extend_fields
        .first()
        .map(|f| LitStr::new(&f.ident.to_string(), proc_macro2::Span::call_site()));

    // Pre-collect token streams for each component of the generated code.

    // Builder struct field declarations: `field_name: Option<FieldType>`
    let builder_field_decls: Vec<TokenStream2> = prop_fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            let ty = &f.ty;
            quote! { #ident: ::core::option::Option<#ty> }
        })
        .collect();

    // Builder::new() field initialisers: `field_name: None`
    let builder_none_inits: Vec<TokenStream2> = prop_fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            quote! { #ident: ::core::option::Option::None }
        })
        .collect();

    // Setter methods on the builder
    let setters: Vec<TokenStream2> = prop_fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            let ty = &f.ty;
            if f.into {
                quote! {
                    pub fn #ident(mut self, val: impl ::core::convert::Into<#ty>) -> Self {
                        self.#ident = ::core::option::Option::Some(val.into());
                        self
                    }
                }
            } else {
                quote! {
                    pub fn #ident(mut self, val: #ty) -> Self {
                        self.#ident = ::core::option::Option::Some(val);
                        self
                    }
                }
            }
        })
        .collect();

    // build() field resolutions
    let struct_name_str = struct_ident.to_string();
    let build_fields: Vec<TokenStream2> = prop_fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            let field_name_str = ident.to_string();
            if let Some(default) = &f.default_expr {
                quote! {
                    #ident: self.#ident.unwrap_or_else(|| #default)
                }
            } else {
                quote! {
                    #ident: self.#ident.unwrap_or_else(|| {
                        panic!(
                            "required field `{}` was not set on `{}::builder()`",
                            #field_name_str,
                            #struct_name_str,
                        )
                    })
                }
            }
        })
        .collect();

    // __BASECOAT_EXTEND_FIELD constant
    let extend_const = if let Some(name) = &extend_field_name {
        quote! {
            pub const __BASECOAT_EXTEND_FIELD: ::core::option::Option<&'static str> =
                ::core::option::Option::Some(#name);
        }
    } else {
        quote! {
            pub const __BASECOAT_EXTEND_FIELD: ::core::option::Option<&'static str> =
                ::core::option::Option::None;
        }
    };

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics #struct_ident #ty_generics #where_clause {
            /// Create a builder for this prop struct.
            pub fn builder() -> #builder_ident #ty_generics {
                #builder_ident {
                    #( #builder_none_inits, )*
                }
            }

            #extend_const
        }

        pub struct #builder_ident #ty_generics #where_clause {
            #( #builder_field_decls, )*
        }

        impl #impl_generics #builder_ident #ty_generics #where_clause {
            #( #setters )*

            pub fn build(self) -> #struct_ident #ty_generics {
                #struct_ident {
                    #( #build_fields, )*
                }
            }
        }

        impl #impl_generics ::core::convert::From<#builder_ident #ty_generics>
            for #struct_ident #ty_generics #where_clause
        {
            fn from(b: #builder_ident #ty_generics) -> Self {
                b.build()
            }
        }
    })
}
