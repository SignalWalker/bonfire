use rocket::get;

#[get("/")]
pub fn user_root() -> &'static str {
    "user root"
}

#[get("/<name>")]
pub fn profile(name: &str) -> String {
    format!("user: {}", name)
}