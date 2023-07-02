mod users;
use clap::Parser;

fn main() {
    let args = Args::parse();
    if args.daemon {
        todo!();
    }
    if args.query != "" {
        let found_user = users::User::find_user(&args.query);
        match found_user {
            Some(user) => println!("{:?}", user),
            None => println!("No user found"),
        }
    }
}

#[derive(Debug, Parser)]
#[command(author, about, version)]
struct Args {
    /// Run as a daemon
    #[arg(short, long)]
    daemon: bool,
    /// Query Users, provide a name.
    #[arg(short, long)]
    query: String,
}
