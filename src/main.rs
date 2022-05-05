
use web_sys::window;

fn start_app() {
    let document = window().and_then(|win| win.document()).expect("Could not access document");
    let body = document.body().expect("Could not access document.body");
    let text_node = document.create_text_node("Hello, world from Vanilla Rust!");
    body.append_child(text_node.as_ref()).expect("Failed to append text");
}

fn main() {
    start_app();
}