use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::{error::Error, fs::OpenOptions};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ConfigFile {
    account_id: Option<String>,
    access_token: Option<String>,
    user_agent: Option<String>,
}

impl ConfigFile {
    pub fn get_field(&self, field_name: &str) -> Option<&String> {
        match field_name {
            "user-agent" => {
                if self.user_agent.is_some() {
                    return Some(self.user_agent.as_ref()?);
                }
                return None;
            }

            "access-token" => {
                if self.access_token.is_some() {
                    return Some(self.access_token.as_ref()?);
                }
                return None;
            }

            "account-id" => {
                if self.account_id.is_some() {
                    return Some(self.account_id.as_ref()?);
                }
                return None;
            }
            _ => None,
        }
    }

    pub fn set_value(&mut self, field: &str, value: &str) -> Result<(), &str> {
        match field {
            "account-id" => {
                self.account_id = Some(value.to_string());
                return Ok(());
            }
            "access-token" => {
                self.access_token = Some(value.to_string());
                return Ok(());
            }
            "user-agent" => {
                self.user_agent = Some(value.to_string());
                return Ok(());
            }
            _ => Err("Given field not found for struct SettingFieldName"),
        }
    }

    pub fn save_to_file(&self, file_path: &Path) -> Result<(), Box<dyn Error>> {
        let mut new_file = File::create(file_path).expect("Could no create file");
        let serialized_data = toml::to_string(&self).unwrap();
        new_file.write_all(serialized_data.as_bytes())?;
        new_file.sync_all()?;
        Ok(())
    }

    pub fn open_and_read_from_path(file_path: &Path) -> Result<ConfigFile, Box<dyn Error>> {
        let mut file = OpenOptions::new().read(true).open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(string_to_configfile(&contents))
    }
}

fn string_to_configfile(data_as_string: &String) -> ConfigFile {
    toml::from_str(&data_as_string).unwrap()
}
