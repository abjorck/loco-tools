use argh::FromArgs;
use crate::commands::ApiCommand;
use loco::models::Asset;
use loco::apis::configuration::Configuration;

#[derive(FromArgs, PartialEq, Debug)]
/// Manage tags of assets
#[argh(subcommand, name = "find")]
pub(crate) struct FindCommand {
    /// language locale to operate on, all if omitted
    #[argh(option, short = 'l')]
    pub(crate) locale: Option<String>,

    ///do validation checks on xml
    #[argh(option, short = 'f')]
    pub(crate) field: Option<String>,

    #[argh(positional)]
    pub(crate) pattern: String,
}

use async_trait::async_trait;
#[async_trait]
impl<'a> ApiCommand<&'a str> for FindCommand{
    async fn call(&'a self, conf: &Configuration) -> &'a str {
        vec!["hej"].first().unwrap()
    }
}
