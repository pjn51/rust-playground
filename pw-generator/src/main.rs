// intent: generate passwords based on user input
use rand::Rng;
use std::env;

fn random_int(min:i32, max:i32) -> i32 {
    let mut rng = rand::thread_rng();
    let random_int = rng.gen_range(min..=max); // n of pw-eligible unicode chars
    return random_int
}

fn fetch_char(index: i32) -> Option<char> {
    let index:usize = index.try_into().unwrap();
    let all_chars = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
                     'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
                     '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', '+', '=', '[', '{', ']', '}', '\\', '|', ';', ':', '\'', '\"', ',', '<', '.', '>', '/', '?'];
    //println!("{}", all_chars.len());
    return all_chars.get(index).map(|c| *c);
} 

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    let pw_length = &args[1].trim().parse().expect("failure");

    let mut pw = String::new();

    for i in 0..=*pw_length {
        let n = random_int(0,82);
        let character = fetch_char(n);
        if let Some(c) = character {
            pw.push(c)
        }
    

    }

    println!("{}", pw);

}
