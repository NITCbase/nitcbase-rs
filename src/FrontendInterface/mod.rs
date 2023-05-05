use crate::RegexHandler::*;
struct RegexHandler {

}
struct FrontendInterface {
};
fn attr_to_truncated_array(name_string: &str, name_array: &mut [u8]) {
    let truncated = &name_string[..ATTR_SIZE - 1];
    name_array.copy_from_slice(truncated.as_bytes());
    if name_string.len() >= ATTR_SIZE {
        println!(
            "(warning: '{}' truncated to '{}')",
            name_string,
            std::str::from_utf8(name_array).unwrap()
        );
    }
}

fn print_error_msg(error: i32) {
    match error {
        FAILURE => println!("Error: Command Failed"),
        E_OUTOFBOUND => println!("Error: Out of bound"),
        E_FREESLOT => println!("Error: Free slot"),
        E_NOINDEX => println!("Error: No index"),
        E_DISKFULL => println!("Error: Insufficient space in disk"),
        E_INVALIDBLOCK => println!("Error: Invalid block"),
        E_RELNOTEXIST => println!("Error: Relation does not exist"),
        E_RELEXIST => println!("Error: Relation already exists"),
        E_ATTRNOTEXIST => println!("Error: Attribute does not exist"),
        E_ATTREXIST => println!("Error: Attribute already exists"),
        E_CACHEFULL => println!("Error: Cache is full"),
        E_RELNOTOPEN => println!("Error: Relation is not open"),
        E_NATTRMISMATCH => println!("Error: Mismatch in number of attributes"),
        E_DUPLICATEATTR => println!("Error: Duplicate attributes found"),
        E_RELOPEN => println!("Error: Relation is open"),
        E_ATTRTYPEMISMATCH => println!("Error: Mismatch in attribute type"),
        E_INVALID => println!("Error: Invalid index or argument"),
        E_MAXRELATIONS => println!("Error: Maximum number of relations already present"),
        E_MAXATTRS => println!("Error: Maximum number of attributes allowed for a relation is 125"),
        E_NOTPERMITTED => println!("Error: This operation is not permitted"),
        E_INDEX_BLOCKS_RELEASED => {
            println!("Warning: Operation succeeded, but some indexes had to be dropped")
        }
        _ => panic!("Unknown error code: {}", error),
    }
}
