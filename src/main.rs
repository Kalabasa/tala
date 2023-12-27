use rouille::Request;
use rouille::Response;

fn main() {
    rouille::start_server("0.0.0.0:8080", move |request| {
        Response::html("<!doctype html>
            <title>My server</title>
            <h1>Hello, world!</h1>
        ")
    });
}
