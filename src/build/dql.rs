use crate::wrapper::{RdbcColumn, RdbcQueryWrapper, RdbcTable};
use crate::{
    CompareColumn, CompareKind, CompareLikeKind, ConditionColumn, ConditionKind, JoinTable,
    RdbcColumnValue, RdbcCondition, RdbcValue,
};
use std::collections::HashMap;

impl RdbcQueryWrapper {
    pub fn new() -> Self {
        Self {
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

    pub fn with_table<T>(table: T) -> Self
    where
        RdbcTable: From<T>,
    {
        let mut query = Self::new();
        query.from(table);
        query
    }

    /// Initializes the query with a table and a list of columns.
    pub fn with_table_columns<T, C>(table: T, columns: impl IntoIterator<Item = C>) -> Self
    where
        RdbcTable: From<T>,
        RdbcColumn: From<C>,
    {
        let mut query = Self::with_table(table);
        query.select_columns(columns);
        query
    }

    pub fn with_columns<C>(columns: impl IntoIterator<Item = C>) -> Self
    where
        RdbcColumn: From<C>,
    {
        let mut query = Self::new();
        query.select_columns(columns);
        query
    }

    /// Adds a join table using a builder function.
    pub fn build_join<F>(&mut self, builder: F) -> &mut Self
    where
        F: FnOnce() -> JoinTable,
    {
        self.join_table.push(builder());
        self
    }

    /// Adds a join table directly.
    pub fn join(&mut self, table: JoinTable) -> &mut Self {
        self.join_table.push(table);
        self
    }
}

impl RdbcQueryWrapper {
    /// Adds a single column to the select clause.
    pub fn select<T>(&mut self, column: T) -> &mut Self
    where
        RdbcColumn: From<T>,
    {
        self.select_columns.push(RdbcColumn::from(column));
        self
    }

    /// Adds multiple columns to the select clause.
    pub fn select_columns<I, T>(&mut self, columns: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        RdbcColumn: From<T>,
    {
        self.select_columns
            .extend(columns.into_iter().map(RdbcColumn::from));
        self
    }

    /// Adds a column directly to the select clause.
    pub fn select_column(&mut self, column: RdbcColumn) -> &mut Self {
        self.select_columns.push(column);
        self
    }

    /// Sets the source table for the query.
    pub fn from<T>(&mut self, table: T) -> &mut Self
    where
        RdbcTable: From<T>,
    {
        self.from_table.push(RdbcTable::from(table));
        self
    }

    /// Adds a LIKE condition to the query.
    pub fn like<C, V>(&mut self, column: C, value: V) -> &mut Self
    where
        RdbcColumn: From<C>,
        RdbcValue: From<V>,
    {
        self.get_or_create_where_condition().like(column, value);
        self
    }
    pub fn like_if<C, V>(&mut self, column: C, value: V, condition: bool) -> &mut Self
    where
        RdbcColumn: From<C>,
        RdbcValue: From<V>,
    {
        if condition {
            self.like(column, value);
        }
        self
    }
    pub fn like_op<C, V>(&mut self, column: C, value: Option<V>) -> &mut Self
    where
        RdbcColumn: From<C>,
        RdbcValue: From<V>,
    {
        if value.is_some() {
            self.like(column, value.unwrap());
        }
        self
    }
    /// Internal helper to get or initialize the `where_condition`.
    fn get_or_create_where_condition(&mut self) -> &mut RdbcCondition {
        self.where_condition.get_or_insert_with(RdbcCondition::new)
    }
}
impl RdbcCondition {
    pub fn new() -> Self {
        Self {
            kind: ConditionKind::AND,
            column: vec![],
        }
    }
    pub fn eq<T, V>(&mut self, column: T, value: V) -> &mut Self
    where
        RdbcColumn: From<T>,
        RdbcValue: From<V>,
    {
        self.add_condition(CompareKind::Equal, column, value)
    }
    pub fn ne<T, V>(&mut self, column: T, value: V) -> &mut Self
    where
        RdbcColumn: From<T>,
        RdbcValue: From<V>,
    {
        self.add_condition(CompareKind::NotEqual, column, value)
    }
    pub fn like<T, V>(&mut self, column: T, value: V) -> &mut Self
    where
        RdbcColumn: From<T>,
        RdbcValue: From<V>,
    {
        self.add_condition(CompareKind::Like(CompareLikeKind::Both), column, value)
    }
    pub fn not_like<T, V>(&mut self, column: T, value: V) -> &mut Self
    where
        RdbcColumn: From<T>,
        RdbcValue: From<V>,
    {
        self.add_condition(CompareKind::NotLike(CompareLikeKind::Both), column, value)
    }
    pub fn eq_column<T, V>(&mut self, column: T, value: V) -> &mut Self
    where
        RdbcColumn: From<T>,
        RdbcColumn: From<V>,
    {
        self.add_column_condition(CompareKind::Equal, column, value)
    }

    fn add_condition<T, V>(&mut self, kind: CompareKind, column: T, value: V) -> &mut Self
    where
        RdbcColumn: From<T>,
        RdbcValue: From<V>,
    {
        self.column.push(ConditionColumn::Compare(CompareColumn {
            column: RdbcColumn::from(column),
            kind,
            value: RdbcColumnValue::StaticValue(RdbcValue::from(value)),
            ignore_null: false,
        }));
        self
    }
    fn add_column_condition<T, V>(&mut self, kind: CompareKind, column: T, value: V) -> &mut Self
    where
        RdbcColumn: From<T>,
        RdbcColumn: From<V>,
    {
        self.column.push(ConditionColumn::Compare(CompareColumn {
            column: RdbcColumn::from(column),
            kind,
            value: RdbcColumnValue::ColumnValue(RdbcColumn::from(value)),
            ignore_null: false,
        }));
        self
    }
    fn add_script_condition<T, V>(&mut self, kind: CompareKind, column: T, value: V) -> &mut Self
    where
        RdbcColumn: From<T>,
        V: ToString,
    {
        self.column.push(ConditionColumn::Compare(CompareColumn {
            column: RdbcColumn::from(column),
            kind,
            value: RdbcColumnValue::StaticValue(RdbcValue::from(format!(
                "#{{{}}}",
                value.to_string()
            ))),
            ignore_null: false,
        }));
        self
    }

    fn add_null_condition<T, V>(&mut self, kind: CompareKind, column: T) -> &mut Self
    where
        RdbcColumn: From<T>,
    {
        self.column.push(ConditionColumn::Compare(CompareColumn {
            column: RdbcColumn::from(column),
            kind,
            value: RdbcColumnValue::NullValue,
            ignore_null: false,
        }));
        self
    }
}
