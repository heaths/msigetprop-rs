// Copyright 2022 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use std::ops::Index;
use std::path::Path;
use std::{fmt::Debug, io::Result};

use msi::{self, Expr, Select, Value};
use tracing::{instrument, trace_span, warn};

#[instrument]
pub fn get_property<P>(path: P, property: &str) -> Result<Option<String>>
where
    P: AsRef<Path> + Debug,
{
    let span = trace_span!("opening package");
    let mut package = span.in_scope(|| msi::open(path))?;

    let columns = vec!["Value"];
    let query = Select::table("Property")
        .columns(&columns)
        .with(Expr::col("Property").eq(Expr::string(property)));

    let span = trace_span!("querying property");
    let _guard = span.enter();

    let rows = package.select_rows(query)?;
    for row in rows {
        if let Value::Str(value) = row.index(0) {
            let value = value.to_owned();
            return Ok(Some(value));
        }
    }

    warn!(parent: &span, "property not defined: {}", property);
    Ok(None)
}
