use clap::Clap;
use cli::Args;
use lazy_static::lazy_static;

pub mod cli;
// pub mod root;
// pub mod api;
// pub mod user;
// use user::*;
// use api::*;

const VERSION_STR: &str = clap::crate_version!();

lazy_static! {
    pub static ref VERSION: [&'static str; 3] = {
        let mut split = VERSION_STR.split('.');
        [
            split.next().unwrap(),
            split.next().unwrap(),
            split.next().unwrap(),
        ]
    };
}

#[actix::main]
fn launch() -> _ {
    let Args { listen_addrs } = Args::parse();

    // let mut server = HttpServer::new(|| {});

    // for addr in listen_addrs {
    //     server = server.bind(addr)?;
    // }

    // server.run().await;

    
}
