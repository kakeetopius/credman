use clap::Parser;
use credman::util::*;
use credman::*;

fn main() {
    let _cli = argparser::Cman::parse();

    // let dbfile = "/home/pius/.creds.db".to_string();
    let con = db::get_db_con();

    match con {
        Ok(_) => println!("Got db con"),
        Err(e) => eprintln!("{}", e),
    }
}
