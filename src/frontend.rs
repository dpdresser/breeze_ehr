use poem::Route;
use poem::endpoint::{StaticFileEndpoint, StaticFilesEndpoint};

pub fn build_frontend_routes() -> Route {
    let static_site = StaticFilesEndpoint::new("public")
        .prefer_utf8(true)
        .index_file("index.html");

    Route::new()
        .at(
            "/favicon.ico",
            StaticFileEndpoint::new("public/breeze_favicon_b.ico"),
        )
        .at("/signup", StaticFileEndpoint::new("public/signup.html"))
        .at("/signin", StaticFileEndpoint::new("public/signin.html"))
        .at(
            "/dashboard",
            StaticFileEndpoint::new("public/dashboard.html"),
        )
        .nest("/", static_site)
}
