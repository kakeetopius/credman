use credman::util::ioutils;
use credman::util::passgen;

fn main() {
    if let Err(e) = ioutils::print_prompt("Enter a number") {
        eprintln!("{e}");
        return;
    }

    let number = match ioutils::get_terminal_input() {
        Ok(num) => num,
        Err(e) => {
            eprintln!("Error: {e}");
            return;
        }
    };

    let num = match number.trim().parse::<i32>() {
        Err(err) => {
            eprintln!("{err}");
            return;
        }
        Ok(num) => num,
    };

    println!("You entered: {num}");
    let pass = passgen::get_random_pass();
    match pass {
        Err(err) => eprintln!("Error: {}", err),
        Ok(pass) => println!("This is your password: {}", pass),
    }
}
