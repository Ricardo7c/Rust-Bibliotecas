use regex::Regex;

fn extract_dates(text: &str) -> Vec<String>{
    let padrao = Regex::new(r"\b\d{2}/\d{2}/\d{4}\b").unwrap();
    padrao.find_iter(text)
        .map(|x| x.as_str().to_string()).collect()
}

fn main(){
    let text = "Hoje é 04/12/2024, amanhã será 05/12/2024.";
    let dates = extract_dates(text);
    println!("{:?}", dates);
}