use clap::Parser;

#[derive(Debug, Parser)]
pub struct PushCommitArgs {
    #[clap(short, long)]
    pub author: String,

    #[clap(short, long)]
    pub repository: String,

    #[clap(short, long)]
    pub commit_message: String,

    #[clap(short, long)]
    pub branch: String,

    #[clap(short, long)]
    pub sha: String,
}
