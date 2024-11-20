use crate::wrapper::{RdbcColumn, RdbcQueryWrapper, RdbcTable};
use crate::RdbcCondition;
use std::collections::HashMap;

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
}
impl RdbcQueryWrapper {
    pub fn select<T>(&mut self, column: T) -> &mut Self
    where
        RdbcColumn: From<T>,
    {
        self.select_columns.push(RdbcColumn::from(column));
        self
    }

    pub fn select_vec<T>(&mut self, columns: Vec<T>) -> &mut Self
    where
        RdbcColumn: From<T>,
    {
        for col in columns {
            self.select_columns.push(RdbcColumn::from(col));
        }
        self
    }

    pub fn select_slice<T>(&mut self, columns: &[T]) -> &mut Self
    where
        RdbcColumn: for<'a> From<&'a T>,
    {
        for col in columns {
            self.select_columns.push(RdbcColumn::from(col));
        }
        self
    }

    pub fn select_column(&mut self, column: RdbcColumn) -> &mut Self {
        self.select_columns.push(column);
        self
    }

    pub fn from<T>(&mut self, table: T) -> &mut Self
    where
        RdbcTable: From<T>,
    {
        self.from_table.push(RdbcTable::from(table));
        self
    }
}

impl RdbcCondition {}
