#[path = "../utils/obter_tipo.rs"] mod obter_tipo;

static VARIAVEL_GLOBAL:&str = "Variavel global"; 
// static mut VARIAVEL_GLOBAL:&str = "Variavel global"; 
//por padrao, o rust nao permite o uso de variaveis globais mutaveis 
//para ignorar isso se faz necessario utilizar a anotacao unfase{} para definir o bloco de codigo mo inseguro

fn main(){
	//unsafe{
		println!("variavel global = {}, tamanho em byte = {}, tipo = {}", 
		VARIAVEL_GLOBAL, 
		std::mem::size_of_val(&VARIAVEL_GLOBAL), 
		obter_tipo::tipo(VARIAVEL_GLOBAL));
	//}
}