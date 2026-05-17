//! rstml → internal AST translation.
//!
//! We translate rstml's generic `Node` tree into our own `RsxNode` enum which
//! carries only the information the emitter needs.

use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use rstml::node::{
    KeyedAttributeValue, Node, NodeAttribute, NodeBlock, NodeElement, NodeFragment, NodeText,
};
use syn::{Expr, spanned::Spanned};

/// A key-value attribute after parsing.
#[derive(Debug)]
pub struct RsxAttr {
    /// The attribute key as a plain string (e.g. `"data-foo"`, `"class"`).
    pub key: String,
    /// The attribute value.
    pub value: RsxAttrValue,
}

#[derive(Debug)]
pub enum RsxAttrValue {
    /// A string literal known at compile time — already stored as the raw
    /// string (not yet HTML-escaped; the emitter handles that).
    Literal(String),
    /// An arbitrary Rust expression.
    Expr(Box<Expr>),
    /// Value-less boolean attribute (e.g. `<input disabled />`).
    None,
}

/// Our simplified AST node.
#[derive(Debug)]
pub enum RsxNode {
    /// A PascalCase component like `<Button variant="primary">...</Button>`.
    Component {
        /// The original PascalCase tag name, e.g. `"Button"`.
        name: String,
        attrs: Vec<RsxAttr>,
        children: Vec<RsxNode>,
        span: Span,
    },
    /// A lowercase/hyphenated HTML element like `<div class="foo">`.
    Element {
        /// The tag name string, e.g. `"div"`.
        name: String,
        attrs: Vec<RsxAttr>,
        children: Vec<RsxNode>,
        span: Span,
    },
    /// A quoted text node — `"hello"` inside a tag.
    Text(String),
    /// A `{expr}` block.
    Block(TokenStream),
    /// `<>...</>` fragment.
    Fragment(Vec<RsxNode>),
}

/// Convert rstml's `Node` list into our `RsxNode` list.
/// Returns a `syn::Error` on any structural problem we detect.
pub fn translate_nodes(nodes: Vec<Node>) -> syn::Result<Vec<RsxNode>> {
    nodes.into_iter().map(translate_node).collect()
}

fn translate_node(node: Node) -> syn::Result<RsxNode> {
    match node {
        Node::Element(el) => translate_element(el),
        Node::Fragment(frag) => translate_fragment(frag),
        Node::Text(text) => translate_text(text),
        Node::Block(block) => translate_block(block),
        Node::Comment(_) => {
            // HTML comments — silently drop.
            Ok(RsxNode::Fragment(vec![]))
        }
        Node::Doctype(_) => Ok(RsxNode::Fragment(vec![])),
        Node::RawText(raw) => {
            // Unquoted text content — rstml gives us the raw token stream.
            // We just re-emit it as a block expression.
            let ts = raw.to_token_stream();
            Ok(RsxNode::Block(ts))
        }
        Node::Custom(_) => unreachable!("no custom nodes configured"),
    }
}

fn translate_element(el: NodeElement<rstml::node::Infallible>) -> syn::Result<RsxNode> {
    let name_str = el.name().to_string();
    let span = el.name().span();
    let attrs = translate_attrs(el.attributes())?;
    let children = translate_nodes(el.children)?;

    if is_pascal_case(&name_str) {
        Ok(RsxNode::Component {
            name: name_str,
            attrs,
            children,
            span,
        })
    } else {
        Ok(RsxNode::Element {
            name: name_str,
            attrs,
            children,
            span,
        })
    }
}

fn translate_fragment(frag: NodeFragment<rstml::node::Infallible>) -> syn::Result<RsxNode> {
    let children = translate_nodes(frag.children)?;
    Ok(RsxNode::Fragment(children))
}

fn translate_text(text: NodeText) -> syn::Result<RsxNode> {
    Ok(RsxNode::Text(text.value_string()))
}

fn translate_block(block: NodeBlock) -> syn::Result<RsxNode> {
    match block {
        NodeBlock::ValidBlock(b) => {
            // `b` is a `syn::Block`; emit its inner statements as a token stream.
            Ok(RsxNode::Block(b.to_token_stream()))
        }
        NodeBlock::Invalid(inv) => Err(syn::Error::new_spanned(
            inv,
            "invalid expression block in rsx!",
        )),
    }
}

fn translate_attrs(attrs: &[NodeAttribute]) -> syn::Result<Vec<RsxAttr>> {
    let mut out = Vec::with_capacity(attrs.len());
    for attr in attrs {
        match attr {
            NodeAttribute::Attribute(keyed) => {
                let key = keyed.key.to_string();
                let value = match &keyed.possible_value {
                    KeyedAttributeValue::Value(v) => {
                        if let Some(s) = v.value_literal_string() {
                            RsxAttrValue::Literal(s)
                        } else if let Some(expr) = keyed.value() {
                            RsxAttrValue::Expr(Box::new(expr.clone()))
                        } else {
                            RsxAttrValue::None
                        }
                    }
                    KeyedAttributeValue::None => RsxAttrValue::None,
                    KeyedAttributeValue::Binding(_) => {
                        // `attr(x)` binding syntax — not supported in basecoat rsx!
                        return Err(syn::Error::new_spanned(
                            keyed,
                            "closure binding attributes are not supported in rsx!; use attr={expr} instead",
                        ));
                    }
                };
                out.push(RsxAttr { key, value });
            }
            NodeAttribute::Block(block) => {
                // `<div {"dyn-key"}>` — not supported in basecoat rsx!
                return Err(syn::Error::new_spanned(
                    block,
                    "dynamic attribute block is not supported in rsx!",
                ));
            }
        }
    }
    Ok(out)
}

/// Returns true if the name's first non-punctuation character is uppercase
/// ASCII (i.e. it is a component).
pub fn is_pascal_case(name: &str) -> bool {
    name.chars()
        .next()
        .map(|c| c.is_ascii_uppercase())
        .unwrap_or(false)
}
