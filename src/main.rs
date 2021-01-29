/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

extern crate exitcode;
extern crate futures;
extern crate reqwest;
extern crate xml;
extern crate serde_derive;

pub mod commands;
mod utils;
use utils::cache;

use std::env;
use xml::reader::XmlEvent;

pub use loco::{
    apis::configuration::{ApiKey, Configuration},
    apis::{assets_api, auth_api, Error, export_api},
    models::{Asset, Credentials, PingResponse},
};


pub use commands::{
    tags::TagsSubCommandEnum,
    TopLevel,
    SubCommandEnum,
    Platform::{Android, IOS},
    Keytype::{DENIED, MISSING, RO, RW},
    ApiCommand
};


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
            let key_type = commands::check_key(&conf).await;
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
        SubCommandEnum::Tags(tag) => {
            let key = conf.api_key.as_ref().map(|k|k.key.as_str());

            match tag.nested {
                TagsSubCommandEnum::Add(add) => {
                    let asset = assets_api::tag_asset(&conf, add.asset_id.as_str(), add.tag.as_str(), key).await;
                    match asset {
                        Ok(asset) => {
                            println!("{} tags: {:?}", asset.id, asset.tags)
                        }
                        Err(e) => {
                            eprintln!("{:?}", e)
                        }
                    }
                }
                TagsSubCommandEnum::List(list) => {
                    println!("listing asset: {:?}", list.asset_id);
                    let asset = assets_api::get_asset(&conf, list.asset_id.as_str(), key).await;
                    match asset {
                        Ok(asset) => {
                            println!("{} tags: {:?}", asset.id, asset.tags)
                        }
                        Err(e) => {
                            eprintln!("{:?}", e)
                        }
                    }
                }
                TagsSubCommandEnum::Remove(remove) => {
                    let asset = assets_api::untag_asset(&conf, remove.asset_id.as_str(), remove.tag.as_str(), key).await;
                    match asset {
                        Ok(asset) => {
                            println!("Ok")
                        }
                        Err(e) => {
                            eprintln!("{:?}", e)
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

