use std::io;

fn main() {
    const NINE_FIVE: f64 = 9.0/5.0;
    const FIVE_NINE: f64 = 5.0/9.0;
    
    let temp: f64 = loop {
        let mut str = String::new();
    
        println!("please input tempeture");
        io::stdin().read_line(&mut str).expect("Error!");

        match str.trim().parse() {
            Ok(num) => break num,
            Err(_) => continue,
        }
    };

    let from_degree: Degree = loop {
        let mut str = String::new();

        println!("From what degree? [C, F, K]");
        io::stdin().read_line(&mut str).expect("Error!!");

        match &str.trim().to_lowercase()[..] {
            "c" => break Degree::CELCIUS,
            "f" => break Degree::FAHRENHEIT,
            "k" => break Degree::KELVIN,
            _ => continue,
        }
    };

    let to_degree: Degree = loop {
        let mut str = String::new();

        println!("Convert to what degree? [C, F, K]");
        io::stdin().read_line(&mut str).expect("Error!!");

        match &str.trim().to_lowercase()[..] {
            "c" => break Degree::CELCIUS,
            "f" => break Degree::FAHRENHEIT,
            "k" => break Degree::KELVIN,
            _ => continue,
        }
    };

    let result: String = match from_degree {
        Degree::CELCIUS => match to_degree {
            Degree::CELCIUS => temp.to_string() + "*C",
            Degree::FAHRENHEIT => (temp * NINE_FIVE + 32.0).to_string() + "*F",
            Degree::KELVIN => (temp + 273.15).to_string() + "K",
        },
        Degree::FAHRENHEIT => match to_degree {
            Degree::CELCIUS => ((temp - 32.0) * FIVE_NINE).to_string() + "*C",
            Degree::FAHRENHEIT => temp.to_string() + "*F",
            Degree::KELVIN => ((temp - 32.0) * FIVE_NINE + 273.15).to_string() + "K",
        },
        Degree::KELVIN => match to_degree {
            Degree::CELCIUS => (temp - 273.15).to_string() + "*C",
            Degree::FAHRENHEIT => ((temp - 273.15) * NINE_FIVE + 32.0).to_string() + "*F",
            Degree::KELVIN => temp.to_string() + "K",
        },
    };

    println!("{result}");
}


#[derive(PartialEq)]
enum Degree {
    CELCIUS,
    FAHRENHEIT,
    KELVIN,
}
