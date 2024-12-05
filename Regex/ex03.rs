use regex::Regex;

fn clean_string(texto: &str) -> String{
    let padrao = Regex::new(r"[^a-zA-Z0-9\s]+").unwrap();
    padrao.replace_all(texto, "").to_string()
}

fn main(){
    let texto = "Pro=gra+-man*&.do em Ru//st";
    let limpo = clean_string(texto);
    println!("{}", limpo);
}