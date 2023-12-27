use rouille::Response;
use rouille::router;

fn main() {
    rouille::start_server("0.0.0.0:8080", move |request| {
        let response = router!(request,
            (GET) (/) => {
                Response::html(r#"<!doctype html>
                    <title>Tiny site</title>
                    <h1>Hello, world!</h1>
                    <p>Welcome to my tiny site!</p>
                    <a href="/about">About</a>
                "#)
            },
        
            (GET) (/about) => {
                Response::html(r#"<!doctype html>
                    <title>About tiny site</title>
                    <h1>About tiny site</h1>
                    <p>This site is one file</p>
                    <a href="/">Back to home</a>
                "#)
            },
        
            _ => Response::empty_404()
        );

        return response;
    });
}
