use std::env;

const ALLOWED_IMAGE_TYPES: [&str; 5] = [
    "image/png",
    "image/jpeg",
    "image/jpg",
    "image/gif",
    "image/webp",
];

pub fn valid_content_type(content_type: &String) -> bool {
    ALLOWED_IMAGE_TYPES.contains(&content_type.as_str())
}

pub fn get_content_type(content_type: &String) -> Option<String> {
    content_type
        .split(".")
        .last()
        .map(|extension| format!("image/{}", extension))
}

pub fn is_prod() -> bool {
    env::var("ENV").unwrap_or_else(|_| "dev".into()) == "prod"
}

pub fn port() -> u16 {
    env::var("PORT")
        .unwrap_or_else(|_| "8080".into())
        .parse()
        .unwrap_or(8080)
}
