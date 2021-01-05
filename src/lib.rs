use anyhow::Result;
use rusqlite::{Connection, Row, ToSql, NO_PARAMS};
use std::path::{Path, PathBuf};

pub(crate) mod dot;
pub mod infoset;
use infoset::{Column, Database, Reference, Table};

/// A SQLite package.
///
/// Holds a connection and the original path to open it.
#[derive(Debug)]
pub struct Package {
    path: PathBuf,
    conn: Connection,
}

impl Package {
    /// Open a database
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(&path)?;

        conn.pragma_update(None, "journal_mode", &"delete")?;

        Ok(Self {
            path: path.as_ref().into(),
            conn,
        })
    }

    /// Extracts the database structure and builds a [dot] graph from it.
    ///
    /// [dot]: https://www.graphviz.org/doc/info/lang.html
    pub fn to_dot(&mut self) -> Result<String> {
        let databases = self.databases()?;
        let mut pack = String::new();

        for db in databases {
            pack.push_str(&format!("{}", db));
        }

        Ok(pack)
    }

    pub fn query<T, P, F>(&mut self, query: &str, params: P, f: F) -> Result<Vec<T>>
    where
        P: IntoIterator,
        P::Item: ToSql,
        F: FnMut(&Row<'_>) -> std::result::Result<T, rusqlite::Error>,
    {
        let mut stmt = self.conn.prepare(query)?;

        let rows = stmt.query_map(params, f)?;

        let mut items = Vec::new();
        for row in rows {
            items.push(row?);
        }

        Ok(items)
    }

    /// Returns the list of databases in the package.
    pub fn databases(&mut self) -> Result<Vec<Database>> {
        let querystring = r#"
        SELECT
            name,
            file
        FROM
            pragma_database_list
        "#;

        let mut res = Vec::new();
        let names: Vec<(String, String)> =
            self.query(querystring, NO_PARAMS, |row| Ok((row.get(0)?, row.get(1)?)))?;

        for (name, path) in names {
            let tables = self.tables(&name)?;
            let references = self.references(&name)?;
            let database = Database::new(name, path, tables, references);

            res.push(database);
        }

        Ok(res)
    }

    /// Returns the list of tables for the given database name.
    pub fn tables(&mut self, db_name: &str) -> Result<Vec<Table>> {
        let querystring = format!(
            r#"
            SELECT
                name
            FROM
                '{}'.sqlite_master
            WHERE
                type = 'table'
            AND
                name NOT IN ('sqlite_sequence')
            "#,
            db_name
        );

        let mut res = Vec::new();
        let names: Vec<String> = self.query(&querystring, NO_PARAMS, |row| Ok(row.get(0)?))?;

        for name in names {
            let columns = self.columns(&name)?;
            let table = Table::new(name, columns);

            res.push(table);
        }

        Ok(res)
    }

    /// Returns the list of columns for the given table name.
    pub fn columns(&mut self, table_name: &str) -> Result<Vec<Column>> {
        let querystring = r#"
        SELECT
          c.cid,
          c.name,
          c.type,
          c.'notnull',
          c.dflt_value,
          c.pk
        FROM
          sqlite_master AS t
        JOIN
          pragma_table_info(t.name) AS c
        WHERE
            t.name = ?
        ORDER BY
          t.name,
          c.cid
        "#;

        let res = self.query(querystring, &[table_name], |row| {
            Ok(Column {
                id: row.get(0)?,
                name: row.get(1)?,
                datatype: row.get(2)?,
                required: row.get(3)?,
                default_value: row.get(4)?,
                primary_key: row.get(5)?,
            })
        })?;

        Ok(res)
    }

    /// Returns the list of references between tables for the given database.
    pub fn references(&mut self, db_name: &str) -> Result<Vec<Reference>> {
        let querystring = format!(
            r#"
        SELECT DISTINCT
            source.name,
            sink.'table'
        FROM
          '{}'.sqlite_master AS source
        JOIN
          pragma_foreign_key_list(source.name) AS sink
        ORDER BY
          source.name
        "#,
            db_name
        );

        let res = self.query(&querystring, NO_PARAMS, |row| {
            Ok(Reference::new(row.get(0)?, row.get(1)?))
        })?;

        Ok(res)
    }
}
