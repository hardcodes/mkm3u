use clap::ArgMatches;
use std::vec;

use crate::PROGRAM_AUTHORS;
use crate::PROGRAM_DESCRIPTION;
use crate::PROGRAM_NAME;
use crate::PROGRAM_VERSION;
use std::path::PathBuf;

const OUTPUT_FILE: &str = "output-file";
const INPUT_FILE: &str = "input-file";
const INPUT_DIRECTORY: &str = "input-directory";
const INPUT_GROUP: &str = "input-group";

/// Parse the command line parameters with help of clap.
pub fn parse_cli_parameters() -> clap::ArgMatches {
    clap::Command::new(PROGRAM_NAME)
        .version(PROGRAM_VERSION)
        .author(PROGRAM_AUTHORS)
        .about(PROGRAM_DESCRIPTION)
        .arg(
            clap::Arg::new(OUTPUT_FILE)
                .long("out")
                .value_name("OUTFILE")
                .help("m3u output file that should be created ")
                .num_args(1)
                .required(true)
                .value_parser(clap::value_parser!(PathBuf)),
        )
        .arg(
            clap::Arg::new(INPUT_FILE)
                .long("file")
                .value_name("INFILE")
                .help("mp3 file name")
                .use_value_delimiter(true)
                .value_delimiter(',')
                .value_parser(clap::value_parser!(PathBuf)),
        )
        .arg(
            clap::Arg::new(INPUT_DIRECTORY)
                .long("dir")
                .value_name("INDIR")
                .help("directory with mp3 files")
                .use_value_delimiter(true)
                .value_delimiter(',')
                .value_parser(clap::value_parser!(PathBuf)),
        )
        .group(
            clap::ArgGroup::new(INPUT_GROUP)
                .args(&[INPUT_FILE, INPUT_DIRECTORY])
                .multiple(true)
                .required(true),
        )
        .after_help(r##""##)
        .get_matches()
}

pub struct CliTask{
    pub file: Option<Vec<PathBuf>>,
    pub dir: Option<Vec<PathBuf>>,
    pub out: PathBuf,
}

pub fn get_cli_details(cli_args: &ArgMatches) -> CliTask{
    let file: Option<Vec<PathBuf>> = match cli_args.contains_id(INPUT_FILE){
        true => Some(cli_args.get_occurrences::<PathBuf>(INPUT_FILE).unwrap().map(Iterator::collect).collect()),
        false => None,
    };
    let dir: Option<Vec<PathBuf>> = match cli_args.contains_id(INPUT_DIRECTORY){
        true => Some(cli_args.get_occurrences::<PathBuf>(INPUT_DIRECTORY).unwrap().map(Iterator::collect).collect()),
        false => None,
    };
    let out: PathBuf = cli_args.get_one::<PathBuf>(OUTPUT_FILE).unwrap().to_owned();
    CliTask { file, dir, out }
}