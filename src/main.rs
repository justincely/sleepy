#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::response::NamedFile;
use std::io;
use std::thread;
use std::time;

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/sleep/<time>")]
fn sleep(time: u64) -> String {
    let duration = time::Duration::from_millis(time * 1000);
    let now = time::Instant::now();

    thread::sleep(duration);

    assert!(now.elapsed() >= duration);
    format!("Just napped for {} seconds!", time)
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

/// Starts a new HTTP server.
fn main() {
    rocket::ignite()
        .mount("/", routes![hello, sleep, index])
        .launch();
}
