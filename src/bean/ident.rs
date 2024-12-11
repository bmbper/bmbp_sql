pub trait RdbcColumnIdent {
    fn name(&self) -> String;
    fn alias() -> String {
        "".to_string()
    }
}

impl<T> RdbcColumnIdent for T
where
    T: ToString,
{
    fn name(&self) -> String {
        self.to_string()
    }
}

pub trait RdbcTableIdent {
    fn name() -> String;
    fn alias() -> String;
    fn columns() -> Vec<impl RdbcColumnIdent>;
    fn primary_key() -> impl RdbcColumnIdent {
        "".to_string()
    }
    fn union_primary_key() -> Vec<impl RdbcColumnIdent> {
        vec!["".to_string()]
    }
}
