use dateparser;
use std::collections::HashMap;
use chrono::prelude::*;
use anyhow::Result;
pub fn parser_main() {
    let date = "23 sep 2023";
    let p = parser(date);
    if let Err(e) = &p {
        println!("{e}");
    }
    println!("{p:?}");

}
fn parser(date: &str) -> Result<HashMap<&str, u32>>{
    let date = date;
    let parse_local = dateparser::datetime::Parse::new(&Local, None);
    let r = parse_local.parse(date)?;
    let mut map = HashMap::new();
    let d = r.day();
    let m = r.month();
    let y = r.year() as u32;
    map.insert("d", d);
    map.insert("m", m);
    map.insert("y", y);
    Ok(map)
}