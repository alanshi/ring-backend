use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
use rand::seq::SliceRandom;
use actix_web::{get, web, App, HttpServer, Responder};


fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn get_ran_line() -> String {
    let file_path = "/home/www/ring/poem.txt";
    let lines = lines_from_file(file_path);
    let mut rng = rand::thread_rng();
    let ran_line = lines.choose(&mut rng).unwrap();
    return ran_line.to_lowercase();
}

#[get("/get-poem/")]
async fn get_poem() -> impl Responder {
    let poem = get_ran_line();
    format!("{}", poem)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(get_poem)
    })
    .bind(("127.0.0.1", 9500))?
    .run()
    .await
}
