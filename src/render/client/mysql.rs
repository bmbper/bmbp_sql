use crate::render::render::RdbcSQLRender;
use crate::{RdbcDeleteWrapper, RdbcInsertWrapper, RdbcQueryWrapper, RdbcUpdateWrapper};
use bmbp_rdbc_type::RdbcValue;
use std::collections::HashMap;

pub struct MysqlSQLRender {}
impl RdbcSQLRender for MysqlSQLRender {
    fn render_query(query: &RdbcQueryWrapper) -> (String, Vec<RdbcValue>) {
        ("".to_string(), vec![])
    }

    fn render_update(query: &RdbcUpdateWrapper) -> (String, Vec<RdbcValue>) {
        ("".to_string(), vec![])
    }

    fn render_insert(query: &RdbcInsertWrapper) -> (String, Vec<RdbcValue>) {
        ("".to_string(), vec![])
    }

    fn render_delete(query: &RdbcDeleteWrapper) -> (String, Vec<RdbcValue>) {
        ("".to_string(), vec![])
    }

    fn render_query_with_params(
        sql_wrapper: &RdbcQueryWrapper,
        params: &HashMap<String, RdbcValue>,
    ) -> (String, Vec<RdbcValue>) {
        ("".to_string(), vec![])
    }

    fn render_update_with_prams(
        sql_wrapper: &RdbcUpdateWrapper,
        params: &HashMap<String, RdbcValue>,
    ) -> (String, Vec<RdbcValue>) {
        ("".to_string(), vec![])
    }

    fn render_insert_with_params(
        sql_wrapper: &RdbcInsertWrapper,
        params: &HashMap<String, RdbcValue>,
    ) -> (String, Vec<RdbcValue>) {
        ("".to_string(), vec![])
    }

    fn render_delete_with_params(
        sql_wrapper: &RdbcDeleteWrapper,
        params: &HashMap<String, RdbcValue>,
    ) -> (String, Vec<RdbcValue>) {
        ("".to_string(), vec![])
    }

    fn render_query_script(sql_wrapper: &RdbcQueryWrapper) -> (String, HashMap<String, RdbcValue>) {
        ("".to_string(), HashMap::new())
    }

    fn render_update_script(
        sql_wrapper: &RdbcUpdateWrapper,
    ) -> (String, HashMap<String, RdbcValue>) {
        ("".to_string(), HashMap::new())
    }

    fn render_insert_script(
        sql_wrapper: &RdbcInsertWrapper,
    ) -> (String, HashMap<String, RdbcValue>) {
        ("".to_string(), HashMap::new())
    }

    fn render_delete_script(
        sql_wrapper: &RdbcDeleteWrapper,
    ) -> (String, HashMap<String, RdbcValue>) {
        ("".to_string(), HashMap::new())
    }

    fn render_query_script_with_params(
        sql_wrapper: &RdbcQueryWrapper,
        params: &HashMap<String, RdbcValue>,
    ) -> (String, HashMap<String, RdbcValue>) {
        ("".to_string(), HashMap::new())
    }

    fn render_update_script_with_params(
        sql_wrapper: &RdbcUpdateWrapper,
        params: &HashMap<String, RdbcValue>,
    ) -> (String, HashMap<String, RdbcValue>) {
        ("".to_string(), HashMap::new())
    }

    fn render_insert_script_with_params(
        sql_wrapper: &RdbcInsertWrapper,
        params: &HashMap<String, RdbcValue>,
    ) -> (String, HashMap<String, RdbcValue>) {
        ("".to_string(), HashMap::new())
    }

    fn render_delete_script_with_params(
        sql_wrapper: &RdbcDeleteWrapper,
        params: &HashMap<String, RdbcValue>,
    ) -> (String, HashMap<String, RdbcValue>) {
        ("".to_string(), HashMap::new())
    }
}
