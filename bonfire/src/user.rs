use crate::{context::InstanceConfig, TEMPLATES};
use actix_web::{get, web, HttpResponse, Responder};
use sea_orm::DeriveEntityModel;
use serde::Serialize;

#[derive(Debug, serde::Serialize, sea_orm::DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct User {

    name: String,
}

#[get("")]
pub async fn user_root(inst_cfg: web::Data<InstanceConfig>) -> impl Responder {
    let mut ctx = tera::Context::new();
    ctx.insert("instance", &**inst_cfg);
    let out = TEMPLATES.render("user/index.html", &ctx).unwrap();
    HttpResponse::Ok().body(out)
}

#[get("{id}")]
pub async fn profile(
    inst_cfg: web::Data<InstanceConfig>,
    id: web::Path<(String,)>,
) -> impl Responder {
    let mut ctx = tera::Context::new();
    ctx.insert("instance", &**inst_cfg);
    ctx.insert(
        "user",
        &User {
            name: id.into_inner().0,
        },
    );
    let out = TEMPLATES.render("user/profile.html", &ctx).unwrap();
    HttpResponse::Ok().body(out)
}
