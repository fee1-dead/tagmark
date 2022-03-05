#![feature(exit_status_error, once_cell)]
use std::env;
use std::fs::{copy, create_dir_all};
use std::lazy::SyncLazy;
use std::path::Path;
use std::process::{Command, Stdio};

use clap::Parser;

static CARGO: SyncLazy<&'static Path> = SyncLazy::new(|| {
    let path = if let Ok(p) = env::var("CARGO") {
        Path::new(Box::leak(p.into_boxed_str()))
    } else {
        Path::new("cargo")
    };

    match Command::new(path).stdout(Stdio::null()).arg("-V").status() {
        Ok(stat) if stat.success() => {}
        Ok(stat) => panic!("E: invalid cargo path ({path:?}), `cargo -V` returned {stat:?}"),
        Err(e) => panic!("E: invalid cargo path ({path:?}), error spawning `cargo -V`{e:?}"),
    }

    path
});

const COMPONENTS: &[&str] = &["popup"];

const SUFFIXES: &[&str] = &[".js", "_bg.wasm"];

#[derive(clap::Args)]
struct BuildArgs {}

#[derive(Parser)]
#[clap(author, version, name = "builder", bin_name = "./z.sh")]
enum Run {
    Build(BuildArgs),
    Serve(BuildArgs),
}

fn main() -> anyhow::Result<()> {
    let subcmd = Run::parse();

    match subcmd {
        Run::Build(_) => {
            create_dir_all("pkg/resources")?;
            for component in COMPONENTS {
                Command::new("wasm-pack")
                    .arg("build")
                    .arg("--target=no-modules")
                    .current_dir(Path::new(component))
                    .status()?
                    .exit_ok()?;
                for suffix in SUFFIXES {
                    copy(
                        format!("{component}/pkg/{component}{suffix}"),
                        format!("pkg/resources/{component}{suffix}"),
                    )?;
                }
            }
        }
        Run::Serve(_) => {}
    }
    println!("Hello, world! {}", CARGO.display());
    Ok(())
}
