use crate::render::client::util::extract_map_params;
use crate::render::render::RdbcSQLRender;
use crate::{
    CompareColumn, CompareKind, CompareLikeKind, ConditionColumn, ConditionKind, FuncColumn,
    JoinTable, JoinType, QueryColumn, QueryTable, RdbcColumn, RdbcColumnValue, RdbcCondition,
    RdbcDeleteWrapper, RdbcFunc, RdbcInsertWrapper, RdbcOrder, RdbcQueryWrapper, RdbcTable,
    RdbcTableIdent, RdbcUpdateWrapper, RdbcValue, SQLTable, SchemaTable, TableColumn, UnionTable,
    UnionType, ValueColumn,
};
use serde_json;
use std::cmp::PartialEq;
use std::collections::HashMap;

pub struct PgSQLRender {}

impl RdbcSQLRender for PgSQLRender {
    fn render_query(sql_wrapper: &RdbcQueryWrapper) -> (String, Vec<RdbcValue>) {
        let (sql, params_map) = Self::render_query_script(sql_wrapper);
        Self::convert_script_to_sql(sql, params_map)
    }
    fn render_update(sql_wrapper: &RdbcUpdateWrapper) -> (String, Vec<RdbcValue>) {
        let (sql, params_map) = Self::render_update_script(sql_wrapper);
        Self::convert_script_to_sql(sql, params_map)
    }
    fn render_insert(sql_wrapper: &RdbcInsertWrapper) -> (String, Vec<RdbcValue>) {
        let (sql, params_map) = Self::render_insert_script(sql_wrapper);
        Self::convert_script_to_sql(sql, params_map)
    }
    fn render_delete(sql_wrapper: &RdbcDeleteWrapper) -> (String, Vec<RdbcValue>) {
        let (sql, params_map) = Self::render_delete_script(sql_wrapper);
        Self::convert_script_to_sql(sql, params_map)
    }
    fn render_query_with_params(
        sql_wrapper: &RdbcQueryWrapper,
        params: &HashMap<String, RdbcValue>,
    ) -> (String, Vec<RdbcValue>) {
        let (sql, params_map) = Self::render_query_script_with_params(sql_wrapper, params);
        Self::convert_script_to_sql(sql, params_map)
    }
    fn render_update_with_prams(
        sql_wrapper: &RdbcUpdateWrapper,
        params: &HashMap<String, RdbcValue>,
    ) -> (String, Vec<RdbcValue>) {
        let (sql, params_map) = Self::render_update_script_with_params(sql_wrapper, params);
        Self::convert_script_to_sql(sql, params_map)
    }
    fn render_insert_with_params(
        sql_wrapper: &RdbcInsertWrapper,
        params: &HashMap<String, RdbcValue>,
    ) -> (String, Vec<RdbcValue>) {
        let (sql, params_map) = Self::render_insert_script_with_params(sql_wrapper, params);
        Self::convert_script_to_sql(sql, params_map)
    }

    fn render_delete_with_params(
        sql_wrapper: &RdbcDeleteWrapper,
        params: &HashMap<String, RdbcValue>,
    ) -> (String, Vec<RdbcValue>) {
        let (sql, params_map) = Self::render_delete_script_with_params(sql_wrapper, params);
        Self::convert_script_to_sql(sql, params_map)
    }

    fn render_query_script(sql_wrapper: &RdbcQueryWrapper) -> (String, HashMap<String, RdbcValue>) {
        Self::render_query_script_with_params(sql_wrapper, &HashMap::new())
    }

    fn render_update_script(
        sql_wrapper: &RdbcUpdateWrapper,
    ) -> (String, HashMap<String, RdbcValue>) {
        Self::render_update_script_with_params(sql_wrapper, &HashMap::new())
    }

    fn render_insert_script(
        sql_wrapper: &RdbcInsertWrapper,
    ) -> (String, HashMap<String, RdbcValue>) {
        Self::render_insert_script_with_params(sql_wrapper, &HashMap::new())
    }

