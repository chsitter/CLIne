extern crate cline;
use cline::{Cli, cline_run};

fn main() {
    let mut cli = Cli::new();

    cli.register(vec!["foo", "bar"], | _ | { println!("running foo bar") }).ok();
    cli.register(vec!["foo", "baz"], | _ | { println!("running foo baz") }).ok();

    cline_run(&mut cli);
}
