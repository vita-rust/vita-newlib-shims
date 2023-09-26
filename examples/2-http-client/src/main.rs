#[cfg(target_os = "vita")]
use vita_newlib_shims as _;

fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_BACKTRACE", "full");

    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(async {
            println!(">>> Trying oxhttp");
            let client = oxhttp::Client::new();
            let body = client
                .request(
                    oxhttp::model::Request::builder(
                        oxhttp::model::Method::GET,
                        "http://example.com".parse()?,
                    )
                    .build(),
                )?
                .into_body()
                .to_string()?;
            println!(">>> oxhttp response: {body}");

            // Requires std to depend on the latest libc
            // println!(">>> Trying ureq");
            // let body = ureq::get("http://example.com").call()?;
            // println!(">>> Ureq response: {:?}", body);

            println!(">>> Trying reqwest");
            let body = reqwest::get("http://example.com").await?.text().await?;
            println!(">>> Reqwest response: {:#?}", body);

            Ok(())
        })
}
