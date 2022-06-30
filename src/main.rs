#![allow(unused_variables)]

use rayon::prelude::*;
use std::{
    sync::{Arc, Mutex},
    time::Instant,
};

// Macro
#[path = "01.rs"]
mod id_01;
#[path = "02.rs"]
mod id_02;
#[path = "03.rs"]
mod id_03;
#[path = "04.rs"]
mod id_04;
#[path = "05.rs"]
mod id_05;
#[path = "06.rs"]
mod id_06;
#[path = "07.rs"]
mod id_07;
#[path = "08.rs"]
mod id_08;
#[path = "09.rs"]
mod id_09;
#[path = "10.rs"]
mod id_10;
#[path = "12.rs"]
mod id_12;
#[path = "13.rs"]
mod id_13;
#[path = "14.rs"]
mod id_14;
#[path = "15.rs"]
mod id_15;
#[path = "18.rs"]
mod id_18;
#[path = "99.rs"]
mod id_99;
mod utils;

fn main() {
    let solution: Vec<(i64, fn() -> i64)> = vec![
        // ...
        (1, id_01::solve),
        (2, id_02::solve),
        (3, id_03::solve),
        (4, id_04::solve),
        (5, id_05::solve),
        (6, id_06::solve),
        (7, id_07::solve),
        (8, id_08::solve),
        (9, id_09::solve),
        (10, id_10::solve),
        (12, id_12::solve),
        (13, id_13::solve),
        (14, id_14::solve),
        (15, id_15::solve),
        (18, id_18::solve),
        (99, id_99::solve),
    ];

    let answers = Arc::new(Mutex::new(Vec::with_capacity(solution.len())));

    solution.par_iter().for_each(|(i, f)| {
        let start = Instant::now();
        let ans = f();
        let duration = start.elapsed();
        answers.lock().unwrap().push((i, ans, duration));
    });

    answers.lock().unwrap().sort();

    for i in answers.lock().unwrap().iter() {
        println!("{:<3} {:<14} {:#?}", i.0, i.1, i.2);
    }
}
