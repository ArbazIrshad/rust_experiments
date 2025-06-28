// use axum::Router;

// use std::{error::Error, thread, time::Duration};

use std::{fmt::Display, thread, time::Duration};

// use axum::{Router, routing::get};
use rust_practice::run_app;

#[derive(Debug, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point(x: {}, y: {})", self.x, self.x)
    }
}

#[tokio::main]
async fn main() {
    // let result = run_app().await;
    let a = Point { x: 22, y: 22 };

    let mut b = Point { x: 44, y: 44 };
    // let  c = &a;

    let d = Box::new(b);
    // d.x = 1;
    // d = Box::new(a);
    // a.x = 11;
    // d.x = 11;
    println!("A: {}", a);
    // println!("B: {}", b);

    // match result {
    //     Ok(_) => println!("It's okay"),
    //     Err(_) => println!("Something went wrong"),
    // }
}

fn string_example() {
    let mut msg = String::from("HELLO WORLD!");

    if msg.contains("world!") {
        println!("CONTAINS world");
    }

    if msg.contains("WORLD!") {
        println!("CONTAINS WORLD");
    }

    msg.push('1');
}

fn thread_example() {
    // println!("Hello, world!");
    // Router::new().route("/health_check", method_router);
    let t1 = thread::spawn(|| {
        for i in 1..100 {
            println!("T1 LOOP COUNT :{i}");
            thread::sleep(Duration::from_secs(1));
        }
    });

    for i in 1..100 {
        println!("LOOP COUNT :{i}");
        thread::yield_now();
    }

    t1.join();
}
