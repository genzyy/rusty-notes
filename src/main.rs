use std::collections::HashMap;
use std::io;

mod helpers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut note: String = String::new();
    io::stdin().read_line(&mut note)?;

    let mut note_map: HashMap<String, String> = HashMap::new();
    note_map.insert(helpers::convert_to_string("body"), note);

    return helpers::post_request("https://engram.xyzdigital.com/api/notes", &note_map).await;

    // return Ok(());
}
