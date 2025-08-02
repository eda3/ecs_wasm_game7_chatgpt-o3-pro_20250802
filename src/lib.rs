// ! WASM entry point for Multiplayer Klondike Solitaire
// !
use wasm_bindgen::prelude::*;

use wasm_bindgen::{JsCast, UnwrapThrowExt};

use wasm_bindgen::JsValue;

use web_sys::{
    CanvasRenderingContext2d, Document, HtmlCanvasElement, MessageEvent, WebSocket, Window,
};

// Optional: shrink WASM binary size
#[cfg(feature = "slim")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Constants
const WS_HOST: &str = "ws://162.43.8.148:8101";

// Public API (called automatically by wasm-bindgen on module load)

/// Entry point executed once the `.wasm` is instantiated.
///
/// `wasm-bindgen` expands this into JS glue that calls our Rust
/// code after the browser finishes loading the module.
///
/// # Errors
///
/// # Panics
///
/// This function will panic only when executed **outside** a browser
/// context—i.e. when `window` or `document` objects are unavailable.
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // 1. Better panic messages in browser dev-tools console
    set_panic_hook();

    // 2. Access core browser objects
    let window: Window = web_sys::window().expect("no global `window` exists");
    let document: Document = window.document().expect("should have a document");

    // 3. Create and insert our <canvas>
    let canvas: HtmlCanvasElement = init_canvas(&document)?;
    let context: CanvasRenderingContext2d = canvas_context(&canvas)?;

    // 4. Connect to the multiplayer server via WebSocket
    let _ws = init_websocket()?;

    // 5. Initial drawing = just a placeholder background for now
    draw_background(&context);
    let _ws = init_websocket()?;

    Ok(())
}

fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

/// Create a full-windows `<canvas>` element and append it to `<body`
/// # Errors
///
fn init_canvas(document: &Document) -> Result<HtmlCanvasElement, JsValue> {
    let canvas: HtmlCanvasElement = document
        .create_element("canvas")?
        .dyn_into::<HtmlCanvasElement>()?;

    // Full-screen canvas sizing
    let width = document.document_element().unwrap_throw().client_width();
    let height = document.document_element().unwrap_throw().client_height();

    canvas.set_width(width as u32);
    canvas.set_height(height as u32);

    // Light-gray background to verify we rendered something
    canvas.style().set_property("background", "#e8e8e8")?;

    // Append to <body>
    document
        .body()
        .expect("document should have a body")
        .append_child(&canvas)?;

    Ok(canvas)
}

/// Obtain the 2D rendering context from our canvas
/// # Errors
///
fn canvas_context(canvas: &HtmlCanvasElement) -> Result<CanvasRenderingContext2d, JsValue> {
    let ctx = canvas
        .get_context("2d")?
        .ok_or_else(|| JsValue::from_str("no 2d context"))?
        .dyn_into::<CanvasRenderingContext2d>()
        .map_err(|e| JsValue::from(e))?;

    Ok(ctx)
}

/// Draw an initial blank background(placeholder)
fn draw_background(ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
    let width = ctx.canvas().unwrap_throw().width() as f64;
    let height = ctx.canvas().unwrap_throw().height() as f64;

    // ctx.set_fill_style(&"#0B6623".into()); // dark greeen felt;
    #[allow(deprecated)]
    ctx.set_fill_style(&"#0B6623".into()); // dark greeen felt;
    ctx.fill_rect(0.0, 0.0, width, height);

    Ok(())
}

/// Establish a WebSocket connection to the game server.
///
/// For now we only print connection events; real game-state sync
/// will be added in later steps.
/// # Errors
///
fn init_websocket() -> Result<WebSocket, JsValue> {
    let ws = WebSocket::new(WS_HOST)?;

    // Handle open event
    let onopen_cb = Closure::<dyn FnMut()>::new(|| {
        web_sys::console::log_1(&"WebSocket connected".into());
    });
    ws.set_onopen(Some(onopen_cb.as_ref().unchecked_ref()));
    onopen_cb.forget(); // leak the closure -> stays alive for lifetime of WS

    // Handle incoming messages
    let onmessage_cb = Closure::<dyn FnMut(MessageEvent)>::new(|evt: MessageEvent| {
        if let Ok(txt) = evt.data().dyn_into::<js_sys::JsString>() {
            web_sys::console::log_2(&"WS message:".into(), &txt);
        }
        // Binary frames will be handled later (bincode-encoded state)
    });
    ws.set_onmessage(Some(onmessage_cb.as_ref().unchecked_ref()));
    onmessage_cb.forget();

    // Handle errors
    let onerror_cb = Closure::<dyn FnMut()>::new(|| {
        web_sys::console::error_1(&"WebSocket error".into());
    });
    ws.set_onerror(Some(onerror_cb.as_ref().unchecked_ref()));
    onerror_cb.forget();

    Ok(ws)
}
