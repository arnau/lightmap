//! DOT helpers.

use crate::Column;
use maud::{html, Markup};

pub fn bold(value: &str) -> String {
    let markup = html! {
        b {
            (value)
        }
    };

    markup.into_string()
}

pub fn node_title(value: &str) -> Markup {
    html! {
        table border="0" cellspacing="0.4" {
            tr {
                td align="left" height="24" valign="bottom" {
                    b {
                        (value)
                    }
                }
            }
        }
    }
}

fn node_body(columns: &[Column]) -> Markup {
    html! {
        table border="0" cellspacing="0.4" width="136" {
            @for column in columns {
                (node_field(column))
            }
        }
    }
}

pub fn node_field(field: &Column) -> Markup {
    let pk = if field.primary_key { "*" } else { "" };

    html! {
        tr {
            td align="left" {
                (field.name)(pk) " " b { (field.datatype) }
            }
        }
    }
}

pub fn node(name: &str, columns: &[Column]) -> String {
    let markup = html! {
        (node_title(name)) "|" (node_body(columns))
    };

    markup.into_string()
}
