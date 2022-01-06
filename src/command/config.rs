mod config_file;
use clap::Parser;
use config_file::ConfigFile;
use std::error::Error;
use std::path::Path;
use std::str::FromStr;
use std::{fmt, fs};
#[derive(Parser, Debug)]
#[clap(about = "Set configration values like account id and access token")]
pub struct Config {
    /// name of the setting field. Accepted values are access-token, user-agent and account-id
    #[clap(required_unless_present = "list")]
    name: Option<SettingFieldName>,
    /// Value to be set to the configuration field
    value: Option<String>,
    /// List the configuration values in the config file
    #[clap(long = "list", short)]
    list: bool,
}

impl Config {
    pub fn run(&self) {
        if self.list {
            Config::print_config_values();
        } else {
            match &self.value {
                Some(v) => {
                    Config::write(&self.name.as_ref().unwrap(), v);
                }
                None => Config::print_config_field_value(&self.name.as_ref().unwrap()),
            }
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
                let file_data = match ConfigFile::open_and_read_from_path(config_file) {
                    Ok(values) => values,
                    Err(e) => {
                        print!(
                            "Error occured when trying to read the config file. Error: {}",
                            e
                        );
                        return;
                    }
                };

                match set_file_data_and_update(file_data, name, value, config_file) {
                    Err(e) => println!("Error occured when updating config file. Error: {:?}", e),
                    _ => return,
                };
            } else {
                //create a empty config file
                match set_file_data_and_update(ConfigFile::default(), name, value, config_file) {
                    Err(e) => println!("Error occured when updating config file. Error: {:?}", e),
                    _ => return,
                };
            }
        } else {
            fs::create_dir_all(config_folder).expect("Failed to create settings folder");
            home_path.push("config.toml");
            let config_file = home_path.as_path();
            match set_file_data_and_update(ConfigFile::default(), name, value, config_file) {
                Err(e) => println!("Error occured when updating config file. Error: {:?}", e),
                _ => return,
            };
        }
    }

    fn print_config_field_value(name: &SettingFieldName) {
        let mut home_path = match dirs::home_dir() {
            Some(folder_path) => folder_path,
            None => {
                println!("Could not find users home path");
                return;
            }
        };

        home_path.push(".harvest/config/config.toml");
        let config_file_path = home_path.as_path();

        if config_file_path.exists() {
            let file_data = match ConfigFile::open_and_read_from_path(config_file_path) {
                Ok(values) => values,
                Err(e) => {
                    print!(
                        "Error occured when trying to read the config file. Error: {}",
                        e
                    );
                    return;
                }
            };

            match file_data.get_field(&name.to_string()) {
                Some(v) => {
                    println!("{}", v);
                }
                _ => {
                    return;
                }
            }
        } else {
            println!("Config file does not exist. Create one with the config write commands");
        }
    }

    fn print_config_values() {
        let mut home_path = match dirs::home_dir() {
            Some(folder_path) => folder_path,
            None => {
                println!("Could not find users home path");
                return;
            }
        };

        home_path.push(".harvest/config/config.toml");
        let config_file_path = home_path.as_path();

        if config_file_path.exists() {
            let file_data = match ConfigFile::open_and_read_from_path(config_file_path) {
                Ok(values) => values,
                Err(e) => {
                    print!(
                        "Error occured when trying to read the config file. Error: {}",
                        e
                    );
                    return;
                }
            };

            match file_data.get_field("access-token") {
                Some(v) => {
                    println!("Access-token={}", v);
                }
                _ => {}
            }

            match file_data.get_field("account-id") {
                Some(v) => {
                    println!("Account-Id={}", v);
                }
                _ => {}
            }

            match file_data.get_field("user-agent") {
                Some(v) => {
                    println!("User-Agent={}", v);
                }
                _ => {}
            }
        } else {
            println!("Config file does not exist. Create one with the config write commands");
        }
    }
}

fn set_file_data_and_update(
    mut config_file: ConfigFile,
    field_name: &SettingFieldName,
    field_value: &str,
    file_path: &Path,
) -> Result<(), Box<dyn Error>> {
    config_file.set_value(&field_name.to_string(), field_value)?;
    config_file.save_to_file(file_path)?;
    Ok(())
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

impl fmt::Display for SettingFieldName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SettingFieldName::AccessToken => write!(f, "access-token"),
            SettingFieldName::AccountId => write!(f, "account-id"),
            SettingFieldName::UserAgent => write!(f, "user-agent"),
        }
    }
}
