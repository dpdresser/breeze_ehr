use poem::Route;
use poem::endpoint::{StaticFileEndpoint, StaticFilesEndpoint};

pub fn build_frontend_routes() -> Route {
    let static_site = StaticFilesEndpoint::new("frontend")
        .prefer_utf8(true)
        .index_file("index.html");

    Route::new()
        .at(
            "/favicon.ico",
            StaticFileEndpoint::new("frontend/favicon.svg"),
        )
        .at("/signup", StaticFileEndpoint::new("frontend/signup.html"))
        .at("/signin", StaticFileEndpoint::new("frontend/signin.html"))
        .at(
            "/dashboard",
            StaticFileEndpoint::new("frontend/dashboard.html"),
        )
        .nest("/", static_site)
}
