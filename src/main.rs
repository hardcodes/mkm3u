use mkm3u::cli_parser::{get_cli_details, parse_cli_parameters};
use mkm3u::playlist::build_playlist;

fn main() {
    let cli_args = parse_cli_parameters();
    let file_args = get_cli_details(&cli_args);

    // Create a multimedia media playlist.
    let playlist = build_playlist(&file_args);

    // Write the playlist to a file.
    {
        let mut file = std::fs::File::create(file_args.out).unwrap();
        let mut writer = m3u::Writer::new(&mut file);
        for entry in &playlist {
            writer.write_entry(entry).unwrap();
        }
    }
}
