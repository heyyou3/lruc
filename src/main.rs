use anyhow::Result;
use clap::{App, Arg};
use http::{HeaderMap, HeaderValue};

mod config;
mod request;

fn main() -> Result<()> {
    let matches = App::new("lruc")
        .version("0.1.0")
        .author("heyyou <heyyou3@gmail.com>")
        .about("curl respect command")
        .arg(
            Arg::with_name("config")
                .short('c')
                .long("config")
                .takes_value(true)
                .required(true),
        )
        .get_matches();
    let mut config_path = "";
    if let Some(o) = matches.value_of("config") {
        config_path = o;
    }
    let config = config::read_config(config_path)?;
    let headers = HeaderMap::new();
    let base_url = &config.base_url;
    let res = request::get(format!("{}{}", base_url, &config.api_info[0].path), headers);
    println!("{:?}", res);
    Ok(())
}
