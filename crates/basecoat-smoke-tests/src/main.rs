//! Headless-browser smoke test for the Leptos full-stack example.
//!
//! Exercises all six runtime bugs caught during live debugging and two
//! additional interactive flows (reactive counter + tab switching).
//!
//! Run via:
//!   cargo xtask smoke
//!
//! Prerequisites:
//!   1. `cargo xtask build-wasm` (produces the WASM controllers bundle)
//!   2. `cargo-leptos` installed (`cargo install cargo-leptos`)
//!   3. A Chromium/Chrome binary reachable on PATH

use std::{
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

use anyhow::{Context, Result, bail};
use headless_chrome::{Browser, protocol::cdp::types::Event};

// ── entry point ───────────────────────────────────────────────────────────────

fn main() -> std::process::ExitCode {
    match run() {
        Ok(all_passed) => {
            if all_passed {
                std::process::ExitCode::SUCCESS
            } else {
                std::process::ExitCode::FAILURE
            }
        }
        Err(e) => {
            eprintln!("smoke: fatal error: {e:#}");
            std::process::ExitCode::from(2)
        }
    }
}

// ── top-level orchestration ───────────────────────────────────────────────────

fn run() -> Result<bool> {
    let repo_root = locate_repo_root()?;

    check_wasm_bundle_exists(&repo_root)?;

    // Detect cargo-leptos; skip gracefully if absent.
    if !cargo_leptos_available() {
        println!("SKIP: cargo-leptos not found on PATH — install with `cargo install cargo-leptos` to run this smoke test.");
        return Ok(true);
    }

    let mut server = spawn_leptos_server(&repo_root)?;

    // Ensure the server process is killed on exit, even on Ctrl-C.
    let server_pid = server.id();
    ctrlc::set_handler(move || {
        eprintln!("\nsmoke: interrupted — killing server (pid {server_pid})");
        // Best-effort kill; process may already be gone.
        let _ = Command::new("kill").arg(server_pid.to_string()).status();
        std::process::exit(130);
    })
    .ok();

    // Wait until the dev server responds (up to 120 s; first run downloads Tailwind).
    wait_for_server("http://127.0.0.1:3001/", Duration::from_secs(120))?;

    let result = run_browser_assertions();

    // Kill the server regardless of assertion outcome.
    let _ = server.kill();
    let _ = server.wait();

    result
}

// ── server lifecycle ──────────────────────────────────────────────────────────

fn cargo_leptos_available() -> bool {
    Command::new("which")
        .arg("cargo-leptos")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn spawn_leptos_server(repo_root: &Path) -> Result<Child> {
    let example_dir = repo_root.join("examples/leptos-fullstack");
    println!("==> spawning: cargo leptos serve  (cwd: {})", example_dir.display());

    let child = Command::new("cargo")
        .args(["leptos", "serve"])
        .current_dir(&example_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("failed to spawn `cargo leptos serve`")?;

    // Forward server output to our stdout/stderr in a background thread so we
    // can see build progress without blocking the poll loop.
    if let Some(stdout) = child.stdout.as_ref() {
        // We can't move stdout out of child here; drain will happen via poll loop below.
        // Instead we rely on the stderr pipe for the "watching" message.
        let _ = stdout;
    }

    Ok(child)
}

fn wait_for_server(url: &str, timeout: Duration) -> Result<()> {
    let deadline = Instant::now() + timeout;
    println!("==> waiting for server at {url}  (timeout: {}s)", timeout.as_secs());

    loop {
        if Instant::now() > deadline {
            bail!("timeout waiting for server at {url} — did `cargo leptos serve` compile successfully?");
        }

        let ready = Command::new("curl")
            .args(["--silent", "--fail", "--max-time", "2", url])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false);

        if ready {
            println!("==> server is up");
            return Ok(());
        }

        thread::sleep(Duration::from_secs(2));
    }
}

// ── repo / wasm checks ────────────────────────────────────────────────────────

fn locate_repo_root() -> Result<PathBuf> {
    // Walk up from CARGO_MANIFEST_DIR (set at compile time for our crate).
    let manifest: PathBuf = env!("CARGO_MANIFEST_DIR").into();
    // crates/basecoat-smoke-tests → ../../ = repo root
    let root = manifest
        .parent()
        .and_then(|p| p.parent())
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    Ok(root)
}

fn check_wasm_bundle_exists(repo_root: &Path) -> Result<()> {
    let wasm = repo_root.join("crates/basecoat-controllers/pkg/basecoat_controllers_bg.wasm");
    if !wasm.exists() {
        bail!(
            "WASM bundle not found at {}\n\
             Run `cargo xtask build-wasm` first.",
            wasm.display()
        );
    }
    Ok(())
}

// ── browser assertion suite ───────────────────────────────────────────────────

struct AssertionResult {
    name: &'static str,
    passed: bool,
    detail: String,
}

fn run_browser_assertions() -> Result<bool> {
    println!("==> launching headless Chrome");
    let browser = Browser::default().context("failed to launch headless Chrome — is Chromium/Chrome installed?")?;
    let tab = browser.new_tab().context("failed to open browser tab")?;

    // Capture console messages (error/warn) via the Log domain.
    // CDP Log.EntryAdded fires for console.error / console.warn output.
    tab.enable_log()
        .context("failed to enable Log domain")?;
    tab.enable_runtime()
        .context("failed to enable Runtime domain")?;

    let console_messages: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let console_messages_clone = Arc::clone(&console_messages);

    tab.add_event_listener(Arc::new(move |event: &Event| {
        if let Event::LogEntryAdded(ev) = event {
            let entry = &ev.params.entry;
            // Capture error and warning entries; these include console.error/warn.
            let level = format!("{:?}", entry.level);
            if level.contains("Error") || level.contains("Warning") {
                console_messages_clone
                    .lock()
                    .unwrap()
                    .push(entry.text.clone());
            }
        }
    }))
    .context("failed to register event listener")?;

    tab.navigate_to("http://127.0.0.1:3001")
        .context("failed to navigate to app")?;
    tab.wait_until_navigated()
        .context("failed to wait for navigation")?;

    // Brief pause for WASM initialisation and controller hydration.
    thread::sleep(Duration::from_millis(500));

    let mut results: Vec<AssertionResult> = Vec::new();

    // ── assertion 1: hydration_succeeded ─────────────────────────────────────
    // Catches bug #2: __basecoat_start ran too early → roots never hydrated.
    // v0.1 contributed Tabs, Dialog, Toaster (3). v0.2 adds Dropdown, Popover,
    // Select, Sidebar, Combobox (5) for a total of 8.
    results.push(assert_js_eq(
        &tab,
        "hydration_succeeded",
        "document.querySelectorAll('[data-basecoat-hydrated=\"true\"]').length",
        serde_json::json!(8),
    ));

    // ── assertion 2: window_basecoat_exposed ──────────────────────────────────
    // Catches bug #3: WASM never initialized → window.basecoat undefined.
    results.push(assert_js_eq(
        &tab,
        "window_basecoat_exposed",
        "typeof window.basecoat?.hydrate + ',' + typeof window.basecoat?.toast",
        serde_json::json!("function,function"),
    ));

    // ── assertion 3: no_panic_in_console ─────────────────────────────────────
    // Catches bug #1: hydration panic → WASM runtime dies.
    {
        let messages = console_messages.lock().unwrap().clone();
        let bad: Vec<&String> = messages
            .iter()
            .filter(|m| {
                let lower = m.to_lowercase();
                lower.contains("hydration") || lower.contains("panicked")
            })
            .collect();
        let passed = bad.is_empty();
        let detail = if passed {
            format!("no panic/hydration errors in {} console message(s)", messages.len())
        } else {
            format!("found {} bad console message(s): {:?}", bad.len(), bad)
        };
        results.push(AssertionResult {
            name: "no_panic_in_console",
            passed,
            detail,
        });
    }

    // ── assertion 4: counter_reactive ────────────────────────────────────────
    // Verifies the Leptos reactive counter works end-to-end.
    {
        let initial = eval_string(
            &tab,
            "Array.from(document.querySelectorAll('button')).find(b => b.textContent.trim().includes('Clicked'))?.textContent.trim() ?? 'NOT FOUND'",
        );
        let initial_ok = initial.as_deref() == Some("Clicked 0 times");

        // Click the counter button.
        let click_result = tab.evaluate(
            "(() => { const b = Array.from(document.querySelectorAll('button')).find(btn => btn.textContent.trim().includes('Clicked')); if (b) { b.click(); return true; } return false; })();",
            false,
        );

        thread::sleep(Duration::from_millis(100));

        let after = eval_string(
            &tab,
            "Array.from(document.querySelectorAll('button')).find(b => b.textContent.trim().includes('Clicked'))?.textContent.trim() ?? 'NOT FOUND'",
        );
        let after_ok = after.as_deref() == Some("Clicked 1 times");

        let passed = initial_ok && click_result.is_ok() && after_ok;
        let detail = format!(
            "initial={:?} click_ok={} after={:?}",
            initial,
            click_result.is_ok(),
            after,
        );
        results.push(AssertionResult {
            name: "counter_reactive",
            passed,
            detail,
        });
    }

    // ── assertion 5: tabs_switch ──────────────────────────────────────────────
    // Verifies the Tabs controller hydrates correctly.
    {
        // Account tab should be selected initially.
        let account_initial = eval_string(
            &tab,
            "document.querySelector('[role=\"tab\"][aria-controls=\"tab-account\"]')?.getAttribute('aria-selected') ?? 'NOT FOUND'",
        );
        let account_initial_ok = account_initial.as_deref() == Some("true");

        // Click the Security tab.
        let _ = tab.evaluate(
            "document.querySelector('[role=\"tab\"][aria-controls=\"tab-security\"]')?.click();",
            false,
        );
        thread::sleep(Duration::from_millis(150));

        let security_after = eval_string(
            &tab,
            "document.querySelector('[role=\"tab\"][aria-controls=\"tab-security\"]')?.getAttribute('aria-selected') ?? 'NOT FOUND'",
        );
        let account_after = eval_string(
            &tab,
            "document.querySelector('[role=\"tab\"][aria-controls=\"tab-account\"]')?.getAttribute('aria-selected') ?? 'NOT FOUND'",
        );
        let panel_text = eval_string(
            &tab,
            "document.getElementById('tab-security')?.textContent?.trim() ?? 'NOT FOUND'",
        );

        let security_ok = security_after.as_deref() == Some("true");
        let account_deselected = account_after.as_deref() == Some("false");
        // The security panel contains "Password" (from "Password & 2FA.")
        let panel_ok = panel_text
            .as_deref()
            .map(|t| t.contains("Password"))
            .unwrap_or(false);

        let passed = account_initial_ok && security_ok && account_deselected && panel_ok;
        let detail = format!(
            "account_initial={:?} security_after={:?} account_after={:?} panel={:?}",
            account_initial, security_after, account_after, panel_text,
        );
        results.push(AssertionResult {
            name: "tabs_switch",
            passed,
            detail,
        });
    }

    // ── assertion 6: dialog_opens ────────────────────────────────────────────
    // Catches bug #5: dialog controller couldn't find trigger → dialog stayed closed.
    {
        let _ = tab.evaluate(
            "document.querySelector('[data-dialog-trigger=\"demo-dialog\"]')?.click();",
            false,
        );
        thread::sleep(Duration::from_millis(150));

        let open = eval_bool(&tab, "document.getElementById('demo-dialog')?.open ?? false");
        let passed = open == Some(true);
        let detail = format!("dialog.open={:?}", open);
        results.push(AssertionResult {
            name: "dialog_opens",
            passed,
            detail,
        });
    }

    // Close the dialog opened by the previous assertion so it doesn't intercept
    // clicks on the v0.2 components rendered below it.
    let _ = tab.evaluate(
        "document.getElementById('demo-dialog')?.close();",
        false,
    );
    thread::sleep(Duration::from_millis(100));

    // ── assertion 7: dropdown_opens ──────────────────────────────────────────
    // The dropdown root is a <details>; clicking the <summary> toggles `open`
    // and the controller mirrors aria-expanded onto the summary.
    {
        let _ = tab.evaluate(
            "document.querySelector('#demo-dropdown > summary')?.click();",
            false,
        );
        thread::sleep(Duration::from_millis(150));

        let open = eval_bool(&tab, "document.getElementById('demo-dropdown')?.open ?? false");
        let expanded = eval_string(
            &tab,
            "document.querySelector('#demo-dropdown > summary')?.getAttribute('aria-expanded') ?? 'NOT FOUND'",
        );
        let menu_visible = eval_bool(
            &tab,
            "(() => { const m = document.querySelector('#demo-dropdown [role=\"menu\"]'); if (!m) return false; return !m.hasAttribute('hidden'); })()",
        );

        let passed = open == Some(true)
            && expanded.as_deref() == Some("true")
            && menu_visible == Some(true);
        let detail = format!(
            "open={:?} aria-expanded={:?} menu_visible={:?}",
            open, expanded, menu_visible,
        );
        results.push(AssertionResult {
            name: "dropdown_opens",
            passed,
            detail,
        });

        // Close the dropdown so it doesn't shadow subsequent clicks.
        let _ = tab.evaluate(
            "(() => { const d = document.getElementById('demo-dropdown'); if (d) d.open = false; })();",
            false,
        );
        thread::sleep(Duration::from_millis(100));
    }

    // ── assertion 8: popover_opens ───────────────────────────────────────────
    // Same <details>-based pattern as dropdown — click summary → open=true.
    {
        let _ = tab.evaluate(
            "document.querySelector('#demo-popover > summary')?.click();",
            false,
        );
        thread::sleep(Duration::from_millis(150));

        let open = eval_bool(&tab, "document.getElementById('demo-popover')?.open ?? false");
        let dialog_present = eval_bool(
            &tab,
            "!!document.querySelector('#demo-popover [role=\"dialog\"]')",
        );

        let passed = open == Some(true) && dialog_present == Some(true);
        let detail = format!(
            "open={:?} dialog_present={:?}",
            open, dialog_present,
        );
        results.push(AssertionResult {
            name: "popover_opens",
            passed,
            detail,
        });

        let _ = tab.evaluate(
            "(() => { const d = document.getElementById('demo-popover'); if (d) d.open = false; })();",
            false,
        );
        thread::sleep(Duration::from_millis(100));
    }

    // ── assertion 9: select_changes_value ────────────────────────────────────
    // Open the listbox, click the second option, then verify the hidden native
    // <select>'s value matches that option's data-value.
    {
        let _ = tab.evaluate(
            "document.querySelector('#demo-select [data-select-trigger]')?.click();",
            false,
        );
        thread::sleep(Duration::from_millis(150));

        let _ = tab.evaluate(
            "(() => { const opts = document.querySelectorAll('#demo-select [role=\"listbox\"] [role=\"option\"]'); if (opts[1]) opts[1].click(); })();",
            false,
        );
        thread::sleep(Duration::from_millis(150));

        let native_value = eval_string(
            &tab,
            "document.querySelector('#demo-select select[data-select-native]')?.value ?? 'NOT FOUND'",
        );

        let passed = native_value.as_deref() == Some("banana");
        let detail = format!("native_value={:?}", native_value);
        results.push(AssertionResult {
            name: "select_changes_value",
            passed,
            detail,
        });
    }

    // ── assertion 10: sidebar_toggle ─────────────────────────────────────────
    // Click the sibling toggle button → aside data-state flips.
    {
        let initial_state = eval_string(
            &tab,
            "document.getElementById('demo-sidebar')?.getAttribute('data-state') ?? 'NOT FOUND'",
        );

        let _ = tab.evaluate(
            "document.querySelector('[data-sidebar-toggle=\"demo-sidebar\"]')?.click();",
            false,
        );
        thread::sleep(Duration::from_millis(150));

        let after_state = eval_string(
            &tab,
            "document.getElementById('demo-sidebar')?.getAttribute('data-state') ?? 'NOT FOUND'",
        );

        let flipped = matches!(
            (initial_state.as_deref(), after_state.as_deref()),
            (Some("expanded"), Some("collapsed")) | (Some("collapsed"), Some("expanded"))
        );
        let detail = format!(
            "initial={:?} after={:?}",
            initial_state, after_state,
        );
        results.push(AssertionResult {
            name: "sidebar_toggle",
            passed: flipped,
            detail,
        });
    }

    // ── assertion 11: combobox_filters ───────────────────────────────────────
    // Type two characters → number of visible options is strictly less than total.
    {
        let total = eval_string(
            &tab,
            "String(document.querySelectorAll('#demo-combobox [role=\"option\"]').length)",
        );

        // Focus the input, then dispatch a real `input` event after setting the value
        // so the controller's listener runs the filter.
        let _ = tab.evaluate(
            "(() => { const i = document.querySelector('#demo-combobox input[role=\"combobox\"]'); if (!i) return; i.focus(); i.value = 'le'; i.dispatchEvent(new Event('input', { bubbles: true })); })();",
            false,
        );
        thread::sleep(Duration::from_millis(200));

        let visible = eval_string(
            &tab,
            "String(Array.from(document.querySelectorAll('#demo-combobox [role=\"option\"]')).filter(o => !o.hasAttribute('hidden') && o.offsetParent !== null).length)",
        );

        let total_n: usize = total.as_deref().and_then(|s| s.parse().ok()).unwrap_or(0);
        let visible_n: usize = visible.as_deref().and_then(|s| s.parse().ok()).unwrap_or(0);

        let passed = total_n > 0 && visible_n < total_n;
        let detail = format!(
            "total={:?} visible={:?}",
            total, visible,
        );
        results.push(AssertionResult {
            name: "combobox_filters",
            passed,
            detail,
        });
    }

    // ── assertion 12: toast_created_and_classed ──────────────────────────────
    // Catches bug #4: no [data-toaster] found; and bug #6: toast missing class="toast".
    {

        // Click the Show Toast button (found by text content).
        let _ = tab.evaluate(
            "Array.from(document.querySelectorAll('button')).find(b => b.textContent.trim() === 'Show Toast')?.click();",
            false,
        );
        thread::sleep(Duration::from_millis(300));

        let toast_exists = eval_bool(
            &tab,
            "!!document.querySelector('#global-toaster [data-toast]')",
        );
        let toast_class = eval_string(
            &tab,
            "document.querySelector('#global-toaster [data-toast]')?.className ?? 'NOT FOUND'",
        );

        let exists_ok = toast_exists == Some(true);
        let class_ok = toast_class.as_deref() == Some("toast");

        let passed = exists_ok && class_ok;
        let detail = format!(
            "toast_exists={:?} toast.className={:?}",
            toast_exists, toast_class,
        );
        results.push(AssertionResult {
            name: "toast_created_and_classed",
            passed,
            detail,
        });
    }

    // ── summary ───────────────────────────────────────────────────────────────
    println!("\n==> Smoke test results:");
    let mut all_passed = true;
    for r in &results {
        let marker = if r.passed { "PASS" } else { "FAIL" };
        println!("  [{marker}] {:<35} {}", r.name, r.detail);
        if !r.passed {
            all_passed = false;
        }
    }

    let total = results.len();
    let passed = results.iter().filter(|r| r.passed).count();
    println!("\n==> {passed}/{total} assertions passed.");

    Ok(all_passed)
}

// ── JS evaluation helpers ─────────────────────────────────────────────────────

fn eval_string(tab: &headless_chrome::browser::tab::Tab, js: &str) -> Option<String> {
    let obj = tab.evaluate(js, false).ok()?;
    obj.value.and_then(|v| {
        if let serde_json::Value::String(s) = v {
            Some(s)
        } else {
            None
        }
    })
}

fn eval_bool(tab: &headless_chrome::browser::tab::Tab, js: &str) -> Option<bool> {
    let obj = tab.evaluate(js, false).ok()?;
    obj.value.and_then(|v| {
        if let serde_json::Value::Bool(b) = v {
            Some(b)
        } else {
            None
        }
    })
}

fn assert_js_eq(
    tab: &headless_chrome::browser::tab::Tab,
    name: &'static str,
    js: &str,
    expected: serde_json::Value,
) -> AssertionResult {
    match tab.evaluate(js, false) {
        Err(e) => AssertionResult {
            name,
            passed: false,
            detail: format!("evaluate error: {e}"),
        },
        Ok(obj) => {
            let actual = obj.value.unwrap_or(serde_json::Value::Null);
            let passed = actual == expected;
            let detail = if passed {
                format!("= {actual}")
            } else {
                format!("expected {expected} got {actual}")
            };
            AssertionResult { name, passed, detail }
        }
    }
}
