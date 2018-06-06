#![feature(proc_macro)]

#[macro_use]
extern crate rouille;


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


            // Default 404 matcher
            _ => rouille::Response::empty_404()
        )
    });
}