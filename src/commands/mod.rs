use async_trait::async_trait;
use loco::apis::configuration::Configuration;
use loco::apis::auth_api;

pub mod export;
pub mod create;
pub mod tags;

use argh::FromArgs;
use crate::commands::export::ExportCommand;
use crate::commands::create::CreateCommand;
use crate::commands::tags::TagsCommand;


#[derive(FromArgs, PartialEq, Debug)]
/// Loco (https://localize.biz) API CLI
pub struct TopLevel {
    /// loco API key generated from project dashboard,
    ///  (required if not set in env var LOCO_APIKEY)
    #[argh(option, short = 'k')]
    pub apikey: Option<String>,

    #[argh(subcommand)]
    pub nested: SubCommandEnum,
}


#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum SubCommandEnum {
    Export(ExportCommand),
    CheckAuth(CheckAuth),
    Create(CreateCommand),
    Tags(TagsCommand)
}

#[derive(Debug, PartialEq)]
pub enum Platform {
    Android,
    IOS,
    Server
}

fn parse_platform(s: &str) -> Result<Platform, String> {
    if s.to_lowercase().eq("ios") {
        Ok(Platform::IOS)
    } else if s.to_lowercase().eq("android") {
        Ok(Platform::Android)
    } else if s.to_lowercase().eq("server") {
        Ok(Platform::Server)
    } else {
        Err("Invalid platform, supported params: [android, ios, server]".to_string())
    }
}

#[async_trait]
pub trait ApiCommand<T> {
    async fn call(&self, conf: &Configuration) -> T;
}

#[derive(FromArgs, PartialEq, Debug)]
/// Check API key access
#[argh(subcommand, name = "checkauth")]
pub struct CheckAuth {}

#[derive(Debug)]
pub enum Keytype {
    MISSING,
    DENIED,
    RO,
    RW,
}

pub async fn check_key(conf: &Configuration) -> Keytype {
    match &conf.api_key {
        Some(k) => {
            let auth_rw = auth_api::auth_verify(&conf, Option::from(k.key.as_str()), true).await;
            match auth_rw {
                Ok(r) => {
                    println!("auth ok for project: {}", r.project.name.unwrap_or_else(|| "<no name>".to_string()));
                    Keytype::RW
                }
                Err(_) => {
                    println!("no rw access with this key, checking ro");
                    let auth_ro = auth_api::auth_verify(&conf, Option::from(k.key.as_str()), false).await;
                    match auth_ro {
                        Ok(r) => {
                            println!("read only access auth ok for project: {}", r.project.name.unwrap_or_else(|| "<no name>".to_string()));
                            Keytype::RO
                        }
                        Err(e) => {
                            eprintln!("No access with specified api key {}, {:?} ", k.key.as_str(), e);
                            Keytype::DENIED
                        }
                    }
                }
            }
        }
        None => {
            Keytype::MISSING
        }
    }
}

