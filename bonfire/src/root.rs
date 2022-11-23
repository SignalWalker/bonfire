
#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}

#[get("/")]
pub fn root() -> IndexTemplate {
    IndexTemplate {}
}

