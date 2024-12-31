use std::env;

pub fn get_port() -> u16 {
    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    return port;
}
