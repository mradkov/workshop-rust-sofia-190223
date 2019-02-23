use std::env;
use simple_server::Server;

mod templates;

fn get_server_port() -> String {
    env::var("PORT").unwrap_or(String::from("7878"))
}

fn main() {
    let app = Server::new(|_request, mut response| {
        let motivation = templates::motivation();
        Ok(response.header("Content-Type", "text/html; charset=utf-8").body(motivation)?)
    });

    let host = "0.0.0.0";
    let port = get_server_port();
    println!("Server is starting at {}:{}", host, port);
    app.listen(host, &port);
}