use crate::wrapper::{
    RdbcColumn, RdbcColumnIdent, RdbcModel, RdbcQueryWrapper, RdbcTable, RdbcTableIdent,
};
use crate::{JoinTable, RdbcCondition};
use std::collections::HashMap;

pub trait TableBuilder {
    fn table_mut(&mut self) -> &mut Vec<RdbcTable>;
    fn join_table_mut(&mut self) -> &mut Vec<JoinTable>;
    fn from<T>(&mut self, table: T) -> &mut Self
    where
        T: RdbcTableIdent,
    {
        self.table_mut().push(RdbcTable::from((
            table.schema(),
            table.table(),
            table.table_alias(),
        )));
        self
    }
}
pub trait ConditionBuilder {
    fn condition_mut(&mut self) -> &mut RdbcCondition;
}

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
    pub fn select<T>(&mut self, column: T) -> &mut Self
    where
        T: RdbcColumnIdent,
    {
        self.select_columns.push(RdbcColumn::from(column));
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

impl TableBuilder for RdbcQueryWrapper {
    fn table_mut(&mut self) -> &mut Vec<RdbcTable> {
        &mut self.from_table
    }
    fn join_table_mut(&mut self) -> &mut Vec<JoinTable> {
        &mut self.join_table
    }
}
impl ConditionBuilder for RdbcQueryWrapper {
    fn condition_mut(&mut self) -> &mut RdbcCondition {
        if self.where_condition.is_none() {
            self.where_condition = Some(RdbcCondition::new());
        }
        self.where_condition.as_mut().unwrap()
    }
}
