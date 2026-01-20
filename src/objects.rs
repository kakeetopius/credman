use std::fmt::Display;

pub enum Secret {
    Account(AccountObj),
    API(APIObj),
}

#[derive(Debug)]
pub struct AccountObj {
    pub account_name: String,
    pub user_name: String,
    pub password: String,
}

#[derive(Debug)]
pub struct APIObj {
    pub api_name: String,
    pub api_service: String,
    pub user_name: String,
    pub api_key: String,
}

impl From<AccountObj> for Secret {
    fn from(value: AccountObj) -> Self {
        Secret::Account(value)
    }
}

impl From<APIObj> for Secret {
    fn from(value: APIObj) -> Self {
        Secret::API(value)
    }
}

impl Secret {
    pub fn print(&self) {
        match self {
            Self::Account(acc) => println!("{}", acc),
            Self::API(api) => println!("{}", api),
        }
    }

    //print_json()
    //add_to_db()
    //print_field()
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
