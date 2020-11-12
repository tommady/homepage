use gotham::router::builder::{build_simple_router, DefineSingleRoute, DrawRoutes};

pub fn main() {
    let addr = "0.0.0.0:9898";
    let router = build_simple_router(|route| {
        route.get("/").to_file("assets/index.html");
    });

    gotham::start(addr, router)
}
