extern crate csv;
use actix_web::{get, App, HttpResponse, HttpServer};
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;

#[get("/")]
async fn index() -> Result<HttpResponse, Box<dyn Error>> {
    let file = File::open("quotes.csv")?;
    let mut rdr = ReaderBuilder::new()
        .delimiter(b',')
        .from_reader(file);

    let mut csv_data = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let csv_line = record.iter().collect::<Vec<_>>().join(", ");
        csv_data.push(csv_line);
    }

    let response_body = csv_data.join("\n");

    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(response_body))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
