use gotham::handler::assets::FileOptions;
use gotham::router::builder::{build_simple_router, DefineSingleRoute, DrawRoutes};

pub fn main() {
    let addr = "0.0.0.0:9898";
    let router = build_simple_router(|route| {
        route.get("/").to_dir(
            FileOptions::new("assets/")
                .with_cache_control("no-cache")
                .with_gzip(true)
                .build(),
        );
    });

    gotham::start(addr, router)
}
