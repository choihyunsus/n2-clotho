// A bounded read-only query subset for the in-memory definition registry.
use super::{N2Registry, QueryResult};
use regex::Regex;
use std::{cmp::Ordering, sync::LazyLock};

static SELECT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?is)^\s*SELECT\s+(?P<cols>\*|[a-z_]\w*(?:\s*,\s*[a-z_]\w*)*)\s+FROM\s+(?P<table>[a-z_]\w*)(?:\s+WHERE\s+(?P<field>[a-z_]\w*)\s*(?P<op>!=|<>|>=|<=|=|>|<)\s*(?P<value>'(?:[^']|'')*'|[+-]?[0-9]+(?:\.[0-9]+)?))?(?:\s+ORDER\s+BY\s+(?P<order>[a-z_]\w*)(?:\s+(?P<direction>ASC|DESC))?)?(?:\s+LIMIT\s+(?P<limit>[0-9]+))?\s*;?\s*$").expect("static SELECT grammar"));

pub(super) fn execute(registry: &N2Registry, sql: &str) -> Result<QueryResult, String> {
    if sql.len() > 4096 { return Err("Query exceeds 4096 bytes".into()); }
    let parsed = SELECT.captures(sql).ok_or("Unsupported query: use SELECT columns FROM table with an optional single comparison, ORDER BY and LIMIT")?;
    let mut data = match parsed["table"].to_ascii_lowercase().as_str() {
        "rule" | "rules" => registry.query_rules(),
        "contract" | "contracts" => registry.query_contracts(),
        "workflow" | "workflows" => registry.query_workflows(),
        "tool" | "tools" => registry.query_tools(),
        _ => return Err("Unknown table; expected rules, contracts, workflows or tools".into()),
    };
    if let Some(field) = parsed.name("field") {
        let index = column(&data, field.as_str())?;
        let raw = &parsed["value"];
        let numeric = numeric_column(&data.columns[index]);
        let value = if raw.starts_with('\'') { raw[1..raw.len()-1].replace("''", "'") } else { raw.to_string() };
        if numeric && value.parse::<f64>().map_or(true, |v| !v.is_finite()) {
            return Err("Numeric column requires a finite numeric comparison".into());
        }
        data.rows.retain(|row| matches_comparison(compare(&row[index], &value, numeric), &parsed["op"]));
    }
    if let Some(order) = parsed.name("order") {
        let index = column(&data, order.as_str())?;
        let numeric = numeric_column(&data.columns[index]);
        let descending = parsed.name("direction").is_some_and(|m| m.as_str().eq_ignore_ascii_case("desc"));
        data.rows.sort_by(|a,b| {
            let order = compare(&a[index], &b[index], numeric);
            if descending { order.reverse() } else { order }
        });
    }
    if let Some(limit) = parsed.name("limit") {
        data.rows.truncate(limit.as_str().parse::<usize>().map_err(|_| "LIMIT is out of range")?);
    }
    if &parsed["cols"] != "*" {
        let indices: Vec<usize> = parsed["cols"].split(',').map(|name| column(&data,name.trim())).collect::<Result<_,_>>()?;
        data.columns = indices.iter().map(|&i| data.columns[i].clone()).collect();
        data.rows = data.rows.into_iter().map(|row| indices.iter().map(|&i| row[i].clone()).collect()).collect();
    }
    Ok(data)
}

fn column(data: &QueryResult, name: &str) -> Result<usize, String> {
    data.columns.iter().position(|c| c.eq_ignore_ascii_case(name)).ok_or_else(|| format!("Unknown column: {name}"))
}

fn numeric_column(name: &str) -> bool {
    matches!(name, "checks" | "blacklist" | "states" | "transitions" | "steps")
}

fn compare(left: &str, right: &str, numeric: bool) -> Ordering {
    if numeric {
        left.parse::<f64>().unwrap_or_default().total_cmp(&right.parse::<f64>().unwrap_or_default())
    } else { left.cmp(right) }
}

fn matches_comparison(order: Ordering, operator: &str) -> bool {
    match operator {
        "=" => order == Ordering::Equal,
        "!=" | "<>" => order != Ordering::Equal,
        ">" => order == Ordering::Greater,
        "<" => order == Ordering::Less,
        ">=" => order != Ordering::Less,
        "<=" => order != Ordering::Greater,
        _ => false,
    }
}
