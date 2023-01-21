
#[path = "./utils/tipo_da_variavel.rs"] mod tipo_da_variavel;

fn main(){
	let variavel:u8 = 128;
	println!("variavel = {}, tamanho em byte = {}, tipo = {}", 
	variavel, 
	std::mem::size_of_val(&variavel), 
	tipo_da_variavel::tipo(variavel))
}
