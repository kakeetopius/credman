use std::fmt::Display;

pub trait Secret {
    fn print(&self)
    where
        Self: Display,
    {
        println!("{}", self);
    }

    //print_json()
    //add_to_db() func
    //print_field() -- to print a particular field from struct
    //
}

#[derive(Debug)]
pub struct Account {
    pub account_name: String,
    pub user_name: String,
    pub password: String,
}

#[derive(Debug)]
pub struct API {
    pub api_name: String,
    pub api_service: String,
    pub user_name: String,
    pub api_key: String,
}

impl Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = format!(
            "Name: {}\nUser: {}\nPass: {}\n",
            self.account_name, self.user_name, self.password
        );
        write!(f, "{output}")
    }
}

impl Display for API {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = format!(
            "Name: {}\nService: {}\nUser: {}\nKey: {}\n",
            self.api_name, self.api_service, self.user_name, self.api_key
        );
        write!(f, "{output}")
    }
}

impl Secret for Account {}
impl Secret for API {}
