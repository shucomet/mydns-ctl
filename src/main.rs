use clap::Parser;
mod data;
use data::*;
mod error;
use error::MydnsCtlError;
use curl;
use std::io::{stdout, Write};
use std::env;

type Result<T> = std::result::Result<T, MydnsCtlError>;

fn get_auth_key(id_var: impl AsRef<str>, pass_var: impl AsRef<str>) -> Result<(String, String)> {
    let id = env::var(id_var.as_ref()).map_err(|_| MydnsCtlError::Auth(format!(
        "Cannot get id from env:{}", id_var.as_ref()
    )))?;
    let pass = env::var(pass_var.as_ref()).map_err(|_| MydnsCtlError::Auth(format!(
        "Cannot get pass from env:{}", pass_var.as_ref()
    )))?;
    Ok((id, pass))
}

fn execute_ipv4(arg: &CmdArg) -> Result<()> {
    let key = get_auth_key(&arg.id, &arg.pass)?;
    let mut auth = curl::easy::Auth::new();
    auth.basic(true);
    let mut c = curl::easy::Easy::new();
    c.http_auth(&auth)?;
    c.username(&key.0)?;
    c.password(&key.1)?;
    c.url(&arg.url)?;
    c.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    })?;
    c.fail_on_error(true)?;
    c.perform()?;
    Ok(())
}

fn execute_certbot_pre(_arg: &CmdArg) -> Result<()> {
    todo!()
}

fn execute_certbot_post(_arg: &CmdArg) -> Result<()> {
    todo!()
}

fn main() -> Result<()> {
    let arg = CmdArg::parse();
    // println!("{:#?}", arg);

    match arg.command {
        Command::Ipv4 => execute_ipv4(&arg),
        Command::CertbotPre => execute_certbot_pre(&arg),
        Command::CertbotPost => execute_certbot_post(&arg),
    }
}
