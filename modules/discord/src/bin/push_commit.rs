use clap::Parser;
use discord::{args::push_commit_args::PushCommitArgs, send_embed_message};
use discord_message_template::github::{Author, Commit, Repository};
use std::env;

/// Send a message to a Discord webhook when a commit is pushed to a repository.
/// args:
/// - author: The author of the commit.
/// - repository: The repository of the commit.
/// - commit_message: The message of the commit.
/// - branch: The branch of the commit.
/// - sha: The sha of the commit.
///
/// env:
/// - DISCORD_WEBHOOK_URL: The URL of the Discord webhook.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let args = PushCommitArgs::parse();

    let author = Author::new(&args.author);
    let repository = Repository::new(&args.repository);
    let commit = Commit::new(&args.commit_message, &args.branch, &args.sha);

    let message = discord_message_template::github::GithubTemplate::CommitPushed {
        author,
        repository,
        commit,
    }
    .build();

    let webhook_url = env::var("DISCORD_WEBHOOK_URL").expect("DISCORD_WEBHOOK_URL must be set");

    send_embed_message(&webhook_url, message).await?;

    Ok(())
}
