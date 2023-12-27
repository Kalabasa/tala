use rouille::Request;
use rouille::Response;

fn main() {
    rouille::start_server("0.0.0.0:8080", move |request| {
        Response::text("hello world")
    });
}
