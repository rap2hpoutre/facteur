use facteur::Facteur;

const INIT_END_TEXT: &'static str =
r#"Init done.

Now you may have to:
• edit your .env file and add your configuration data
• run `php artisan key:generate`  in `current` dir
• setup nginx
• run your migrations
• chown your basedir with user you want to use"#;

pub fn init(postman: Facteur) {
    postman
        .welcome("Initialisation")
        .mkdir_base()
        .canonicalize_basedir()
        .mkdir_release()
        .clone()
        .composer()
        .init_env()
        .init_storage()
        .symlink()
        .bye(INIT_END_TEXT);
}

pub fn deploy(postman: Facteur) {
    postman
        .welcome("Deployment")
        .canonicalize_basedir()
        .mkdir_release()
        .clone()
        .copy_env()
        .composer()
        .switch_storage()
        .migrate()
        .symlink()
        .clean_old_releases()
        .bye("Deployment Success");
}

pub fn rollback(postman: Facteur) {
    postman
        .welcome("Rollback")
        .rollback()
        .bye("Rollback done");
}