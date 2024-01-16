use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command()]
struct AzArtifactsUniversalDownloadCommand {
    /// Azure DevOps organization URL.
    /// Example: https://dev.azure.com/MyOrganizationName/.
    #[arg(long = "org", long = "organization", verbatim_doc_comment)]
    organization: String,

    /// Name or ID of the project.
    #[arg(long, short)]
    project: Option<String>,

    /// Name or ID of the feed.
    #[arg(long, verbatim_doc_comment)]
    feed: String,

    /// Name of the package, e.g. 'foo-package'.
    #[arg(long, short, verbatim_doc_comment)]
    name: String,

    /// Version of the package, e.g. 1.0.0.
    #[arg(long, short, verbatim_doc_comment)]
    version: String,

    /// Directory to place the package contents.
    #[arg(long, verbatim_doc_comment)]
    path: PathBuf,
}

fn main() {
    let command = AzArtifactsUniversalDownloadCommand::parse();
    dbg!(command);
}
