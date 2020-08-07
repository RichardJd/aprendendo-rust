fn main() {
    let number = 5.5;

    if number < 5.0 {
        println!("Aluno aprovado!");
    } else if number >= 5.0 && number < 7.0 {
        println!("Aluno de recuperação!");
    } else {
        println!("Aluno reprovado!");
    }
}
