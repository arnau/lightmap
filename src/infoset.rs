//! A representation of a relational database infoset.

use crate::dot;
use std::fmt;

/// A database infoset.
#[derive(Debug)]
pub struct Database {
    path: String,
    name: String,
    tables: Vec<Table>,
    references: Vec<Reference>,
}

impl Database {
    pub fn new(name: String, path: String, tables: Vec<Table>, references: Vec<Reference>) -> Self {
        Self {
            path,
            name,
            tables,
            references,
        }
    }
}

impl fmt::Display for Database {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "digraph {} {{", &self.name)?;
        writeln!(f, r#"  rankdir="{}";"#, "LR")?;
        writeln!(f, r#"  ranksep="{}";"#, "0.8")?;
        writeln!(f, r#"  nodesep="{}";"#, "0.6")?;
        writeln!(f, r#"  overlap="{}";"#, "false")?;
        writeln!(f, r#"  sep="{}";"#, "+16")?;
        writeln!(f, r#"  splines="{}";"#, "compound")?;
        writeln!(f, r#"  concentrate="{}";"#, "true")?;
        writeln!(f, r#"  pad="{}";"#, "0.4,0.4")?;
        writeln!(f, r#"  fontname="{}";"#, "Helvetica")?;
        writeln!(f, r#"  fontsize="{}";"#, "12")?;
        writeln!(f, r#"  label=<{}>;"#, dot::bold(&self.path))?;
        writeln!(
            f,
            r#"node[shape="Mrecord", fontsize="12", fontname="Helvetica", margin="0.07,0.04", penwidth="1.0"];"#
        )?;
        writeln!(
            f,
            r#"edge[arrowsize="0.8", fontsize="10", style="solid", penwidth="0.9", fontname="Helvetica", labelangle="33", labeldistance="2.0"];"#
        )?;

        for table in &self.tables {
            write!(f, "{}", table)?;
        }

        for reference in &self.references {
            write!(f, "{}", reference)?;
        }

        write!(f, "}}")
    }
}

/// A reference between two tables.
#[derive(Debug)]
pub struct Reference {
    source: String,
    sink: String,
}

impl Reference {
    pub fn new(source: String, sink: String) -> Self {
        Self { source, sink }
    }
}

impl fmt::Display for Reference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}->{}[]", &self.source, &self.sink)
    }
}

#[derive(Debug)]
pub struct Table {
    name: String,
    columns: Vec<Column>,
}

impl Table {
    pub fn new(name: String, columns: Vec<Column>) -> Self {
        Self { name, columns }
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, r#"  {} ["#, &self.name)?;
        writeln!(f, r#"    label=<{}>"#, dot::node(&self.name, &self.columns))?;
        writeln!(f, r#"  ];"#)
    }
}

#[derive(Debug)]
pub struct Column {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) datatype: String,
    pub(crate) required: bool,
    pub(crate) default_value: Option<String>,
    pub(crate) primary_key: bool,
}
