fn main() {
    other_function();

    let x = sum(3, 2);
    println!("{}", x);
    
    let y = return_explicit(4, 7);
    println!("{}", y);
    
    let z = return_values();
    println!("{} - {}", z.0, z.1);
}

// função sem retorno
fn other_function() {
    println!("Testando função sem retorno");
}

// função com retorno
fn sum(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

fn return_explicit(num1: i32, num2: i32) -> i32 {
    return num1 + num2;
}

fn return_values() -> (i32, f64) {
	return (17, 3.14);
}