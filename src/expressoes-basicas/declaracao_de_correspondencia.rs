fn main() {
    let dia_semana = "Sábado";
    let dia_util = match dia_semana {
        "Segunda" => "Sim",
        "Terça" => "Sim",
        "Quarta" => "Sim",
        "Quinta" => "Sim",
        "Sexta" => "Sim",
        "Sábado" => "Não",
        "Domingo" => "Não",
        _ => "Não",
    };
    println!("{} é dia útil? {}", dia_semana, dia_util);
}
