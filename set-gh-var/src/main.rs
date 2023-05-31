use crate::gh_var::{get_gh_var, GhVar};
use std::{
    fs,
    io::{self, Write},
};

mod cli;
mod env_name;
mod gh_var;

fn main() -> io::Result<()> {
    let (cmd, args) = cli::get_args();
    let gh_var = get_gh_var(&cmd);

    write_to_file(&gh_var, &args)
}

fn write_to_file(gh_var: &GhVar, args: &[String]) -> io::Result<()> {
    let mut file = fs::File::options()
        .append(true)
        .create(true)
        .open(&gh_var.file)?;

    for (k, v) in args
        .chunks_exact(2)
        .map(|chunk| (&chunk[0], &chunk[1]))
    {
        eprintln!(r#"K: {},	V: "{}""#, k, v);
        writeln!(file, r#"{k}="{v}""#)?;
    }
    Ok(())
}
