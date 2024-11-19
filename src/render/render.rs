use crate::RdbcValue;
use std::iter::Map;

pub trait RdbcSQLRender {
    /// 渲染sql 生成 SQL 语句 和  参数集合
    fn render(&self) -> (String, Vec<RdbcValue>);
    /// 通过参数渲染sql
    fn render_with_params(&self, params: Map<String, RdbcValue>) -> (String, Vec<RdbcValue>);
}
