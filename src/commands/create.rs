use async_trait::async_trait;
use loco::apis::{assets_api, Error};
use loco::apis::configuration::Configuration;
use core::option::Option::{Some, None};
use core::option::Option;
use core::result::Result::{Ok, Err};
use argh::FromArgs;
use crate::commands::ApiCommand;

#[derive(FromArgs, PartialEq, Debug)]
/// Create new translatable assets
#[argh(subcommand, name = "create")]
pub struct CreateCommand {
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

