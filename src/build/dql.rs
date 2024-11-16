use crate::wrapper::{
    RdbcColumn, RdbcColumnIdent, RdbcModel, RdbcQueryWrapper, RdbcTable, RdbcTableIdent,
};
use std::collections::HashMap;

pub trait TableBuilder {}
pub trait ConditionBuilder {}

impl RdbcQueryWrapper {
    pub fn new() -> Self {
        RdbcQueryWrapper {
            select_columns: vec![],
            from_table: vec![],
            join_table: vec![],
            where_condition: None,
            group_columns: vec![],
            having_condition: None,
            order_columns: vec![],
            union_table: vec![],
            limit_count: None,
            offset_count: None,
            params: HashMap::new(),
        }
    }
    pub fn from<T>() -> Self
    where
        T: RdbcModel,
    {
        let mut query = RdbcQueryWrapper::new();
        query.select_vec(T::columns()).table(T::table());
        query
    }
}
impl RdbcQueryWrapper {
    pub fn select<T>(&mut self, columns: T) -> &mut Self
    where
        T: RdbcColumnIdent,
    {
        self.select_columns.push(RdbcColumn::from(columns));
        self
    }

    pub fn select_vec<T>(&mut self, columns: Vec<T>) -> &mut Self
    where
        T: RdbcColumnIdent,
    {
        for col in columns {
            self.select_columns.push(RdbcColumn::from(col));
        }
        self
    }

    pub fn table<T>(&mut self, table: T) -> &mut Self
    where
        T: RdbcTableIdent,
    {
        self.from_table.push(RdbcTable::from(table));
        self
    }
}

impl TableBuilder for RdbcQueryWrapper {}
impl ConditionBuilder for RdbcQueryWrapper {}
