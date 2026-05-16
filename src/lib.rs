use sqlite3_ext::{function::FunctionOptions, *};

#[sqlite3_ext_main]
pub fn init(db: &Connection) -> Result<()> {
    db.create_scalar_function(
        "hello",
        &FunctionOptions::default().set_n_args(0),
        |ctx, _args| ctx.set_result("hello from rust"),
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlite3_ext::{Database, FallibleIteratorMut, FromValue};

    #[test]
    fn test_hello() {
        let db = Database::open(":memory:").unwrap();
        init(&db).unwrap();
        let mut stmt = db.prepare("SELECT hello()").unwrap();
        let rows = stmt.query(()).unwrap();
        let row = rows.next().unwrap().unwrap();
        assert_eq!(row[0].get_str().unwrap(), "hello from rust");
    }
}
