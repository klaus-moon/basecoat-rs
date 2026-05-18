use crate::{AttrMap, BasecoatProps, Children};
use std::borrow::Cow;

/// A single option in a Combobox listbox.
#[derive(Clone, Debug, Default)]
pub struct ComboboxOption {
    /// Submitted form value for the option.
    pub value: Cow<'static, str>,
    /// User-visible label rendered inside the option.
    pub label: Cow<'static, str>,
}

impl ComboboxOption {
    pub fn new(
        value: impl Into<Cow<'static, str>>,
        label: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
        }
    }
}

/// Combobox — input with a filtered listbox of suggestions.
///
/// Reuses the upstream `.select` CSS class for the wrapper because Basecoat
/// styles `input[role=combobox]` through the `.select` layer.
///
/// The DOM contract emitted by the matching component function is:
///
/// ```html
/// <div class="select" id="{id}" data-combobox
///      data-basecoat-hydrate="combobox"
///      data-basecoat-version="0.2">
///   <input type="text" role="combobox"
///          aria-controls="{id}-listbox"
///          aria-expanded="false"
///          aria-autocomplete="list"
///          autocomplete="off"
///          data-combobox-input>
///   <div role="listbox" id="{id}-listbox" data-combobox-listbox hidden>
///     <button type="button" role="option"
///             id="{id}-option-{idx}"
///             data-value="{value}"
///             tabindex="-1">{label}</button>
///     ...
///   </div>
/// </div>
/// ```
#[derive(BasecoatProps, Default, Clone, Debug)]
pub struct ComboboxProps {
    /// Unique DOM id — required for the WASM controller to wire ARIA and the
    /// listbox association.
    #[prop(optional, into)]
    pub id: Option<Cow<'static, str>>,
    /// Form field name for the hidden value.
    #[prop(optional, into)]
    pub name: Option<Cow<'static, str>>,
    /// Placeholder text rendered in the empty input.
    #[prop(optional, into)]
    pub placeholder: Option<Cow<'static, str>>,
    /// Initial input value (rendered as `value="..."`).
    #[prop(optional, into)]
    pub value: Option<Cow<'static, str>>,
    /// Static set of options. Dynamic loading is planned for a future release.
    #[prop(default)]
    pub options: Vec<ComboboxOption>,
    /// Disable the input element.
    #[prop(default)]
    pub disabled: bool,
    /// Extra CSS classes appended to the canonical `.select` class.
    #[prop(optional, into)]
    pub class: Option<Cow<'static, str>>,
    #[prop(extend)]
    pub attrs: AttrMap,
    /// Slot reserved for downstream extensions; the bundled component
    /// function ignores `children` and renders options from `options`.
    pub children: Children,
}
