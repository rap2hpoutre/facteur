use std::process::Command;
use std::fs;
use slack_hook::{Slack, PayloadBuilder};
use time;
use std::path::PathBuf;

macro_rules! done {
    ($v:expr) => {{
        println!(" ...OK");
        $v
    }}
}

macro_rules! pretend {
    ($v:expr) => {
        print!(" <[Pretend] {}>", $v);
    }
}

pub struct Facteur {
    basedir: String,
    git: Option<String>,
    ts: String,
    pretend: bool
}

impl Facteur {

    pub fn new(dir: &str, pretend: bool) -> Self {
        Facteur {
            basedir: dir.to_string(),
            git: None,
            ts: Self::ts(),
            pretend: pretend
        }
    }

    pub fn git(mut self, git: &str) -> Self {
        self.git = Some(git.to_string());
        self
    }

    pub fn welcome(self, text: &str) -> Self {
        println!("{}", text);
        self
    }

    pub fn canonicalize_basedir(mut self) -> Self {
        if !self.pretend {
            self.basedir = Self::canonicalize(&self.basedir);
        }
        self
    }

    pub fn mkdir_base(self) -> Self {
        print!("Making basedir {}", &self.basedir);
        match self.pretend {
            false => {
                Self::mkdir_or_die(&self.basedir);
                Self::mkdir_or_die(&format!("{}/releases", self.basedir));
                Self::mkdir_or_die(&format!("{}/shared", self.basedir));
            },
            true => {
                pretend!(&format!("mkdir {}", self.basedir));
                pretend!(&format!("mkdir {}/releases", self.basedir));
                pretend!(&format!("mkdir {}/releases", self.basedir));
            }
        }
        done!(self)
    }

    pub fn mkdir_release(mut self) -> Self {
        print!("Initializing Release dir");
        match self.pretend {
            false => Self::mkdir_or_die(&self.release_dir()),
            true => pretend!(format!("mkdir {}", &self.release_dir()))
        }
        done!(self)
    }

    fn release_dir(&self) -> String {
        format!("{}/releases/{}", self.basedir, &self.ts)
    }

    pub fn checkout(self) -> Self {
        print!("Checkout");
        match self.pretend {
            false => {
                {
                    let git = self.git.as_ref().unwrap();
                    if !Self::clone(&self.basedir, &git) {
                        Self::abort("Failed to clone repo");
                    }
                }

            },
            true => pretend!(format!("clone {:?} in {}", &self.git, &self.release_dir()))
        }
        self
    }

    fn clone(dir: &str, git: &str) -> bool {
        let output = Command::new("git")
            .arg("clone")
            .arg(git)
            .arg(dir)
            .output()
            .expect("Failed to clone repo");

        println!("{}", String::from_utf8_lossy(&output.stdout));
        println!("{}", String::from_utf8_lossy(&output.stderr));

        output.status.success()
    }

    pub fn composer(self) -> Self {
        println!("composer");
        self
    }
    pub fn init_env(self) -> Self {
        println!("init_env");
        self
    }
    pub fn init_storage(mut self) -> Self {
        println!("init_storage");
        self
    }
    pub fn symlink(mut self) -> Self {
        println!("symlink");
        self
    }
    pub fn bye(mut self, text: &str) -> Self {
        println!("text");
        self
    }
    pub fn copy_env(mut self) -> Self {
        println!("copy_env");
        self
    }
    pub fn switch_storage(mut self) -> Self {
        println!("switch_storage");
        self
    }
    pub fn migrate(mut self) -> Self {
        println!("migrate");
        self
    }
    pub fn clean_old_releases(mut self) -> Self {
        println!("clean_old_releases");
        self
    }

    pub fn rollback(mut self) -> Self {
        println!("rollback");
        self
    }

    fn canonicalize(dir: &str) -> String {
        let dir = fs::canonicalize(&PathBuf::from(&dir)).unwrap();
        dir.as_os_str().to_str().unwrap().to_string()
    }

    fn mkdir_or_die(dir: &str) {
        fs::create_dir_all(dir)
            .unwrap_or_else(|why| {
                Self::abort(&format!("Cannot create dir. {:?}", why.kind()));
            });
    }

