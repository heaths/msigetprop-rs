// Copyright 2022 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use std::error::Error;
use std::io;
use std::path::PathBuf;
use std::result::Result;

use clap::{arg, command, value_parser, Arg, ArgAction};
use msigetprop::get_property;
use tracing::{self, Level};
use tracing_subscriber::{self, filter::LevelFilter, fmt::format::FmtSpan};

fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let matches = command!()
        .arg(
            Arg::new("path")
                .help("Path to a Windows Installer package (MSI)")
                .value_parser(value_parser!(PathBuf))
                .required(true),
        )
        .arg(arg!(-p --property <NAME> "The property name to get").default_value("ProductCode"))
        .arg(arg!(-v --verbose ... "Log information to stderr").action(ArgAction::Count))
        .get_matches();

    let path = matches.get_one::<PathBuf>("path").expect("path required");
    let property = matches
        .get_one::<String>("property")
        .expect("property name required");
    let verbose = matches.get_count("verbose");

    let mut collector = tracing_subscriber::fmt()
        .with_writer(io::stderr)
        .with_span_events(FmtSpan::ACTIVE);
    collector = match verbose {
        0 => collector.with_max_level(LevelFilter::OFF),
        1 => collector.with_max_level(Level::INFO),
        _ => collector.with_max_level(Level::TRACE),
    };
    collector.try_init()?;

    if let Some(value) = get_property(path, property)? {
        println!("{}", value);
    }

    Ok(())
}
