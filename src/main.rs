// io permite input de usuário e sua impressão
use std::io;

fn main() {

    loop {

        println!("Enter the temperature scale to be converted");

        let mut scale_temp = String::new();

        io::stdin()
        .read_line(&mut scale_temp)
        .expect("Failed to read line");

        let scale_temp = scale_temp.trim();

        if scale_temp == "Celsius" {
            println!("Ok");
            break
        } else if scale_temp == "Fahrenheit" {
            println!("Ok");
            break
        } else {
            println!("This is not a valid scale!");
            continue
        }
    }

    loop {

        println!("Enter the chosen scale number.");

        let mut number_to_convert = String::new();

        io::stdin()
        .read_line(&mut number_to_convert)
        .expect("Failed to read line");

        let number_to_convert: u32 = match number_to_convert.trim().parse() {
            // parse retorna um tipo Result e um enum que possui Ok ou Err
            Ok(num) => {
                println!("Ok, this is a valid number!");
                num
            },
            // o _ indica que estamos falando de todos os erros possíveis
            Err(_) => continue,
        };
        println!("The number is {}",number_to_convert);
        break
    }


}
