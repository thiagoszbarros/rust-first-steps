fn propriedade(){
    let uma_string = String::from("Thiago");
    pede_uma_var(uma_string);
    println!("{}", uma_string);
}

fn pede_uma_var(string: String){
    println!("{}", string);
}

fn main(){
    propriedade();
}