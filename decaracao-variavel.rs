use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main(){
	let variavel:u8 = 128;
	println!("variavel = {}, tamanho em byte = {}, tipo = {}", 
	variavel, 
	std::mem::size_of_val(&variavel), 
	type_of(variavel))
}