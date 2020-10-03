extern crate zbox;

use std::{
    cell::RefCell,
    fmt,
    io::{Seek, SeekFrom},
    path::{Path, PathBuf},
    sync::Arc,
};

use console::style;

use crate::{errors::Result, repo, Opt};

pub struct Application {
    pub opt: Opt,
    pub cwd: PathBuf,
    pub repo: Arc<RefCell<zbox::Repo>>,
    // Used just for change password.
    password: String,
}

impl Application {
    pub fn new(opt: Opt) -> Result<Self> {
        let (repo, password) = repo::open(&opt)?;
        Ok(Application {
            opt,
            cwd: "/".into(),
            repo: Arc::new(RefCell::new(repo)),
            password,
        })
    }

    pub fn repo(&self) -> Arc<RefCell<zbox::Repo>> {
        self.repo.clone()
    }

    pub(crate) fn set_<P: AsRef<Path>>(&mut self, path: P, contents: &[u8]) -> Result<()> {
        let mut file = zbox::OpenOptions::new()
            .create(true)
            .open(&mut self.repo.borrow_mut(), path)?;

        file.seek(SeekFrom::Start(0))?;
        file.write_once(contents)?;
        file.set_len(contents.len())?;

        Ok(())
    }
}

pub type Time = chrono::DateTime<chrono::offset::Local>;
pub type Args = std::collections::HashMap<String, repl_rs::Value>;

pub struct Prompt;

impl fmt::Display for Prompt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} ",
            style("✔").green(),
            style("nv").white().bold(),
            style("›").dim(),
        )
    }
}

pub mod cmd {
    use std::io::{Read, Seek, SeekFrom};

    use console::{style, Term};
    use prettytable::{Cell, Row, Table};
    use zbox::OpenOptions;

    use clipboard::{ClipboardContext, ClipboardProvider};

    use path_abs::PathAbs;
    use repl_rs::Convert;

    use crate::{
        app::{Application, Args, Time},
        common::{confirm_prompt, rand_password, secret_prompt},
        errors::{Error, Result},
        password, seed,
    };

    pub fn pwd(_args: Args, app: &mut Application) -> Result<Option<String>> {
        Ok(Some(app.cwd.display().to_string()))
    }

    pub fn cd(args: Args, app: &mut Application) -> Result<Option<String>> {
        let path: String = args["path"].convert()?;
        let full_path = PathAbs::new(app.cwd.join(&path))?;
        if app.repo.borrow_mut().is_dir(&full_path)? {
            app.cwd = full_path.as_path().to_owned();
            Ok(None)
        } else {
            Err(Error::NotDirectory(path))
        }
    }

    pub fn mkdir(args: Args, app: &mut Application) -> Result<Option<String>> {
        let path: String = args["path"].convert()?;
        let full_path = PathAbs::new(app.cwd.join(path))?;
        app.repo.borrow_mut().create_dir_all(full_path)?;
        Ok(None)
    }

    pub fn rm(args: Args, app: &mut Application) -> Result<Option<String>> {
        let path: String = args["path"].convert()?;
        let full_path = PathAbs::new(app.cwd.join(path))?;

        let mut repo = app.repo.borrow_mut();
        let metadata = repo.metadata(&full_path)?;
        if metadata.is_dir() {
            if confirm_prompt(format!(
                "Delete directory {} with all files?",
                full_path.as_path().display()
            ))? {
                repo.remove_dir_all(full_path)?;
            }
        } else if confirm_prompt(format!("Delete file {}?", full_path.as_path().display()))? {
            repo.remove_file(full_path)?;
        }

        Ok(None)
    }

    pub fn set(args: Args, app: &mut Application) -> Result<Option<String>> {
        let path: String = args["path"].convert()?;
        let full_path = PathAbs::new(app.cwd.join(path))?;
        let contents = secret_prompt("Secret")?;
        app.set_(full_path, contents.as_bytes())?;
        Ok(None)
    }

    pub fn setcp(args: Args, app: &mut Application) -> Result<Option<String>> {
        let path: String = args["path"].convert()?;
        let full_path = PathAbs::new(app.cwd.join(path))?;
        let contents = {
            let mut ctx: ClipboardContext =
                ClipboardProvider::new().map_err(|e| Error::Clipboard(e))?;
            let secret = ctx.get_contents().map_err(|e| Error::Clipboard(e))?;
            // Clear the contents
            ctx.set_contents("".to_owned())
                .map_err(|e| Error::Clipboard(e))?;
            secret
        };
        app.set_(full_path, contents.as_bytes())?;
        Ok(None)
    }

