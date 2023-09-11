use std::collections::HashMap;

use oxhttp::{
    model::{HeaderName, Method, Request, Status},
    Client,
};

fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");

    tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap()
        .block_on(async {
            println!(">>> Entering async main in tokio");

            let client = Client::new();
            let response = client
                .request(
                    Request::builder(Method::GET, "http://example.com".parse().unwrap()).build(),
                )
                .unwrap();
            println!(">>> oxhttp response status, {:?}", response.status());
            assert_eq!(response.status(), Status::OK);
            assert_eq!(
                response.header(&HeaderName::CONTENT_TYPE).unwrap().as_ref(),
                b"text/html; charset=UTF-8"
            );

            let body = response.into_body().to_string().unwrap();
            println!(">>> oxhttp response body, {}", body);
        });
}
