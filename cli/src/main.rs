use std::fs;
use std::io;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::Duration;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::Colorize;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use hyper_staticfile::Static;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use tokio::sync::mpsc;

#[derive(Parser)]
#[clap(author, version, about = "Salt development CLI")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build the WASM module
    Build {
        /// Build in release mode
        #[clap(long, short)]
        release: bool,
    },

    /// Start development server with hot reload
    Dev {
        /// Port to serve on
        #[clap(long, short, default_value = "8080")]
        port: u16,

        /// Disable auto-rebuild on changes
        #[clap(long)]
        no_watch: bool,

        /// Build in release mode
        #[clap(long)]
        release: bool,
    },

    /// Check if dependencies are installed
    Check,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build { release } => {
            build_wasm(release)?;
        }
        Commands::Dev {
            port,
            no_watch,
            release,
        } => {
            // Build first
            build_wasm(release)?;

            // Start the watcher if requested
            if !no_watch {
                // Set up a channel for file change notifications
                let (tx, mut rx) = mpsc::channel(100);

                // Start the file watcher in a separate thread
                std::thread::spawn(move || {
                    watch_for_changes(tx).unwrap();
                });

                // Process file change notifications
                tokio::spawn(async move {
                    while rx.recv().await.is_some() {
                        println!("{}", "File changes detected, rebuilding...".blue());
                        if let Err(e) = build_wasm(release) {
                            println!("{} {}", "Error rebuilding:".red(), e);
                        }
                    }
                });
            }

            // Start the development server
            start_server(port).await?;
        }
        Commands::Check => {
            check_dependencies()?;
        }
    }

    Ok(())
}

fn check_dependencies() -> Result<()> {
    println!("{}", "Checking dependencies...".blue());

    // Check Rust version
    let rustc_version = Command::new("rustc")
        .arg("--version")
        .output()
        .context("Failed to check Rust version")?;

    if rustc_version.status.success() {
        let version = String::from_utf8_lossy(&rustc_version.stdout);
        println!("{} {}", "Rust version:".green(), version.trim());
    } else {
        println!("{}", "Rust not found or not working correctly".red());
        return Err(anyhow::anyhow!("Rust toolchain not working"));
    }

    // Check wasm-pack
    let wasm_pack = Command::new("wasm-pack").arg("--version").output();

    match wasm_pack {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            println!("{} {}", "wasm-pack version:".green(), version.trim());
        }
        _ => {
            println!(
                "{}",
                "wasm-pack not found, will be installed when needed".yellow()
            );
        }
    }

    println!("{}", "Dependency check complete!".green());
    Ok(())
}

fn build_wasm(release: bool) -> Result<()> {
    println!("{}", "Building WebAssembly module...".green());

    // Check if wasm-pack is installed
    if Command::new("wasm-pack").arg("--version").output().is_err() {
        println!("{}", "wasm-pack not found, installing...".yellow());
        let status = Command::new("cargo")
            .args(["install", "wasm-pack"])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .context("Failed to install wasm-pack")?;

        if !status.success() {
            return Err(anyhow::anyhow!("Failed to install wasm-pack"));
        }
    }

    // Clean and create web directory
    println!("{}", "Cleaning web directory...".blue());
    let web_dir = Path::new("web");
    if web_dir.exists() {
        std::fs::remove_dir_all(web_dir).context("Failed to clean web directory")?;
    }
    std::fs::create_dir_all("web/js/pkg").context("Failed to create web directory structure")?;

    // Build the WebAssembly module
    let mut cmd = Command::new("wasm-pack");
    cmd.arg("build")
        .arg("--target")
        .arg("web")
        .arg("--out-dir")
        .arg("web/js/pkg")
        .arg("--out-name")
        .arg("app");

    if release {
        cmd.arg("--release");
    }

    let status = cmd
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("Failed to execute wasm-pack")?;

    if !status.success() {
        return Err(anyhow::anyhow!("wasm-pack build failed"));
    }

    // Copy template files
    println!("{}", "Writing template files...".blue());
    copy_dir_contents(web_dir).context("Failed to write template files")?;

    println!("{}", "WebAssembly build completed successfully!".green());
    Ok(())
}

