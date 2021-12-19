use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "auth", about = "autentikointi", rename_all = "kebab-case")]
pub struct Auth {
    // kikkelis kokkelis
    #[structopt(short, long)]
    bearer_token: String,
    #[structopt(short, long)]
    account_id: String,
    #[structopt(short, long)]
    user_agent: String,
}
s