use actix_files::Files;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, get, post, web};
// use actix_ws::AggregatedMessage;
// use actix_ws::Message;
use actix_ws::Message;
// use futures_util::StreamExt as _;
// use askama::Template;
// use rand::{self, random_range};
use serde::{Deserialize, Serialize};
extern crate askama;
// use actix_web::pages::index;

// mysql相关
// use mysql_async::{Pool, prelude::*};

// #[derive(Template)]
// #[template(path = "index.html")]
// struct IndexTemplate {
//     title: String,
// }

// #[get("/")]
// async fn index() -> impl Responder {
//     let random_str = random_range(100000..999999).to_string();
//     // HttpResponse::Ok().body(format!("<h1>Hello world! random:{}</h1>", random_str))
//     let template = IndexTemplate {
//         title: format!("Hello Askama {}", random_str),
//     };

//     HttpResponse::Ok()
//         .content_type("text/html")
//         .body(template.render().unwrap())
// }

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/hello/{name}")]
async fn test_get(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

#[derive(Deserialize)]
struct FormData {
    user_name: String,
    user_email: String,
}
#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}
#[post("/post")]
async fn test_post(form: web::Form<FormData>) -> impl Responder {
    let user = User {
        id: 1,
        name: form.user_name.to_string(),
        email: form.user_email.to_string(),
    };
    // HttpResponse::Ok().body(req_body)
    HttpResponse::Ok().json(user)
}
async fn api_test() -> impl Responder {
    HttpResponse::Ok().body("api")
}

// 获取产品
#[actix_web::get("/products/{id}")]
async fn get_product(id: web::Path<u32>) -> impl Responder {
    // let db_user = "root";
    // let db_password = "root";
    // let db_name = "test";
    // let db_config = format!(
    //     "mysql://{username}:{password}@localhost:3306/{database_name}",
    //     username = "root",
    //     password = "root",
    //     database_name = "test",
    // );

    // let pool = Pool::new(db_config).await.unwrap();

    // let mut conn = pool.get_conn().await.unwrap();

    // let results = conn.query("SELECT * FROM table_name", []).await.unwrap();

    // for result in results {
    //     let row = result.unwrap();

    //     // 处理每一行数据
    //     println!("id: {}, name: {}", row.get(0).unwrap(), row.get(1).unwrap());
    // }
    // Ok(HttpResponse::Ok().json({results}))
    HttpResponse::Ok().body(format!("id:{}", id))
}

// async fn ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
//     let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

//     let mut stream = stream
//         .aggregate_continuations()
//         // aggregate continuation frames up to 1MiB
//         .max_continuation_size(2_usize.pow(20));

//     // start task but don't wait for it
//     rt::spawn(async move {
//         // receive messages from websocket
//         while let Some(msg) = stream.next().await {
//             match msg {
//                 Ok(AggregatedMessage::Text(text)) => {
//                     // echo text message
//                     session.text(text).await.unwrap();
//                 }

//                 Ok(AggregatedMessage::Binary(bin)) => {
//                     // echo binary message
//                     session.binary(bin).await.unwrap();
//                 }

//                 Ok(AggregatedMessage::Ping(msg)) => {
//                     // respond to PING frame with PONG frame
//                     session.pong(&msg).await.unwrap();
//                 }

//                 _ => {}
//             }
//         }
//     });

//     // respond immediately with response connected to WS session
//     Ok(res)
// }
async fn ws(req: HttpRequest, body: web::Payload) -> actix_web::Result<impl Responder> {
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, body)?;

    actix_web::rt::spawn(async move {
        while let Some(Ok(msg)) = msg_stream.recv().await {
            match msg {
                Message::Ping(bytes) => {
                    if session.pong(&bytes).await.is_err() {
                        return;
                    }
                }
                Message::Text(msg) => {
                    let ser_msg = format!("server : {}", &msg);
                    session.text(ser_msg).await.unwrap();
                    println!("Got text: {}", &msg);
                }
                _ => break,
            }
        }

        let _ = session.close(None).await;
    });

    Ok(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("服务已启动: http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            // WebSocket
            .route("/ws", web::get().to(ws))
            // 测试数据库链接
            .service(get_product)
            // 宏定义首页
            .service(web_actix::pages::index)
            // 直接路由 get
            .route("/hey", web::get().to(manual_hello))
            // 宏定义 get
            .service(test_get)
            // 宏定义 post
            .service(test_post)
            // 前缀路由 api
            .service(web::scope("/api").route("/test", web::get().to(api_test)))
            // 模板处理
            // 三种静态文件处理方法
            // .service(Files::new("/", ".").prefer_utf8(true))
            // .service(Files::new("/", ".").show_files_listing())
            .service(Files::new("/", "."))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
