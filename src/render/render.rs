use crate::{RdbcDeleteWrapper, RdbcInsertWrapper, RdbcQueryWrapper, RdbcUpdateWrapper, RdbcValue};
use std::collections::HashMap;

pub trait RdbcSQLRender {
    fn render_query(sql_wrapper: &RdbcQueryWrapper) -> (String, Vec<RdbcValue>);
    fn render_update(sql_wrapper: &RdbcUpdateWrapper) -> (String, Vec<RdbcValue>);
    fn render_insert(sql_wrapper: &RdbcInsertWrapper) -> (String, Vec<RdbcValue>);
    fn render_delete(sql_wrapper: &RdbcDeleteWrapper) -> (String, Vec<RdbcValue>);
    fn render_query_with_params(
        sql_wrapper: &RdbcQueryWrapper,
        params: &HashMap<String, RdbcValue>,
    ) -> (String, Vec<RdbcValue>);
    fn render_update_with_prams(
        sql_wrapper: &RdbcUpdateWrapper,
        params: &HashMap<String, RdbcValue>,
    ) -> (String, Vec<RdbcValue>);
    fn render_insert_with_params(
        sql_wrapper: &RdbcInsertWrapper,
        params: &HashMap<String, RdbcValue>,
    ) -> (String, Vec<RdbcValue>);
    fn render_delete_with_params(
        sql_wrapper: &RdbcDeleteWrapper,
        params: &HashMap<String, RdbcValue>,
    ) -> (String, Vec<RdbcValue>);

    fn render_query_script(sql_wrapper: &RdbcQueryWrapper) -> (String, HashMap<String, RdbcValue>);
    fn render_update_script(
        sql_wrapper: &RdbcUpdateWrapper,
    ) -> (String, HashMap<String, RdbcValue>);
    fn render_insert_script(
        sql_wrapper: &RdbcInsertWrapper,
    ) -> (String, HashMap<String, RdbcValue>);
    fn render_delete_script(
        sql_wrapper: &RdbcDeleteWrapper,
    ) -> (String, HashMap<String, RdbcValue>);

    fn render_query_script_with_params(
        sql_wrapper: &RdbcQueryWrapper,
        params: &HashMap<String, RdbcValue>,
    ) -> (String, HashMap<String, RdbcValue>);
    fn render_update_script_with_params(
        sql_wrapper: &RdbcUpdateWrapper,
        params: &HashMap<String, RdbcValue>,
    ) -> (String, HashMap<String, RdbcValue>);
    fn render_insert_script_with_params(
        sql_wrapper: &RdbcInsertWrapper,
        params: &HashMap<String, RdbcValue>,
    ) -> (String, HashMap<String, RdbcValue>);
    fn render_delete_script_with_params(
        sql_wrapper: &RdbcDeleteWrapper,
        params: &HashMap<String, RdbcValue>,
    ) -> (String, HashMap<String, RdbcValue>);
}
