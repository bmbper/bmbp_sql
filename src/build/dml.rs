use crate::build::condition::RdbcWhereCondition;
use crate::{
    DmlColumn, JoinTable, RdbcColumn, RdbcColumnValue, RdbcCondition, RdbcDeleteWrapper,
    RdbcInsertWrapper, RdbcQueryWrapper, RdbcTable, RdbcTableIdent, RdbcUpdateWrapper, RdbcValue,
};

impl RdbcInsertWrapper {}

impl RdbcUpdateWrapper {
    pub fn with_table<T>() -> RdbcUpdateWrapper
    where
        T: RdbcTableIdent,
    {
        let mut update_wrapper = RdbcUpdateWrapper::default();
        update_wrapper.from(T::name());
        update_wrapper
    }
    pub fn set<C, V>(&mut self, column: C, value: V) -> &mut Self
    where
        RdbcColumn: From<C>,
        RdbcColumnValue: From<V>,
    {
        self.column_dml.push(DmlColumn::new(column, value));
        self
    }
    fn get_or_create_where_condition(&mut self) -> &mut RdbcCondition {
        self.where_condition.get_or_insert_with(RdbcCondition::new)
    }
}

impl RdbcWhereCondition for RdbcUpdateWrapper {
    fn get_or_create_where_condition(&mut self) -> &mut RdbcCondition {
        self.where_condition.get_or_insert_with(RdbcCondition::new)
    }
    fn get_from_table_mut(&mut self) -> &mut Vec<RdbcTable> {
        &mut self.from_table
    }
}
impl RdbcDeleteWrapper {
    pub fn with_table<T>() -> Self
    where
        T: RdbcTableIdent,
    {
        let mut wrapper = RdbcDeleteWrapper::default();
        wrapper.from(T::name());
        wrapper
    }
}
impl RdbcWhereCondition for RdbcDeleteWrapper {
    fn get_or_create_where_condition(&mut self) -> &mut RdbcCondition {
        self.where_condition.get_or_insert_with(RdbcCondition::new)
    }
    fn get_from_table_mut(&mut self) -> &mut Vec<RdbcTable> {
        &mut self.from_table
    }
}