    fn render_delete_script(
        sql_wrapper: &RdbcDeleteWrapper,
    ) -> (String, HashMap<String, RdbcValue>) {
        Self::render_delete_script_with_params(sql_wrapper, &HashMap::new())
    }
    fn render_query_script_with_params(
        sql_wrapper: &RdbcQueryWrapper,
        params: &HashMap<String, RdbcValue>,
    ) -> (String, HashMap<String, RdbcValue>) {
        let mut query_vec = vec![];
        let mut map_params = extract_map_params(params);

        // select
        let (select, select_params) =
            Self::render_select_columns(sql_wrapper.select_columns.as_slice());
        if !select.is_empty() {
            query_vec.push(format!("SELECT {}", select));
            map_params.extend(select_params);
        }

        let (table_sql, table_params) = Self::render_table_slice(sql_wrapper.from_table.as_slice());
        if !table_sql.is_empty() {
            query_vec.push(format!("FROM {}", table_sql));
            map_params.extend(table_params);
        }

        let (join_sql, join_params) = Self::render_join_table(sql_wrapper.join_table.as_slice());
        if !join_sql.is_empty() {
            query_vec.push(join_sql);
            map_params.extend(join_params);
        }

        let (where_condition, where_params) =
            Self::render_where_condition(sql_wrapper.where_condition.as_ref());
        if !where_condition.is_empty() {
            query_vec.push(format!("WHERE {}", where_condition));
            map_params.extend(where_params);
        }

        let (group, group_params) =
            Self::render_group_columns(sql_wrapper.group_columns.as_slice());
        if !group.is_empty() {
            query_vec.push(format!("GROUP BY {}", group));
            map_params.extend(group_params);
        }

        let (having_condition, having_params) =
            Self::render_having_condition(sql_wrapper.having_condition.as_ref());
        if !having_condition.is_empty() {
            query_vec.push(format!("HAVING {}", having_condition));
            map_params.extend(having_params);
        }

        let (order, order_params) =
            Self::render_order_columns(sql_wrapper.order_columns.as_slice());
        if !order.is_empty() {
            query_vec.push(format!("ORDER BY {}", order));
            map_params.extend(order_params);
        }

        let (union_sql, union_params) =
            Self::render_union_table(sql_wrapper.union_table.as_slice());
        if !union_sql.is_empty() {
            query_vec.push(format!("{}", union_sql));
            map_params.extend(union_params);
        }

        let limit_offset_sql = Self::render_limit(
            sql_wrapper.limit_count.as_ref(),
            sql_wrapper.offset_count.as_ref(),
        );
        if !limit_offset_sql.is_empty() {
            query_vec.push(limit_offset_sql);
        }

        (query_vec.join(" \n "), map_params)
    }

    fn render_update_script_with_params(
        sql_wrapper: &RdbcUpdateWrapper,
        params: &HashMap<String, RdbcValue>,
    ) -> (String, HashMap<String, RdbcValue>) {
        let value_params = extract_map_params(params);
        ("".to_string(), HashMap::new())
    }
    fn render_insert_script_with_params(
        sql_wrapper: &RdbcInsertWrapper,
        params: &HashMap<String, RdbcValue>,
    ) -> (String, HashMap<String, RdbcValue>) {
        let value_params = extract_map_params(params);
        ("".to_string(), HashMap::new())
    }
    fn render_delete_script_with_params(
        sql_wrapper: &RdbcDeleteWrapper,
        params: &HashMap<String, RdbcValue>,
    ) -> (String, HashMap<String, RdbcValue>) {
        let value_params = extract_map_params(params);
        ("".to_string(), HashMap::new())
    }
}

impl PartialEq for CompareKind {
    fn eq(&self, other: &Self) -> bool {
        self.compare() == other.compare()
    }
}

impl PgSQLRender {
    fn convert_script_to_sql(
        mut sql: String,
        params_map: HashMap<String, RdbcValue>,
    ) -> (String, Vec<RdbcValue>) {
        let mut params_vec = vec![];
        for (k, v) in params_map.iter() {
            if sql.contains(k) {
                sql = sql.replace(
                    format!("#{{{}}}", k).as_str(),
                    params_vec.len().to_string().as_str(),
                );
                params_vec.push(v.clone());
            }
        }
        (sql, params_vec)
    }
    fn render_select_columns(
        select_columns: &[RdbcColumn],
    ) -> (String, HashMap<String, RdbcValue>) {
        let mut params_map = HashMap::new();
        let mut select_vec = vec![];
        for column in select_columns {
            let (item_sql, item_params) = Self::render_column_for_select(column);
            select_vec.push(item_sql);
            params_map.extend(item_params);
        }
        (select_vec.join(","), params_map)
    }

