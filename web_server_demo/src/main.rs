use warp::Filter;

#[tokio::main]
async fn main() {
    // Create a warp filter for "hello world"
    let hello = warp::path!("hello"/"world")
        .map(|| warp::reply::html("Hello, World!"));

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
