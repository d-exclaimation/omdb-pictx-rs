mod constant;
mod macros;
use constant::{get_content_type, is_prod, port, valid_content_type};
use macros::f;
use std::fs::{create_dir_all, read, remove_file, write};
use warp::body::{bytes, content_length_limit};
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

async fn read_image(name: String) -> Result<Response<Vec<u8>>, Rejection> {
    let content_type = get_content_type(&name);

    if content_type.is_none() {
        return Response::builder()
            .status(400)
            .body(f!("Undefined file type for {}!", name).into_bytes())
            .map_err(|_| warp::reject::reject());
    }

    let res = read(&name);

    let status = if res.is_ok() { 200 } else { 500 };
    let message = res.unwrap_or(f!("Error reading image {name}!").into_bytes());

    Response::builder()
        .status(status)
        .header("Content-Type", content_type.unwrap())
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

#[tokio::main]
async fn main() {
    let upload_path = warp::put()
        .and(path!("images" / String))
        .and(content_length_limit(1024 * 1024 * 10)) // 10MB
        .and(bytes())
        .and(header("content-type"))
        .and_then(upload_image);

    let read_path = warp::get()
        .and(path!("images" / String))
        .and_then(read_image);

    let delete_path = warp::delete()
        .and(path!("images" / String))
        .and_then(delete_image);

    let routes = upload_path.or(read_path).or(delete_path);

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
