
use clap::{arg, command, ArgMatches};
const ALLOWED_LANGUAGES: [&str; 2] = ["es", "en"];
const DEFAULT_LANGUAGE: &str = "en";
pub fn get_arg_matches() -> ArgMatches {
    let matches = command!()
        .arg(arg!(--data <PATH> "data folder").required(false).short('d'))
        .arg(
            arg!(--lang <LANGUAGE> "language")
                .required(false)
                .short('l'),
        )
        .get_matches();
    matches
}
pub fn get_language(matches: &ArgMatches) -> String {
    let mut language = String::from(matches.value_of("lang").unwrap_or(DEFAULT_LANGUAGE));
    if !is_valid_language(&language) {
        println!("{}", create_unknown_lang_error_text(&language));
        language = DEFAULT_LANGUAGE.to_string();
    };
    language
}
fn is_valid_language(language: &str) -> bool {
    ALLOWED_LANGUAGES.contains(&language)
}

fn create_unknown_lang_error_text(language: &str) -> String {
    format!(
        "Error: Unknown language: {}. Available languages: {}. Default to english",
        language,
        ALLOWED_LANGUAGES.join(", ")
    )
}
