use super::errors::Error;
use super::{debug, error};
use std::fs::read_to_string;

#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    hosts: Vec<(String, String)>,
}

impl Config {
    pub fn from_machine() -> Result<Self, Error> {
        let home = std::env::var("HOME").expect("Can't get HOME env variable!");
        let configpath = format!("{}/.config/wol-rs/config", &home);
        if !std::path::Path::new(&configpath).exists() {
            return Ok(Self { hosts: Vec::new() });
        }
        let configlines = read_to_string(&configpath)?;
        Config::parse_config(configlines)
    }
    /// Get information from config lines
    pub fn parse_config(lines: String) -> Result<Self, Error> {
        let mut configuration = Self { hosts: Vec::new() };
        for line in lines.lines() {
            if line.is_empty() {
                continue;
            };
            debug!("Parsing config line {line}");
            let parts: Vec<&str> = line.split_whitespace().collect();
            debug!("Found parts: {:#?}", &parts);

            if parts.len() < 4 {
                error!("Invalid config line {line}");
                continue;
            }
            match parts[0] {
                "host" => {
                    let name = parts[1].replace("'", "").replace('"', "").to_string();
                    let mac = parts[3..]
                        .join(" ")
                        .replace('"', "")
                        .replace("'", "")
                        .trim()
                        .to_string();
                    debug!("Got host {name} mac {mac}");
                    configuration.hosts.push((name, mac))
                }
                _ => {
                    error!("Invalid config line {line}");
                    continue;
                }
            }
        }
        Ok(configuration)
    }
    /// Find an host by name
    pub fn find_host(&self, name: String) -> Option<(String, String)> {
        let host = self.hosts.iter().find(|h| h.0 == name);
        if let Some(h) = host {
            return Some((h.0.clone(), h.1.clone()));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::Config;
    #[test]
    /// Test if config parses correctly
    pub fn test_config_parse_single_line() {
        assert_eq!(
            Config::parse_config("host pc = 90:1b:0e:53:92:de".to_string()).unwrap(),
            Config {
                hosts: vec![("pc".to_string(), "90:1b:0e:53:92:de".to_string())]
            }
        )
    }
    /// Test if multiple lines config loading works correctly
    #[test]
    pub fn test_config_parse_multiple_lines() {
        assert_eq!(
            Config::parse_config(
                "host pc = FF:FF:FF:FF:FF:FF\nhost other = EE:EE:EE:EE:EE:EE".to_string()
            )
            .unwrap(),
            Config {
                hosts: vec![
                    ("pc".to_string(), "FF:FF:FF:FF:FF:FF".to_string()),
                    ("other".to_string(), "EE:EE:EE:EE:EE:EE".to_string())
                ]
            }
        )
    }
    /// Test if config test is ok with       spaces
    #[test]
    pub fn test_config_parse_spaces() {
        assert_eq!(
            Config::parse_config(
                "host    pc =          FF:FF:FF:FF:FF:FF            \n".to_string()
            )
            .unwrap(),
            Config {
                hosts: vec![("pc".to_string(), "FF:FF:FF:FF:FF:FF".to_string())]
            }
        )
    }
    /// Test if " " and '' are removed
    #[test]
    pub fn test_config_remove_single_quotes() {
        assert_eq!(
            Config::parse_config("host 'pc' = 'FF:FF:FF:FF:FF:FF'".to_string()).unwrap(),
            Config {
                hosts: vec![("pc".to_string(), "FF:FF:FF:FF:FF:FF".to_string())]
            }
        )
    }
    /// Test if " " and '' are removed
    #[test]
    pub fn test_config_remove_double_quotes() {
        assert_eq!(
            Config::parse_config("host \"pc\" = \"FF:FF:FF:FF:FF:FF\"".to_string()).unwrap(),
            Config {
                hosts: vec![("pc".to_string(), "FF:FF:FF:FF:FF:FF".to_string())]
            }
        )
    }
    /// Test if invalid lines are skipped
    #[test]
    pub fn test_skip_invalid_lines() {
        assert_eq!(
            Config::parse_config(
                "host pc = FF:FF:FF:FF:FF:FF\nhost inv alid\nhost nm = EE:EE:EE:EE:EE:EE"
                    .to_string()
            )
            .unwrap(),
            Config {
                hosts: vec![
                    ("pc".to_string(), "FF:FF:FF:FF:FF:FF".to_string()),
                    ("nm".to_string(), "EE:EE:EE:EE:EE:EE".to_string())
                ]
            }
        )
    }
}
