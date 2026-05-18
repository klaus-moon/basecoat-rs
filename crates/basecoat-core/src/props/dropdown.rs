use crate::{AttrMap, BasecoatProps, Children, Markup};
use std::borrow::Cow;

/// A single item in a dropdown menu.
///
/// Items render as `<button type="button" role="menuitem" tabindex="-1">`
/// inside the floating menu container.
#[derive(Clone, Debug, Default)]
pub struct DropdownItem {
    /// Visible text label.
    pub label: Cow<'static, str>,
    /// Optional unique value attached to the item (rendered as
    /// `data-value="..."`). Useful when the host application needs to map
    /// item activation back to a domain-level identifier.
    pub value: Option<Cow<'static, str>>,
    /// When true, the rendered button carries `disabled` and
    /// `aria-disabled="true"`.
    pub disabled: bool,
}

impl DropdownItem {
    /// Convenience constructor for a simple label-only item.
    pub fn new(label: impl Into<Cow<'static, str>>) -> Self {
        Self {
            label: label.into(),
            value: None,
            disabled: false,
        }
    }
}

/// Dropdown — maps to CSS class `.dropdown-menu`.
///
/// The DOM contract is:
///
/// ```html
/// <details class="dropdown-menu" data-basecoat-hydrate="dropdown"
///          data-basecoat-version="0.2" data-dropdown id="{id}">
///   <summary aria-haspopup="menu" aria-expanded="false">{trigger}</summary>
///   <div role="menu">
///     <button type="button" role="menuitem" tabindex="-1">{label}</button>
///     ...
///   </div>
/// </details>
/// ```
///
/// `id` is required so the WASM controller can attach.
#[derive(BasecoatProps, Default, Clone, Debug)]
pub struct DropdownProps {
    /// Unique DOM id — required for the WASM controller.
    #[prop(optional, into)]
    pub id: Option<Cow<'static, str>>,
    /// Trigger content (rendered inside `<summary>`).
    #[prop(optional)]
    pub trigger: Option<Markup>,
    /// Items rendered inside the menu container.
    #[prop(default)]
    pub items: Vec<DropdownItem>,
    /// Optional aria-label for the menu container.
    #[prop(optional, into)]
    pub menu_label: Option<Cow<'static, str>>,
    /// Floating placement string passed to `@floating-ui/dom`
    /// (default `"bottom-start"`).
    #[prop(optional, into)]
    pub placement: Option<Cow<'static, str>>,
    /// Extra CSS classes appended after `dropdown-menu`.
    #[prop(optional, into)]
    pub class: Option<Cow<'static, str>>,
    #[prop(extend)]
    pub attrs: AttrMap,
    pub children: Children,
}
