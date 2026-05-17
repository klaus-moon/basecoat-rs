use basecoat_core::{
    BadgeProps as CoreBadgeProps, BadgeVariant, ButtonProps as CoreButtonProps, ButtonVariant,
    classes,
};
use basecoat_rs::leptos::{
    DialogContent, DialogTrigger, Tabs, TabsList, TabsPanel, TabsTab, Toaster,
};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::Router;

/// The root application component. Runs identically under SSR and hydration.
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        // cargo-leptos compiles style/tailwind.css (which imports basecoat-css)
        // and serves the result alongside style/main.css as /pkg/leptos-fullstack.css.
        // No external CDN needed — Tailwind v4 and basecoat are bundled.
        <Stylesheet id="leptos" href="/pkg/leptos-fullstack.css"/>
        <Script type_="module" src="/static/basecoat-controllers.init.js"/>
        <Router>
            <main class="p-8 max-w-2xl mx-auto space-y-8">
                <h1 class="text-3xl font-bold">"basecoat-rs \u{00b7} Leptos full-stack"</h1>

                <section class="space-y-3">
                    <h2 class="text-xl font-semibold">"Buttons (reactive)"</h2>
                    <CounterDemo/>
                </section>

                <section class="space-y-3">
                    <h2 class="text-xl font-semibold">"Tabs (hydrated controller)"</h2>
                    // Tabs: TabsTab controls= must match TabsPanel id=
                    <Tabs id="demo-tabs">
                        <TabsList>
                            <TabsTab controls="tab-account" selected=true>"Account"</TabsTab>
                            <TabsTab controls="tab-security">"Security"</TabsTab>
                        </TabsList>
                        <TabsPanel id="tab-account" selected=true>
                            <p>"Account settings."</p>
                        </TabsPanel>
                        <TabsPanel id="tab-security">
                            <p>"Password & 2FA."</p>
                        </TabsPanel>
                    </Tabs>
                </section>

                <section class="space-y-3">
                    <h2 class="text-xl font-semibold">"Dialog (hydrated controller)"</h2>
                    // DialogTrigger renders its own <button>; pass text children
                    // directly (a Button component inside would produce invalid
                    // nested <button><button></button></button> markup that
                    // browsers auto-fix, breaking hydration).
                    <DialogTrigger target="demo-dialog">"Open Dialog"</DialogTrigger>
                    <DialogContent id="demo-dialog">
                        <p>"This dialog is server-rendered then hydrated by the WASM controller."</p>
                    </DialogContent>
                </section>

                <section class="space-y-3">
                    <h2 class="text-xl font-semibold">"Toast (JavaScript API)"</h2>
                    <button
                        class="btn-outline"
                        on:click=move |_| show_toast(
                            "Hello!",
                            "Toast from Leptos.",
                            "success",
                        )
                    >
                        "Show Toast"
                    </button>
                    // Toaster requires children; the WASM controller appends
                    // toasts dynamically so we start with an invisible span.
                    <Toaster id="global-toaster">
                        <span class="sr-only">"Toast container"</span>
                    </Toaster>
                </section>
            </main>
        </Router>
    }
}

/// Calls `window.basecoat.toast({...})` from a Leptos event handler. Leptos
/// strips inline `attr:onclick="..."` HTML attributes (XSS guard), so we go
/// through the exposed JS API via wasm-bindgen.
///
/// Server build is a no-op — this code path runs only on the hydrate client.
#[cfg(feature = "hydrate")]
fn show_toast(title: &str, description: &str, category: &str) {
    use wasm_bindgen::{JsCast, JsValue};
    let Some(window) = web_sys::window() else {
        return;
    };
    let opts = js_sys::Object::new();
    let _ = js_sys::Reflect::set(&opts, &"title".into(), &title.into());
    let _ = js_sys::Reflect::set(&opts, &"description".into(), &description.into());
    let _ = js_sys::Reflect::set(&opts, &"category".into(), &category.into());
    let Ok(basecoat) = js_sys::Reflect::get(&window, &"basecoat".into()) else {
        return;
    };
    let Ok(toast_fn) = js_sys::Reflect::get(&basecoat, &"toast".into()) else {
        return;
    };
    if let Ok(func) = toast_fn.dyn_into::<js_sys::Function>() {
        let _ = func.call1(&JsValue::NULL, &opts.into());
    }
}

#[cfg(not(feature = "hydrate"))]
fn show_toast(_title: &str, _description: &str, _category: &str) {}

/// A reactive counter demo that increments on button click.
///
/// Renders a native `<button>` and `<span>` directly, applying class strings
/// from `basecoat_core::classes`. Using the `basecoat_rs::leptos::Button` /
/// `Badge` wrappers with reactive closure children currently triggers a Leptos
/// hydration mismatch (`button.rs:34` — expected text node, found element).
/// Tracked for v0.2 of `basecoat-leptos`; native HTML + shared class strings
/// is the supported pattern for reactive content today.
#[component]
fn CounterDemo() -> impl IntoView {
    let (count, set_count) = signal(0i32);
    let btn_class = classes::button(&CoreButtonProps {
        variant: ButtonVariant::Primary,
        ..Default::default()
    });
    let badge_class = classes::badge(&CoreBadgeProps {
        variant: BadgeVariant::Secondary,
        ..Default::default()
    });
    view! {
        <div class="flex gap-3 items-center">
            <button
                class=btn_class
                on:click=move |_| set_count.update(|n| *n += 1)
            >
                {move || format!("Clicked {} times", count.get())}
            </button>
            <span class=badge_class>
                {move || count.get().to_string()}
            </span>
        </div>
    }
}

/// SSR shell — wraps the App in a full HTML document with hydration scripts.
#[cfg(feature = "ssr")]
pub fn shell(options: leptos::config::LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}
