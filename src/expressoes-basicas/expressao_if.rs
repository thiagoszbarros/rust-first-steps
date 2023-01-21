//retorna o valor do if diretamente
fn verifica_maioridade(idade: u8) -> bool {
    // if idade >= 18 {
    //     true
    // } else {
    //     false
    // }

    //ou assim

    if idade >= 18 {
        return true
    }
    return false
}

//atribui o retono do if a uma variavel e retorna a variavel
fn verifica_adulto(idade: u8) -> &'static str {
    let maioridade: bool = verifica_maioridade(idade);
    let adulto: &str = if maioridade == true { "Sim" } else { "Não" };
    adulto
}

fn main() {
    let adulto = verifica_adulto(19);
    println!("John Doe é adulto? {}", adulto);
}
