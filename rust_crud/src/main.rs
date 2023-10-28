extern crate rusqlite;
extern crate csv;

use rusqlite::{params,Connection, Result};
use csv::ReaderBuilder;

fn create(database_name: &str) -> Result<Vec<Vec<String>>> {
    let conn = Connection::open(database_name)?;
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path("/workspaces/Individual_Project_2_Simrun/data/auto_data.csv")?;
    let mut stmt = conn.prepare("CREATE TABLE IF NOT EXISTS auto (id INTEGER PRIMARY KEY, make TEXT, model TEXT, year INTEGER, horsepower INTEGER, price INTEGER)")?;
    stmt.execute(params![])?;
    let mut insert_stmt = conn.prepare("INSERT INTO auto (make, model, year, horsepower, price) VALUES (?, ?, ?, ?, ?)")?;
    for result in reader.records() {
        let record = result?;
        insert_stmt.execute(params![record[0], record[1], record[2], record[3], record[4]])?;
    }
    let mut select_stmt = conn.prepare("SELECT * FROM auto")?;
    let results = select_stmt.query_map(params![], |row| {
        Ok(vec![
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            row.get(4)?,
            row.get(5)?,
        ])
    })?;
    let mut result_vec = Vec::new();
    for result in results {
        result_vec.push(result?);
    }
    Ok(result_vec)
}


fn read(database_name: &str) -> Result<(Vec<Vec<String>>, Vec<Vec<String>>, Vec<Vec<String>>)> {
    let conn = Connection::open(database_name)?;
    let mut result1 = Vec::new();
    let mut cursor = conn.prepare("SELECT \"model year\" FROM auto WHERE \"car name\" == \"amc hornet\"")?.query(params![])?;
    while let Some(row) = cursor.next()? {
        result1.push(vec![row.get(0)?]);
    }
    println!("");

    let mut result2 = Vec::new();
    cursor = conn.prepare("SELECT \"car name\", \"cylinders\" FROM auto WHERE \"cylinders\" > 6")?.query(params![])?;
    while let Some(row) = cursor.next()? {
        result2.push(vec![row.get(0)?, row.get(1)?]);
    }

    println!("");

    let mut result3 = Vec::new();
    cursor = conn.prepare("SELECT \"car name\", \"cylinders\" FROM auto WHERE (\"cylinders\" < 8) AND (\"car name\" == \"amc hornet\")")?.query(params![])?;
    while let Some(row) = cursor.next()? {
        result3.push(vec![row.get(0)?, row.get(1)?]);
    }

    conn.commit()?;
    Ok((result1, result2, result3))
}

fn update(database_name: &str) -> Result<Vec<Vec<String>>> {
    let conn = Connection::open(database_name)?;
    conn.execute("UPDATE auto SET \"cylinders\" = '4' WHERE \"cylinders\" = '3'", params![])?;
    let mut cursor = conn.prepare("SELECT * FROM auto")?.query(params![])?;
    let mut results = Vec::new();
    while let Some(row) = cursor.next()? {
        results.push(vec![
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            row.get(4)?,
            row.get(5)?,
        ]);
    }
    conn.commit()?;
    Ok(results)
}

fn delete(database_name: &str) -> Result<Vec<Vec<String>>> {
    let conn = Connection::open(database_name)?;
    conn.execute("DELETE FROM auto WHERE \"cylinders\" = 5", params![])?;
    let mut cursor = conn.prepare("SELECT \"car name\" FROM auto")?.query(params![])?;
    let mut results = Vec::new();
    while let Some(row) = cursor.next()? {
        results.push(vec![row.get(0)?]);
    }
    conn.commit()?;
    Ok(results)
}

fn main() -> Result<()> {
    let database_name = "auto.db";
    create(database_name)?;
    read(database_name)?;
    update(database_name)?;
    delete(database_name)?;
    Ok(())
}