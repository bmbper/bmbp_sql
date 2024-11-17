use bmbp_sql::{JoinTable, RdbcQueryWrapper, RdbcTable};

#[test]
pub fn test_query_wrapper() {
    let mut query = RdbcQueryWrapper::new();
    query.build_join(|| -> JoinTable {
        let join_table = JoinTable {
            table: RdbcTable::from("abc"),
            join_type: bmbp_sql::JoinType::Inner,
            condition: None,
        };
        join_table
    });
    query.join({
        let mut c = JoinTable {
            table: RdbcTable::from("ccc"),
            join_type: bmbp_sql::JoinType::Inner,
            condition: None,
        };
        c.table = RdbcTable::from("c");
        c
    });
}