    pub fn cat(args: Args, app: &mut Application) -> Result<Option<String>> {
        let path: String = args["path"].convert()?;
        let full_path = PathAbs::new(app.cwd.join(path))?;

        // create a file and write content to it
        let mut file = OpenOptions::new()
            .read(true)
            .open(&mut app.repo.borrow_mut(), full_path)?;
        // read all full_path
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(Some(content))
    }

    pub fn cp(args: Args, app: &mut Application) -> Result<Option<String>> {
        let contents = cat(args, app)?.expect("file contents");
        let mut ctx: ClipboardContext =
            ClipboardProvider::new().map_err(|e| Error::Clipboard(e))?;
        ctx.set_contents(contents)
            .map_err(|e| Error::Clipboard(e))?;
        Ok(None)
    }

    pub fn vi(args: Args, app: &mut Application) -> Result<Option<String>> {
        if !confirm_prompt("Insecure access that leaks secrets to your file system, continue?")? {
            return Ok(None);
        }
        let path: String = args["path"].convert()?;
        let full_path = PathAbs::new(app.cwd.join(path))?;

        let mut file = OpenOptions::new()
            .read(true)
            .create(true)
            .open(&mut app.repo.borrow_mut(), full_path)?;

        // read all content
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        if let Some(changed) = dialoguer::Editor::new().edit(&content)? {
            file.seek(SeekFrom::Start(0))?;
            file.write_once(changed.as_bytes())?;
            file.set_len(changed.len())?;
            Ok(None)
        } else {
            Err(Error::Abort)
        }
    }

    pub fn ls(args: Args, app: &mut Application) -> Result<Option<String>> {
        let path: String = args["path"].convert()?;
        let full_path = PathAbs::new(app.cwd.join(path))?;

        let dirs = app.repo.borrow_mut().read_dir(full_path)?;
        let mut table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new("name"),
            Cell::new("size"),
            Cell::new("created"),
            Cell::new("modified"),
            Cell::new("ver"),
        ]));

        for node in dirs {
            table.add_row(Row::new(vec![
                Cell::new(&if node.metadata().is_dir() {
                    style(node.file_name()).blue().to_string()
                } else {
                    node.file_name().to_owned()
                }),
                Cell::new(&node.metadata().content_len().to_string()),
                Cell::new(&format!(
                    "{}",
                    Time::from(node.metadata().created_at()).format("%d/%m/%Y %T")
                )),
                Cell::new(&format!(
                    "{}",
                    Time::from(node.metadata().modified_at()).format("%d/%m/%Y %T")
                )),
                Cell::new(&node.metadata().curr_version().to_string()),
            ]));
        }

        table.set_format(*prettytable::format::consts::FORMAT_CLEAN);
        table.printstd();
        Ok(None)
    }

    pub fn clear(_args: Args, _app: &mut Application) -> Result<Option<String>> {
        Term::stdout().clear_screen()?;
        Ok(None)
    }

    pub fn gen(args: Args, app: &mut Application) -> Result<Option<String>> {
        let path: String = args["path"].convert()?;
        let length: usize = args["length"].convert()?;
        let full_path = PathAbs::new(app.cwd.join(&path))?;
        if app.repo.borrow_mut().path_exists(&full_path)? {
            Err(Error::FileExists(path))
        } else {
            let password = rand_password(length);
            app.set_(full_path, &password)?;
            let mut ctx: ClipboardContext =
                ClipboardProvider::new().map_err(|e| Error::Clipboard(e))?;
            let password = String::from_utf8(password)?;
            ctx.set_contents(password)
                .map_err(|e| Error::Clipboard(e))?;
            Ok(None)
        }
    }

    pub fn changepwd(_args: Args, app: &mut Application) -> Result<Option<String>> {
        let password = password::prompt(true)?;
        if app.opt.debug {
            println!("Your password is {} characters long", password.len());
        }
        let password = seed::create(&app.opt, &password)?;

        app.repo.borrow_mut().reset_password(
            &app.password,
            &password,
            zbox::OpsLimit::Sensitive,
            zbox::MemLimit::Sensitive,
        )?;

        app.password = password;

        Ok(None)
    }

    pub fn info(_args: Args, app: &mut Application) -> Result<Option<String>> {
        let info = app.repo.borrow_mut().info()?;
        println!("Zbox Version: {}", info.version());
        println!("Repository ID: {}", info.volume_id().to_string());
        println!("Cipher: {:?}", info.cipher());
        println!("Read only: {:?}", info.is_read_only());
        println!("Compression: {:?}", info.compress());
        println!(
            "Created: {}",
            Time::from(info.created_at()).format("%d/%m/%Y %T")
        );
        Ok(None)
    }
}
