use regex::Regex;

fn extract_dates(text: &str) -> Vec<String>{
    let padrao = Regex::new(r"\b\d{2}/\d{2}/\d{4}\b").unwrap();
    let mut resultado : Vec<String>= Vec::new();
    for data in padrao.find_iter(text){
        resultado.push(data.as_str().to_string());
    }
    resultado
}

fn main(){
    let text = "Hoje é 04/12/2024, amanhã será 05/12/2024.";
    let dates = extract_dates(text);
    println!("{:?}", dates);
}