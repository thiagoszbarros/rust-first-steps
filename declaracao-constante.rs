#[path = "./utils/obter-tipo.rs"] mod obter_tipo;

const PI:f32 = 3.14; //uma constante nao tem espaco reservado na memoria para si
//em tempo de compilacao o valor da constante sera substituido em todos os lugares do codigo onde a constante for chamada

fn main(){
    println!("constante = {}, tamanho em byte = {}, tipo = {}", 
	PI, 
	std::mem::size_of_val(&PI), 
	obter_tipo::tipo(PI));
}