use chrono::prelude::*;
use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, SubCommand};

fn main() {
    fn datetime_validator(s: String) -> Result<(), String> {
        match NaiveDate::parse_from_str(&s, healthtracker::DATE_FORMAT) {
            Ok(_) => Ok(()),
            Err(_) => Err(format!(
                "Must comply with {} format!",
                healthtracker::DATE_FORMAT
            )),
        }
    }

    let date_help_str = format!("Date formatted as \"{}\"", healthtracker::DATE_FORMAT);
    let current_datetime = Local::today()
        .format(&healthtracker::DATE_FORMAT)
        .to_string();

    let date_argument = Arg::with_name("date")
        .short("d")
        .long("date")
        .value_name("DATE")
        .default_value(&current_datetime)
        .validator(datetime_validator)
        .help(&date_help_str);

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(
            SubCommand::with_name("weight")
                .author(crate_authors!())
                .version(crate_version!())
                .about("Track an analyze someones body weight")
                .arg(&date_argument)
                .arg(
                    Arg::with_name("weight")
                        .value_name("WEIGHT")
                        .required(true)
                        .help("The weight to be entered"),
                ),
        )
        .subcommand(
            SubCommand::with_name("analyze")
                .author(crate_authors!())
                .version(crate_version!())
                .about("Analyze all tracked data"),
        )
        .get_matches();

    println!("{:?}", matches);

    if let Some(matches) = matches.subcommand_matches("weight") {
        healthtracker::log_weight(
            matches.value_of("weight").unwrap().parse::<f32>().unwrap(),
            Some(matches.value_of("date").unwrap().to_string()),
        )
        .unwrap();
    }
}
