use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

use rand::seq::SliceRandom;
use actix_web::{get, App, HttpServer, Responder};

use std::sync::Mutex;

static GLOBAL_DATA: Mutex<Vec<String>> = Mutex::new(Vec::new());


fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn get_ran_line() -> String {
    let mut rng = rand::thread_rng();
    let ran_line = GLOBAL_DATA.lock().unwrap().choose(&mut rng).unwrap().to_string();
    return ran_line;
}

#[get("/get-poem/")]
async fn get_poem() -> impl Responder {
    let poem = get_ran_line();
    format!("{}", poem)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let file_path = "/home/www/ring/poem.txt";
    *GLOBAL_DATA.lock().unwrap() = lines_from_file(file_path);
    HttpServer::new(|| {
        App::new().service(get_poem)
    })
    .bind(("127.0.0.1", 9500))?
    .run()
    .await
}
