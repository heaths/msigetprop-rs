// Copyright 2022 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use std::path::PathBuf;
use std::result::Result;
use std::{error::Error, ops::Index};

use clap::{arg, command, value_parser, Arg};
use msi::{self, Expr, Select, Value};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = command!()
        .arg(
            Arg::new("path")
                .value_parser(value_parser!(PathBuf))
                .required(true),
        )
        .arg(arg!(-p --property <NAME> "The property name to get").default_value("ProductCode"))
        .get_matches();

    let path = matches.get_one::<PathBuf>("path").expect("path required");
    let property = matches
        .get_one::<String>("property")
        .expect("property name required");

    let mut package = msi::open(path)?;
    let columns = vec!["Value"];
    let query = Select::table("Property")
        .columns(&columns)
        .with(Expr::col("Property").eq(Expr::string(property)));
    let rows = package.select_rows(query)?;
    for row in rows {
        if let Value::Str(value) = row.index(0) {
            println!("{}", value);
        }
    }

    Ok(())
}
