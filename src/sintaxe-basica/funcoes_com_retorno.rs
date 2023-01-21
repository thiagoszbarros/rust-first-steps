//obrigatoriamente uma funcao que retorno algo deve ter este retorno tipado
//aqui o uso do return sera opcional
//para que o valor de a + b seja retornado nao deve ser utilizado o ";"
//o ";" identifica o trecho de código com um expressao
//usar o ";" fara a funcao retornar () enquanto espera retornar um i32

fn soma(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("{} + {} = {}", 2, 2, soma(2, 2));
}
