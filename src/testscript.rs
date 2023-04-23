// let's see how fast we can loop.
// use std::time::Instant;
// use num_format::{Locale, ToFormattedString};

// extern crate num_format;

// fn looper(n: i32) {
//     for i in 0..=n {
//         if i % (n / 4) == 0 {
//             println!("{}% done!", i / (n / 100));
//         }
//     }
// }

// fn main() {
//     let n = 1000000;
//     let start = Instant::now();
//     println!("Hello, world!");
//     looper(n);
//     let duration = start.elapsed();
//     let formatted_n = n.to_formatted_string(&Locale::en);
//     println!("Took {:?} for {} iterations.", duration, formatted_n);
// }

use num_format::{Locale, ToFormattedString};

fn main() {
    let n = 1000000;
    let formatted_n = n.to_formatted_string(&Locale::en);
    println!("Formatted integer: {}", formatted_n);
}