    fn abort(msg: &str) -> ! {
        panic!("ABORTED")
    }

    fn ts() -> String {
        time::now().strftime("%Y%m%d%H%M%S").unwrap().to_string()
    }
}

/*

// Get sorted paths
fn get_sorted_paths(dir: &str) -> Vec<fs::DirEntry>{
    let mut paths: Vec<_> = fs::read_dir(dir).unwrap()
        .map(|r| r.unwrap())
        .collect();
    paths.sort_by_key(|dir| dir.path());
    paths
}

// Get previous release dir
pub fn get_previous_release(dir: &str) -> String {
    let mut paths = get_sorted_paths(&format!("{}/releases", dir));
    paths.pop();
    format!("{}", paths.pop().unwrap().path().display())
}

// Destroy old directories
pub fn clean_old_releases(dir: &str) {
    let mut paths = get_sorted_paths(&format!("{}/releases", dir));
    for _ in 0..3 {
        paths.pop();
    }
    for path in paths {
        println!("Destroying old release: {}", path.path().display());
        fs::remove_dir_all(path.path()).ok();
    }
}

// Clone a git repo in a dir
pub fn clone(dir: &str, git: &str) {
    let output = Command::new("git")
        .arg("clone")
        .arg(git)
        .arg(dir)
        .output()
        .expect("Failed to clone repo");

    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("{}", String::from_utf8_lossy(&output.stderr));

    if !output.status.success() {
        abort("Failed to clone repo");
    }
}

// Generate a release timestamp
pub fn release_timestamp() -> String {
    return time::now().strftime("%Y%m%d%H%M%S").unwrap().to_string();
}

// Create initial dirs
pub fn mkdirs(dir: &str) {
    mkdir_or_die(dir);
    mkdir_or_die(&format!("{}/releases", dir));
    mkdir_or_die(&format!("{}/shared", dir));
}

// Create a dir or die.
pub fn mkdir_or_die(dir: &str) {
    fs::create_dir_all(dir)
        .unwrap_or_else(|why| {
            abort(&format!("Cannot create dir. {:?}", why.kind()));
        });
}

// Install composer
pub fn composer(dir: &str) {

    let output = Command::new("composer")
        .arg("install")
        .arg("-d")
        .arg(dir)
        .arg("--no-dev")
        .arg("--prefer-dist")
        .output()
        .expect("Composer installation failed");

    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("{}", String::from_utf8_lossy(&output.stderr));

    if !output.status.success() {
        abort("Composer installation failed");
    }
}

// PHP artisan migrate
pub fn migrate(dir_ts: &str) {
    let output = Command::new("php")
        .current_dir(dir_ts)
        .arg("artisan")
        .arg("migrate")
        .arg("--force")
        .output()
        .expect("Failed to clone repo");

    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("{}", String::from_utf8_lossy(&output.stderr));

    if !output.status.success() {
        println!("Migration failed, not sure it's a real problem");
    }
}

// Abort slack message
pub fn abort(msg: &str) -> ! {
    slack(&format!("Deployment failed: {}", msg), ":skull_and_crossbones:");
    panic!("ABORTED");
}

// Success slack message
pub fn success()  {
    slack("Deployment success.", ":tropical_drink:");
}

// Success slack message
pub fn rollback_done()  {
    slack("Rollback success.", ":japanese_ogre:");
}

// Slack message
fn slack(message: &str, emoji: &str) {
    let slack = Slack::new("https://hooks.slack.com/services/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").unwrap();
    let p = PayloadBuilder::new()
        .text(message)
        .channel("#app-notifier")
        .username("Deploy Bot")
        .icon_emoji(emoji)
        .build()
        .unwrap();

    let res = slack.send(&p);
    match res {
        Ok(()) => println!("msg sent."),
        Err(x) => println!("<!> Error, msg not sent: {:?}", x)
    }
}

// Canonical dir
pub fn canonicalize(dir: &str) -> String {
    let dir = fs::canonicalize(&PathBuf::from(dir))
        .unwrap();
    dir.as_os_str().to_str().unwrap().to_string()
}

*/