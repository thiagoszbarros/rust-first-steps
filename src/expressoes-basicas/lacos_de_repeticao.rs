fn repeticoes() {
    let multiplicador:u8 = 5;
    let mut contador:u8 = 0;

    println!("Tabuada do 5 com while");
    while contador < 10 {
        contador+=1;
        println!("{} x {} = {}", multiplicador, contador, contador * multiplicador);
    }

    contador = 0;
    println!("Tabuada do 5 com loop");
    loop {
        contador+=1;
        println!("{} x {} = {}", multiplicador, contador, contador * multiplicador);
        if contador == 10 { 
            break;
        }
    }
    println!("Tabuada do 5 com for");
    for i in 1..11{
        println!("{} x {} = {}", multiplicador, i, i * multiplicador);
    }
}

fn main (){
    repeticoes();
}