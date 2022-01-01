use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
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
        let mut home_path = match dirs::home_dir() {
            Some(folder_path) => folder_path,
            None => {
                println!("Could not find users home path");
                return;
            }
        };

        home_path.push(".harvest/config");
        let config_folder = home_path.as_path();

        if config_folder.exists() {
            home_path.push("config.toml");
            let config_file = home_path.as_path();

            if config_file.exists() {
                let mut file = File::open(config_file).expect("Config File not found");
                let mut contents = String::new();
                file.read_to_string(&mut contents);
                let config_file: ConfigFile = toml::from_str(&contents).unwrap();
                println!("{:?}", config_file);
            } else {
                let mut f = File::create(config_file).expect("Could no create file");

                let config = ConfigFile {
                    account_id: value.to_string(),
                    access_token: None,
                    user_agent: None,
                };

                let serialized = toml::to_string(&config).unwrap();

                f.write_all(serialized.as_bytes());

                f.sync_all();
            }
        } else {
            fs::create_dir_all(config_folder).expect("Failed to create settings folder");
        }
    }

    fn printValue() {}
}
#[derive(Serialize, Deserialize, Debug)]
struct ConfigFile {
    account_id: String,
    access_token: Option<String>,
    user_agent: Option<String>,
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
