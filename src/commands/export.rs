use crate::commands::{Platform, ApiCommand};
use async_trait::async_trait;
use loco::apis::configuration::Configuration;
use core::option::Option::{Some, None};
use loco::apis::export_api;
use argh::FromArgs;
use super::parse_platform;
use serde_derive::{Serialize, Deserialize};
use std::borrow::Borrow;

#[derive(FromArgs, PartialEq, Debug)]
/// Export string resources to file
#[argh(subcommand, name = "export")]
pub struct ExportCommand {
    /// language locale to operate on, all if omitted
    #[argh(option, short = 'l')]
    pub locale: Option<String>,

    /// exports iOS or Android or server file format
    #[argh(option, short = 'p', from_str_fn(parse_platform))]
    pub platform: Platform,

    ///do validation checks on xml
    #[argh(switch, short = 'v')]
    pub validate: bool,

    /// custom explicit filter (like for tags)
    #[argh(option)]
    pub filter: Option<String>,
    /// file format specific subtype
    #[argh(option)]
    pub format: Option<String>
}

#[async_trait]
impl ApiCommand<String> for ExportCommand {
    async fn call(&self, conf: &Configuration) -> String {
        //println!("calling {:?}", self);
        let (ext, filter) = match self.platform {
            Platform::IOS => ("strings", Some("iOS - Localizable.strings")), //TODO: no idea how ios handles their tags/files
            Platform::Android => ("xml", Some("android")),
            Platform::Server => ("json", None)
        };
        let key = Some(conf.api_key.as_ref().expect("api key").key.as_str());
        let res = export_api::export_locale(conf, self.locale.as_ref().unwrap_or(&"all".to_string()).as_str(),
                                            ext, self.format.as_deref(), self.filter.as_deref().or(filter.as_deref()), Some("name"), None, None, Some("en_GB"), None, None,
                                            None, Some("UTF-8"), None, Some(true), None, key).await;
        res.expect("exporterror")
    }

}

