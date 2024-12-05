use serde::Deserialize;
use serde_json;


#[derive(Deserialize)]
struct User {
    id: u32,
    nome: String,
    email: String
}

fn main(){
    let json = r#"{"id":2,"nome":"Bob","email":"bob@example.com"}"#;

    let p1: User = serde_json::from_str(json).unwrap();

    println!("Id: {}", p1.id);
    println!("Nome: {}", p1.nome);
    println!("E-mail: {}", p1.email);
}