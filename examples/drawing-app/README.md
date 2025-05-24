# Drawing App Example

This is an example application that uses the Salt framework to create a simple drawing app. Users can click and drag to draw lines on the canvas.

## How It Works

The `DrawingApp` struct implements the Salt `App` trait, which requires:

1. `new()` - Creates a new instance of the app
2. `handle_event()` - Processes mouse events (clicks, movements, etc.)
3. `render()` - Generates SVG output based on the current state

The app tracks points added when the user drags the mouse and renders them as an SVG path.

## Building and Running

From the example directory:

```
wasm-pack build --target web
```

Then serve the generated files using a web server and open the index.html file.

## How Salt Works

The `salt_app!` macro at the bottom of `lib.rs` connects our Rust code to the WebAssembly/JavaScript interface. Salt handles the conversion between browser events and the Rust app, allowing us to focus on the application logic rather than the WASM plumbing.