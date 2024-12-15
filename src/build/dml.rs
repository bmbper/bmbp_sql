use crate::{
    DmlColumn, JoinTable, RdbcColumn, RdbcColumnValue, RdbcCondition, RdbcDeleteWrapper,
    RdbcInsertWrapper, RdbcTable, RdbcTableIdent, RdbcUpdateWrapper, RdbcValue,
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
    pub fn from<T>(&mut self, table: T) -> &mut Self
    where
        RdbcTable: From<T>,
    {
        self.from_table.push(RdbcTable::from(table));
        self
    }

    pub fn set<C, V>(&mut self, column: C, value: V) -> &mut Self
    where
        RdbcColumn: From<C>,
        RdbcColumnValue: From<V>,
    {
        self.column_dml.push(DmlColumn::new(column, value));
        self
    }
    pub fn eq<C, V>(&mut self, column: C, value: V) -> &mut Self
    where
        RdbcColumn: From<C>,
        RdbcValue: From<V>,
    {
        self.get_or_create_where_condition().eq(column, value);
        self
    }
    fn get_or_create_where_condition(&mut self) -> &mut RdbcCondition {
        self.where_condition.get_or_insert_with(RdbcCondition::new)
    }
}

impl RdbcDeleteWrapper {}
