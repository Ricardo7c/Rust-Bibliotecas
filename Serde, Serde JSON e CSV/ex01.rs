use serde::Serialize;
use serde_json;

#[derive(Serialize)]
struct User {
    id: u32,
    nome: String,
    email: String
}

fn main(){
    let p1 = User {
        id: 2,
        nome: "Ricardo".to_string(),
        email: "Ricardo@rust.com".to_string()
    };

    let json = serde_json::to_string(&p1).unwrap();
    println!("Json: {}", json)
}

