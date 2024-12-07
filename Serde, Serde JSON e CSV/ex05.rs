use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct User{
    id: u32,
    nome: String,
    email: String
}

fn main(){
    let u1 = User{
        id: 1,
        nome: "Bob".to_string(),
        email: "bob@rust.com".to_string(),
    };
    let u2 = User{
        id: 2,
        nome: "Pedro".to_string(),
        email: "pedro@rust.com".to_string(),
    };
    let u3 = User{
        id: 3,
        nome: "Maria".to_string(),
        email: "maria@rust.com".to_string(),
    };

    let vetor = vec![u1, u2, u3];

    // Transforma o vetor de Users em String no formato json
    let serialized = serde_json::to_string(&vetor).unwrap();
    println!("{}", serialized);

    // Transforma a string json de volta para vetor
    let deserialized:Vec<User> = serde_json::from_str(&serialized).unwrap();

    for cada in deserialized{
        println!("User: {} ({})", cada.nome, cada.email)
    }


}
