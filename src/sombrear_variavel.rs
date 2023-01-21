fn main(){
    let variavel_generica:&str = "escopo externo";
    println!("valor: {}", variavel_generica);
    {
        let variavel_generica:&str = "escopo interno";
        println!("valor: {}", variavel_generica);
    }
    println!("valor: {}", variavel_generica)
}