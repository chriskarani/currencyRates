extern crate futures;
extern crate hyper;
extern crate tokio_core;

use futures::{Future};
use hyper::{Client, Uri};
use tokio_core::reactor::Core;


const API_KEY: &str = "7e8bd3b93485d813085a941b5a0cfc1e";
const API_URL : &str = "http://data.fixer.io/api/";




fn main() {
    println!("Hello, world!");
}
