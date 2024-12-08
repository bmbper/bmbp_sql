use bmbp_rdbc_type::{RdbcIdent, RdbcValue};
use std::collections::HashMap;

/// Query wrapper to construct complex SQL queries.
#[derive(Clone, Debug)]
pub struct RdbcQueryWrapper {
    pub select_columns: Vec<RdbcColumn>,
    pub from_table: Vec<RdbcTable>,
    pub join_table: Vec<JoinTable>,
    pub where_condition: Option<RdbcCondition>,
    pub group_columns: Vec<RdbcColumn>,
    pub having_condition: Option<RdbcCondition>,
    pub order_columns: Vec<RdbcOrder>,
    pub union_table: Vec<UnionTable>,
    pub limit_count: Option<u64>,
    pub offset_count: Option<u64>,
    pub params: HashMap<String, RdbcValue>,
}

/// Representation of a database table in a query.
#[derive(Clone, Debug)]
pub enum RdbcTable {
    SchemaTable(SchemaTable),
    SQLTable(SQLTable),
    QueryTable(QueryTable),
}

impl RdbcTable {
    /// Constructs a new schema table.
    pub fn new(
        schema: impl Into<String>,
        table: impl Into<String>,
        alias: impl Into<String>,
    ) -> Self {
        RdbcTable::SchemaTable(SchemaTable {
            schema: schema.into(),
            table_name: table.into(),
            table_alias: alias.into(),
        })
    }

    pub fn table_alias(&self) -> String {
        match self {
            RdbcTable::SchemaTable(schema_table) => schema_table.table_alias.clone(),
            RdbcTable::SQLTable(sql_table) => sql_table.table_alias.clone(),
            RdbcTable::QueryTable(query_table) => query_table.table_alias.clone(),
        }
    }
}

/// Schema table structure.
#[derive(Clone, Debug)]
pub struct SchemaTable {
    pub schema: String,
    pub table_name: String,
    pub table_alias: String,
}

/// SQL table structure for raw SQL queries.
#[derive(Clone, Debug)]
pub struct SQLTable {
    pub sql: String,
    pub table_alias: String,
}

/// Query table structure for subqueries.
#[derive(Clone, Debug)]
pub struct QueryTable {
    pub query: RdbcQueryWrapper,
    pub table_alias: String,
}

/// Join table structure for join operations.
#[derive(Clone, Debug)]
pub struct JoinTable {
    pub table: RdbcTable,
    pub join_type: JoinType,
    pub condition: Option<RdbcCondition>,
}

/// Types of SQL joins.
#[derive(Clone, Debug)]
pub enum JoinType {
    Inner,
    Left,
    Right,
    Full,
}

/// Union table structure for union queries.
#[derive(Clone, Debug)]
pub struct UnionTable {
    pub table: QueryTable,
    pub union_type: UnionType,
}

/// Types of SQL unions.
#[derive(Clone, Debug)]
pub enum UnionType {
    Union,
    UnionAll,
}

/// Column representation in a query.
#[derive(Clone, Debug)]
pub enum RdbcColumn {
    TableColumn(TableColumn),
    QueryColumn(QueryColumn),
    FuncColumn(FuncColumn),
    ValueColumn(ValueColumn),
}

/// Table column representation.
#[derive(Clone, Debug)]
pub struct TableColumn {
    pub table: Option<RdbcTable>,
    pub column_name: String,
    pub column_alias: String,
}

/// Subquery column representation.
#[derive(Clone, Debug)]
pub struct QueryColumn {
    pub query: RdbcQueryWrapper,
    pub column_alias: String,
}

/// Function column representation.
#[derive(Clone, Debug)]
pub struct FuncColumn {
    pub columns: Vec<RdbcColumn>,
    pub func_type: RdbcFunc,
    pub separator: String,
    pub column_alias: String,
}

/// Value column representation.
#[derive(Clone, Debug)]
pub struct ValueColumn {
    pub value: RdbcValue,
    pub column_alias: String,
}

/// Supported SQL functions.
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

/// SQL query conditions.
#[derive(Clone, Debug)]
pub struct RdbcCondition {
    pub kind: ConditionKind,
    pub column: Vec<ConditionColumn>,
}

/// Types of conditions.
#[derive(Clone, Debug)]
pub enum ConditionKind {
    AND,
    OR,
}

/// Representation of a condition column.
#[derive(Clone, Debug)]
pub enum ConditionColumn {
    Compare(CompareColumn),
    SubCondition(RdbcCondition),
}

/// Comparison column structure.
#[derive(Clone, Debug)]
pub struct CompareColumn {
    pub column: RdbcColumn,
    pub kind: CompareKind,
    pub value: RdbcColumnValue,
    pub ignore_null: bool,
}

/// Types of SQL comparisons.
#[derive(Clone, Debug)]
pub enum CompareKind {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterEqualThan,
    LessThan,
    LessEqualThan,
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

/// Types of LIKE conditions.
#[derive(Clone, Debug)]
pub enum CompareLikeKind {
    Left,
    Right,
    Both,
}

impl CompareKind {
    pub fn compare(&self) -> String {
        match self {
            CompareKind::Equal => "=".to_string(),
            CompareKind::NotEqual => "!=".to_string(),
            CompareKind::GreaterThan => ">".to_string(),
            CompareKind::GreaterEqualThan => ">=".to_string(),
            CompareKind::LessThan => "<".to_string(),
            CompareKind::LessEqualThan => "<=".to_string(),
            CompareKind::Like(_) => "LIKE".to_string(),
            CompareKind::NotLike(_) => "NOT LIKE".to_string(),
            CompareKind::Between => "BETWEEN".to_string(),
            CompareKind::NotBetween => "NOT BETWEEN".to_string(),
            CompareKind::In => "IN".to_string(),
            CompareKind::NotIn => "NOT IN".to_string(),
            CompareKind::IsNull => "IS NULL".to_string(),
            CompareKind::IsNotNull => "IS NOT NULL".to_string(),
            CompareKind::Exists => "EXISTS".to_string(),
            CompareKind::NotExists => "NOT EXISTS".to_string(),
        }
    }
}

/// Possible column values in conditions.
#[derive(Debug, Clone)]
pub enum RdbcColumnValue {
    ColumnValue(RdbcColumn),
    StaticValue(RdbcValue),
    ScriptValue(String),
    NullValue,
}

/// Order by clause structure.
#[derive(Debug, Clone)]
pub struct RdbcOrder {
    pub column: Vec<RdbcColumn>,
    pub order_type: OrderType,
}

/// Types of order directions.
#[derive(Debug, Clone)]
pub enum OrderType {
    Asc,
    Desc,
}

impl<T> From<T> for RdbcTable
where
    T: RdbcIdent,
{
    fn from(value: T) -> Self {
        RdbcTable::SchemaTable(SchemaTable {
            schema: "".to_string(),
            table_name: value.get_ident(),
            table_alias: "".to_string(),
        })
    }
}

impl<T> From<T> for RdbcColumn
where
    T: RdbcIdent,
{
    fn from(value: T) -> Self {
        RdbcColumn::TableColumn(TableColumn {
            table: None,
            column_name: value.get_ident(),
            column_alias: "".to_string(),
        })
    }
}
