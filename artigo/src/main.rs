use artigo::templates::render;
use serde_json::json;



fn main() {
    let res = render("Hello Viktor, my name is {{ name |> upper }}. Your name is {{ doido }}.".to_string(), json!({ "name": "Yo", "doido": "zimbas" }));
    println!("{res}");
}
