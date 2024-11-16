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
    pub func: RdbcFunc,
    pub column_alias: String,
}
#[derive(Clone, Debug)]
pub struct ValueColumn {
    pub value: RdbcValue,
    pub column_alias: String,
}
#[derive(Clone, Debug)]
pub enum RdbcFunc {
    Single(SingleColumnFunc),
    Multi(MultiColumnFunc),
}
#[derive(Clone, Debug)]
pub struct SingleColumnFunc {
    pub func_type: SingleFuncType,
    pub column: RdbcColumn,
}
#[derive(Clone, Debug)]
pub enum SingleFuncType {
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
}
#[derive(Clone, Debug)]
pub struct MultiColumnFunc {
    pub func_type: MultiFuncType,
    pub column: Vec<RdbcColumn>,
    pub separator: String,
}
#[derive(Clone, Debug)]
pub enum MultiFuncType {
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
    kind: ConditionKind,
    column: Vec<ConditionColumn>,
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
    RdbcValue(RdbcValue),
    ScriptValue(String),
    NullValue,
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
    T: RdbcTableIdent,
{
    fn from(value: T) -> Self {
        RdbcTable::SchemaTable(SchemaTable {
            schema: value.schema(),
            table_name: value.table(),
            table_alias: value.table_alias(),
        })
    }
}
impl<T> From<T> for RdbcTable
where
    T: RdbcColumnIdent,
{
    fn from(value: T) -> Self {
        RdbcTable::SchemaTable(SchemaTable {
            schema: value.schema(),
            table_name: value.table(),
            table_alias: value.table_alias(),
        })
    }
}

impl<T> From<T> for RdbcColumn
where
    T: RdbcColumnIdent,
{
    fn from(value: T) -> Self {
        RdbcColumn::TableColumn(TableColumn {
            table: Some(RdbcTable::from(value)),
            column_name: "".to_string(),
            column_alias: "".to_string(),
        })
    }
}
