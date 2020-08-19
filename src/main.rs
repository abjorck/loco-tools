extern crate exitcode;
extern crate futures;
extern crate reqwest;
extern crate xml;

use std::env;

use argh::FromArgs;
use async_trait::async_trait;

use loco::{
    apis::configuration::{ApiKey, Configuration},
    apis::{assets_api, auth_api, Error, export_api, ping_api},
    models::{Asset, Credentials, PingResponse},
};

use xml::reader::XmlEvent;

use crate::Keytype::{DENIED, MISSING, RO, RW};
use crate::Platform::{Android, IOS};


#[derive(FromArgs, PartialEq, Debug)]
/// Loco (https://localize.biz) API CLI
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
    Create(CreateCommand),
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

    ///do validation checks on xml
    #[argh(switch, short = 'v')]
    validate: bool,

}

#[derive(FromArgs, PartialEq, Debug)]
/// Create new translatable assets
#[argh(subcommand, name = "create")]
struct CreateCommand {
    /// language locale to operate on, all if omitted
    #[argh(option, short = 't')]
    tags: Option<String>,

    /// asset id to create, strongly suggested - if omitted will be generated
    #[argh(option, short = 'i')]
    id: Option<String>,

    #[argh(positional)]
    content: String,
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
        let res = export_api::export_locale(conf, self.locale.as_ref().unwrap_or(&"all".to_string()).as_str(),
                                            ext, None, Some(filter), Some("name"), None, None, Some("en_GB"), None, None,
                                            None, Some("UTF-8"), None, Some(true), None, key).await;
        res.expect("exporterror")
    }
}

#[async_trait]
impl ApiCommand<Result<String, Error<assets_api::CreateAssetError>>> for CreateCommand {
    async fn call(&self, conf: &Configuration) -> Result<String, Error<assets_api::CreateAssetError>> {
        let key = Some(conf.api_key.as_ref().expect("api key").key.as_str());
        let id = self.id.as_ref().map(|s| s.as_ref());
        let ret = assets_api::create_asset(conf, key, id, Some(self.content.as_str()), None, None, None, Option::from("provisional"), true).await;
        match ret {
            Ok(asset) => {
                println!("created asset: {:?}", asset);
                let tags = match self.tags.as_ref()
                    .map(|s| s.split(',')) {
                    Some(t) => {
                        t
                    }
                    None => {
                        return Ok(asset.id);
                    }
                };

                for tag in tags {
                    let res = assets_api::tag_asset(conf, asset.id.as_str(), tag, key).await;
                    match res {
                        Ok(asset) => {
                            println!("tagged asset: {:?}", asset);
                        }
                        Err(e) => {
                            println!("WARNING: tagging asset failed {:?}", e);
                        }
                    }
                }

                Ok(asset.id)
            }
            Err(e) => {
                println!("Error: create asset failed {:?}", e);

                Err(e)
            }
        }
    }
}

#[derive(FromArgs, PartialEq, Debug)]
/// Check API key access
#[argh(subcommand, name = "checkauth")]
struct CheckAuth {}

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
            eprintln!("export for locale. {:?}", export);
            let export_data = export.call(&conf).await;

            if export.validate {
                let xml = xml::EventReader::from_str(export_data.as_str());
                let mut el_name = String::new();
                for e in xml {
                    match e {
                        Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                            //el_name =
                            //let attr =
                            // attributes.iter()
                            //     .for_each(|a| println!("{:?}", a));
                            //.find(|&a| a.name.local_name == "name");
                            // match attr {
                            //     Some(a) => {
                            //         println!("{:?}", a.value);
                            //         el_name = a.value.replace('.', "_");
                            //     }
                            //     None => {
                            //         println!("{}", "Not a named string, skipping");
                            //         continue;
                            //     }
                            // }
                            // .map(|a|a.value.clone())
                            // .expect("no xml attr name?");
//                        el_name = name.local_name.replace('.', "_");
                        }
                        Ok(XmlEvent::Characters(string)) => {
                            if string.contains("% @") { // a common error case when translators break printf things
                                eprintln!("{} - '{}' contains broken formatting sequence % @", export.locale.clone().unwrap_or("".to_string()), el_name)
                            }
                        }
                        Ok(XmlEvent::EndElement { .. }) => break,
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            break;
                        }
                        _ => {}
                    }
                }
            }
            println!("{}", export_data);
        }
        SubCommandEnum::CheckAuth(_) => {
            eprintln!("check auth");
            let key_type = check_key(&conf).await;
            eprintln!("keytype result {:?}", key_type);
        }
        SubCommandEnum::Create(create) => {
            match create.call(&conf).await {
                Ok(_) => {
                    eprintln!("Ok!");
                }
                Err(e) => {
                    eprintln!("{:?}", e);
                }
            };
        }
    }

    Ok(())
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
