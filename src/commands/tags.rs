use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
/// Manage tags of assets
#[argh(subcommand, name = "tags")]
pub struct TagsCommand {
    /// tag an asset with the supplied tag
    #[argh(subcommand)]
    pub nested: TagsSubCommandEnum,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Show tags of asset
#[argh(subcommand, name = "list")]
pub struct ListTagsCommand {
    #[argh(positional)]
    pub asset_id: String,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Tag an asset with the supplied tag
#[argh(subcommand, name = "add")]
pub struct AddTagsCommand {
    #[argh(positional)]
    pub asset_id: String,

    #[argh(positional)]
    pub tag: String,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Untag an asset with the supplied tag
#[argh(subcommand, name = "remove")]
pub struct RemoveTagsCommand {
    #[argh(positional)]
    pub asset_id: String,

    #[argh(positional)]
    pub tag: String,
}


#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum TagsSubCommandEnum {
    List(ListTagsCommand),
    Add(AddTagsCommand),
    Remove(RemoveTagsCommand),
}

