#[macro_use] extern crate rocket;
use rocket_dyn_templates::Template;
// use yew::prelude::*;
use std::collections::HashMap;
use rocket::fs::FileServer;
use rocket::Request;
use rocket::fs::NamedFile;
use rocket::http::Status;

use std::path::{Path, PathBuf};
// #[function_component(App)]
// fn app() -> Html {
//     html! {
//         <h1>{ "Hello World" }</h1>
//     }
// }

/* To do: make this use yew the frontend framework 

for cargo.toml |  yew = { version = "0.20", features = ["csr"] }
getrandom = { version = "0.2", features = ["js"] } |
   


*/


#[get("/")]
async fn index() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("index",&context)
}

#[get("/hydrogen")]
async fn hydrogen() -> Template{
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("elements/hydrogen",&context)
}

#[get("/favicon.ico")]
async fn favicon() -> Option<NamedFile> {
    let path: PathBuf = Path::new("static/favicon.ico").into();
    NamedFile::open(path).await.ok()
}

// error handling section *
#[catch(500)]
fn internal_error() -> &'static str {
    "Whoops! Looks like we messed up, Tell me how you got here though"
}

#[catch(404)]
fn not_found(req: &Request) -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("error/404",&context)
}

#[catch(default)]
fn default(status: Status, req: &Request) -> String {
    format!("{} ({})", status, req.uri())
}

// error handling section end **



#[launch]
fn rocket() -> _ {
    //yew::Renderer::<App>::new().render();
    rocket::build()
        .mount("/", routes![index])
        .mount("/elements", routes![hydrogen])
        .mount("/static", FileServer::from("static"))
        .mount("/", routes![favicon])
        .register("/", catchers![internal_error, not_found, default])
        .attach(Template::fairing())
    

}