fn watch_for_changes(tx: mpsc::Sender<()>) -> Result<()> {
    let (watcher_tx, watcher_rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(
        watcher_tx,
        notify::Config::default().with_poll_interval(Duration::from_secs(1)),
    )?;

    // Watch the src directory for changes
    let src_path = "src";
    watcher.watch(Path::new(src_path), RecursiveMode::Recursive)?;

    // Also watch the templates directory
    let templates_path = "templates";
    if Path::new(templates_path).exists() {
        watcher.watch(Path::new(templates_path), RecursiveMode::Recursive)?;
        println!("{} {}", "Watching for changes in:".blue(), templates_path);
    }

    println!("{} {}", "Watching for changes in:".blue(), src_path);

    // Debounce to avoid rebuilding too frequently
    let mut last_rebuild = std::time::Instant::now() - Duration::from_secs(10);

    loop {
        match watcher_rx.recv() {
            Ok(_) => {
                let now = std::time::Instant::now();
                if now.duration_since(last_rebuild) > Duration::from_secs(1) {
                    last_rebuild = now;
                    let _ = tx.blocking_send(());
                }
            }
            Err(e) => {
                println!("{} {}", "Watch error:".red(), e);
                break;
            }
        }
    }

    Ok(())
}

// Helper function to copy template files to destination
fn copy_dir_contents(dst: &Path) -> io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    // Write index.html
    fs::write(
        dst.join("index.html"),
        include_str!("../../templates/index.html"),
    )?;

    // Create js directory if it doesn't exist
    let js_dir = dst.join("js");
    if !js_dir.exists() {
        fs::create_dir_all(&js_dir)?;
    }

    // Write salt.js
    fs::write(
        js_dir.join("salt.js"),
        include_str!("../../templates/js/salt.js"),
    )?;

    // No CSS directory needed

    Ok(())
}

async fn start_server(port: u16) -> Result<()> {
    let web_dir = Path::new("web");
    if !web_dir.exists() {
        return Err(anyhow::anyhow!("Web directory not found"));
    }

    // Check if we need to rebuild the app due to missing files
    if !web_dir.join("js").join("pkg").exists() {
        println!("{}", "Missing build files, rebuilding...".yellow());
        build_wasm(false)?;
    }

    let static_handler = Static::new(web_dir);

    let make_service = make_service_fn(move |_| {
        let static_handler = static_handler.clone();
        async move {
            Ok::<_, hyper::Error>(service_fn(move |req: Request<Body>| {
                let static_handler = static_handler.clone();
                async move {
                    let path = req.uri().path();

                    // Log the request (only show paths, not query params for cleaner output)
                    let display_path = path.split('?').next().unwrap_or(path);
                    println!("{} {}", "Request:".blue(), display_path);

                    let response = static_handler.serve(req).await?;
                    Ok::<Response<Body>, std::io::Error>(response)
                }
            }))
        }
    });

    let addr = ([127, 0, 0, 1], port).into();
    let server = Server::bind(&addr).serve(make_service);

    println!("{} http://localhost:{}", "Server running at:".green(), port);
    println!("{} {}", "Serving files from:".green(), web_dir.display());
    println!("{}", "Press Ctrl+C to stop the server".blue());

    // Set up graceful shutdown
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        println!("\n{}", "Shutting down server...".yellow());
        r.store(false, Ordering::SeqCst);
    })?;

    let graceful = server.with_graceful_shutdown(async move {
        while running.load(Ordering::SeqCst) {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    });

    graceful.await?;
    println!("{}", "Server shutdown complete".green());

    Ok(())
}
