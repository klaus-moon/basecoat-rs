use basecoat_core::classes::select::select as select_class;
use basecoat_core::props::select::SelectProps as CoreProps;
use leptos::prelude::*;
use std::borrow::Cow;

/// A single option in a [`Select`] listbox.
///
/// Mirrors `basecoat_core::props::select::SelectOption` with leptos-friendly
/// construction.
#[derive(Clone, Debug)]
pub struct SelectOption {
    /// The form-submitted value.
    pub value: Cow<'static, str>,
    /// The user-visible label.
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

/// Leptos wrapper for the basecoat Select component.
///
/// Emits a wrapping `<div class="select">` with `data-basecoat-hydrate="select"`
/// and `data-basecoat-version="0.2"` so the WASM controller can attach. The
/// wrapper contains a hidden native `<select>` (the source of truth for form
/// submission), a visible trigger button, and a floating listbox of options.
///
/// The `id` is required for the controller to wire trigger/listbox together.
#[component]
pub fn Select(
    /// Unique DOM id for the trigger button — required for the controller.
    #[prop(optional, into)]
    id: Option<Cow<'static, str>>,
    /// Form name for the hidden native `<select>` element.
    #[prop(optional, into)]
    name: Option<Cow<'static, str>>,
    /// Accessible label for the trigger button.
    #[prop(optional, into)]
    label: Option<Cow<'static, str>>,
    /// Initial selected value.
    #[prop(optional, into)]
    value: Option<Cow<'static, str>>,
    /// Placeholder shown when no option is selected.
    #[prop(optional, into)]
    placeholder: Option<Cow<'static, str>>,
    /// Whether the select is disabled.
    #[prop(default = false)]
    disabled: bool,
    /// The list of selectable options.
    #[prop(optional, into)]
    options: Vec<SelectOption>,
    /// Extra CSS classes appended to the `.select` wrapper.
    #[prop(optional, into)]
    class: Option<Signal<Cow<'static, str>>>,
) -> impl IntoView {
    let class_str = Signal::derive(move || {
        let extra = class.as_ref().map(|s| s.get());
        let props = CoreProps {
            class: extra,
            ..Default::default()
        };
        select_class(&props)
    });

    let id_owned = id.clone().unwrap_or(Cow::Borrowed("select-1"));
    let wrapper_id = format!("{id_owned}-root");
    let native_id = format!("{id_owned}-native");
    let listbox_id = format!("{id_owned}-listbox");

    let selected_value: Cow<'static, str> = value.clone().unwrap_or_default();
    let selected_value_for_lookup = selected_value.clone();
    let placeholder_text: Cow<'static, str> = placeholder
        .clone()
        .unwrap_or(Cow::Borrowed("Select an option"));

    let selected_label = options
        .iter()
        .find(|o| o.value == selected_value_for_lookup)
        .map(|o| o.label.clone())
        .unwrap_or(placeholder_text);

    let trigger_id_for_listbox = id_owned.to_string();
    let listbox_id_for_trigger = listbox_id.clone();

    // Pre-compute per-option flags once so we can render the native <option>
    // list and the listbox <button> list without cloning the options vec.
    let rendered: Vec<(SelectOption, bool)> = options
        .into_iter()
        .map(|o| {
            let is_selected = o.value == selected_value;
            (o, is_selected)
        })
        .collect();

    let native_options: Vec<_> = rendered
        .iter()
        .map(|(o, is_selected)| {
            view! {
                <option
                    value=o.value.to_string()
                    selected=*is_selected
                    disabled=o.disabled
                >
                    {o.label.to_string()}
                </option>
            }
        })
        .collect();

    let listbox_options: Vec<_> = rendered
        .into_iter()
        .map(|(o, is_selected)| {
            view! {
                <button
                    type="button"
                    role="option"
                    data-value=o.value.to_string()
                    aria-selected=if is_selected { "true" } else { "false" }
                    aria-disabled=if o.disabled { Some("true") } else { None::<&str> }
                    disabled=o.disabled
                    tabindex=if is_selected { "0" } else { "-1" }
                >
                    {o.label.to_string()}
                </button>
            }
        })
        .collect();

    view! {
        <div
            class=class_str
            id=wrapper_id
            data-basecoat-hydrate="select"
            data-basecoat-version="0.2"
        >
            <select
                class="select-native"
                hidden
                id=native_id
                name=name.map(|s| s.to_string())
                disabled=disabled
                data-select-native
            >
                {native_options}
            </select>
            <button
                type="button"
                class="select-trigger"
                id=id_owned.to_string()
                data-select-trigger
                aria-haspopup="listbox"
                aria-expanded="false"
                aria-controls=listbox_id_for_trigger
                aria-label=label.map(|s| s.to_string())
                disabled=disabled
            >
                <span data-select-value>{selected_label.to_string()}</span>
            </button>
            <div
                id=listbox_id
                role="listbox"
                data-select-listbox
                tabindex="-1"
                hidden
                aria-labelledby=trigger_id_for_listbox
            >
                {listbox_options}
            </div>
        </div>
    }
}