    fn render_table_slice(table_slice: &[RdbcTable]) -> (String, HashMap<String, RdbcValue>) {
        let mut table_vec = vec![];
        let mut table_params = HashMap::new();
        for item in table_slice {
            let (item_sql, item_params) = Self::render_table(item);
            table_vec.push(item_sql);
            table_params.extend(item_params);
        }
        ("".to_string(), HashMap::new())
    }
    fn render_table(table: &RdbcTable) -> (String, HashMap<String, RdbcValue>) {
        match table {
            RdbcTable::SchemaTable(c) => Self::render_schema_table(c),
            RdbcTable::SQLTable(c) => Self::render_sql_table(c),
            RdbcTable::QueryTable(c) => Self::render_query_table(c),
        }
    }
    fn render_join_table(table_slice: &[JoinTable]) -> (String, HashMap<String, RdbcValue>) {
        let mut join_vec = vec![];
        let mut join_params = HashMap::new();
        for item in table_slice {
            let mut join_sql = match &item.join_type {
                JoinType::Inner => "INNER JOIN".to_string(),
                JoinType::Left => "LEFT JOIN".to_string(),
                JoinType::Right => "RIGHT JOIN".to_string(),
                JoinType::Full => "FULL JOIN".to_string(),
            };
            let (table_sql, tale_params) = Self::render_table(&item.table);
            join_sql = format!("{} {}", join_sql, table_sql);
            join_params.extend(tale_params);
            let (condition_sql, condition_params) =
                Self::render_where_condition(item.condition.as_ref());
            if !condition_sql.is_empty() {
                join_sql = format!("{} ON {}", join_sql, condition_sql);
                join_params.extend(condition_params);
            }
            join_vec.push(join_sql);
        }
        (join_vec.join("\n"), join_params)
    }
    fn render_where_condition(
        condition_op: Option<&RdbcCondition>,
    ) -> (String, HashMap<String, RdbcValue>) {
        if condition_op.is_none() {
            return ("".to_string(), HashMap::new());
        }
        let condition = condition_op.unwrap();
        let split_tag = match &condition.kind {
            ConditionKind::AND => " AND ".to_string(),
            ConditionKind::OR => " OR ".to_string(),
        };
        let mut condition_vec = vec![];
        let mut condition_params = HashMap::new();
        for item in condition.column.iter() {
            let (column_sql, column_params) = match item {
                ConditionColumn::Compare(c) => Self::render_compare_column(c),
                ConditionColumn::SubCondition(sc) => {
                    let (temp_sql, temp_params) = Self::render_where_condition(Some(sc));
                    (format!("({})", temp_sql), temp_params)
                }
            };
            if !column_sql.is_empty() {
                condition_vec.push(column_sql);
                condition_params.extend(column_params);
            }
        }
        (condition_vec.join(&*split_tag), condition_params)
    }
    fn render_compare_column(column: &CompareColumn) -> (String, HashMap<String, RdbcValue>) {
        let mut params = HashMap::new();
        let (mut column_sql, mut column_params) = Self::render_column_for_compare(&column.column);
        params.extend(column_params);
        if column.kind == CompareKind::IsNotNull || column.kind == CompareKind::IsNull {
            column_sql = format!("{} {}", column_sql, column.kind.compare());
            return (column_sql, params);
        }
        match &column.value {
            RdbcColumnValue::ColumnValue(c) => {
                let (value_sql, value_params) = Self::render_column_for_compare(c);
                column_sql = format!("{} {} {}", column_sql, column.kind.compare(), value_sql);
                params.extend(value_params);
            }
            RdbcColumnValue::StaticValue(v) => match &column.kind {
                CompareKind::Like(like) | CompareKind::NotLike(like) => {
                    let value_id = uuid::Uuid::new_v4().to_string();
                    column_sql =
                        format!("{} {} #{{{}}}", column_sql, column.kind.compare(), value_id);
                    let like_value = match like {
                        CompareLikeKind::Left => format!("%'{}'", v.to_string()),
                        CompareLikeKind::Right => format!("'{}'%", v.to_string()),
                        CompareLikeKind::Both => format!("%'{}'%", v.to_string()),
                    };
                    params.insert(value_id, RdbcValue::from(like_value));
                }
                CompareKind::Between | CompareKind::NotBetween => {
                    if v.is_array() {
                        let array_value = v.as_array().unwrap();
                        if array_value.len() >= 2 {
                            let start_id = uuid::Uuid::new_v4().to_string();
                            let end_id = uuid::Uuid::new_v4().to_string();
                            column_sql = format!(
                                "{} {} #{{{}}} AND #{{{}}}",
                                column_sql,
                                column.kind.compare(),
                                start_id,
                                end_id
                            );
                            params.insert(start_id, array_value[0].clone());
                            params.insert(end_id, array_value[1].clone());
                        } else {
                            column_sql = format!(
                                "{} {} {}",
                                column_sql,
                                column.kind.compare(),
                                "请以Vec的形式，传递两个参数"
                            )
                        }
                    } else {
                        column_sql = format!(
                            "{} {} {}",
                            column_sql,
                            column.kind.compare(),
                            "请以Vec的形式，传递两个参数"
                        )
                    }
                }
                _ => {
                    let value_id = uuid::Uuid::new_v4().to_string();
                    column_sql =
                        format!("{} {} #{{{}}}", column_sql, column.kind.compare(), value_id);
                    params.insert(value_id, v.clone());
                }
            },
            RdbcColumnValue::ScriptValue(v) => {
                column_sql = format!("{} {} {} ", column_sql, column.kind.compare(), v);
            }
            RdbcColumnValue::NullValue => {
                if column.ignore_null {
                    column_sql = format!("{} {} NULL", column_sql, column.kind.compare());
                } else {
                    column_sql = "".to_string();
                }
            }
        };
        (column_sql, params)
    }
    fn render_column_for_select(column: &RdbcColumn) -> (String, HashMap<String, RdbcValue>) {
        match column {
            RdbcColumn::TableColumn(c) => Self::render_table_column_with_alias(c, true),
            RdbcColumn::QueryColumn(c) => Self::render_query_column_with_alias(c, true),
            RdbcColumn::FuncColumn(c) => Self::render_func_column_with_alias(c, true),
            RdbcColumn::ValueColumn(c) => Self::render_value_column_with_alias(c, true),
        }
    }
    fn render_column_for_compare(column: &RdbcColumn) -> (String, HashMap<String, RdbcValue>) {
        match column {
            RdbcColumn::TableColumn(c) => Self::render_table_column_with_alias(c, false),
            RdbcColumn::QueryColumn(c) => Self::render_query_column_with_alias(c, false),
            RdbcColumn::FuncColumn(c) => Self::render_func_column_with_alias(c, false),
            RdbcColumn::ValueColumn(c) => Self::render_value_column_with_alias(c, false),
        }
    }

