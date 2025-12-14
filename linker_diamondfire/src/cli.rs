use std::{
    env,
    path::PathBuf
};
use anstream::ColorChoice;


#[derive(Default, Debug)]
pub struct Cli {
    /// Whether to print logs in colour.
    pub colour            : ColorChoice,

    /// Crate codegen output paths.
    pub input_paths       : Vec<PathBuf>,
    /// Linking output path.
    pub output_path       : Option<PathBuf>,
    /// Names which may not be mangled or have their signature edited.
    pub exports           : Vec<String>,
    /// Remove debug information.
    pub strip_debug       : bool,
    /// Remove all non-exported names.
    pub strip_all         : bool,
    /// How aggressively to optimise.
    pub opt_level         : u8,
    /// Whether to eliminate dead code.
    pub eliminate_dead    : bool,

    /// Whether to track function backtraces.
    pub track_backtraces  : bool,
    /// Whether to track per-process CPU usage.
    pub track_process_cpu : bool,

    /// Whether to send templates to CCAPI for placement.
    pub export_ccapi      : bool
}

impl Cli {
    pub fn parse() -> Cli {
        let mut cli = Cli::default();

        let mut args = env::args().skip(1);
        while let Some(arg) = args.next() {

            if (arg == "--color") {
                let mode = args.next().unwrap(); // TODO: Error handler instead of unwrap.
                cli.colour = { match (mode.as_str()) {
                    "auto"   => ColorChoice::Auto,
                    "ansi"   => ColorChoice::AlwaysAnsi,
                    "always" => ColorChoice::Always,
                    "never"  => ColorChoice::Never,
                    _        => { panic!(); } // TODO: Error handler instead of panic.
                } };
            }

            else if (arg == "-flavor") {
                args.next().unwrap(); // TODO: Error handler instead of unwrap.
            }

            else if (arg == "-o") {
                let output_path = PathBuf::from(args.next().unwrap()); // TODO: Error handler instead of unwrap.
                cli.input_paths.push(output_path.with_extension("dfrs-cg"));
                cli.output_path = Some(output_path);
            }

            else if (arg == "--export") {
                let export = args.next().unwrap().to_string(); // TODO: Error handler instead of unwrap.
                cli.exports.push(export);
            }

            else if (arg == "--strip-debug") {
                cli.strip_debug = true
            }
            else if (arg == "--strip-all") {
                cli.strip_all = true
            }

            else if let Some(opt_level) = arg.strip_prefix("-O") {
                cli.opt_level = opt_level.parse::<u8>().unwrap(); // TODO: Error handler instead of unwrap.
            }

            else if (arg == "--gc-sections") {
                cli.eliminate_dead = true
            }

            else if (arg == "--track-backtraces") {
                cli.track_backtraces = true
            }
            else if (arg == "--track-process-cpu") {
                cli.track_process_cpu = true
            }

            else if (arg == "--ccapi") {
                cli.export_ccapi = true
            }

            else if (! arg.starts_with('-')) {
                if (arg.ends_with(".rlib")) {
                    let mut input_path = PathBuf::from(arg);
                    input_path.set_extension("dfrs-cg");
                    cli.input_paths.push(input_path);
                } else {
                    eprintln!("Ignoring unrecognised arguement {:?}", arg);
                }
            }

            else {
                eprintln!("Ignoring unrecognised flag {:?}", arg);
            }

        }

        cli
    }
}
