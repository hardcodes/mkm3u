use m3u::url::form_urlencoded::parse;
use mkm3u::cli_parser::{parse_cli_parameters, get_cli_details};

fn main() {
    let cli_args = parse_cli_parameters();
    let cli_task = get_cli_details(&cli_args);
}

