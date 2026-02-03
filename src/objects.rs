use crate::util::argparser::FieldType;
use crate::util::ioutils::print_result;

use serde::{Deserialize, Serialize};

use std::fmt::Display;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Secret {
    Account(AccountObj),
    API(APIObj),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountObj {
    pub account_name: String,
    pub user_name: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIObj {
    pub api_name: String,
    pub description: String,
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
        write!(f, "{}", self.account_name)
    }
}

impl Display for APIObj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.api_name)
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

    fn get_field(&self, field: FieldType) -> String {
        match field {
            FieldType::User => self.user_name.clone(),
            FieldType::Secname => self.account_name.clone(),
            FieldType::Pass => self.password.clone(),
            _ => "".to_string(),
        }
    }

    fn get_json_str(&self) -> String {
        let json = match serde_json::to_string_pretty(self) {
            Err(_) => "".to_string(),
            Ok(j) => j,
        };

        json
    }

    fn get_field_json_str(&self, field: FieldType) -> String {
        let json_str = match field {
            FieldType::User => serde_json::json!({"User": self.user_name}),
            FieldType::Secname => serde_json::json!({"Name": self.account_name}),
            FieldType::Pass => serde_json::json!({"Pass": self.password}),
            _ => return "".to_string(),
        };

        json_str.to_string()
    }
}

impl APIObj {
    fn print(&self) {
        print_result("Name", &self.api_name);
        print_result("User", &self.user_name);
        print_result("Desc", &self.description);
        print_result("Key", &self.api_key);
        println!();
    }

    fn print_field(&self, field: FieldType) {
        match field {
            FieldType::Secname => print_result("Name", &self.api_name),
            FieldType::Desc => print_result("Desc", &self.description),
            FieldType::User => print_result("User", &self.user_name),
            FieldType::Key => print_result("Key", &self.api_key),
            _ => return,
        }
    }

    fn get_field(&self, field: FieldType) -> String {
        match field {
            FieldType::Secname => self.api_name.clone(),
            FieldType::Desc => self.description.clone(),
            FieldType::User => self.user_name.clone(),
            FieldType::Key => self.api_key.clone(),
            _ => return "".to_string(),
        }
    }
    fn get_json_str(&self) -> String {
        let json = match serde_json::to_string_pretty(self) {
            Err(_) => "".to_string(),
            Ok(j) => j,
        };

        json
    }

    fn get_field_json_str(&self, field: FieldType) -> String {
        let json_str = match field {
            FieldType::Secname => serde_json::json!({"Name": &self.user_name}),
            FieldType::Desc => serde_json::json!({"Description": &self.description}),
            FieldType::User => serde_json::json!({"User": &self.user_name}),
            FieldType::Key => serde_json::json!({"Key": &self.api_key}),
            _ => return "".to_string(),
        };
        json_str.to_string()
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

    pub fn get_field(&self, field: FieldType) -> String {
        match self {
            Self::API(api) => api.get_field(field),
            Self::Account(acc) => acc.get_field(field),
        }
    }

    pub fn get_json_str(&self) -> String {
        match self {
            Self::API(api) => api.get_json_str(),
            Self::Account(acc) => acc.get_json_str(),
        }
    }

    pub fn get_field_json_str(&self, field: FieldType) -> String {
        match self {
            Self::API(api) => api.get_field_json_str(field),
            Self::Account(acc) => acc.get_field_json_str(field),
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            Self::Account(acc) => acc.account_name.clone(),
            Self::API(api) => api.api_name.clone(),
        }
    }
}
