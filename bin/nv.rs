extern crate zbox;

use std::io::{Read, Seek, SeekFrom};

use console::style;
use dialoguer::theme::ColorfulTheme;
use prettytable::{Cell, Row, Table};
use structopt::StructOpt;
use zbox::{init_env, OpenOptions, RepoOpener};

fn main() {
    let opt = nv::Opt::from_args();
    let repo_dir = opt.repo_dir.clone().unwrap_or_else(|| {
        dirs::home_dir()
            .map(|mut home| {
                home.push(".local");
                home.push("nv");
                home.push(&opt.namespace);
                home
            })
            .unwrap()
    });

    let repo_exists = std::fs::metadata(&repo_dir).is_ok();
    if !repo_exists && !opt.create {
        println!(
            "Repository {} doesn't exist. Use --create to create a repository.",
            repo_dir.display()
        );
        std::process::exit(1);
    }

    if opt.debug {
        println!("Options: {:#?}", opt);
        println!("Repository {}", repo_dir.display());
    }

    std::fs::create_dir_all(repo_dir.parent().unwrap()).unwrap();

    // initialise Zbox environment
    init_env();

    let create = !repo_exists && opt.create;
    let mut repo = loop {
        let password = nv::password::prompt(create);
        if opt.debug {
            println!("Your password is {} characters long", password.len());
        }

        let password = if create {
            let start = std::time::Instant::now();
            let (mnemonic, password) = nv::seed::mine(&password, opt.diff, opt.round);
            if opt.debug {
                println!("Seed mined in {:?}", start.elapsed());
            }
            println!("Mnemonic: {}", mnemonic);
            password
        } else {
            let mnemonic = nv::mnemonic::prompt();
            let start = std::time::Instant::now();
            let password = nv::seed::mnemonic(&password, mnemonic.as_slice(), opt.diff);
            if opt.debug {
                println!("Hash computed in {:?}", start.elapsed());
            }
            password
        };

        let repo = RepoOpener::new()
            .create(!repo_exists)
            .compress(true)
            .force(opt.force)
            .cipher(zbox::Cipher::Xchacha)
            .open(&format!("file://{}", repo_dir.display()), &password);
        if repo.is_err() && repo_exists {
            println!("error: {:?}", repo.unwrap_err());
            continue;
        }

        break repo.unwrap();
    };

    loop {
        let cmd: String = dialoguer::Input::with_theme(&ColorfulTheme::default())
            .with_prompt("nv")
            .interact()
            .unwrap();
        let cmdline = cmd
            .split(' ')
            .map(ToOwned::to_owned)
            .collect::<Vec<String>>();
        let (cmd, args) = cmdline.split_first().unwrap();
        match cmd.as_ref() {
            "close" => {
                break;
            }
            "cat" => {
                let path = args.first().map(|s| s.as_ref()).unwrap_or("/");

                // create a file and write content to it
                match OpenOptions::new().read(true).open(&mut repo, path) {
                    Err(err) => {
                        println!("Error: Path {}: {:?}", path, err);
                    }
                    Ok(mut file) => {
                        // read all content
                        let mut content = String::new();
                        file.read_to_string(&mut content).unwrap();
                        assert_eq!(content, "Hello, World!");
                    }
                }
            }
            "ls" => {
                let path = args.first().map(|s| s.as_ref()).unwrap_or("/");
                match repo.read_dir(path) {
                    Ok(dirs) => {
                        let mut table = Table::new();
                        table.add_row(Row::new(vec![Cell::new("filename"), Cell::new("version")]));

                        for node in dirs {
                            // println!("Dir: {:?}", node);
                            table.add_row(Row::new(vec![
                                Cell::new(&if node.metadata().is_dir() {
                                    style(node.file_name()).blue().to_string()
                                } else {
                                    node.file_name().to_owned()
                                }),
                                Cell::new(node.file_name()),
                            ]));
                        }

                        table.set_format(*prettytable::format::consts::FORMAT_CLEAN);
                        table.printstd();
                    }
                    Err(err) => {
                        println!("Error: Path {}: {:?}", path, err);
                    }
                }
            }
            cmd => println!("Command unrecognized: {}", cmd),
        }
    }

    // create a file and write content to it
    let mut file = OpenOptions::new()
        .create(true)
        .open(&mut repo, "/hello_world.txt")
        .unwrap();

    file.write_once(b"Hello, World!").unwrap();

    // seek to the beginning of file
    file.seek(SeekFrom::Start(0)).unwrap();

    // read all content
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    assert_eq!(content, "Hello, World!");
}