    fn render_func_column(column: &FuncColumn) -> (String, HashMap<String, RdbcValue>) {
        match &column.func_type {
            RdbcFunc::Count => {}
            RdbcFunc::Sum => {}
            RdbcFunc::Avg => {}
            RdbcFunc::Max => {}
            RdbcFunc::Min => {}
            RdbcFunc::SubStr => {}
            RdbcFunc::Trim => {}
            RdbcFunc::Length => {}
            RdbcFunc::Upper => {}
            RdbcFunc::Lower => {}
            RdbcFunc::Date => {}
            RdbcFunc::Abs => {}
            RdbcFunc::Floor => {}
            RdbcFunc::Concat => {}
            RdbcFunc::DateDiff => {}
            RdbcFunc::DateAdd => {}
            RdbcFunc::DateSub => {}
            RdbcFunc::Add => {}
            RdbcFunc::Sub => {}
            RdbcFunc::Mul => {}
            RdbcFunc::Div => {}
            RdbcFunc::Mod => {}
            RdbcFunc::Pow => {}
            RdbcFunc::Round => {}
        }
        ("".to_string(), HashMap::new())
    }

    fn render_table_column_with_alias(
        column: &TableColumn,
        has_alias: bool,
    ) -> (String, HashMap<String, RdbcValue>) {
        let mut column_sql = column.column_name.to_string();
        if !column.column_alias.is_empty() && has_alias {
            column_sql = format!("{} AS {}", column_sql, column.column_alias);
        }
        if let Some(table) = column.table.as_ref() {
            let table_alias = table.table_alias();
            if !table_alias.is_empty() {
                column_sql = format!("{}.{}", table_alias, column_sql);
            }
        }
        return (column_sql.to_string(), HashMap::new());
    }
    fn render_query_column_with_alias(
        column: &QueryColumn,
        has_alias: bool,
    ) -> (String, HashMap<String, RdbcValue>) {
        let (mut column_sql, mut params_map) = Self::render_query_script(&column.query);
        if !column.column_alias.is_empty() && has_alias {
            column_sql = format!("{} AS {}", column_sql, column.column_alias);
        }
        return (column_sql, params_map);
    }
    fn render_func_column_with_alias(
        column: &FuncColumn,
        has_alias: bool,
    ) -> (String, HashMap<String, RdbcValue>) {
        let (mut column_sql, mut params_map) = Self::render_func_column(column);
        if !column.column_alias.is_empty() && has_alias {
            column_sql = format!("{} AS {}", column_sql, column.column_alias);
        }
        return (column_sql, params_map);
    }
    fn render_value_column_with_alias(
        column: &ValueColumn,
        has_alias: bool,
    ) -> (String, HashMap<String, RdbcValue>) {
        let mut column_sql = Self::render_rdbc_value(&column.value);
        if !column.column_alias.is_empty() && has_alias {
            column_sql = format!("{} AS {}", column_sql, column.column_alias);
        }
        return (column_sql.to_string(), HashMap::new());
    }

