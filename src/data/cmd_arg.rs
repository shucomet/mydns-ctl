use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Notify IPv4 address
    Ipv4,
    /// Pre validation hook of Certbot
    CertbotPre,
    /// Post validation hook of Certbot
    CertbotPost,
}

#[derive(Debug, Parser)]
pub struct CmdArg {
    /// Master ID of MyDNS
    #[arg(long)]
    pub id: String,
    /// Master password of MyDNS
    #[arg(long)]
    pub pass: String,
    /// URL of MyDNS
    #[arg(long, default_value = "http://localhost:8080/")]
    pub url: String,
    /// Subcommand
    #[clap(subcommand)]
    pub command: Command,
}
