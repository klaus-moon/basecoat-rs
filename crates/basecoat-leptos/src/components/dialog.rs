use basecoat_core::{DialogProps as CoreProps, classes};
use leptos::prelude::*;
use std::borrow::Cow;

/// Leptos wrapper for the basecoat Dialog component.
///
/// Emits a native `<dialog>` element with the canonical `.dialog` class and the
/// `data-basecoat-hydrate="dialog"` + `data-basecoat-version="0.1"` markers so
/// the WASM controller (`basecoat-controllers`) can attach.
///
/// The `id` is **required** for the controller to find the element.
///
/// ## Compound components
///
/// For fine-grained control, use the sub-components:
/// - [`DialogTrigger`] — the button that opens the dialog
/// - [`DialogContent`] — the inner content wrapper
/// - [`DialogHeader`] — header slot (title + description)
/// - [`DialogFooter`] — footer slot
#[component]
pub fn Dialog(
    /// Unique DOM id — required for the WASM controller.
    #[prop(optional, into)]
    id: Option<Cow<'static, str>>,
    /// Dialog title (rendered in `<header>`).
    #[prop(optional, into)]
    title: Option<Cow<'static, str>>,
    /// Dialog description (rendered below title).
    #[prop(optional, into)]
    description: Option<Cow<'static, str>>,
    /// Whether to render a close button (default true).
    #[prop(default = true)]
    close_button: bool,
    /// Extra CSS classes.
    #[prop(optional, into)]
    class: Option<Signal<Cow<'static, str>>>,
    children: Children,
) -> impl IntoView {
    let class_str = Signal::derive(move || {
        let extra = class.as_ref().map(|s| s.get());
        let props = CoreProps {
            class: extra,
            ..Default::default()
        };
        classes::dialog(&props)
    });

    let id_attr = id.clone();
    let trigger_id = id.map(|s| format!("{s}-trigger"));

    view! {
        <>
            {trigger_id.map(|tid| view! {
                <button
                    class="btn-outline"
                    data-dialog-trigger=tid
                    type="button"
                >
                    "Open"
                </button>
            })}
            <dialog
                class=class_str
                id=id_attr
                data-basecoat-hydrate="dialog"
                data-basecoat-version="0.1"
            >
                <div class="dialog-content">
                    {title.map(|t| view! {
                        <header class="dialog-header">
                            <h2 class="dialog-title">{t}</h2>
                            {description.map(|d| view! {
                                <p class="dialog-description">{d}</p>
                            })}
                        </header>
                    })}
                    <div class="dialog-body">
                        {children()}
                    </div>
                    {close_button.then(|| view! {
                        <button
                            class="dialog-close"
                            data-dialog-close=""
                            type="button"
                            aria-label="Close"
                        >
                            "×"
                        </button>
                    })}
                </div>
            </dialog>
        </>
    }
}

/// The trigger button for a Dialog — opens the dialog with the given `target` id.
#[component]
pub fn DialogTrigger(
    /// The `id` of the `<dialog>` to open.
    #[prop(into)]
    target: Cow<'static, str>,
    children: Children,
) -> impl IntoView {
    view! {
        <button
            class="btn-outline"
            data-dialog-trigger=target.to_string()
            type="button"
        >
            {children()}
        </button>
    }
}

/// The content container inside a Dialog (the `<dialog>` element itself).
///
/// Use this with [`DialogTrigger`] for fully custom dialog composition.
#[component]
pub fn DialogContent(
    /// Unique DOM id — required for the WASM controller.
    #[prop(optional, into)]
    id: Option<Cow<'static, str>>,
    /// Extra CSS classes.
    #[prop(optional, into)]
    class: Option<Signal<Cow<'static, str>>>,
    children: Children,
) -> impl IntoView {
    let class_str = Signal::derive(move || {
        let extra = class.as_ref().map(|s| s.get());
        let props = CoreProps {
            class: extra,
            ..Default::default()
        };
        classes::dialog(&props)
    });
    view! {
        <dialog
            class=class_str
            id=id.map(|s| s.to_string())
            data-basecoat-hydrate="dialog"
            data-basecoat-version="0.1"
        >
            <div class="dialog-content">
                {children()}
            </div>
        </dialog>
    }
}

/// Header slot for a Dialog (title + optional description).
#[component]
pub fn DialogHeader(
    /// Dialog title text.
    #[prop(into)]
    title: Cow<'static, str>,
    /// Optional description below the title.
    #[prop(optional, into)]
    description: Option<Cow<'static, str>>,
) -> impl IntoView {
    view! {
        <header class="dialog-header">
            <h2 class="dialog-title">{title.to_string()}</h2>
            {description.map(|d| view! {
                <p class="dialog-description">{d}</p>
            })}
        </header>
    }
}

/// Footer slot for a Dialog.
#[component]
pub fn DialogFooter(children: Children) -> impl IntoView {
    view! { <footer class="dialog-footer">{children()}</footer> }
}
