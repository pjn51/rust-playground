// imports
use dotenv::dotenv;
use std::env;
use reqwest::blocking::Client;

// #[tokio::main]
// async fn main() -> Result<()> {
//     let response = reqwest::get("https://jsonplaceholder.typicode.com/todos/1")
//         .await?
//         .text()
//         .await?;
//     println!("{}", response);
//     Ok(())
// }


async fn caller(api_key:String, word:String) -> Result<(), E> {
    // url construction
    let mut site_url = String::from("https://www.dictionaryapi.com/api/v3/references/collegiate/json/word_val?key=key_val");
    site_url = site_url.replace("word_val", &word);
    site_url = site_url.replace("key_val", &api_key);
    println!("Getting definition for '{}'", word);

    // api call
    let response = Client::new()
        .get(&site_url)
        .send()?
        .text()?;

    println!("\n{}\n", response);

    Ok(())
}

fn main() {
    dotenv().ok();
    let api_key = env::var("API_KEY").unwrap();
    let args: Vec<String> = env::args().collect();
    let word = &args[1];

    let result = caller(api_key, word.to_string());

    //dict_url.push_str(api_key);
   
}
