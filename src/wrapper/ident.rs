use std::fmt::Debug;

pub trait RdbcModel {
    fn schema() -> String {
        "".to_string()
    }
    fn table() -> String {
        "".to_string()
    }
    fn table_alias() -> String {
        "".to_string()
    }
    fn columns() -> Vec<String> {
        vec![]
    }
}
pub trait RdbcTableIdent {
    fn schema(&self) -> String {
        "".to_string()
    }
    fn table(&self) -> String {
        "".to_string()
    }
    fn table_alias(&self) -> String {
        "".to_string()
    }
}
pub trait RdbcColumnIdent {
    fn schema(&self) -> String {
        "".to_string()
    }
    fn table(&self) -> String {
        "".to_string()
    }
    fn table_alias(&self) -> String {
        "".to_string()
    }
    fn column(&self) -> String {
        "".to_string()
    }
    fn column_alias(&self) -> String {
        "".to_string()
    }
}

impl<T> RdbcTableIdent for T
where
    T: ToString,
{
    fn table(&self) -> String {
        self.to_string()
    }
}

impl<T> RdbcColumnIdent for T
where
    T: ToString,
{
    fn column(&self) -> String {
        self.to_string()
    }
}
