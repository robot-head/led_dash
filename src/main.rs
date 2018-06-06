#![feature(proc_macro)]

#[macro_use]
extern crate rouille;


fn main() {
    let material_js = include_str!("./assets/js/material-components-web.min.js");
    let material_css = include_str!("./assets/css/material-components-web.min.css");

    println!("Now listening on 0.0.0.0:8000");

    rouille::start_server("0.0.0.0:8000", move |request| {
        router!(request,
            (GET) (/) => {
                rouille::Response::redirect_302("/static/html/dashboard.html")
            },


            (GET) (/material.js) => {
                rouille::Response::text(material_js)
            },


            (GET) (/material.css) => {
                rouille::Response::text(material_css)
            },


            // Default 404 matcher
            _ => rouille::Response::empty_404()
        )
    });
}