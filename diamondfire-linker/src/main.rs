use std::{
    env,
    path::PathBuf
};


#[derive(Default, Debug)]
struct Cli {
    input_paths : Vec<PathBuf>
}
impl Cli {
    fn parse() -> Cli {
        let mut cli = Cli::default();

        let mut args = env::args();
        while let Some(arg) = args.next() {
            if (arg == "-o") {
                let mut input_path = PathBuf::from(args.next().unwrap()); // TODO: Error handler instead of unwrap.
                input_path.set_extension("dfrs-cg");
                cli.input_paths.push(input_path);
            } else if (! arg.starts_with('-')) {
                if (arg.ends_with(".rlib")) {
                    let mut input_path = PathBuf::from(arg);
                    input_path.set_extension("dfrs-cg");
                    cli.input_paths.push(input_path);
                } else {
                    eprintln!("Ignoring unrecognised arguement {:?}", arg);
                }
            } else {
                eprintln!("Ignoring unrecognised flag {:?}", arg);
            }
        }

        cli
    }
}


fn main() {
    let cli = Cli::parse();
    println!("{:#?}", cli);
    todo!();
}
