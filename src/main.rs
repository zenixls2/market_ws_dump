use awc::ws::Frame;
use futures_util::SinkExt;
use futures_util::StreamExt;
use log::info;
use serde_json::json;
use std::str;

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, _record| out.finish(format_args!("{}", message)))
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(fern::log_file("out.log")?)
        .apply()?;
    Ok(())
}

async fn main_thread() {
    setup_logger().unwrap();
    let client = awc::Client::builder()
        .max_http_version(awc::http::Version::HTTP_11)
        .finish();
    // btcusdt@trade
    let (resp, mut connection) = client
        .ws("wss://fstream.binance.com/ws")
        .connect()
        .await
        .unwrap();
    info!("{:?}", resp);
    let params = vec!["btcusdt@forceOrder", "btcusdt@trade", "btcusdt@bookTicker"];
    let request = json!({
        "id": 1,
        "method": "SUBSCRIBE",
        "params": params,
    });
    let _ = connection
        .send(awc::ws::Message::Text(request.to_string().into()))
        .await
        .unwrap();
    while let Some(msg) = connection.next().await {
        if let Ok(Frame::Text(text)) = msg {
            info!("{}", str::from_utf8(&text).unwrap());
        }
    }
}

fn main() {
    actix::System::with_tokio_rt(|| {
        tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap()
    }).block_on(main_thread());
}
/*
 * Something similar:
 *
 * #[actix::main]
 * async fn main() {
 * }
 * */
