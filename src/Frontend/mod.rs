pub mod Frontend {
    use crate::define::*;
    use std::os::raw::c_char;
    pub fn create_table(
        relname: &String,
        no_attrs: usize,
        attributes: &Vec<String>,
        type_attrs: &Vec<AttributeType>,
    ) -> Result<(), ErrorType> {
        Ok(())
    }

    pub fn drop_table(relname: &String) -> Result<(), ErrorType> {
        Ok(())
    }

    pub fn open_table(relname: &String) -> Result<(), ErrorType> {
        Ok(())
    }

    pub fn close_table(relname: &String) -> Result<(), ErrorType> {
        Ok(())
    }

    pub fn create_index(relname: &String, attrname: &String) -> Result<(), ErrorType> {
        Ok(())
    }

    pub fn drop_index(relname: &String, attrname: &String) -> Result<(), ErrorType> {
        Ok(())
    }

    pub fn alter_table_rename(relname_from: &String, relname_to: &String) -> Result<(), ErrorType> {
        Ok(())
    }

    pub fn alter_table_rename_column(
        relname: &String,
        attrname_from: &String,
        attrname_to: &String,
    ) -> Result<(), ErrorType> {
        Ok(())
    }

    // DML
    pub fn insert_into_table_values(
        relname: &String,
        attr_count: i32,
        attr_values: *mut [c_char; ATTR_SIZE],
    ) -> Result<(), ErrorType> {
        Ok(())
    }

    pub fn select_from_table(
        relname_source: &String,
        relname_target: &String,
    ) -> Result<(), ErrorType> {
        Ok(())
    }

    pub fn select_attrlist_from_table(
        relname_source: &String,
        relname_target: &String,
        attr_count: i32,
        attr_list: *mut [c_char; ATTR_SIZE],
    ) -> Result<(), ErrorType> {
        Ok(())
    }

    pub fn select_from_table_where(
        relname_source: &String,
        relname_target: &String,
        attribute: &String,
        op: i32,
        value: &String,
    ) -> Result<(), ErrorType> {
        Ok(())
    }

    pub fn select_attrlist_from_table_where(
        relname_source: &String,
        relname_target: &String,
        attr_count: i32,
        attr_list: *mut [c_char; ATTR_SIZE],
        attribute: &String,
        op: i32,
        value: &String,
    ) -> Result<(), ErrorType> {
        Ok(())
    }

    pub fn select_from_join_where(
        relname_source_one: &String,
        relname_source_two: &String,
        relname_target: &String,
        join_attr_one: &String,
        join_attr_two: &String,
    ) -> Result<(), ErrorType> {
        Ok(())
    }

    pub fn select_attrlist_from_join_where(
        relname_source_one: &String,
        relname_source_two: &String,
        relname_target: &String,
        join_attr_one: &String,
        join_attr_two: &String,
        attr_count: i32,
        attr_list: *mut [c_char; ATTR_SIZE],
    ) -> Result<(), ErrorType> {
        Ok(())
    }

    pub fn custom_function(argc: i32, argv: *mut [c_char; ATTR_SIZE]) -> Result<(), ErrorType> {
        Ok(())
    }
}
