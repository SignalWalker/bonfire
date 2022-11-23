#![doc = include_str!("../README.md")]

use actix_web::{web, HttpRequest, HttpResponse, HttpServer, Responder};
use bonfire_core::meta::CrateVersion;
use clap::Parser;
use cli::Args;
use context::InstanceConfig;
use lazy_static::lazy_static;
use tera::Tera;

pub mod cli;
pub mod context;
// pub mod root;
// pub mod api;
pub mod user;
// use user::*;
// use api::*;

lazy_static! {
    pub static ref VERSION: CrateVersion<'static> = bonfire_core::crate_version!();
    pub static ref TEMPLATES: Tera = {
        #[allow(clippy::let_and_return)]
        let res = Tera::new("templates/**/*.html").unwrap();
        res
    };
}

#[actix_web::get("/")]
async fn root(inst_cfg: web::Data<InstanceConfig>) -> impl Responder {
    let mut ctx = tera::Context::new();
    ctx.insert("instance", &**inst_cfg);
    let out = TEMPLATES.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().body(out)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let ctx = InstanceConfig::new(
        &args.instance_name,
        &args.instance_url,
        &args.instance_lang,
        &args.instance_desc,
    );
    let mut server = HttpServer::new(move || {
        actix_web::App::new()
            .data(ctx.clone())
            .service(root)
            .service(web::scope("/a")) // api
            .service(
                web::scope("/u")
                    .service(user::profile)
                    .service(user::user_root),
            ) // users
            .service(web::scope("/f")) // forums
    });

    for addr in args.listen_addrs {
        server = server.bind(addr)?;
    }

    server.run().await
}
