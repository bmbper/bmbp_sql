use crate::{
    RdbcColumn, RdbcColumnIdent, RdbcCondition, RdbcDeleteWrapper, RdbcTable, RdbcTableIdent,
    RdbcValue,
};

pub trait RdbcWhereCondition {
    fn get_or_create_where_condition(&mut self) -> &mut RdbcCondition;
    fn get_from_table_mut(&mut self) -> &mut Vec<RdbcTable>;
    fn from<T>(&mut self, table: T) -> &mut Self
    where
        RdbcTable: From<T>,
    {
        self.get_from_table_mut().push(RdbcTable::from(table));
        self
    }

    fn eq<C, V>(&mut self, column: C, value: V) -> &mut Self
    where
        RdbcColumn: From<C>,
        RdbcValue: From<V>,
    {
        self.get_or_create_where_condition().eq(column, value);
        self
    }
}
