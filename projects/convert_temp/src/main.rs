use std::io;

fn main() {
    println!("**** Welcome to App converting Fahrenheit and Celsius *****");

    'repeat_loop: loop {

        println!("press 1 to convert to Fahrenheit");
        println!("press 2 to convert to Celcius");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut  choice)
            .expect("Failed to read choice line ");

        let choice: i32 = choice
            .trim()
            .parse()
            .expect("choice={choice} entered was not a number");

        println!("choice is {choice}");

        println!("Insert temperature : ");
        let mut fc_val = String::new();
        io::stdin()
            .read_line(&mut fc_val)
            .expect("Failed to read  fc_val line ");

        let fc_val: f64 = fc_val
            .trim()
            .parse()
            .expect("fc_val={fc_val} entered was not a float");
        
        println!("fc_val is {fc_val}");

        let mut temperature: f64 ;
        if choice == 1 {
            // Celcius to Fahrenheit
            let temperature = convert_c_2_f(fc_val);
            println!("temperature of {fc_val}°C is {temperature}F");
        } else if choice == 2 {
            // Fahrenheit to Celcius
            let temperature = convert_f_2_c(fc_val);
            println!("temperature of {fc_val}F is {temperature}°C");
        } else {
            println!("Please press integer 1 or 2");
        }

        println!("Will you repeat the operation");
        println!("press (Y) to Yes");
        println!("press (N) to N");

        'rep_choice_loop: loop {

            let mut repeat_choice = String::new();
            io::stdin()
                .read_line(&mut repeat_choice)
                .expect("Failed to read  repeat_choice line ");

            println!("repeat_choice is {repeat_choice}");
            let mut tmp = [0u8; 4];
            let your_string = 'y'.encode_utf8(&mut tmp);

            println!("your_string is {your_string}");

            if repeat_choice == "Y".to_string() || repeat_choice == "yes" || repeat_choice == 'y'.to_string() {
                println!("1");
                break 'repeat_loop;
            } else if repeat_choice == "N".to_string() || repeat_choice == 'n'.to_string() {
                println!("2");
                break;
            } else {
                println!("Please press Y(es) or N(o)");
                break 'rep_choice_loop;
            }
        }
    }
    

}

fn convert_f_2_c(f_val: f64) -> f64{
    // f_val is the temperature value in Fahrenheit
    // f_val is a float value
    let c_val: f64 = 5.0/9.0 * (f_val - 32.0);
    c_val
}

fn convert_c_2_f(c_val: f64) -> f64{
    // c_val is the temperature value in Celcius
    // c_val is a float value
    let f_val: f64 = 9.0/5.0 * c_val + 32.0;
    f_val
}
