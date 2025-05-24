// salt.js - Main entry point for Salt SVG App

// Initialize and maintain application state
let saltApp = null;
let container = null;
let resizeObserver = null;

// Initialize the application when WASM is loaded
async function initApp() {
  try {
    // Import the WASM module
    const wasm = await import("./pkg/app.js");
    await wasm.default();

    // Get container
    container = document.getElementById("app-container");

    // Initialize the Rust app
    saltApp = new wasm.SaltApp();

    // Render initial SVG
    renderSvg();

    // Set up event handlers
    setupEventListeners();

    // Set up resize observer
    setupResizeObserver();

    console.log("Salt app initialized successfully");
  } catch (err) {
    console.error("Failed to initialize Salt app:", err);
    container.innerHTML = `<div style="color: red; padding: 20px;">Error: ${err.message}</div>`;
  }
}

// Render the SVG output from Rust
function renderSvg() {
  if (!saltApp || !container) return;
  const { width, height } = container.getBoundingClientRect();
  container.innerHTML = saltApp.render_svg(
    Math.floor(width),
    Math.floor(height),
  );
}

// Handle user input events
function handleEvent(event) {
  if (!saltApp) return;

  const rect = container.getBoundingClientRect();
  const x = event.clientX - rect.left;
  const y = event.clientY - rect.top;

  // Pass event to Rust
  const stateChanged = saltApp.handle_mouse_event(event.type, x, y);

  // Re-render if needed
  if (stateChanged) {
    renderSvg();
  }
}

// Set up event listeners for user input
function setupEventListeners() {
  if (!container) return;

  // Mouse events
  const events = ["click", "mousedown", "mouseup", "mousemove"];

  events.forEach((eventType) => {
    container.addEventListener(eventType, handleEvent);
  });
}

// Update the app when the window is resized
function setupResizeObserver() {
  if (resizeObserver) {
    resizeObserver.disconnect();
  }

  resizeObserver = new ResizeObserver((entries) => {
    for (const entry of entries) {
      if (entry.target === container && saltApp) {
        renderSvg();
      }
    }
  });

  if (container) {
    resizeObserver.observe(container);
  }
}

// Start the application when the document is loaded
document.addEventListener("DOMContentLoaded", initApp);
