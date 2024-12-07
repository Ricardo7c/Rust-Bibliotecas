use serde::Serialize;
use serde_json;

#[derive(Serialize)]
struct Person{
    #[serde(rename = "Nome")]
    first_name: String,

    #[serde(rename = "Sobrenome")]
    last_name: String,
    
    age: u32
}

fn main(){
    let p1 = Person{
        first_name: "Rust".to_string(),
        last_name: "lang".to_string(),
        age: 11
    };

    let serialized = serde_json::to_string(&p1).unwrap();
    println!("{}", serialized);
}