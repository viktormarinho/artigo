use artigo::templates::render;
use serde_json::json;



fn main() {
    let res = render("Olá, meu nome é {{ nome |> upper }}.".to_string(), json!({ "nome": "ventura" }));
    println!("{res}");
}
