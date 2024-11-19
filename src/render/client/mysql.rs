use crate::render::render::RdbcSQLRender;
use crate::RdbcValue;
use std::iter::Map;

pub struct MysqlSQLRender {}

impl RdbcSQLRender for MysqlSQLRender {
    fn render(&self) -> (String, Vec<RdbcValue>) {
        ("".to_string(), vec![])
    }

    fn render_with_params(&self, params: Map<String, RdbcValue>) -> (String, Vec<RdbcValue>) {
        ("".to_string(), vec![])
    }
}
