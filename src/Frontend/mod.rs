#[allow(unused)]
#[allow(non_snake_case)]
pub mod Frontend {
    use crate::define::*;
    pub fn create_table(
        relname: &str,
        no_attrs: usize,
        attributes: &Vec<String>,
        type_attrs: &Vec<AttributeType>,
    ) -> Result<(), ErrorType> {
        println!("create_table()");
        Ok(())
    }

    pub fn drop_table(relname: &str) -> Result<(), ErrorType> {
        println!("drop_table()");
        Ok(())
    }

    pub fn open_table(relname: &str) -> Result<(), ErrorType> {
        println!("open_table()");
        Ok(())
    }

    pub fn close_table(relname: &str) -> Result<(), ErrorType> {
        println!("close_table()");
        Ok(())
    }

    pub fn create_index(relname: &str, attrname: &str) -> Result<(), ErrorType> {
        println!("create_index()");
        Ok(())
    }

    pub fn drop_index(relname: &str, attrname: &str) -> Result<(), ErrorType> {
        println!("drop_index()");
        Ok(())
    }

    pub fn alter_table_rename(relname_from: &str, relname_to: &str) -> Result<(), ErrorType> {
        println!("alter_table_rename()");
        Ok(())
    }

    pub fn alter_table_rename_column(
        relname: &str,
        attrname_from: &str,
        attrname_to: &str,
    ) -> Result<(), ErrorType> {
        println!("alter_table_rename_column()");
        Ok(())
    }

    // DML
    pub fn insert_into_table_values(
        relname: &str,
        attr_count: usize,
        attr_values: &Vec<String>,
    ) -> Result<(), ErrorType> {
        println!("insert_into_table_values()");
        Ok(())
    }

    pub fn select_from_table(relname_source: &str, relname_target: &str) -> Result<(), ErrorType> {
        println!("select_from_table()");
        Ok(())
    }

    pub fn select_attrlist_from_table(
        relname_source: &str,
        relname_target: &str,
        attr_count: usize,
        attr_list: &Vec<String>,
    ) -> Result<(), ErrorType> {
        println!("select_attrlist_from_table()");
        Ok(())
    }

    pub fn select_from_table_where(
        relname_source: &str,
        relname_target: &str,
        attribute: &str,
        op: &ConditionalOperators,
        value: &str,
    ) -> Result<(), ErrorType> {
        println!("select_from_table_where()");
        Ok(())
    }

    pub fn select_attrlist_from_table_where(
        relname_source: &str,
        relname_target: &str,
        attr_count: usize,
        attr_list: &Vec<String>,
        attribute: &str,
        op: &ConditionalOperators,
        value: &str,
    ) -> Result<(), ErrorType> {
        println!("select_attrlist_from_table_where()");
        Ok(())
    }

    pub fn select_from_join_where(
        relname_source_one: &str,
        relname_source_two: &str,
        relname_target: &str,
        join_attr_one: &str,
        join_attr_two: &str,
    ) -> Result<(), ErrorType> {
        println!("select_from_join_where()");
        Ok(())
    }

    pub fn select_attrlist_from_join_where(
        relname_source_one: &str,
        relname_source_two: &str,
        relname_target: &str,
        join_attr_one: &str,
        join_attr_two: &str,
        attr_count: usize,
        attr_list: &Vec<String>,
    ) -> Result<(), ErrorType> {
        println!("select_attrlist_from_join_where()");
        Ok(())
    }

    pub fn custom_function(argc: usize, argv: &Vec<String>) -> Result<(), ErrorType> {
        println!("custom_function()");
        Ok(())
    }
}
