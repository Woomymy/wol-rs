use std::fs::read_to_string;
use super::errors::Error;
use super::{debug,error};

#[derive(Debug)]
pub struct Config {
    hosts: Vec<(String, String)>
}

impl Config {
    pub fn from_machine() -> Result<Self, Error> {
        let home = std::env::var("HOME").expect("Can't get HOME env variable!");
        let configpath = format!("{}/.config/wol-rs/config", &home);
        if !std::path::Path::new(&configpath).exists() {
            return Ok(Self {
                hosts: Vec::new(),
            });
        }
        let configlines = read_to_string(&configpath)?;
        return Config::parse_config(configlines);
    }
    /// Get information from config lines
    pub fn parse_config(lines: String) -> Result<Self, Error> {
        let mut configuration = Self { hosts: Vec::new() };
        for line in lines.lines() {
            if line.is_empty() { continue };
            debug!("Parsing config line {line}");
            let parts: Vec<&str> = line.split(' ').collect();
            debug!("Found parts: {:#?}", &parts);

            if parts.len() < 4 {
                error!("Invalid config line {line}");
                continue
            }
            match parts[0] {
                "host" => {
                    let name = parts[1].to_string();
                    let mac = parts[3..].join(" ").replace('"', "").trim().to_string();
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
    pub fn find_host(&self, name: String) -> Option<(String,String)> {
        let host = self.hosts.iter().find(|h| h.0 == name);
        if let Some(h) = host {
            return Some((h.0.clone(), h.1.clone()));
        }
        None
    }
}