use std::fmt::Display;

pub enum Secret {
    Account(AccountObj),
    APIKey(APIObj),
}

#[derive(Debug)]
pub struct AccountObj {
    account_name: String,
    user_name: String,
    password: String,
}

#[derive(Debug)]
pub struct APIObj {
    api_name: String,
    api_service: String,
    user_name: String,
    api_key: String,
}

impl Display for AccountObj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = format!(
            "Name: {}\nUser: {}\nPass: {}\n",
            self.account_name, self.user_name, self.password
        );
        write!(f, "{output}")
    }
}

impl Display for APIObj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = format!(
            "Name: {}\nService: {}\nUser: {}\nKey: {}\n",
            self.api_name, self.api_service, self.user_name, self.api_key
        );
        write!(f, "{output}")
    }
}
