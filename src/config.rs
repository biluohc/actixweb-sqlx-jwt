use crate::state::*;
use crate::state::{redis::Client, KvPool, RedisConnectionManager};

use std::path::PathBuf;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Config {
    pub sql: String,
    pub redis: String,
    pub listen: String,
    pub jwt_priv: String,
}

impl Config {
    pub fn parse_from_file(file: &PathBuf) -> Self {
        use std::fs::read_to_string;

        info!("confp: {}", file.display());
        let confstr = read_to_string(file).expect("confile read");
        json5::from_str(&confstr).expect("confile deser")
    }
    pub async fn into_state(self) -> AppStateRaw {
        info!("config: {:?}", self);
        let sql = SqlPool::new(&self.sql).await.expect("sql open");
        let kvm =
            RedisConnectionManager::new(Client::open(self.redis.clone()).expect("redis open"));
        let kv = KvPool::builder().build(kvm);

        Arc::new(State {
            config: self,
            sql,
            kv,
        })
    }
    // generate and show config string
    pub fn show() {
        let de: Self = Default::default();
        println!("{}", serde_json::to_string_pretty(&de).unwrap())
    }
}

pub fn version_with_gitif() -> &'static str {
    concat!(
        env!("CARGO_PKG_VERSION"),
        " ",
        env!("VERGEN_COMMIT_DATE"),
        ": ",
        env!("VERGEN_SHA_SHORT")
    )
}

#[derive(structopt::StructOpt, Debug)]
// #[structopt(name = "template")]
#[structopt(version = version_with_gitif())]
pub struct Opt {
    // /// Activate debug mode
    // #[structopt(short, long)]
    // debug: bool,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,

    /// Output file
    #[structopt(
        short = "c",
        long = "config",
        parse(from_os_str),
        default_value = "template.json"
    )]
    pub config: PathBuf,
}

impl Opt {
    pub fn parse_from_args() -> Self {
        use structopt::StructOpt;

        let opt: Self = Opt::from_args();

        let level = match opt.verbose {
            0 => "warn",
            1 => "info",
            2 => "debug",
            _more => "trace",
        };

        std::env::set_var("RUST_LOG", level);
        std::env::set_var("LOGE_FORMAT", "target");
        loge::init();

        info!("opt: {:?}", opt);
        opt
    }
}
