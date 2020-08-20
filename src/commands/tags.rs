use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
/// Manage tags of assets
#[argh(subcommand, name = "tags")]
pub(crate) struct TagsCommand {
    /// tag an asset with the supplied tag
    #[argh(subcommand)]
    pub(crate) nested: TagsSubCommandEnum,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Show tags of asset
#[argh(subcommand, name = "list")]
pub(crate) struct ListTagsCommand {
    #[argh(positional)]
    pub(crate) asset_id: String,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Tag an asset with the supplied tag
#[argh(subcommand, name = "add")]
pub(crate) struct AddTagsCommand {
    #[argh(positional)]
    pub(crate) asset_id: String,

    #[argh(positional)]
    pub(crate) tag: String,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Untag an asset with the supplied tag
#[argh(subcommand, name = "remove")]
pub(crate) struct RemoveTagsCommand {
    #[argh(positional)]
    pub(crate) asset_id: String,

    #[argh(positional)]
    pub(crate) tag: String,
}


#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub(crate) enum TagsSubCommandEnum {
    List(ListTagsCommand),
    Add(AddTagsCommand),
    Remove(RemoveTagsCommand),
}

