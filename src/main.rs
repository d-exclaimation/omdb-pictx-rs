mod constant;
mod macros;
use constant::{is_prod, port, valid_content_type};
use macros::f;
use std::fs::{create_dir_all, remove_file, write};
use warp::body::{bytes, content_length_limit};
use warp::fs::dir;
use warp::hyper::body::Bytes;
use warp::{header, http::Response, path, serve, Filter, Rejection};

async fn upload_image(
    name: String,
    body: Bytes,
    content_type: String,
) -> Result<Response<String>, Rejection> {
    if !valid_content_type(&content_type) {
        return Response::builder()
            .status(400)
            .body(f!("Not allowed content type {content_type}!"))
            .map_err(|_| warp::reject::reject());
    }

    let res = create_dir_all("./images").and_then(|_| write(f!("./images/{name}"), &body.to_vec()));

    let status = if res.is_ok() { 201 } else { 500 };
    let message = if res.is_ok() {
        f!("Image {name} saved successfully")
    } else {
        f!("Error saving image {name}!")
    };

    Response::builder()
        .status(status)
        .body(message)
        .map_err(|_| warp::reject::reject())
}

async fn delete_image(name: String) -> Result<Response<String>, Rejection> {
    let res = remove_file(&name);

    let status = if res.is_ok() { 200 } else { 500 };
    let message = if res.is_ok() {
        f!("Image {name} deleted successfully")
    } else {
        f!("Error deleting image {name}!")
    };

    Response::builder()
        .status(status)
        .body(message)
        .map_err(|_| warp::reject::reject())
}

async fn not_found(_: Rejection) -> Result<Response<String>, Rejection> {
    Response::builder()
        .status(404)
        .body(f!("Not found!"))
        .map_err(|_| warp::reject::reject())
}

#[tokio::main]
async fn main() {
    let upload_path = warp::put()
        .and(path!("images" / String))
        .and(content_length_limit(1024 * 1024 * 10)) // 10MB
        .and(bytes())
        .and(header("content-type"))
        .and_then(upload_image);

    let read_path = warp::path("images").and(dir("./images"));

    let delete_path = warp::delete()
        .and(path!("images" / String))
        .and_then(delete_image);

    let routes = upload_path.or(read_path).or(delete_path).recover(not_found);

    serve(routes)
        .run((
            if is_prod() {
                [0, 0, 0, 0]
            } else {
                [127, 0, 0, 1]
            },
            port(),
        ))
        .await;
}
