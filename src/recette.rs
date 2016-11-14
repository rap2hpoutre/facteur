use cuisinier;
use std::fs;
use std::os;

const INIT_END_TEXT: &'static str =
r#"Init done.

Now you may have to:
• edit your .env file and add your configuration data
• run `php artisan key:generate`  in `current` dir
• setup nginx
• run your migrations
• chown your basedir with user you want to use"#;

// Initialize a Laravel app
pub fn init(dir: &str, git: &str) {
    println!("Initialization");

    let release_ts = helpers::release_timestamp();
    if fs::metadata(dir).is_ok() {
        helpers::abort(&format!("Dir {} exists, please remove it if you want to initialize your app.", dir));
    }

    println!("Building dirs (base {})", dir);
    helpers::mkdirs(dir);
    println!("Dirs created created.✅");

    let dir = &helpers::canonicalize(dir);

    println!("Git Checkout ({})", git);
    let dir_ts = &format!("{}/releases/{}", dir, &release_ts);
    helpers::mkdir_or_die(dir_ts);
    helpers::clone(dir_ts, git);
    println!("Git Checkout done ✅");

    println!("Composer install");
    helpers::composer(dir_ts);
    println!("Composer install done ✅");

    println!("Prepare .env");
    fs::copy(format!("{}/.env.example", dir_ts), format!("{}/.env", dir_ts)).ok();
    println!(".env ready  ✅");

    println!("Moving storage DIR");
    fs::rename(format!("{}/storage", dir_ts), format!("{}/shared/storage", dir)).ok();
    os::unix::fs::symlink(format!("{}/shared/storage", dir), format!("{}/storage", dir_ts))
        .unwrap_or_else(|why| { helpers::abort(&format!("Cannot create symlink. {:?}", why.kind())); });
    println!("Storage DIR moved✅");

    println!("Building Symlink");
    fs::remove_file(format!("{}/current", dir)).ok();
    os::unix::fs::symlink(format!("{}/releases/{}", dir, &release_ts), format!("{}/current", dir))
        .unwrap_or_else(|why| { helpers::abort(&format!("Cannot create symlink. {:?}", why.kind())); });
    println!("Symlink built ✅");

    println!("{}", INIT_END_TEXT);
}

// Run deployment script
pub fn deploy(dir: &str, git: &str) {
    println!("Deployment.");

    let release_ts = helpers::release_timestamp();
    let dir = &helpers::canonicalize(dir);

    println!("Git Checkout ({})", git);
    let dir_ts = &format!("{}/releases/{}", dir, &release_ts);
    helpers::mkdir_or_die(dir_ts);
    helpers::clone(dir_ts, git);
    println!("Git Checkout done ✅");

    println!("Copy .env");
    fs::copy(format!("{}/current/.env", dir), format!("{}/.env", dir_ts)).ok();
    println!(".env copied ✅");

    println!("Composer install");
    helpers::composer(dir_ts);
    println!("Composer install done ✅");

    println!("Do not use current storage DIR");
    fs::remove_dir_all(format!("{}/storage", dir_ts)).ok();
    os::unix::fs::symlink(format!("{}/shared/storage", dir), format!("{}/storage", dir_ts))
        .unwrap_or_else(|why| { helpers::abort(&format!("Cannot create symlink. {:?}", why.kind())); });
    println!("Storage point to symlink moved✅");

    println!("Run migrations");
    helpers::migrate(dir_ts);

    println!("Migrations done✅");

    println!("Building Symlink");
    fs::remove_file(format!("{}/current", dir)).ok();
    os::unix::fs::symlink(format!("{}/releases/{}", dir, &release_ts), format!("{}/current", dir))
        .unwrap_or_else(|why| {
            helpers::abort(&format!("Cannot create symlink. {:?}", why.kind()));
        });
    println!("Symlink built ✅");

    println!("Clean previous releases");
    helpers::clean_old_releases(dir);
    println!("Previous releases cleaned ✅");

    println!("Notif slack?");
    helpers::success();
    println!("FINISH");
}

pub fn rollback(dir: &str) {
    println!("Starting rollback.");

    let dir = &helpers::canonicalize(dir);

    println!("Change simlink.");
    let previous = &helpers::get_previous_release(dir);
    fs::remove_file(format!("{}/current", dir)).ok();
    os::unix::fs::symlink(previous, format!("{}/current", dir))
        .unwrap_or_else(|why| {
            helpers::abort(&format!("Cannot create symlink. {:?}", why.kind()));
        });
    println!("Symlink changed  ✅");

    println!("Notif slack?");
    helpers::rollback_done();
    println!("Rollback done.");
}