use std::env;

fn count_occurences(search_str:&str, target:char) -> i32 {
    let mut n = 0;
    //let char_vec: Vec<char> = search_str.chars().collect();
    for c in search_str.to_lowercase().chars() {
        if c == target {
            n += 1;
        }
    }
    return n
}

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();

    let search_str = &args[1];
    let target = &args[2].chars().next().unwrap();
    let n = count_occurences(search_str, *target);
    println!("{} appears {} times in {}", target, n, search_str);
}
