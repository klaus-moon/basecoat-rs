//! xtask — build automation for basecoat-rs.
//!
//! Usage:
//!   cargo xtask build-wasm   Build the WASM controllers bundle via wasm-pack
//!   cargo xtask check        Run check + test + clippy across the workspace
//!   cargo xtask smoke        Run the headless-browser smoke test (requires cargo-leptos)

use std::process::{Command, ExitCode};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let subcommand = args.get(1).map(String::as_str).unwrap_or("help");

    match subcommand {
        "build-wasm" => cmd_build_wasm(),
        "check" => cmd_check(),
        "smoke" => cmd_smoke(),
        _ => {
            eprintln!("Usage: cargo xtask <subcommand>");
            eprintln!("  build-wasm   Build WASM controllers via wasm-pack");
            eprintln!("  check        cargo check + test + clippy (workspace)");
            eprintln!("  smoke        Headless-browser end-to-end smoke test");
            ExitCode::FAILURE
        }
    }
}

fn cmd_smoke() -> ExitCode {
    // Ensure the WASM bundle exists before launching the smoke test; the
    // server can serve cached old bytes otherwise.
    let wasm = "crates/basecoat-controllers/pkg/basecoat_controllers_bg.wasm";
    if !std::path::Path::new(wasm).exists() {
        println!("==> {wasm} not found — running build-wasm first");
        let result = cmd_build_wasm();
        if result != ExitCode::SUCCESS {
            return result;
        }
    }

    println!("==> cargo run -p basecoat-smoke-tests --bin smoke --release");
    let status = Command::new("cargo")
        .args(["run", "-p", "basecoat-smoke-tests", "--bin", "smoke", "--release"])
        .status();
    match status {
        Err(e) => {
            eprintln!("error: could not run smoke binary: {e}");
            ExitCode::FAILURE
        }
        Ok(s) if !s.success() => ExitCode::FAILURE,
        Ok(_) => ExitCode::SUCCESS,
    }
}

// ── subcommands ───────────────────────────────────────────────────────────────

/// Bootstrap shim written to `pkg/` after every wasm-pack build.
///
/// wasm-pack --target web emits an ES module that must be explicitly
/// initialized. Importing this shim from a `<script type="module">` tag is
/// enough to bring up the WASM controllers.
const INIT_JS_SHIM: &str = "\
// Bootstrap shim for basecoat-controllers.\n\
// wasm-pack --target web emits an ES module that must be explicitly\n\
// initialized. Importing this shim from a <script type=\"module\"> tag is\n\
// enough to bring up the WASM controllers.\n\
import init from './basecoat_controllers.js';\n\
init().catch(err => console.error('[basecoat] failed to initialize WASM controllers:', err));\n\
";

fn cmd_build_wasm() -> ExitCode {
    println!("==> wasm-pack build --release --target web crates/basecoat-controllers");
    let status = Command::new("wasm-pack")
        .args([
            "build",
            "--release",
            "--target",
            "web",
            "crates/basecoat-controllers",
        ])
        .status();

    match status {
        Err(e) => {
            eprintln!("error: wasm-pack not found or could not run: {e}");
            eprintln!("Install it with: cargo install wasm-pack");
            return ExitCode::FAILURE;
        }
        Ok(s) if !s.success() => {
            eprintln!("error: wasm-pack build failed");
            return ExitCode::FAILURE;
        }
        Ok(_) => {}
    }

    // Write the init shim deterministically after every build.
    let shim_path = "crates/basecoat-controllers/pkg/basecoat-controllers.init.js";
    match std::fs::write(shim_path, INIT_JS_SHIM) {
        Ok(()) => println!("==> wrote {shim_path}"),
        Err(e) => {
            eprintln!("error: could not write init shim: {e}");
            return ExitCode::FAILURE;
        }
    }

    // Report gzipped size of the output .wasm file.
    let wasm_path = "crates/basecoat-controllers/pkg/basecoat_controllers_bg.wasm";
    match gzip_size(wasm_path) {
        Ok(gz) => {
            let kb = gz as f64 / 1024.0;
            println!("==> {wasm_path}: {kb:.1} KB gzipped");
            if gz > 120 * 1024 {
                eprintln!("warning: gzipped size exceeds 120 KB budget");
            }
        }
        Err(e) => eprintln!("warning: could not read wasm file for size: {e}"),
    }

    println!("==> build-wasm done");
    ExitCode::SUCCESS
}

fn cmd_check() -> ExitCode {
    let steps: &[(&str, &[&str])] = &[
        ("cargo check --workspace", &["check", "--workspace"]),
        (
            "cargo test --workspace",
            &["test", "--workspace", "--exclude", "basecoat-controllers"],
        ),
        (
            "cargo clippy --workspace -- -D warnings",
            &["clippy", "--workspace", "--", "-D", "warnings"],
        ),
    ];

    let mut all_passed = true;

    for (label, argv) in steps {
        println!("==> {label}");
        let status = Command::new("cargo").args(*argv).status();
        match status {
            Err(e) => {
                eprintln!("  FAIL: could not run cargo: {e}");
                all_passed = false;
            }
            Ok(s) if !s.success() => {
                eprintln!("  FAIL: {label}");
                all_passed = false;
            }
            Ok(_) => println!("  ok"),
        }
    }

    if all_passed {
        println!("==> All checks passed.");
        ExitCode::SUCCESS
    } else {
        eprintln!("==> Some checks failed.");
        ExitCode::FAILURE
    }
}

// ── helpers ───────────────────────────────────────────────────────────────────

/// Returns the gzip-compressed size of a file in bytes using the system `gzip`.
fn gzip_size(path: &str) -> Result<u64, String> {
    let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
    // Compress in-memory using a minimal gzip call: pipe bytes through gzip -c.
    let mut child = Command::new("gzip")
        .args(["-c"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;

    if let Some(stdin) = child.stdin.take() {
        use std::io::Write;
        let mut stdin = stdin;
        stdin.write_all(&bytes).ok();
    }

    let output = child.wait_with_output().map_err(|e| e.to_string())?;
    Ok(output.stdout.len() as u64)
}
