#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate tslite;

#[macro_use]
extern crate lazy_static;

use std::path::Path;
use std::sync::Mutex;

lazy_static! {
    static ref DB: Mutex<tslite::PhysicalDB> =
        Mutex::new(tslite::PhysicalDB::new(Path::new("data.db"), None).unwrap());
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/test")]
fn test() -> &'static str {
    let rec_nfo = tslite::RecordInfo {
        time_offset: 10,
        value: 5,
    };
    DB.lock()
        .expect("Could not lock on DB")
        .append_record(rec_nfo)
        .expect("Could not write record");
    "Written"
}

fn main() {
    rocket::ignite().mount("/", routes![index, test]).launch();
}
