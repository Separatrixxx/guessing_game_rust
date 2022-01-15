use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Угадай число!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Пожалуйста, введите число :)");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Ваше предположение: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Слишком маленькое!"),
            Ordering::Greater => println!("Слишком большое!"),
            Ordering::Equal => {
                println!("Вы угадали, загаданное число: {}!", guess);
                break;
            }
        }
    }
}
