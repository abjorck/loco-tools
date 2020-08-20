use crate::commands::{Platform, ApiCommand};
use async_trait::async_trait;
use loco::apis::configuration::Configuration;
use core::option::Option::{Some, None};
use loco::apis::export_api;
use argh::FromArgs;
use super::parse_platform;

#[derive(FromArgs, PartialEq, Debug)]
/// Export string resources to file
#[argh(subcommand, name = "export")]
pub(crate) struct ExportCommand {
    /// language locale to operate on, all if omitted
    #[argh(option, short = 'l')]
    pub(crate) locale: Option<String>,

    /// export iOS or Android file format
    #[argh(option, short = 'p', from_str_fn(parse_platform))]
    pub(crate) platform: Platform,

    ///do validation checks on xml
    #[argh(switch, short = 'v')]
    pub(crate) validate: bool,

}

#[async_trait]
impl ApiCommand<String> for ExportCommand {
    async fn call(&self, conf: &Configuration) -> String {
        let (ext, filter) = match &self.platform {
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

