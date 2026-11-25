use std::{fmt::Display, thread::sleep, time};

use arboard::Clipboard;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::util::{argparser::FieldType, ioutils::print_result};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Secret {
    #[serde(rename = "login")]
    Login(LoginDetails),
    #[serde(rename = "key")]
    Key(KeyDetails),
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoginDetails {
    pub account_name: String,
    pub user_name: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub password_last_changed: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KeyDetails {
    pub key_name: String,
    pub description: String,
    pub user_name: String,
    pub key_value: String,
    pub created_at: DateTime<Utc>,
    pub key_last_changed: DateTime<Utc>,
}

impl From<LoginDetails> for Secret {
    fn from(value: LoginDetails) -> Self {
        Secret::Login(value)
    }
}

impl From<KeyDetails> for Secret {
    fn from(value: KeyDetails) -> Self {
        Secret::Key(value)
    }
}

impl Display for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Login(acc) => write!(f, "{}", acc),
            Self::Key(key) => write!(f, "{}", key),
        }
    }
}

impl Display for LoginDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.account_name)
    }
}

impl Display for KeyDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.key_name)
    }
}

impl LoginDetails {
    fn print(&self) {
        println!("Name:          {}", self.account_name);
        println!("User:          {}", self.user_name);
        println!("Pass:          {}", self.password);
        println!(
            "Created:       {}",
            self.created_at.with_timezone(&chrono::Local)
        );
        println!(
            "Last Changed:  {}",
            self.password_last_changed.with_timezone(&chrono::Local)
        );
        println!();
    }

    fn print_field(&self, field: FieldType) {
        match field {
            FieldType::User => print_result("User", &self.user_name),
            FieldType::Secname => print_result("Name", &self.account_name),
            FieldType::Pass => print_result("Pass", &self.password),
            _ => (),
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
        match serde_json::to_string_pretty(self) {
            Err(_) => "".to_string(),
            Ok(j) => j,
        }
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

    fn send_field_to_clipboard(&self, field: FieldType) -> anyhow::Result<()> {
        let data = match field {
            FieldType::User => &self.user_name,
            FieldType::Secname => &self.account_name,
            FieldType::Pass => &self.password,
            _ => "",
        };

        let mut clipboard = Clipboard::new()?;

        clipboard.set_text(data)?;
        // to stop clipboard from being dropped early which causes an error on linux
        sleep(time::Duration::from_millis(5));
        Ok(())
    }

    fn to_comma_separated_string(&self) -> String {
        format!(
            "login,{},{},{}",
            self.account_name, self.user_name, self.password
        )
    }
}

impl KeyDetails {
    fn print(&self) {
        println!("Name:          {}", self.key_name);
        println!("User:          {}", self.user_name);
        println!("Desc:          {}", self.description);
        println!("Key:           {}", self.key_value);
        println!(
            "Created:       {}",
            self.created_at.with_timezone(&chrono::Local)
        );
        println!(
            "Last Changed:  {}",
            self.key_last_changed.with_timezone(&chrono::Local)
        );
        println!();
    }

    fn print_field(&self, field: FieldType) {
        match field {
            FieldType::Secname => print_result("Name", &self.key_name),
            FieldType::Desc => print_result("Desc", &self.description),
            FieldType::User => print_result("User", &self.user_name),
            FieldType::Key => print_result("Key", &self.key_value),
            _ => (),
        }
    }

    fn get_field(&self, field: FieldType) -> String {
        match field {
            FieldType::Secname => self.key_name.clone(),
            FieldType::Desc => self.description.clone(),
            FieldType::User => self.user_name.clone(),
            FieldType::Key => self.key_value.clone(),
            _ => "".to_string(),
        }
    }
    fn get_json_str(&self) -> String {
        match serde_json::to_string_pretty(self) {
            Err(_) => "".to_string(),
            Ok(j) => j,
        }
    }

    fn get_field_json_str(&self, field: FieldType) -> String {
        let json_str = match field {
            FieldType::Secname => serde_json::json!({"Name": &self.key_name}),
            FieldType::Desc => serde_json::json!({"Description": &self.description}),
            FieldType::User => serde_json::json!({"User": &self.user_name}),
            FieldType::Key => serde_json::json!({"Key": &self.key_value}),
            _ => return "".to_string(),
        };
        json_str.to_string()
    }

    fn send_field_to_clipboard(&self, field: FieldType) -> anyhow::Result<()> {
        let data = match field {
            FieldType::Secname => &self.key_name,
            FieldType::Desc => &self.description,
            FieldType::User => &self.user_name,
            FieldType::Key => &self.key_value,
            _ => "",
        };

        let mut clipboard = Clipboard::new()?;

        clipboard.set_text(data)?;
        // to stop clipboard from being dropped early which causes an error on linux
        sleep(time::Duration::from_millis(5));
        Ok(())
    }

    fn to_comma_separated_string(&self) -> String {
        format!(
            "key,{},{},{},{}",
            self.key_name, self.user_name, self.description, self.key_value
        )
    }
}

impl Secret {
    pub fn print_field(&self, field: FieldType) {
        match self {
            Self::Key(keyobj) => keyobj.print_field(field),
            Self::Login(accountobj) => accountobj.print_field(field),
        }
    }

    pub fn print(&self) {
        match self {
            Self::Key(key) => key.print(),
            Self::Login(acc) => acc.print(),
        }
    }

    pub fn get_field(&self, field: FieldType) -> String {
        match self {
            Self::Key(key) => key.get_field(field),
            Self::Login(acc) => acc.get_field(field),
        }
    }

    pub fn get_json_str(&self) -> String {
        match self {
            Self::Key(key) => key.get_json_str(),
            Self::Login(acc) => acc.get_json_str(),
        }
    }

    pub fn get_field_json_str(&self, field: FieldType) -> String {
        match self {
            Self::Key(key) => key.get_field_json_str(field),
            Self::Login(acc) => acc.get_field_json_str(field),
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            Self::Login(acc) => acc.account_name.clone(),
            Self::Key(key) => key.key_name.clone(),
        }
    }

    pub fn send_field_to_clipboard(&self, field: FieldType) -> anyhow::Result<()> {
        match self {
            Self::Login(acc) => acc.send_field_to_clipboard(field),
            Self::Key(key) => key.send_field_to_clipboard(field),
        }
    }

    pub fn to_comma_separated_string(&self) -> String {
        match self {
            Self::Login(acc) => acc.to_comma_separated_string(),
            Self::Key(key) => key.to_comma_separated_string(),
        }
    }
}
