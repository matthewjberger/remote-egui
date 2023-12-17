use std::net::SocketAddr;
use warp::{
    http::{Response, StatusCode},
    Filter,
};

pub(crate) async fn launch_browser_ui(address: &str, port: u16) {
    let address = if address == "localhost" {
        "127.0.0.1"
    } else {
        address
    }
    .to_string();
    let socket_address = SocketAddr::new(address.parse().expect("Invalid IP address"), port);

    let root_route =
        warp::path::end().and_then(|| async { serve_asset("index.html".to_string()).await });

    let assets_route = warp::path::tail()
        .map(|tail: warp::path::Tail| tail.as_str().to_string())
        .and_then(serve_asset);

    let routes = root_route.or(assets_route);

    if let Err(error) = webbrowser::open(&format!("http://{address}:{port}#dev")) {
        log::error!("Failed to open browser: {error}");
    }
    warp::serve(routes).run(socket_address).await;
}

// This directory can be generated with `trunk build`
#[derive(rust_embed::RustEmbed)]
#[folder = "dist/"]
struct Site;

async fn serve_asset(path: String) -> Result<impl warp::Reply, warp::Rejection> {
    let asset = Site::get(&path).or_else(|| Site::get("index.html"));

    if let Some(data) = asset {
        let mime_guess = mime_guess::from_path(&path).first_or_octet_stream();
        let response = Response::builder()
            .header("Content-Type", mime_guess.as_ref())
            .body(data.data)
            .unwrap();

        Ok(warp::reply::with_status(response, StatusCode::OK))
    } else {
        Err(warp::reject::not_found())
    }
}
