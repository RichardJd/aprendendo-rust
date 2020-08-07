fn main() {
    let mut number = 0;

    loop {
        print!("{} - ", number);
        if number == 10 {
            break;
        }

        number += 1;
    }

    while number <= 10 {
        print!("{} - ", number);
        number += 1;
    }

    let a = [10, 20, 30, 40, 50];

    for element in a.iter().rev() {
        println!("the value is: {}", element);
    }
}
