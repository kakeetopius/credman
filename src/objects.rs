use crate::util::argparser::FieldType;
use crate::util::ioutils::print_result;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize)]
pub enum Secret {
    Account(AccountObj),
    API(APIObj),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountObj {
    pub account_name: String,
    pub user_name: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
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

impl Display for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Account(acc) => write!(f, "{}", acc),
            Self::API(api) => write!(f, "{}", api),
        }
    }
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

impl AccountObj {
    fn print(&self) {
        print_result("Name", &self.account_name);
        print_result("User", &self.user_name);
        print_result("Pass", &self.password);
        println!();
    }

    fn print_field(&self, field: FieldType) {
        match field {
            FieldType::User => print_result("User", &self.user_name),
            FieldType::Secname => print_result("Name", &self.account_name),
            FieldType::Pass => print_result("Pass", &self.password),
            _ => return,
        }
    }

    fn print_json(&self) {
        let json = match serde_json::to_string_pretty(self) {
            Err(_) => "".to_string(),
            Ok(j) => j,
        };

        println!("{}", json);
    }
}

impl APIObj {
    fn print(&self) {
        print_result("Name", &self.api_name);
        print_result("User", &self.user_name);
        print_result("Desc", &self.api_service);
        print_result("Key", &self.api_key);
        println!();
    }

    fn print_field(&self, field: FieldType) {
        match field {
            FieldType::Secname => print_result("Name", &self.api_name),
            FieldType::Service => print_result("Desc", &self.api_service),
            FieldType::User => print_result("User", &self.user_name),
            FieldType::Key => print_result("Key", &self.api_key),
            _ => return,
        }
    }

    fn print_json(&self) {
        let json = match serde_json::to_string(self) {
            Err(_) => "".to_string(),
            Ok(j) => j,
        };

        println!("{}", json);
    }
}

impl Secret {
    pub fn print_field(&self, field: FieldType) {
        match self {
            Self::API(apiobj) => apiobj.print_field(field),
            Self::Account(accountobj) => accountobj.print_field(field),
        }
    }

    pub fn print(&self) {
        match self {
            Self::API(api) => api.print(),
            Self::Account(acc) => acc.print(),
        }
    }

    pub fn print_json(&self) {
        match self {
            Self::API(api) => api.print_json(),
            Self::Account(acc) => acc.print_json(),
        }
    }
}
