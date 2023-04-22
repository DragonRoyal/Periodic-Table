#[macro_use] extern crate rocket;
use rocket_dyn_templates::Template;
// use yew::prelude::*;
use std::collections::HashMap;
use rocket::fs::FileServer;
use rocket::Request;
use rocket::fs::NamedFile;
use rocket::http::Status;
use serde_json::{Result, Value};
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::{Path, PathBuf};
use serde::Serialize;
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



#[derive(Serialize)]
struct PageData {
    image_t: String,
    image_url: String,
    title: String,
    summary: String,
    model: String,
    appearance: String,
    atomic_mass: f64,
    boil: f64,
    category: String,
    density: f64,
    discovered_by: String,
    melt: f64,
    molar_heat: f64,
    named_by: String,
}

#[get("/")]
async fn index() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("index",&context)
}

#[get("/<element>")]
async fn hydrogen(element: String) -> Template{
    let mut elNum: usize = 1;
    let mut file = File::open("elements.json").unwrap();
    
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();
    let v: Value = serde_json::from_str(&buff).unwrap();
    
    
    for i in 0..=117{
        println!(" this is i {i}");
        if element == v["elements"][i]["name"].as_str().unwrap(){
            elNum = i;
            println!(" this is i 2 {i}");
            break;
        }
        else{
            
            println!("should have went to 404 page (not desired)");
        }
       
    }
    println!("This is i 3 {elNum}");
    let element_t = v["elements"][elNum]["image"]["title"].as_str().unwrap().to_owned();
    let element_url = v["elements"][elNum]["image"]["url"].as_str().unwrap().to_owned();
    let element_name = v["elements"][elNum]["name"].as_str().unwrap().to_owned();
    let element_summary = v["elements"][elNum]["summary"].as_str().unwrap().to_owned();
    let element_model = v["elements"][elNum]["bohr_model_3d"].as_str().unwrap().to_owned();
    let element_app = v["elements"][elNum]["appearance"].as_str().unwrap().to_owned();
    let element_mass = v["elements"][elNum]["atomic_mass"].as_f64().unwrap().to_owned();
    let element_boil = v["elements"][elNum]["boil"].as_f64().unwrap().to_owned();
    let element_cat = v["elements"][elNum]["category"].as_str().unwrap().to_owned();
    let element_density = v["elements"][elNum]["density"].as_f64().unwrap().to_owned();
    let element_dis = v["elements"][elNum]["discovered_by"].as_str().unwrap().to_owned();
    let element_nameby = v["elements"][elNum]["named_by"].as_str().unwrap().to_owned();
    let element_melt = v["elements"][elNum]["melt"].as_f64().unwrap().to_owned();
    let element_heat = v["elements"][elNum]["molar_heat"].as_f64().unwrap().to_owned();
    let page_data = PageData {
        image_t: element_t,
        image_url: element_url,
        title: element_name,
        summary: element_summary,
        model: element_model,
        appearance: element_app,
        atomic_mass: element_mass,
        boil: element_boil,
        category: element_cat,
        density: element_density,
        discovered_by: element_dis,
        melt: element_melt,
        molar_heat: element_heat,
        named_by: element_nameby,
    };
    let context = serde_json::to_value(&page_data).unwrap();
    Template::render("elements/element",&context)
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
async fn not_found() -> Template {
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
    

    println!("\"\"");
    rocket::build()
        
        .mount("/", routes![index])
        .mount("/elements", routes![hydrogen])
        .mount("/static", FileServer::from("static"))
        .mount("/", routes![favicon])
        .register("/", catchers![internal_error, not_found, default])
        .attach(Template::fairing())
    

}
