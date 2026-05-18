use crate::{AttrMap, BasecoatProps, Children};
use std::borrow::Cow;

/// A single option in a [`SelectProps::options`] list.
///
/// Mirrors a native `<option>` element: `value` is the form-submitted value,
/// `label` is the user-visible text, and `disabled` blocks selection.
#[derive(Clone, Debug, Default)]
pub struct SelectOption {
    /// The form-submitted value for the `<option>`.
    pub value: Cow<'static, str>,
    /// The user-visible label shown in the trigger and listbox.
    pub label: Cow<'static, str>,
    /// Whether this option is disabled.
    pub disabled: bool,
}

impl SelectOption {
    /// Construct a new enabled option.
    pub fn new(
        value: impl Into<Cow<'static, str>>,
        label: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }

    /// Mark this option as disabled.
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// Select — maps to CSS class `.select` (custom-styled native form select).
///
/// Renders a hidden native `<select>` (the source of truth for form
/// submission) alongside a visible trigger button and a floating listbox.
/// The controller wires the listbox open/close behaviour, keyboard
/// navigation, dismiss handling, and synchronizes the hidden element on
/// option click.
#[derive(BasecoatProps, Default, Clone, Debug)]
pub struct SelectProps {
    /// Unique DOM id for the trigger button — required for the controller
    /// to find the element. The hidden `<select>` and listbox derive their
    /// ids from this value.
    #[prop(optional, into)]
    pub id: Option<Cow<'static, str>>,
    /// Form name for the hidden native `<select>` element.
    #[prop(optional, into)]
    pub name: Option<Cow<'static, str>>,
    /// Accessible label associated via `aria-label` on the trigger.
    #[prop(optional, into)]
    pub label: Option<Cow<'static, str>>,
    /// Initial selected value. Must match one of the option values.
    #[prop(optional, into)]
    pub value: Option<Cow<'static, str>>,
    /// Placeholder text shown when no option is selected.
    #[prop(optional, into)]
    pub placeholder: Option<Cow<'static, str>>,
    /// Whether the entire select is disabled.
    #[prop(default = false)]
    pub disabled: bool,
    /// The list of selectable options.
    #[prop(default)]
    pub options: Vec<SelectOption>,
    /// Extra CSS classes appended to the `.select` wrapper.
    #[prop(optional, into)]
    pub class: Option<Cow<'static, str>>,
    /// Extra HTML attributes for the outer wrapper.
    #[prop(extend)]
    pub attrs: AttrMap,
    /// Optional inline children (rendered inside the listbox before the
    /// option buttons). Most callers should leave this empty and use
    /// `options` instead.
    pub children: Children,
}
