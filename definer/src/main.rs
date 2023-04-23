// imports
use dotenv::dotenv;
use std::env;

fn api_call(api_url:&str, api_key:String) {
    let api_url = api_url.to_owned() + &api_key;

    
}

fn main() {
    let dict_url = "https://www.dictionaryapi.com/api/v3/references/collegiate/json/voluminous?key=";
    dotenv().ok();
    let api_key = env::var("API_KEY").unwrap();

    api_call(dict_url, api_key);

    //dict_url.push_str(api_key);
   
}
