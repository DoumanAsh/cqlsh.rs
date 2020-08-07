use arg::Args;

#[derive(Args, Debug)]
///cqlsh 0.1.0
///CQL CLI for Apache Cassandra
pub struct Cli {
    #[arg(long)]
    ///Prints version
    pub version: bool,

    #[arg(short = "C", long = "color")]
    ///Always use color output
    pub color: bool,

    #[arg(long = "no-color")]
    ///Never use color output
    pub no_color: bool,

    #[arg(long)]
    ///Use SSL
    pub ssl: bool,

    #[arg(short, long)]
    ///Authenticate as user.
    pub username: Option<String>,

    #[arg(short, long)]
    ///Authenticate using password.
    pub password: Option<String>,

    #[arg(short, long)]
    ///Authenticate to the given keyspace.
    pub keyspace: Option<String>,

    #[arg(short, long)]
    ///Execute the statement and quit.
    pub execute: Option<String>,

    #[arg(short, long)]
    ///Execute commands from FILE, then exit.
    pub file: Option<String>,

    #[arg(long = "connection-timeout", default_value = "5")]
    ///Specifies the connection timeout in seconds (default: 5 seconds).
    pub connection_timeout: u64,

    ///Specifies database host, optionally can include port
    pub host: Option<String>,
}

pub fn parse<'a, T: IntoIterator<Item = &'a str>>(args: T) -> Result<Cli, i32> {
    let args = args.into_iter().skip(1);

    Cli::from_args(args).map_err(|err| match err.is_help() {
        true => {
            println!("{}", Cli::HELP);
            0
        },
        false => {
            eprintln!("{}", err);
            2
        },
    })
}
