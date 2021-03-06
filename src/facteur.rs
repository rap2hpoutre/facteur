use std::process::Command;
use std::os;
use std::fs;
use std::env;
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
        print!(" ([Pretend] {})", $v);
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
        self.basedir = Self::canonicalize(&self.basedir);
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
                pretend!(&format!("mkdir {}/shared", self.basedir));
            }
        }
        done!(self)
    }

    pub fn mkdir_release(self) -> Self {
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

    pub fn clone(self) -> Self {
        print!("Checkout");
        {
            let git = self.git.as_ref().unwrap();

            match self.pretend {
                false => {
                    let output = Command::new("git")
                        .arg("clone")
                        .arg(git)
                        .arg(&self.release_dir())
                        .output()
                        .expect("Failed to clone repo");

                    println!("{}", String::from_utf8_lossy(&output.stdout));
                    println!("{}", String::from_utf8_lossy(&output.stderr));

                    if !output.status.success() {
                        Self::abort("Failed to clone repo");
                    }
                },
                true => pretend!(format ! ("git clone {:?} {}", git, &self.release_dir()))
            }
        }
        done!(self)
    }

    pub fn composer(self) -> Self {
        print!("Composer install");
        match self.pretend {
            false => {
                let output = Command::new("composer")
                    .arg("install")
                    .arg("-d")
                    .arg(&self.release_dir())
                    .arg("--no-dev")
                    .arg("--prefer-dist")
                    .arg("--optimize-autoloader")
                    .output()
                    .expect("Composer installation failed");

                println!("{}", String::from_utf8_lossy(&output.stdout));
                println!("{}", String::from_utf8_lossy(&output.stderr));

                if !output.status.success() {
                    Self::abort("Composer installation failed");
                }
            },
            true => {
                pretend!(format!("composer install -d {} --no-dev --prefer-dist", &self.release_dir()));
            }
        }
        done!(self)
    }

    pub fn init_env(self) -> Self {
        print!("Init .env file");
        match self.pretend {
            false => {
                fs::copy(
                    format!("{}/.env.example", &self.release_dir()),
                    format!("{}/.env", &self.release_dir())).ok();
            },
            true => {
                pretend!(format!("cp {}/.env.example {}/.env", &self.release_dir(), &self.release_dir()));
            }
        }
        done!(self)
    }

    pub fn init_storage(self) -> Self {
        print!("Init storage");
        match self.pretend {
            false => {
                fs::rename(
                    format!("{}/storage", &self.release_dir()),
                    format!("{}/shared/storage", &self.basedir)).ok();
                os::unix::fs::symlink(
                    format!("{}/shared/storage", &self.basedir),
                    format!("{}/storage", &self.release_dir())
                ).unwrap_or_else(|why| {
                    Self::abort(&format!("Cannot create symlink. {:?}", why.kind()));
                });
            },
            true => {
                pretend!(format!("mv {}/storage {}/shared/storage", &self.release_dir(), &self.basedir));
                pretend!(format!("ln -s {}/shared/storage {}/storage", &self.basedir, &self.release_dir()));
            }
        }
        done!(self)
    }
    pub fn symlink(self) -> Self {
        print!("Symlink");
        match self.pretend {
            false => {
                fs::remove_file(format!("{}/current", &self.basedir)).ok();
                os::unix::fs::symlink(&self.release_dir(), format!("{}/current", &self.basedir))
                    .unwrap_or_else(|why| { Self::abort(&format!("Cannot create symlink. {:?}", why.kind())); });
            },
            true => {
                pretend!(format!("rm -Rf {}/current", &self.basedir));
                pretend!(format!("ln -s {} {}/current", &self.release_dir(), &self.basedir));
            }
        }
        done!(self)
    }

    pub fn bye(self, text: &str) -> Self {
        println!("{}", text);
        done!(self)
    }
    pub fn copy_env(self) -> Self {
        print!("copy_env");
        match self.pretend {
            false => {
                fs::copy(format!("{}/current/.env", &self.basedir), format!("{}/.env", &self.release_dir())).ok();
            },
            true => {
                pretend!(format!("cp {}/current/.env {}/.env", &self.basedir, &self.release_dir()));
            }
        }
        done!(self)
    }
    pub fn switch_storage(self) -> Self {
        print!("Link storage dir");
        match self.pretend {
            false => {
                fs::remove_dir_all(format!("{}/storage", &self.release_dir())).ok();
                os::unix::fs::symlink(
                    format!("{}/shared/storage", &self.basedir),
                    format!("{}/storage", &self.release_dir())
                ).unwrap_or_else(|why| {
                    Self::abort(&format!("Cannot create symlink. {:?}", why.kind()));
                }
                );
            },
            true => {
                pretend!(format!("rm {}/storage", &self.release_dir()));
                pretend!(format!("ln -s {}/shared/storage {}/storage", &self.basedir, &self.release_dir()));
            }
        }
        done!(self)
    }
    pub fn migrate(self) -> Self {
        print!("Artisan migrate");
        match self.pretend {
            false => {
                let output = Command::new("php")
                    .current_dir(&self.release_dir())
                    .arg("artisan")
                    .arg("migrate")
                    .arg("--force")
                    .output()
                    .expect("Failed to migrate");

                println!("{}", String::from_utf8_lossy(&output.stdout));
                println!("{}", String::from_utf8_lossy(&output.stderr));

                if !output.status.success() {
                    Self::abort("Artisan migration failed");
                }
            },
            true => {
                pretend!(format!("php artisan migrate --force"));
            }
        }
        done!(self)
    }
    pub fn clean_old_releases(self) -> Self {
        print!("Delete old releases");
        let mut paths = Self::get_sorted_paths(&format!("{}/releases", &self.basedir));
        for _ in 0..3 {
            paths.pop();
        }
        for path in paths {
            println!("Destroying old release: {}", path.path().display());
            match self.pretend {
                false => {
                    fs::remove_dir_all(path.path()).ok();
                },
                true => {
                    pretend!(format!("rm -Rf {}", path.path().display()))
                }
            }
        }
        done!(self)
    }

    pub fn rollback(self) -> Self {
        print!("Rollback");
        let previous = &Self::get_previous_release(&self.basedir);
        fs::remove_file(format!("{}/current", &self.basedir)).ok();
        os::unix::fs::symlink(previous, format!("{}/current", &self.basedir))
            .unwrap_or_else(|why| {
                Self::abort(&format!("Cannot create symlink. {:?}", why.kind()));
            });
        done!(self)
    }

    fn canonicalize(dir: &str) -> String {
        match fs::canonicalize(&PathBuf::from(&dir)) {
            Ok(dir) => dir.as_os_str().to_str().unwrap().to_string(),
            Err(_) => format!("{}/{}", env::current_dir().unwrap().display(), dir)
        }
    }

    fn mkdir_or_die(dir: &str) {
        fs::create_dir_all(dir)
            .unwrap_or_else(|why| {
                Self::abort(&format!("Cannot create dir. {:?}", why.kind()));
            });
    }

    fn abort(msg: &str) -> ! {
        panic!("ABORTED: {}", msg)
    }

    fn ts() -> String {
        time::now().strftime("%Y%m%d%H%M%S").unwrap().to_string()
    }

    fn get_sorted_paths(dir: &str) -> Vec<fs::DirEntry> {
        let mut paths: Vec<_> = fs::read_dir(dir).unwrap()
            .map(|r| r.unwrap())
            .collect();
        paths.sort_by_key(|dir| dir.path());
        paths
    }

    fn get_previous_release(dir: &str) -> String {
        let mut paths = Self::get_sorted_paths(&format!("{}/releases", dir));
        paths.pop();
        format!("{}", paths.pop().unwrap().path().display())
    }
}
