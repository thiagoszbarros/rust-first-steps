fn correspondencia_de_padroes() {
    for x in 1..=20 {
        println!("{} -> {}", x, match x {
            1 => "Pouco",
            2 | 3 => "Bom e demais",
            4..=10 => "Entre 4 e 10",
            _ if x % 2 == 0 => "Um par maior que 10",
            _ => "Um impar maior que 10"
        });
    }
}

fn main() {
    correspondencia_de_padroes();
}