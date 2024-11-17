use crate::wrapper::dql::{QueryTable, RdbcColumn, RdbcColumnValue, RdbcCondition, RdbcTable};
use crate::RdbcValue;
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
    pub(crate) table: RdbcTable,
    column_dml: Vec<DmlColumn>,
    condition: Option<RdbcCondition>,
    limit_count: Option<u64>,
    offset_count: Option<u64>,
}
#[derive(Debug, Clone)]
pub struct RdbcDeleteWrapper {
    table: RdbcTable,
    condition: Option<RdbcCondition>,
    limit_count: Option<u64>,
    offset_count: Option<u64>,
    params: HashMap<String, RdbcValue>,
}
#[derive(Debug, Clone)]
pub struct DmlColumn {
    column: RdbcColumn,
    value: RdbcColumnValue,
}
