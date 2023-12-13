#[macro_use] extern crate rocket;

mod paste_id;

use paste_id::PasteId;
use rocket::data::{Data, ToByteUnit};
use rocket::http::uri::Absolute;
use rocket::tokio::fs::File;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, retrieve, upload])
}

#[get("/")]
fn index() -> &'static str {
    "
    USAGE

      POST /

          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content

      GET /<id>

          retrieves the content for the paste with id `<id>`
    "
}

#[get("/<id>")]
async fn retrieve(id: PasteId<'_>) -> Option<File> {
    File::open(id.file_path()).await.ok()
}

// In a real application, these would be retrieved dynamically from a config.
const ID_LENGTH: usize = 3;
const HOST: Absolute<'static> = uri!("http://localhost:8000");

#[post("/", data = "<paste>")]
async fn upload(paste: Data<'_>) -> std::io::Result<String> {
    let id = PasteId::new(ID_LENGTH);
    paste.open(128.kibibytes()).into_file(id.file_path()).await?;
    Ok(uri!(HOST, retrieve(id)).to_string())
}