//! CSR smoke test — verifies that all components compile under the `csr` feature.
//!
//! No runtime assertions: this file only exercises that the types resolve and
//! the `view!` macro accepts the components. The actual DOM mounting is a
//! Phase 3 headless-browser concern.
//!
//! Build with: `cargo check -p basecoat-leptos` (default = csr)

use basecoat_core::{AlertVariant, BadgeVariant, ButtonSize, ButtonVariant};
use basecoat_leptos::*;
use leptos::prelude::*;

/// This function is never called — it exists solely so the compiler checks that
/// every component builds under CSR and produces `impl IntoView`.
#[allow(dead_code)]
fn _csr_type_check() {
    let _ = view! {
        <>
            <Button>"Hi"</Button>
            <Button variant=ButtonVariant::Outline size=ButtonSize::Sm>"Save"</Button>
            <Badge>"New"</Badge>
            <Badge variant=BadgeVariant::Destructive>"Error"</Badge>
            <Alert>"Info message"</Alert>
            <Alert variant=AlertVariant::Destructive>"Critical"</Alert>
            <Card>"Card body"</Card>
            <Label>"Field label"</Label>
            <Input />
            <Textarea />
            <Separator />
            <Dialog id="d1" title="Test Dialog">"Content"</Dialog>
            <DialogTrigger target="d1">"Open"</DialogTrigger>
            <DialogContent id="d1">
                <DialogHeader title="Hello" />
                <DialogFooter>"Footer"</DialogFooter>
            </DialogContent>
            <Tabs id="t1">
                <TabsList>
                    <TabsTab controls="p1" selected=true>"One"</TabsTab>
                </TabsList>
                <TabsPanel id="p1" selected=true>"Panel"</TabsPanel>
            </Tabs>
            <Toaster>
                <Toast title="Done" />
            </Toaster>
            <Tooltip content="Tip">
                <button type="button">"Trigger"</button>
            </Tooltip>
        </>
    };
}
