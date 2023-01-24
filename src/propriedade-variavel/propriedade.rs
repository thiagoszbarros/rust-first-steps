fn propriedade(){
    let mut uma_string = String::from("Thiago");
    pede_uma_var(&mut uma_string);
    println!("{}", uma_string);
}

fn pede_uma_var(string: &mut String){
    string.push_str(" Barros");
    println!("{}", string);
}

fn main(){
    propriedade();
}

// deve ser passada uma referencia ao valor original gravado na memoria
// caso haja necessidade de tratar o valor da variavel de qualquer maneira,
// a refarencia de ser passada da e recebida com mutavel, bem como a variavel
// original deve ser mutavel 
