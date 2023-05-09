pub mod Algebra {
    use crate::define::*;
    // Insert
    pub fn insert(rel_name: &str, number_of_attributes: usize, record: &Vec<String>) -> i32 {
        // Implementation
        // ...
        0
    }

    // Select
    pub fn select(
        src_rel: &str,
        target_rel: &str,
        attr: &str,
        op: &ConditionalOperators,
        str_val: &str,
    ) -> i32 {
        // Implementation
        // ...
        0
    }

    // Project all (Copy)
    pub fn project(src_rel: &str, target_rel: &str) -> i32 {
        // Implementation
        // ...
        0
    }

    // Project
    pub fn project_with_attributes(
        src_rel: &str,
        target_rel: &str,
        tar_n_attrs: usize,
        tar_attrs: &Vec<String>,
    ) -> i32 {
        // Implementation
        // ...
        0
    }

    // Join
    pub fn join(
        src_rel_one: &str,
        src_rel_two: &str,
        target_rel: &str,
        attr_one: &str,
        attr_two: &str,
    ) -> i32 {
        // Implementation
        // ...
        0
    }
}
pub use self::Algebra::*;
