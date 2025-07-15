use actix_web::{HttpResponse, Responder, get};
use askama::Template;
use rand::{self, random_range};
extern crate askama;

#[derive(Template)]
#[template(path = "index.html")]
struct ThisTemplate {
    title: String,
    random: String,
}

#[get("/")]
pub async fn index() -> impl Responder {
    let template: ThisTemplate = ThisTemplate {
        title: format!("首页 Askama"),
        random: random_range(100000..999999).to_string(),
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())

    // HttpResponse::Ok().body(format!("<h1>Hello world! random:{}</h1>", random_str))
}
