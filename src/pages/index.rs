use rouille::Request;
use rouille::Response;

pub fn page(_request: &Request) -> Response {
  return Response::html(r#"<!doctype html>
    <title>Tiny site</title>
    <h1>Hello, world!</h1>
    <p>Welcome to my tiny site!</p>
    <a href="/about">About</a>
  "#);
}
