static mut VARIAVEL_GLOBAL:u8 = 1; // Pode ser mut ou não

fn soma(a:i32, b:i32) -> i32 {
	println!("{} + {} = {}", a, b, a + b);
	a + b
}

//Entendendo Shadowing em escopos
fn shadowing() {
	let a:u8 = 123;

	{
		let a:i16 = 777;
		println!("a dentro = {}", a);
	}

	println!("a fora = {}", a);
}

fn escopo() {

	unsafe {
		println!("Variável global = {}, tamanho = {}", VARIAVEL_GLOBAL, std::mem::size_of_val(&VARIAVEL_GLOBAL));
	}

	const PI:f32 = 3.14; // Também pode ser acessado globalmente
	println!("PI = {}, tamanho = {}", PI, std::mem::size_of_val(&PI));

	let variavel:i32 = 300;
	println!("Variável = {}, tamanho = {}", variavel, std::mem::size_of_val(&variavel));

	let decimal:f32 = 2.5;
	println!("decimal = {}", decimal);

	let booleano:bool = false;
	println!("Booleano = {}, Tamanho booleano = {}", booleano, std::mem::size_of_val(&booleano));

	let letra:char = 'C';
	println!("Tamanho char = {}", std::mem::size_of_val(&letra));
}

fn main() {
	shadowing();

	soma(1, 2);

	escopo();

	condicionais();

	repeticoes();

	ownership();

	pattern_matching();

	erros();
}

fn condicionais() {
	let idade: u8 = 19;
	let responsavel_autorizou: bool = true;
	let is_maior = idade >= 18;

	if is_maior {
		println!("Pode entrar na balada!");
	} else if idade > 16 && responsavel_autorizou {
		println!("Pode entrar com assinatura do responsável!");
	} else {
		println!("Não pode entrar na balada!");
	};

	let condicao = if idade >= 18 { "maior" } else { "menor" };

	println!("É {} de idade", condicao);

	let linguagem = "PHP";
	let proposito = match linguagem {
		"PHP" => "Web",
		"Kotlin" => "Android",
		"Python" => "Data Science",
		_ => "Desconhecido",
	};

	println!("O proposito de {} é {}", linguagem, proposito);
}

fn repeticoes() {

	let multiplicador:u8 = 5;

	let mut contador:u8 = 0;

	while contador < 10 {
		contador += 1;

		if contador == 5 {
			continue;
		}

		println!("{} x {} = {}", multiplicador, contador, multiplicador * contador);
	}

	contador = 0;
	loop {
		contador += 1;

		println!("{} x {} = {}", multiplicador, contador, multiplicador * contador);

		if contador == 10 {
			break;
		}
	}

	for i in 1..=10 { // ou 1..11
		println!("{} x {} = {}", multiplicador, i, multiplicador * i);
	}

}

fn ownership() {

	let uma_string = String::from("Cristopher"); // Pode utilizar mut

	roubar(&uma_string);

	println!("{}", uma_string)
}

fn roubar(string: &String) {

	println!("{}", string)

}

fn pattern_matching() {

	for x in 1..21 {
		println!("{}: {}", x, match x {
			1 => "Pouco",
			2 | 3 => "Um pouquinho",
			4..=10 => "Um bocado",
			_ if x % 2 == 0 => "É uma boa quantidade",
			_ => "Muito"
		});
	}

}

fn erros() {


	match result() {
		Ok(s) => println!("String de sucesso = {}", s),
		Err(numero) => println!("Código de erro = {}", numero),
	};


}

fn result() -> Result<String, u8> {
	//Ok(String::from("Tudo deu certo"))
	Err(42)
}