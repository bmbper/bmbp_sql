use crate::wrapper::dql::{QueryTable, RdbcColumn, RdbcColumnValue, RdbcCondition, RdbcTable};
use crate::{JoinTable, RdbcOrder, RdbcValue, UnionTable};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RdbcInsertWrapper {
    table: RdbcTable,
    column_dml: Vec<DmlColumn>,
    columns: Vec<RdbcColumn>,
    column_value: Vec<RdbcColumnValue>,
    column_query: Option<QueryTable>,
    params: HashMap<String, RdbcValue>,
}
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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
    column: RdbcColumn,
    value: RdbcColumnValue,
}
