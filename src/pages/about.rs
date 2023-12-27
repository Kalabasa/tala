use rouille::Request;
use rouille::Response;

pub fn page(_request: &Request) -> Response {
  return Response::html(r#"<!doctype html>
    <title>About tiny site</title>
    <h1>About tiny site</h1>
    <p>This site is contained in a single executable!</p>
    <a href="/">Back to home</a>
  "#);
}
