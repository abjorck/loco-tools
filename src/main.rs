use loco::apis::{auth_api, ping_api, Error};
use loco::apis::configuration::{Configuration, ApiKey};

extern crate futures;
extern crate reqwest;
extern crate exitcode;

use argh::FromArgs;
use futures::Future;
use std::alloc::handle_alloc_error;
use loco::apis::ping_api::PingError;
use loco::models::{PingResponse, Credentials};
use std::env;
use crate::Keytype::{RO, MISSING, RW, DENIED};
use loco::apis::auth_api::AuthVerifyError;
use std::str::FromStr;
use crate::Platform::{IOS, Android};
use loco::apis::export_api::{export_all, export_locale, ExportLocaleError};
use async_trait::async_trait;


#[derive(FromArgs, PartialEq, Debug)]
/// Loco (locailize.biz) API CLI
struct TopLevel {
    /// loco API key generated from project dashboard,
    ///  (required if not set in env var LOCO_APIKEY)
    #[argh(option, short = 'k')]
    apikey: Option<String>,

    #[argh(subcommand)]
    nested: SubCommandEnum,
}


#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum SubCommandEnum {
    Export(ExportCommand),
    CheckAuth(CheckAuth),
}

#[derive(Debug, PartialEq)]
enum Platform {
    Android,
    IOS,
}

fn parse_platform(s: &str) -> Result<Platform, String> {
    if s.to_lowercase().eq("ios") {
        Ok(IOS)
    } else if s.to_lowercase().eq("android") {
        Ok(Android)
    } else {
        Err("Invalid platform, supported params: [android, ios]".to_string())
    }
}

#[derive(FromArgs, PartialEq, Debug)]
/// Export string resources to file
#[argh(subcommand, name = "export")]
struct ExportCommand {
    /// language locale to operate on, all if omitted
    #[argh(option, short = 'l')]
    locale: Option<String>,
    /// export iOS or Android file format
    #[argh(option, short = 'p', from_str_fn(parse_platform))]
    platform: Platform,

}

#[async_trait]
trait ApiCommand<T> {
    async fn call(&self, conf: &Configuration) -> T;
}
/*
LANGUAGES = [
    Language('da', 'values-da'),
    Language('de_DE', 'values-de'),
    Language('de_CH', 'values-de-rCH'),
    Language('en_GB', 'values'),
    Language('es', 'values-es'),
    Language('fr_FR', 'values-fr'),
    Language('fr_BE', 'values-fr-rBE'),
    Language('fr_LU', 'values-fr-rLU'),
    Language('fr_CH', 'values-fr-rCH'),
    Language('nl_NL', 'values-nl'),
    Language('nl_BE', 'values-nl-rBE'),
    Language('it', 'values-it'),
    Language('pl_PL', 'values-pl'),
    Language('sv', 'values-sv')
    ]
 */

#[async_trait]
impl ApiCommand<String> for ExportCommand {
    async fn call(&self, conf: &Configuration) -> String {
        let (ext, filter) = match self.platform {
            IOS => ("strings", "iOS - Localizable.strings"), //TODO: no idea how ios handles their tags/files
            Android => ("xml", "android")
        };
        let key = Some(conf.api_key.as_ref().expect("api key").key.as_str());
        let res = export_locale(conf, self.locale.as_ref().unwrap_or(&"all".to_string()).as_str(),
                                ext, None, Some(filter), None, None, None, Some("en_GB"), None, None,
                                None, Some("UTF-8"), None, Some(true), None, key).await;
        res.expect("exporterror")
    }
}

#[derive(FromArgs, PartialEq, Debug)]
/// Check API key access
#[argh(subcommand, name = "checkauth")]
struct CheckAuth {
// /// loco API key generated from project dashboard, required if not set in env var LOCO_APIKEY
// #[argh(option, short = 'k')]
// apikey: Option<String>,
//
// /// language locale to operate on, all if omitted
// #[argh(option, short = 'l')]
// locale: Option<String>,

// /// how high to go
// #[argh(option)]
// height: usize,
//
// /// an optional nickname for the pilot
// #[argh(option)]
// pilot_nickname: Option<String>,eller de va
}

struct AppState {
    conf: Configuration
}

#[derive(Debug)]
enum Keytype {
    MISSING,
    DENIED,
    RO,
    RW,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let args: TopLevel = argh::from_env();

    let key = args.apikey.unwrap_or_else(||
        env::var("LOCO_APIKEY").unwrap_or_else(|_| {
            eprintln!("Missing api key. use -k or set LOCO_APIKEY env var.");
            std::process::exit(exitcode::USAGE);
        })
    );
    let mut conf = Configuration::new();

    conf.api_key = Some(ApiKey {
        prefix: None,
        key: key.clone(),
    });

    match args.nested {
        SubCommandEnum::Export(export) => {
            println!("export for locale. {:?}", export);
            let export = export.call(&conf).await;
            println!("{}", export);
        }
        SubCommandEnum::CheckAuth(_) => {
            println!("check auth");
            let key_type = check_key(&conf).await;
            println!("keytype result {:?}", key_type);
        }
    }
    Ok(())
//
// let work = apicli.ping_api().ping().and_then(|res|{
//     println!("pong? {:?}", res);
//     futures::future::ok(())
// });
//
// core.run(work).expect("failed to run core");
}


async fn check_key(conf: &Configuration) -> Keytype {
    match &conf.api_key {
        Some(k) => {
            let auth_rw = auth_api::auth_verify(&conf, Option::from(k.key.as_str()), true).await;
            match auth_rw {
                Ok(r) => {
                    println!("auth ok for project: {}", r.project.name.unwrap_or_else(|| "<no name>".to_string()));
                    RW
                }
                Err(_) => {
                    println!("no rw access with this key, checking ro");
                    let auth_ro = auth_api::auth_verify(&conf, Option::from(k.key.as_str()), false).await;
                    match auth_ro {
                        Ok(r) => {
                            println!("read only access auth ok for project: {}", r.project.name.unwrap_or_else(|| "<no name>".to_string()));
                            RO
                        }
                        Err(e) => {
                            eprintln!("No access with specified api key {}, {:?} ", k.key.as_str(), e);
                            DENIED
                        }
                    }
                }
            }
        }
        None => {
            MISSING
        }
    }
}
