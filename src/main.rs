use rouille::Response;
use rouille::router;

mod pages {
  pub mod index;
  pub mod about;
}

fn main() {
  rouille::start_server("0.0.0.0:8080", move |request| {
    let response = router!(request,
      (GET) (/) => {
        pages::index::page(request)
      },

      (GET) (/about) => {
        pages::about::page(request)
      },

      _ => Response::empty_404()
    );

    return response;
  });
}
