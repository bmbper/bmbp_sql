use crate::wrapper::dql::{QueryTable, RdbcColumn, RdbcColumnValue, RdbcCondition, RdbcTable};
use crate::{JoinTable, RdbcOrder, RdbcTableIdent, RdbcValue, UnionTable};

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RdbcInsertWrapper {
    pub table: RdbcTable,
    pub column_dml: Vec<DmlColumn>,
    pub columns: Vec<RdbcColumn>,
    pub column_value: Vec<RdbcColumnValue>,
    pub column_query: Option<QueryTable>,
    pub params: HashMap<String, RdbcValue>,
}
#[derive(Debug, Clone, Default)]
pub struct RdbcUpdateWrapper {
    pub column_dml: Vec<DmlColumn>,
    pub from_table: Vec<RdbcTable>,
    pub join_table: Vec<JoinTable>,
    pub where_condition: Option<RdbcCondition>,
    pub(crate) group_columns: Vec<RdbcColumn>,
    pub(crate) having_condition: Option<RdbcCondition>,
    pub(crate) order_columns: Vec<RdbcOrder>,
    pub(crate) union_table: Vec<UnionTable>,
    pub(crate) limit_count: Option<u64>,
    pub(crate) offset_count: Option<u64>,
    pub(crate) params: HashMap<String, RdbcValue>,
}

#[derive(Debug, Clone, Default)]
pub struct RdbcDeleteWrapper {
    pub from_table: Vec<RdbcTable>,
    pub join_table: Vec<JoinTable>,
    pub where_condition: Option<RdbcCondition>,
    pub(crate) group_columns: Vec<RdbcColumn>,
    pub(crate) having_condition: Option<RdbcCondition>,
    pub(crate) order_columns: Vec<RdbcOrder>,
    pub(crate) union_table: Vec<UnionTable>,
    pub(crate) limit_count: Option<u64>,
    pub(crate) offset_count: Option<u64>,
    pub(crate) params: HashMap<String, RdbcValue>,
}

#[derive(Debug, Clone)]
pub struct DmlColumn {
    pub column: RdbcColumn,
    pub value: RdbcColumnValue,
}

impl DmlColumn {
    pub fn new<C, V>(column: C, value: V) -> Self
    where
        RdbcColumn: From<C>,
        RdbcColumnValue: From<V>,
    {
        DmlColumn {
            column: RdbcColumn::from(column),
            value: RdbcColumnValue::from(value),
        }
    }
}
