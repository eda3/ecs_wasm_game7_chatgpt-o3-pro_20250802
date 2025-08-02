// ! WASM entry point for Multiplayer Klondike Solitaire
// !
use wasm_bindgen::prelude::*;

use wasm_bindgen::{JsCast, UnwrapThrowExt};

use web_sys::{
    CanvasRenderingContext2d, Document, HtmlCanvasElement, MessageEvent, WebSocket, Window,
};

// Optional: shrink WASM binary size
#[cfg(feture = "slim")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Constants
const WS_HOST: &str = "ws://162.43.8.148:8101";

// Public API (called automatically by wasm-bindgen on module load)

/*
/// Entry point excuted once the `.wasm` is instantiated.
///
/// `wasm-bindgen` expands this into JS glue that calls our Rust
/// code after the browser finishes loading the module.
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
    draw_background(&context)?;

    ok(())
}
*/

/// Establish a WebSocket connection to the game server.
///
/// For now we only print connection events; real game-state sync
/// will be added in later steps.
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

    Ok(ws)
}
