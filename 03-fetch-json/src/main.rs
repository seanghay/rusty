use curl::easy::Easy;
use serde::{Deserialize, Serialize};
use serde_json::{Error};

#[derive(Debug, Serialize, Deserialize)]
struct Post {
    id: u32,
    title: String,
    body: String,
}

fn main() -> Result<(), Error> {
    fetch_posts()
}

fn fetch_posts() -> Result<(), Error> {

    let mut json = Vec::new();
    let mut easy = Easy::new();

    easy.url("https://jsonplaceholder.typicode.com/posts").unwrap();

    {
        let mut transfer = easy.transfer();
        transfer
            .write_function(|data| {
                json.extend_from_slice(data);
                Ok(data.len())
            })
            .unwrap();

        transfer.perform().unwrap();
    }

    assert_eq!(200, easy.response_code().unwrap());
    let data: Vec<Post> = serde_json::from_slice(&json)?;

    for post in data {
        println!("id={}", post.id);
    }

    Ok(())
}
