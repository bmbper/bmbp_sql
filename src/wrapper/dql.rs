use crate::wrapper::{RdbcColumnIdent, RdbcTableIdent};
use crate::RdbcValue;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct RdbcQueryWrapper {
    pub select_columns: Vec<RdbcColumn>,
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

impl RdbcQueryWrapper {
    pub fn build_join(&mut self, builder: fn() -> JoinTable) -> &mut Self {
        self.join_table.push(builder());
        self
    }
    pub fn join(&mut self, table: JoinTable) -> &mut Self {
        self.join_table.push(table);
        self
    }
}

#[derive(Clone, Debug)]
pub enum RdbcTable {
    SchemaTable(SchemaTable),
    SQLTable(SQLTable),
    QueryTable(QueryTable),
}

impl RdbcTable {
    pub(crate) fn new(schema: String, table: String, alias: String) -> RdbcTable {
        RdbcTable::SchemaTable(SchemaTable {
            schema,
            table_name: table,
            table_alias: alias,
        })
    }
}

#[derive(Clone, Debug)]
pub struct SchemaTable {
    pub schema: String,
    pub table_name: String,
    pub table_alias: String,
}
#[derive(Clone, Debug)]
pub struct SQLTable {
    pub sql: String,
    pub table_alias: String,
}
#[derive(Clone, Debug)]
pub struct QueryTable {
    pub query: RdbcQueryWrapper,
    pub table_alias: String,
}
#[derive(Clone, Debug)]
pub struct JoinTable {
    pub table: RdbcTable,
    pub join_type: JoinType,
    pub condition: Option<RdbcCondition>,
}
#[derive(Clone, Debug)]
pub enum JoinType {
    Inner,
    Left,
    Right,
    Full,
}
#[derive(Clone, Debug)]
pub struct UnionTable {
    pub table: QueryTable,
    pub union_type: UnionType,
}
#[derive(Clone, Debug)]
pub enum UnionType {
    Union,
    UnionAll,
}
#[derive(Clone, Debug)]
pub enum RdbcColumn {
    TableColumn(TableColumn),
    QueryColumn(QueryColumn),
    FuncColumn(FuncColumn),
    ValueColumn(ValueColumn),
}

#[derive(Clone, Debug)]
pub struct TableColumn {
    pub table: Option<RdbcTable>,
    pub column_name: String,
    pub column_alias: String,
}
#[derive(Clone, Debug)]
pub struct QueryColumn {
    pub query: RdbcQueryWrapper,
    pub column_alias: String,
}
#[derive(Clone, Debug)]
pub struct FuncColumn {
    pub columns: Vec<RdbcColumn>,
    pub func_type: RdbcFunc,
    pub separator: String,
    pub column_alias: String,
}
#[derive(Clone, Debug)]
pub struct ValueColumn {
    pub value: RdbcValue,
    pub column_alias: String,
}
#[derive(Clone, Debug)]
pub enum RdbcFunc {
    Count,
    Sum,
    Avg,
    Max,
    Min,
    SubStr,
    Trim,
    Length,
    Upper,
    Lower,
    Date,
    Abs,
    Floor,
    Concat,
    DateDiff,
    DateAdd,
    DateSub,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Round,
}
#[derive(Clone, Debug)]
pub struct RdbcCondition {
    pub(crate) kind: ConditionKind,
    pub(crate) column: Vec<ConditionColumn>,
}

impl RdbcCondition {
    pub(crate) fn new() -> RdbcCondition {
        RdbcCondition {
            kind: ConditionKind::AND,
            column: vec![],
        }
    }
    pub(crate) fn eq_v<T, V>(&mut self, column: T, value: V) -> &mut Self
    where
        RdbcColumn: From<T>,
        RdbcValue: From<V>,
    {
        self.column.push(ConditionColumn::Compare(CompareColumn {
            column: RdbcColumn::from(column),
            kind: CompareKind::Equal,
            value: RdbcColumnValue::StaticValue(RdbcValue::from(value)),
            ignore_null: false,
        }));
        self
    }
}

#[derive(Clone, Debug)]
pub enum ConditionColumn {
    Compare(CompareColumn),
    SubCondition(RdbcCondition),
}
#[derive(Clone, Debug)]
pub enum ConditionKind {
    AND,
    OR,
}
#[derive(Clone, Debug)]
pub struct CompareColumn {
    pub column: RdbcColumn,
    pub kind: CompareKind,
    pub value: RdbcColumnValue,
    pub ignore_null: bool,
}

#[derive(Clone, Debug)]
pub enum CompareKind {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Like(CompareLikeKind),
    NotLike(CompareLikeKind),
    Between,
    NotBetween,
    In,
    NotIn,
    IsNull,
    IsNotNull,
    Exists,
    NotExists,
}
#[derive(Clone, Debug)]
pub enum CompareLikeKind {
    Left,
    Right,
    Both,
}
#[derive(Debug, Clone)]
pub enum RdbcColumnValue {
    ColumnValue(RdbcColumn),
    StaticValue(RdbcValue),
    ScriptValue(String),
    NullValue,
}

impl From<RdbcValue> for RdbcColumnValue {
    fn from(value: RdbcValue) -> Self {
        RdbcColumnValue::StaticValue(value)
    }
}

impl From<RdbcColumn> for RdbcColumnValue {
    fn from(value: RdbcColumn) -> Self {
        RdbcColumnValue::ColumnValue(value)
    }
}

impl From<String> for RdbcColumnValue {
    fn from(value: String) -> Self {
        RdbcColumnValue::ScriptValue(value)
    }
}
impl From<&str> for RdbcColumnValue {
    fn from(value: &str) -> Self {
        RdbcColumnValue::ScriptValue(value.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct RdbcOrder {
    pub column: Vec<RdbcColumn>,
    pub order_type: OrderType,
}
#[derive(Debug, Clone)]
pub enum OrderType {
    Asc,
    Desc,
}

impl<T> From<T> for RdbcTable
where
    T: ToString,
{
    fn from(value: T) -> Self {
        RdbcTable::SchemaTable(SchemaTable {
            schema: "".to_string(),
            table_name: value.to_string(),
            table_alias: "".to_string(),
        })
    }
}

impl<T> From<T> for RdbcColumn
where
    T: ToString,
{
    fn from(value: T) -> Self {
        RdbcColumn::TableColumn(TableColumn {
            table: None,
            column_name: value.to_string(),
            column_alias: "".to_string(),
        })
    }
}
