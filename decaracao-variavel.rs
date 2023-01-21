
#[path = "./utils/tipo_da_variavel.rs"] mod tipo_da_variavel;

fn main(){
	let numero_inteiro:u8 = 128;
	println!("variavel = {}, tamanho em byte = {}, tipo = {}", 
	numero_inteiro, 
	std::mem::size_of_val(&numero_inteiro), 
	tipo_da_variavel::tipo(numero_inteiro))

}
