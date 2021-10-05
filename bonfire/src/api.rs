use rocket::get;
use crate::VERSION;

#[get("/")]
pub fn api_root() -> String {
    format!(
        "supported api version: {}.{}",
        VERSION[0],
        VERSION[1]
    )
}

#[get("/0.1")]
pub fn api_0_1() -> &'static str {
    "api v0.1"
}