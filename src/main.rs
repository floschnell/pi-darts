extern crate rand;

use std::thread;
use rand::Rng;

fn main() {
    let mut hits_total = 0;
    let number_threads = 4;
    let number_throws_per_thread = 10000000;
    let mut handles = Vec::<thread::JoinHandle<usize>>::new();

    for _i in 0..number_threads {
        handles.push(thread::spawn(
            move || perform_throws(number_throws_per_thread),
        ));
    }

    while !handles.is_empty() {
        let handle_option = handles.pop();
        hits_total += handle_option.unwrap().join().unwrap();
    }

    let total_throws = number_throws_per_thread * number_threads;
    println!(
        "estimation of PI: {:.32}",
        4. * (hits_total as f64 / total_throws as f64)
    );
}

fn perform_throws(number_of_throws: usize) -> usize {
    let mut hits = 0;
    for _i in 0..number_of_throws {
        let (x, y) = throw();
        // using sqrt here would give us the real length
        // but sqrt only distorts the function's curve
        // it will not change the number of results that are below or above 1.
        if x * x + y * y <= 1f64 {
            hits += 1;
        }
    }
    hits
}

fn throw() -> (f64, f64) {
    (
        rand::thread_rng().gen::<f64>().abs() % 1.0f64,
        rand::thread_rng().gen::<f64>().abs() % 1.0f64,
    )
}
