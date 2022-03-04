// io permite input de usuário e sua impressão
use std::io;
use std::{thread, time};

fn main() {

    let ten_millis = time::Duration::from_millis(2000);

    let scale = loop {

        println!("Welcome, enter the temperature scale to be converted");

        let mut scale_temp = String::new();

        io::stdin()
        .read_line(&mut scale_temp)
        .expect("Failed to read line");

        if scale_temp.trim() == "Celsius" {
            println!("Ok, the conversion is from Celsius to Fahrenheint");
            break scale_temp;
        } else if scale_temp.trim() == "Fahrenheit" {
            println!("Ok, the conversion is from Fahrenheint to Celsius");
            break scale_temp;
        } else {
            println!("This is not a valid scale!");
            continue
        }
    };

    let number = loop {

        println!("Enter the chosen scale number.");

        let mut number_to_convert = String::new();

        io::stdin()
        .read_line(&mut number_to_convert)
        .expect("Failed to read line");

        let number_to_convert: f32 = match number_to_convert.trim().parse() {
            // parse retorna um tipo Result e um enum que possui Ok ou Err
            Ok(num) => {
                println!("Ok, this is a valid number!");
                num
            },
            // o _ indica que estamos falando de todos os erros possíveis
            Err(_) => continue,
        };
        println!("The number is {}",number_to_convert);
        break number_to_convert;
    };

    let final_scale = scale.trim();

    if final_scale == "Celsius" {

      let final_number = number * 1.8 + 32.0;

      println!("The Input in Fahrenheit is {} Celsius...", final_number);
      thread::sleep(ten_millis);

    } else if final_scale == "Fahrenheit" {
        let final_number =  (5.0/9.0) * (number-32.0);

      println!("The Input in Celsius is {} Fahrenheit ...", final_number);
      thread::sleep(ten_millis);
    }



}
