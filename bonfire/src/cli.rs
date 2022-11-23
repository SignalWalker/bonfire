use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// IP addresses on which to listen to requests
    #[clap(short = 'a', long, default_value = "127.0.0.1:8080")]
    pub listen_addrs: Vec<String>,
    /// Whether to watch templates and reload them when they're changed
    #[clap(short, long = "watch")]
    pub watch_templates: bool,
    /// Name to display for this instance
    #[clap(long, default_value = "Unnamed Bonfire Instance")]
    pub instance_name: String,
    /// Base URL for internal links within this instance
    #[clap(long = "url", default_value = "https://forum.example")]
    pub instance_url: String,
    /// Instance description
    #[clap(
        long = "desc",
        default_value = "An instance of Bonfire, a decentralized network of web forums."
    )]
    pub instance_desc: String,
    /// Reported language for pages served from this instance
    #[clap(long = "lang", default_value = "en-US")]
    pub instance_lang: String,
}
