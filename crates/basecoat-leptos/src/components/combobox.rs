use basecoat_core::classes::combobox::combobox as combobox_class;
use basecoat_core::props::combobox::{ComboboxOption, ComboboxProps as CoreProps};
use leptos::prelude::*;
use std::borrow::Cow;

/// Leptos wrapper for the basecoat Combobox component.
///
/// Emits the canonical wrapper, input, and listbox markup expected by the
/// `combobox` WASM controller. The `id` is **required** so the controller
/// can wire ARIA associations correctly.
///
/// ## Compound components
///
/// - [`ComboboxInput`] — the `<input role="combobox">` element.
/// - [`ComboboxListbox`] — the `<div role="listbox">` wrapper.
/// - [`ComboboxOptionView`] — a single `<button role="option">`.
#[component]
pub fn Combobox(
    /// Unique DOM id — required for the WASM controller.
    #[prop(optional, into)]
    id: Option<Cow<'static, str>>,
    /// Form field name for the input.
    #[prop(optional, into)]
    name: Option<Cow<'static, str>>,
    /// Placeholder text.
    #[prop(optional, into)]
    placeholder: Option<Cow<'static, str>>,
    /// Initial input value.
    #[prop(optional, into)]
    value: Option<Cow<'static, str>>,
    /// Static set of options.
    #[prop(default = Vec::new())]
    options: Vec<ComboboxOption>,
    /// Disabled state.
    #[prop(default = false)]
    disabled: bool,
    /// Extra CSS classes appended to the canonical `.select` wrapper class.
    #[prop(optional, into)]
    class: Option<Signal<Cow<'static, str>>>,
) -> impl IntoView {
    let id_value = id.as_deref().unwrap_or("combobox-1").to_owned();
    let listbox_id = format!("{id_value}-listbox");
    let id_for_options = id_value.clone();

    let class_str = Signal::derive(move || {
        let extra = class.as_ref().map(|s| s.get());
        let props = CoreProps {
            class: extra,
            ..Default::default()
        };
        combobox_class(&props)
    });

    let name_attr = name.map(|s| s.to_string());
    let placeholder_attr = placeholder.map(|s| s.to_string());
    let value_attr = value.map(|s| s.to_string());

    let option_views = options
        .into_iter()
        .enumerate()
        .map(|(idx, option)| {
            let option_id = format!("{id_for_options}-option-{idx}");
            view! {
                <button
                    type="button"
                    role="option"
                    id=option_id
                    data-value=option.value.to_string()
                    tabindex="-1"
                >
                    {option.label.to_string()}
                </button>
            }
        })
        .collect_view();

    view! {
        <div
            class=class_str
            id=id_value.clone()
            data-combobox=""
            data-basecoat-hydrate="combobox"
            data-basecoat-version="0.2"
        >
            <input
                type="text"
                role="combobox"
                aria-controls=listbox_id.clone()
                aria-expanded="false"
                aria-autocomplete="list"
                autocomplete="off"
                data-combobox-input=""
                name=name_attr
                placeholder=placeholder_attr
                value=value_attr
                disabled=disabled
            />
            <div
                role="listbox"
                id=listbox_id
                data-combobox-listbox=""
                hidden
            >
                {option_views}
            </div>
        </div>
    }
}

/// The `<input role="combobox">` portion of a Combobox.
///
/// Use this when you want to compose the combobox markup by hand instead of
/// relying on [`Combobox`].
#[component]
pub fn ComboboxInput(
    /// The `id` of the associated listbox — wired to `aria-controls`.
    #[prop(into)]
    listbox_id: Cow<'static, str>,
    /// Form field name.
    #[prop(optional, into)]
    name: Option<Cow<'static, str>>,
    /// Placeholder text.
    #[prop(optional, into)]
    placeholder: Option<Cow<'static, str>>,
    /// Initial value.
    #[prop(optional, into)]
    value: Option<Cow<'static, str>>,
    /// Disabled state.
    #[prop(default = false)]
    disabled: bool,
) -> impl IntoView {
    let name_attr = name.map(|s| s.to_string());
    let placeholder_attr = placeholder.map(|s| s.to_string());
    let value_attr = value.map(|s| s.to_string());
    view! {
        <input
            type="text"
            role="combobox"
            aria-controls=listbox_id.to_string()
            aria-expanded="false"
            aria-autocomplete="list"
            autocomplete="off"
            data-combobox-input=""
            name=name_attr
            placeholder=placeholder_attr
            value=value_attr
            disabled=disabled
        />
    }
}

/// The `<div role="listbox">` portion of a Combobox.
#[component]
pub fn ComboboxListbox(
    /// Listbox DOM id — must match the input's `aria-controls`.
    #[prop(into)]
    id: Cow<'static, str>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            role="listbox"
            id=id.to_string()
            data-combobox-listbox=""
            hidden
        >
            {children()}
        </div>
    }
}

/// A single `<button role="option">` inside a [`ComboboxListbox`].
#[component]
pub fn ComboboxOptionView(
    /// Option DOM id — typically `"{combobox-id}-option-{idx}"`.
    #[prop(into)]
    id: Cow<'static, str>,
    /// Value submitted when the option is selected.
    #[prop(into)]
    value: Cow<'static, str>,
    children: Children,
) -> impl IntoView {
    view! {
        <button
            type="button"
            role="option"
            id=id.to_string()
            data-value=value.to_string()
            tabindex="-1"
        >
            {children()}
        </button>
    }
}
