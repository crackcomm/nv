extern crate zbox;

use std::path::PathBuf;

use repl_rs::{Command, Parameter, Repl};
use structopt::StructOpt;
use zbox::init_env;

use nv::{
    app::{cmd, Application, Prompt},
    errors::Result,
    Opt,
};

fn main() -> Result<()> {
    let mut opt = Opt::from_args();
    opt.repo_uri = opt
        .repo_uri
        .replace("$HOME", &dirs::home_dir().unwrap().display().to_string());
    opt.repo_uri = opt.repo_uri.replace("$NAMESPACE", &opt.namespace);

    if opt.suri {
        opt.repo_uri = nv::common::secret_prompt("Repo URI");
    }

    if opt.debug {
        println!("Repository: {}", opt.repo_uri);
    }

    if opt.repo_uri.starts_with("file://") {
        let path: PathBuf = opt.repo_uri.strip_prefix("file://").unwrap().into();
        let repo_exists = std::fs::metadata(&path).is_ok();
        if !repo_exists && !opt.create {
            println!(
                "Repository {} doesn't exist. Use -c or --create flag to create a repository.",
                opt.repo_uri
            );
            std::process::exit(1);
        } else if !repo_exists {
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        }
        opt.create = opt.create && !repo_exists;
    }

    // initialise Zbox environment
    init_env();

    let app = Application::new(opt).unwrap();
    let mut repl = Repl::new(app)
        .with_name("nv")
        .with_version(&format!(
            "{} ({})",
            env!("CARGO_PKG_VERSION"),
            env!("GIT_HASH")
        ))
        .with_description("secure password store")
        .with_prompt(&Prompt)
        .add_command(
            Command::new("cat", cmd::cat)
                .with_parameter(Parameter::new("path").set_required(true)?)?
                .with_help("Print contents of file to terminal"),
        )
        .add_command(
            Command::new("cd", cmd::cd)
                .with_parameter(Parameter::new("path").set_default("/")?)?
                .with_help("Change current working directory"),
        )
        .add_command(
            Command::new("changepwd", cmd::changepwd).with_help("Change repository password"),
        )
        .add_command(Command::new("clear", cmd::clear).with_help("Clear the current screen"))
        .add_command(
            Command::new("cp", cmd::cp)
                .with_parameter(Parameter::new("path").set_required(true)?)?
                .with_help("Copy contents of file to clipboard"),
        )
        .add_command(
            Command::new("gen", cmd::gen)
                .with_parameter(Parameter::new("path").set_required(true)?)?
                .with_parameter(Parameter::new("length").set_default("36")?)?
                .with_help("Generate random password and save to path"),
        )
        .add_command(
            Command::new("info", cmd::info).with_help("Print password repository information"),
        )
        .add_command(
            Command::new("ls", cmd::ls)
                .with_parameter(Parameter::new("path").set_default(".")?)?
                .with_help("List all files in directory"),
        )
        .add_command(
            Command::new("mkdir", cmd::mkdir)
                .with_parameter(Parameter::new("path").set_required(true)?)?
                .with_help("Create a directory"),
        )
        .add_command(Command::new("pwd", cmd::pwd).with_help("Print current working directory"))
        .add_command(
            Command::new("rm", cmd::rm)
                .with_parameter(Parameter::new("path").set_required(true)?)?
                .with_help("Remove file or directory"),
        )
        .add_command(
            Command::new("set", cmd::set)
                .with_parameter(Parameter::new("path").set_required(true)?)?
                .with_help("Write file contents from secret prompt"),
        )
        .add_command(
            Command::new("setcp", cmd::setcp)
                .with_parameter(Parameter::new("path").set_required(true)?)?
                .with_help("Write file contents from clipboard and clear clipboard"),
        )
        .add_command(
            Command::new("vi", cmd::vi)
                .with_parameter(Parameter::new("path").set_required(true)?)?
                .with_help("Insecure file access that leaks files to your /tmp"),
        );

    Ok(repl.run()?)
}
