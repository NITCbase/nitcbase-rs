pub mod Frontend {
    use crate::define::*;
    use std::os::raw::c_char;
    pub fn create_table(
        relname: *mut c_char,
        no_attrs: i32,
        attributes: *mut [c_char; ATTR_SIZE],
        type_attrs: *mut i32,
    ) -> Result<(), ErrorType> {
        Ok(())
    }

    pub fn drop_table(relname: &[char; ATTR_SIZE]) -> Result<(), ErrorType> {
        Ok(())
    }

    pub fn open_table(relname: &[char; ATTR_SIZE]) -> Result<(), ErrorType> {
        Ok(())
    }

    pub fn close_table(relname: &[char; ATTR_SIZE]) -> Result<(), ErrorType> {
        Ok(())
    }

    pub fn create_index(
        relname: &[char; ATTR_SIZE],
        attrname: &[char; ATTR_SIZE],
    ) -> Result<(), ErrorType> {
        Ok(())
    }

    pub fn drop_index(
        relname: &[char; ATTR_SIZE],
        attrname: &[char; ATTR_SIZE],
    ) -> Result<(), ErrorType> {
        Ok(())
    }

    pub fn alter_table_rename(
        relname_from: &[char; ATTR_SIZE],
        relname_to: &[char; ATTR_SIZE],
    ) -> Result<(), ErrorType> {
        Ok(())
    }

    pub fn alter_table_rename_column(
        relname: &[char; ATTR_SIZE],
        attrname_from: &[char; ATTR_SIZE],
        attrname_to: &[char; ATTR_SIZE],
    ) -> Result<(), ErrorType> {
        Ok(())
    }

    // DML
    pub fn insert_into_table_values(
        relname: &[char; ATTR_SIZE],
        attr_count: i32,
        attr_values: *mut [c_char; ATTR_SIZE],
    ) -> Result<(), ErrorType> {
        Ok(())
    }

    pub fn select_from_table(
        relname_source: &[char; ATTR_SIZE],
        relname_target: &[char; ATTR_SIZE],
    ) -> Result<(), ErrorType> {
        Ok(())
    }

    pub fn select_attrlist_from_table(
        relname_source: &[char; ATTR_SIZE],
        relname_target: &[char; ATTR_SIZE],
        attr_count: i32,
        attr_list: *mut [c_char; ATTR_SIZE],
    ) -> Result<(), ErrorType> {
        Ok(())
    }

    pub fn select_from_table_where(
        relname_source: &[char; ATTR_SIZE],
        relname_target: &[char; ATTR_SIZE],
        attribute: &[char; ATTR_SIZE],
        op: i32,
        value: &[char; ATTR_SIZE],
    ) -> Result<(), ErrorType> {
        Ok(())
    }

    pub fn select_attrlist_from_table_where(
        relname_source: &[char; ATTR_SIZE],
        relname_target: &[char; ATTR_SIZE],
        attr_count: i32,
        attr_list: *mut [c_char; ATTR_SIZE],
        attribute: &[char; ATTR_SIZE],
        op: i32,
        value: &[char; ATTR_SIZE],
    ) -> Result<(), ErrorType> {
        Ok(())
    }

    pub fn select_from_join_where(
        relname_source_one: &[char; ATTR_SIZE],
        relname_source_two: &[char; ATTR_SIZE],
        relname_target: &[char; ATTR_SIZE],
        join_attr_one: &[char; ATTR_SIZE],
        join_attr_two: &[char; ATTR_SIZE],
    ) -> Result<(), ErrorType> {
        Ok(())
    }

    pub fn select_attrlist_from_join_where(
        relname_source_one: &[char; ATTR_SIZE],
        relname_source_two: &[char; ATTR_SIZE],
        relname_target: &[char; ATTR_SIZE],
        join_attr_one: &[char; ATTR_SIZE],
        join_attr_two: &[char; ATTR_SIZE],
        attr_count: i32,
        attr_list: *mut [c_char; ATTR_SIZE],
    ) -> Result<(), ErrorType> {
        Ok(())
    }

    pub fn custom_function(argc: i32, argv: *mut [c_char; ATTR_SIZE]) -> Result<(), ErrorType> {
        Ok(())
    }
}
