//! Projeto de exemplo sobre tipos de variáveis

/// Função principal do projeto
fn main() {
    let signed: i32 = -10;
    let unsigned: u32 = 5;
    // let unsignedWithErr: u32 = -3; // Vai causar um erro por tentarmos adicionar um numero negativo.

    println!("Integers: {}, {}", signed, unsigned);

    let float: f64 = -3.14;
    println!("Float: {}", float);

    let sum = 5 + 3; // Soma
    println!("Soma: 5 + 3 = {}", sum);

    let difference = 2.3 - 1.2; // Subtracao
    println!("Subtracao: 2.3 - 1.2 = {}", difference);

    let product = 2 * 5; // Multiplicacao
    println!("Multiplicacao: 2 * 5 = {}", product);

    let quotient = 10 / 5; // Divisao
    println!("Divisao: 10 / 5 = {}", quotient);

    let remainder = 43 % 5; // Resto
    println!("Resto: 43 % 5 = {}", remainder);

    let c: char = 'R';
    println!("Char: {}", c);

    let is_true: bool = true;
    println!("Boolean: {}", is_true);

    let meses = [
        "Janeiro",
        "Fevereiro",
        "Março",
        "Abril",
        "Maio",
        "Junho",
        "Julho",
        "Agosto",
        "Setembro",
        "Outubro",
        "Novembro",
        "Dezembro",
    ];
    println!("Array: {}", meses[0]);

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("O valor de x eh: {}", x);
    println!("O valor de y eh: {}", y);
    println!("O valor de z eh: {}", z);
}
