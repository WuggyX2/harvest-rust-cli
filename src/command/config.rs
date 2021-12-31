use dirs::home_dir;
use std::fs;
use std::path::Path;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Config {
    name: SettingFieldName,
    value: Option<String>,
}

impl Config {
    pub fn run(&self) {
        match &self.value {
            Some(v) => {
                Config::write(&self.name, v);
            }
            None => println!("Config would run print command for {:?}", self.name),
        }
    }

    fn write(name: &SettingFieldName, value: &str) {
        let mut home_path = dirs::home_dir().unwrap();
        home_path.push(".harvest/config");
        let config_folder = home_path.as_path();

        if config_folder.exists() {
            println!("settings folder exists");
        } else {
            fs::create_dir_all(config_folder).expect("Failed to create settings folder");
        }
    }

    fn printValue() {}
}

#[derive(Debug)]
enum SettingFieldName {
    AccountId,
    AccessToken,
    UserAgent,
}

type ParseError = &'static str;

impl FromStr for SettingFieldName {
    type Err = ParseError;
    fn from_str(field_name: &str) -> Result<Self, Self::Err> {
        match field_name {
            "access-token" => Ok(SettingFieldName::AccessToken),
            "account-id" => Ok(SettingFieldName::AccountId),
            "user-agent" => Ok(SettingFieldName::UserAgent),
            _ => Err("Could not parse value for name. Accepted values are access-token, account-id and user-agent"),
        }
    }
}
