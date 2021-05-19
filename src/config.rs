use crate::state::*;
use crate::state::{redis::Client, KvPool, RedisConnectionManager};

use std::path::PathBuf;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub db: String,
    pub kv: String,
    pub listen: String,
    pub jwt_priv: String,
}

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct DbOptions {
    timeout: u64,
    #[serde(default)]
    server_timezone: String,
}

impl Config {
    pub fn parse_from_file(file: &PathBuf) -> Self {
        use std::fs::read_to_string;

        info!("confp: {}", file.display());
        let confstr = read_to_string(file).expect("confile read");
        json5::from_str(&confstr).expect("confile deser")
    }
    pub async fn into_state(self) -> AppState {
        info!("config: {:?}", self);
        let mut pool_options = PoolOptions::new();

        if let Some(opstr) = url::Url::parse(&self.db)
            .expect("Invalid SqlDB URL")
            .query()
        {
            if let Some(ops) = serde_qs::from_str::<DbOptions>(opstr)
                .map_err(|e| error!("serde_qs::from_str::<DbOptions> failed: {}", e))
                .ok()
            {
                pool_options =
                    pool_options.connect_timeout(std::time::Duration::from_secs(ops.timeout));

                if !ops.server_timezone.is_empty() {
                    let key = if cfg!(feature = "mysql") {
                        "@@session.time_zone ="
                    } else if cfg!(feature = "postgres") {
                        "TIME ZONE"
                    } else {
                        panic!("sqlite can't set timezone!")
                    };
                    // UTC, +00:00, HongKong, etc
                    let set = format!("SET {} '{}'", key, ops.server_timezone.clone());

                    // cannot move out of `set_str`, a captured variable in an `Fn` closure
                    let set_str = unsafe { std::mem::transmute::<_, &'static str>(set.as_str()) };
                    std::mem::forget(set);
                    pool_options = pool_options.after_connect(move |conn| {
                        Box::pin(async move {
                            use crate::sqlx::Executor;
                            conn.execute(set_str).await.map(|_| ())
                        })
                    })
                }
            }
        }

        let db = pool_options.connect(&self.db).await.expect("sql open");
        let kvm = RedisConnectionManager::new(Client::open(self.kv.clone()).expect("redis open"));
        let kv = KvPool::builder().build(kvm);

        Arc::new(State {
            config: self,
            db,
            kv,
        })
    }
    // generate and show config string
    pub fn show() {
        let de = Self::default();
        println!("{}", serde_json::to_string_pretty(&de).unwrap())
    }
}

pub fn version_with_gitif() -> &'static str {
    concat!(
        env!("CARGO_PKG_VERSION"),
        " ",
        env!("VERGEN_GIT_COMMIT_DATE"),
        ": ",
        env!("VERGEN_GIT_SHA_SHORT")
    )
}

#[derive(clap::Clap, Debug)]
// #[clap(name = "template")]
#[clap(version = version_with_gitif())]
pub struct Opts {
    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: u8,

    /// Config file
    #[clap(
        short = 'c',
        long = "config",
        parse(from_os_str),
        default_value = "template.json"
    )]
    pub config: PathBuf,
}

impl Opts {
    pub fn parse_from_args() -> (JoinHandle, Self) {
        use clap::Clap;
        let opt: Self = Opts::parse();

        let level = match opt.verbose {
            0 => LevelFilter::Warn,
            1 => LevelFilter::Info,
            2 => LevelFilter::Debug,
            _more => LevelFilter::Trace,
        };

        let formater = BaseFormater::new()
            .local(true)
            .color(true)
            .level(4)
            .formater(format);
        let filter = BaseFilter::new()
            .starts_with(true)
            .notfound(true)
            .max_level(level)
            .chain("sqlx", LevelFilter::Warn);

        let handle = NonblockLogger::new()
            .filter(filter)
            .unwrap()
            .formater(formater)
            .log_to_stdout()
            .map_err(|e| eprintln!("failed to init nonblock_logger: {:?}", e))
            .unwrap();

        info!("opt: {:?}", opt);

        (handle, opt)
    }
}

use nonblock_logger::{
    log::{LevelFilter, Record},
    BaseFilter, BaseFormater, FixedLevel, JoinHandle, NonblockLogger,
};

pub fn format(base: &BaseFormater, record: &Record) -> String {
    let level = FixedLevel::with_color(record.level(), base.color_get())
        .length(base.level_get())
        .into_colored()
        .into_coloredfg();

    format!(
        "[{} {}#{}:{} {}] {}\n",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S.%3f"),
        level,
        record.target(),
        // record.file().unwrap_or("*"),
        record.line().unwrap_or(0),
        current_thread_name(),
        record.args()
    )
}

pub fn current_thread_name() -> &'static str {
    use std::*;
    struct ThreadId(u64);

    thread_local!(static THREAD_NAME: String = {
        let thread = thread::current();
        format!("{:2}.{}", unsafe { mem::transmute::<_, ThreadId>(thread.id()).0 }, thread.name()
        .map(|s| s.to_owned())
        // unamed thread, main has 4 chars, aligned
        .unwrap_or_else(||"****".to_owned()))
    });

    THREAD_NAME.with(|tname| unsafe { mem::transmute::<&str, &'static str>(tname.as_str()) })
}
