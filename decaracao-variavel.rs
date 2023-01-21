
#[path = "./utils/tipo_da_variavel.rs"] mod tipo_da_variavel;

fn main(){
	let numero_inteiro:u8 = 128;
	println!("variavel = {}, tamanho em byte = {}, tipo = {}", 
	numero_inteiro, 
	std::mem::size_of_val(&numero_inteiro), 
	tipo_da_variavel::tipo(numero_inteiro));
	
	let numero_decimal:f32 = 2.5;
	println!("variavel = {}, tamanho em byte = {}, tipo = {}", 
	numero_decimal, 
	std::mem::size_of_val(&numero_decimal), 
	tipo_da_variavel::tipo(numero_decimal));

	let letra:char = 'R';
	println!("variavel = {}, tamanho em byte = {}, tipo = {}", 
	letra, 
	std::mem::size_of_val(&letra), 
	tipo_da_variavel::tipo(letra));

	let mut booleano:bool = true; //por padrao no rust as variaveis tem seus valores imutaveis, a palavra reservada mut permite a reatribuicao do valor da variavel
	println!("variavel = {}, tamanho em byte = {}, tipo = {}", 
	booleano, 
	std::mem::size_of_val(&booleano), 
	tipo_da_variavel::tipo(booleano));
	booleano = false;
	println!("Novo valor da variavel mutavel = {}, tamanho em byte = {}, tipo = {}", 
	booleano, 
	std::mem::size_of_val(&booleano), 
	tipo_da_variavel::tipo(booleano))
}
