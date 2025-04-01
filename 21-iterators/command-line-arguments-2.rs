/*
 * Cargo Crates
 */
use std::env;
use std::process;

/*
 * **Video Player Application**
 * Command-Line Arguments:
 *   - Video File Name
 *   - Subtitles (optional)
 *   - High Definition (optional)
 */

/*
 * Structs
 */
#[derive(Debug)]
struct Settings {
    video_file: String,
    subtitles: bool,
    high_definition: bool,
}

fn main() {
    /*
     * Examples of invocation:
     *   - ./../binary
     *   - ./../binary rust.mp4
     *   - ./../binary rust.mp4 true
     *   - ./../binary rust.mp4 false
     *   - ./../binary rust.mp4 true false 
     *   - ./../binary rust.mp4 false true
     *   - ./../binary rust.mp4 true true
     *   - ./../binary rust.mp4 true true
     *   - ./../binary rust.mp4 false false nonsense
     *   - ./../binary rust.mp4 false true nonsense
     *   - ./../binary rust.mp4 true true nonsense
     *   - ./../binary rust.mp4 false false nonsense
     */
    let settings: Settings = collect_settings();
    println!("{:?}", settings);
}

fn collect_settings() -> Settings {
    // The first value of the `.args()` iterator is always the filename.
    // ./../binary rust.mp4 true false nonsense
    //          -> rust.mp4 true false nonsense
    let mut args = env::args().skip(1).take(3);

    let video_file: String = args.next().unwrap_or_else(|| {
        eprintln!("No video file specified!");
        process::exit(1);
    });

    let mut settings = args.map(|setting| {
        setting.parse::<bool>().unwrap_or(false)
    });

    let subtitles: bool = settings.next().unwrap_or(false);
    let high_definition: bool = settings.next().unwrap_or(false);

    Settings {
        video_file,
        subtitles,
        high_definition,
    }
}
