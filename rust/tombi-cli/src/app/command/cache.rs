#[derive(clap::Args)]
pub struct Args {
    #[command(subcommand)]
    command: CacheCommand,
}

#[derive(clap::Subcommand)]
enum CacheCommand {
    /// Remove all cached files
    Clean {
        /// Allow cleaning a custom TOMBI_CACHE_HOME directory
        #[arg(long)]
        force: bool,
    },
    /// Print the cache directory path
    Dir,
}

pub fn run(args: Args) -> Result<(), crate::Error> {
    match args.command {
        CacheCommand::Clean { force } => {
            super::runtime(true)?.block_on(tombi_cache::clean_cache(force))?;
        }
        CacheCommand::Dir => println!("{}", tombi_cache::cache_dir_path()?.display()),
    }
    Ok(())
}