    fn render_rdbc_value(value: &RdbcValue) -> String {
        match value {
            RdbcValue::Char(c) => {
                format!("'{}'", c)
            }
            RdbcValue::Varchar(v) => {
                format!("'{}'", v)
            }
            RdbcValue::Text(v) => {
                format!("'{}'", v)
            }
            RdbcValue::LongText(v) => {
                format!("'{}'", v)
            }
            RdbcValue::SmallInt(v) => {
                format!("{}", v)
            }
            RdbcValue::Int(v) => {
                format!("{}", v)
            }
            RdbcValue::BigInt(v) => {
                format!("{}", v)
            }
            RdbcValue::Double(v) => {
                format!("{}", v)
            }
            RdbcValue::BigDouble(v) => {
                format!("{}", v)
            }
            RdbcValue::Date(v) => {
                format!("'{}'", v.format("%Y-%m-%d"))
            }
            RdbcValue::DateTime(v) => {
                format!("'{}'", v.format("%Y-%m-%d %H:%M:%S"))
            }
            RdbcValue::Time(t) => {
                format!("'{}'", t.format("%H:%M:%S"))
            }
            RdbcValue::TimeStamp(v) => {
                format!("{}", v)
            }
            RdbcValue::Bytes(v) => String::from_utf8(v.to_vec()).unwrap_or("".to_string()),
            RdbcValue::Boolean(v) => {
                if v == &true {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            RdbcValue::Array(v) => serde_json::to_string(&v).unwrap_or("".to_string()),
            RdbcValue::Object(v) => serde_json::to_string(&v).unwrap_or("".to_string()),
            RdbcValue::Null => "NULL".to_string(),
        }
    }
    fn render_group_columns(columns: &[RdbcColumn]) -> (String, HashMap<String, RdbcValue>) {
        ("".to_string(), HashMap::new())
    }
    fn render_having_condition(
        condition: Option<&RdbcCondition>,
    ) -> (String, HashMap<String, RdbcValue>) {
        ("".to_string(), HashMap::new())
    }
    fn render_order_columns(columns: &[RdbcOrder]) -> (String, HashMap<String, RdbcValue>) {
        ("".to_string(), HashMap::new())
    }
    fn render_limit(limit: Option<&u64>, offset: Option<&u64>) -> String {
        "".to_string()
    }

    fn render_union_table(table_slice: &[UnionTable]) -> (String, HashMap<String, RdbcValue>) {
        let mut union_vec = vec![];
        let mut union_params = HashMap::new();
        for item in table_slice {
            let union_sql = match &item.union_type {
                UnionType::Union => "UNION".to_string(),
                UnionType::UnionAll => "UNION ALL".to_string(),
            };
            let (query_sql, query_params) = Self::render_query_table(&item.table);
            union_vec.push(format!("{} ({})", union_sql, query_sql));
            union_params.extend(query_params);
        }
        (union_vec.join("\n"), union_params)
    }

    fn render_schema_table(table: &SchemaTable) -> (String, HashMap<String, RdbcValue>) {
        let mut table_sql = table.table_name.clone();
        if !table.table_alias.is_empty() {
            table_sql = format!("{} AS {}", table_sql, table.table_alias);
        }
        if !table.schema.is_empty() {
            table_sql = format!("{}.{}", table.schema, table_sql);
        }
        (table_sql, HashMap::new())
    }

    fn render_sql_table(sql_table: &SQLTable) -> (String, HashMap<String, RdbcValue>) {
        let mut table = format!("({})", sql_table.sql);
        if !sql_table.table_alias.is_empty() {
            table = format!("{} AS {}", table, sql_table.table_alias);
        }
        return (table, HashMap::new());
    }

    fn render_query_table(query_table: &QueryTable) -> (String, HashMap<String, RdbcValue>) {
        let (query_sql, query_params) = Self::render_query_script(&query_table.query);
        let mut table = format!("({})", query_sql);
        if !query_table.table_alias.is_empty() {
            table = format!("{} AS {}", table, query_table.table_alias);
        }
        return (table, query_params);
    }
}
