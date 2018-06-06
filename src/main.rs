#![feature(proc_macro)]

#[macro_use]
extern crate rouille;

use rouille::websocket;
use std::thread;


fn main() {
    println!("Now listening on 0.0.0.0:8000");

    rouille::start_server("0.0.0.0:8000", move |request| {
        {
            println!("{:#?}", &request);
            if let Some(request) = request.remove_prefix("/static") {
                let response = rouille::match_assets(&request, "./src/assets");
                if response.is_success() {
                    return response;
                }
            }
        }


        router!(request,
            (GET) (/) => {
                rouille::Response::redirect_302("/static/html/dashboard.html")
            },

            (GET) (/ws) => {
                let (response, websocket) = try_or_400!(websocket::start(&request, Some("echo")));
                thread::spawn(move || {
                    let ws = websocket.recv().unwrap();
                    websocket_handling_thread(ws);
                });

                response
            },

            // Default 404 matcher
            _ => rouille::Response::empty_404()
        )
    });
}

// Function run in a separate thread.
fn websocket_handling_thread(mut websocket: websocket::Websocket) {
    // We wait for a new message to come from the websocket.
    while let Some(message) = websocket.next() {
        match message {
            websocket::Message::Text(txt) => {
                // If the message is text, send it back with `send_text`.
                println!("received {:?} from a websocket", txt);
                websocket.send_text(&txt).unwrap();
            }
            websocket::Message::Binary(_) => {
                println!("received binary from a websocket");
            }
        }
    }
}