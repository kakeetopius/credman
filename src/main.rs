use clap::Parser;
use credman::util::argparser;
use credman::util::passgen;

fn main() {
    let _cli = argparser::Cman::parse();
    // let uname = match ioutils::get_terminal_input("Enter User Name") {
    //     Ok(num) => num,
    //     Err(e) => {
    //         eprintln!("Error: {e}");
    //         return;
    //     }
    // };
    //
    // let aname = match ioutils::get_terminal_input("Enter Account Name") {
    //     Ok(num) => num,
    //     Err(e) => {
    //         eprintln!("Error: {e}");
    //         return;
    //     }
    // };
    //
    let pass = match passgen::get_random_pass() {
        Ok(pass) => pass,
        Err(e) => {
            eprintln!("Error: {e}");
            return;
        }
    };

    println!("Here's your password: {}", pass)
    // let account = objects::Account {
    //     account_name: aname,
    //     user_name: uname,
    //     password: pass,
    // };

    // account.print()
}
