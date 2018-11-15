#![feature(plugin)]
#![plugin(rocket_codegen)]
#![allow(unused)]

use std::io;
use std::path::{Path, PathBuf};

use rocket::response::NamedFile;

use std::thread;
use std::time;
extern crate rocket;

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/sleep/<time>")]
fn sleep(time: u64) -> String {
    let duration = time::Duration::from_millis(time*1000);
    let now = time::Instant::now();

    thread::sleep(duration);

    assert!(now.elapsed() >= duration);
    format!("Just napped for {} seconds!", time)

}

#[get("/")]
fn index() -> io::Result<NamedFile> {
  NamedFile::open("static/index.html")
}

fn main() {
    rocket::ignite().mount("/", routes![hello, sleep, index]).launch();
}
