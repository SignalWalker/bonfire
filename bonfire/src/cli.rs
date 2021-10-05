use clap::Clap;

#[derive(Clap)]
pub struct Args {
    #[clap(short = 'a', long, default_value = "127.0.0.1:8080")]
    pub listen_addrs: Vec<String>
}