#[allow(non_snake_case)]
pub mod RegexHandler {
    use crate::define::*;
    use crate::Frontend::*;
    use regex::Regex;
    use regex::RegexBuilder;
    use std::fs::File;
    use std::fs::OpenOptions;
    use std::io::stdout;
    use std::io::BufRead;
    use std::io::BufReader;
    use std::io::Write;

    /* External File System Commands */
    const SUCCESS: i32 = 0;
    const HELP_CMD: &str = r"\s*HELP\s*;?";
    const EXIT_CMD: &str = r"\s*EXIT\s*;?";
    const RUN_CMD: &str = r"\s*RUN\s+([a-zA-Z0-9_/.-]+)\s*;?";
    const ECHO_CMD: &str = r"\s*ECHO\s*([a-zA-Z0-9 _,()'?:+*.-]*)\s*;?";

    /* DDL Commands */
    const CREATE_TABLE_CMD: &str = r"\s*CREATE\s+TABLE\s+([A-Za-z0-9_-]+)\s*\(\s*((?:[#A-Za-z0-9_-]+\s+(?:STR|NUM)\s*,\s*)*(?:[#A-Za-z0-9_-]+\s+(?:STR|NUM)))\s*\)\s*;?";
    const DROP_TABLE_CMD: &str = r"\s*DROP\s+TABLE\s+([A-Za-z0-9_-]+)\s*;?";
    const OPEN_TABLE_CMD: &str = r"\s*OPEN\s+TABLE\s+([A-Za-z0-9_-]+)\s*;?";
    const CLOSE_TABLE_CMD: &str = r"\s*CLOSE\s+TABLE\s+([A-Za-z0-9_-]+)\s*;?";
    const CREATE_INDEX_CMD: &str =
        r"\s*CREATE\s+INDEX\s+ON\s+([A-Za-z0-9_-]+)\s*\.\s*([#A-Za-z0-9_-]+)\s*;?";
    const DROP_INDEX_CMD: &str =
        r"\s*DROP\s+INDEX\s+ON\s+([A-Za-z0-9_-]+)\s*\.\s*([#A-Za-z0-9_-]+)\s*;?";
    const RENAME_TABLE_CMD: &str =
        r"\s*ALTER\s+TABLE\s+RENAME\s+([a-zA-Z0-9_-]+)\s+TO\s+([a-zA-Z0-9_-]+)\s*;?";
    const RENAME_COLUMN_CMD: &str = r"\s*ALTER\s+TABLE\s+RENAME\s+([a-zA-Z0-9_-]+)\s+COLUMN\s+([#a-zA-Z0-9_-]+)\s+TO\s+([#a-zA-Z0-9_-]+)\s*;?";

    /* DML Commands */
    const SELECT_FROM_CMD: &str =
        r"\s*SELECT\s+\*\s+FROM\s+([A-Za-z0-9_-]+)\s+INTO\s+([A-Za-z0-9_-]+)\s*;?";
    const SELECT_ATTR_FROM_CMD: &str = r"\s*SELECT\s+((?:[#A-Za-z0-9_-]+\s*,\s*)*(?:[#A-Za-z0-9_-]+))\s+FROM\s+([A-Za-z0-9_-]+)\s+INTO\s+([A-Za-z0-9_-]+)\s*;?";
    const SELECT_FROM_WHERE_CMD: &str = r"\s*SELECT\s+\*\s+FROM\s+([A-Za-z0-9_-]+)\s+INTO\s+([A-Za-z0-9_-]+)\s+WHERE\s+([#A-Za-z0-9_-]+)\s*(<|<=|>|>=|=|!=)\s*([A-Za-z0-9_-]+|([0-9]+(\.)[0-9]+))\s*;?";
    const SELECT_ATTR_FROM_WHERE_CMD: &str = r"\s*SELECT\s+((?:[#A-Za-z0-9_-]+\s*,\s*)*(?:[#A-Za-z0-9_-]+))\s+FROM\s+([A-Za-z0-9_-]+)\s+INTO\s+([A-Za-z0-9_-]+)\s+WHERE\s+([#A-Za-z0-9_-]+)\s*(<|<=|>|>=|=|!=)\s*([A-Za-z0-9_-]+|([0-9]+(\.)[0-9]+))\s*;?";
    const SELECT_FROM_JOIN_CMD: &str = r"\s*SELECT\s+\*\s+FROM\s+([A-Za-z0-9_-]+)\s+JOIN\s+([A-Za-z0-9_-]+)\s+INTO\s+([A-Za-z0-9_-]+)\s+WHERE\s+([A-Za-z0-9_-]+)\s*\.\s*([#A-Za-z0-9_-]+)\s*\=\s*([A-Za-z0-9_-]+)\s*\.\s*([#A-Za-z0-9_-]+)\s*;?";
    const SELECT_ATTR_FROM_JOIN_CMD: &str = r"\s*SELECT\s+((?:[#A-Za-z0-9_-]+\s*,\s*)*(?:[#A-Za-z0-9_-]+))\s+FROM\s+([A-Za-z0-9_-]+)\s+JOIN\s+([A-Za-z0-9_-]+)\s+INTO\s+([A-Za-z0-9_-]+)\s+WHERE\s+([A-Za-z0-9_-]+)\s*\.\s*([#A-Za-z0-9_-]+)\s*\=\s*([A-Za-z0-9_-]+)\s*\.\s*([#A-Za-z0-9_-]+)\s*;?";
    const INSERT_SINGLE_CMD: &str = r"\s*INSERT\s+INTO\s+([A-Za-z0-9_-]+)\s+VALUES\s*\(\s*((?:(?:[A-Za-z0-9_-]+|[0-9]+\.[0-9]+)\s*,\s*)*(?:[A-Za-z0-9_-]+|[0-9]+\.[0-9]+))\s*\)\s*;?";
    const INSERT_MULTIPLE_CMD: &str =
        r"\s*INSERT\s+INTO\s+([A-Za-z0-9_-]+)\s+VALUES\s+FROM\s+([a-zA-Z0-9_-]+\.csv)\s*;?";
    const CUSTOM_CMD: &str = r"\s*FUNCTION\s+([A-Za-z,#0-9\s()_-]+)\s*;?";

    macro_rules! regex {
        ($expr:expr) => {{
            RegexBuilder::new($expr)
                .case_insensitive(true)
                .build()
                .unwrap()
        }};
    }
    type HandlerFunction<'a> = fn(&mut RegexHandler<'a>) -> Result<(), ErrorType>;

    pub struct RegexHandler<'a> {
        m: Option<regex::Captures<'a>>,
        handlers: Vec<(Regex, HandlerFunction<'a>)>,
    }

    impl RegexHandler<'_> {
        fn new() -> Self {
            let handlers: Vec<(Regex, HandlerFunction)> = vec![
                (regex!(HELP_CMD), Self::helpHandler),
                (regex!(EXIT_CMD), Self::exitHandler),
                (regex!(ECHO_CMD), Self::echoHandler),
                (regex!(RUN_CMD), Self::runHandler),
                (regex!(OPEN_TABLE_CMD), Self::openHandler),
                (regex!(CLOSE_TABLE_CMD), Self::closeHandler),
                (regex!(CREATE_TABLE_CMD), Self::createTableHandler),
                (regex!(DROP_TABLE_CMD), Self::dropTableHandler),
                (regex!(CREATE_INDEX_CMD), Self::createIndexHandler),
                (regex!(DROP_INDEX_CMD), Self::dropIndexHandler),
                (regex!(RENAME_TABLE_CMD), Self::renameTableHandler),
                (regex!(RENAME_COLUMN_CMD), Self::renameColumnHandler),
                (regex!(INSERT_SINGLE_CMD), Self::insertSingleHandler),
                (regex!(INSERT_MULTIPLE_CMD), Self::insertFromFileHandler),
                (regex!(SELECT_FROM_CMD), Self::selectFromHandler),
                (regex!(SELECT_FROM_WHERE_CMD), Self::selectFromWhereHandler),
                (regex!(SELECT_ATTR_FROM_CMD), Self::selectAttrFromHandler),
                (
                    regex!(SELECT_ATTR_FROM_WHERE_CMD),
                    Self::selectAttrFromWhereHandler,
                ),
                (regex!(SELECT_FROM_JOIN_CMD), Self::selectFromJoinHandler),
                (
                    regex!(SELECT_ATTR_FROM_JOIN_CMD),
                    Self::selectAttrFromJoinHandler,
                ),
                (regex!(CUSTOM_CMD), Self::customFunctionHandler),
            ];
            RegexHandler { m: None, handlers }
        }
        #[allow(non_snake_case)]
        fn helpHandler(&mut self) -> Result<(), ErrorType> {
            printHelp();
            Ok(())
        }
        fn exitHandler(&mut self) -> Result<(), ErrorType> {
            Err(ErrorType::EXIT)
        }
        fn echoHandler(&mut self) -> Result<(), ErrorType> {
            let message: &str = &self.m.unwrap()[0];
            println!("{}", message);
            Ok(())
        }
        fn runHandler(&mut self) -> Result<(), ErrorType> {
            let fileName: &str = &self.m.unwrap()[1];
            const filePath: &str = BATCH_FILES_PATH;
            let mut commands_file = match OpenOptions::new()
                .read(true)
                .open(filePath.to_string() + fileName)
            {
                Ok(file) => file,
                Err(_) => {
                    println!("The file {} does not exist", fileName);
                    return Err(ErrorType::FAILURE);
                }
            };

            let mut command = String::new();
            let mut line_number = 1;
            let reader = BufReader::new(commands_file);
            for line in reader.lines() {
                let ret = self.handle(command.trim());
                if ret == ErrorType::EXIT {
                    break;
                } else if ret != SUCCESS {
                    println!("Executed up till line {}. Error at line number {}. Subsequent lines will be skipped.", line_number - 1, line_number);
                    break;
                }
                line_number += 1;
                command.clear();
            }
            Ok(())
        }
        fn openHandler(&mut self) -> Result<(), ErrorType> {
            let mut rel_name: [char; ATTR_SIZE] = ['\0'; ATTR_SIZE];
            Self::attrToTruncatedArray(self.m[1], &mut rel_name);
            let ret = Frontend::open_table(&rel_name);
            match ret {
                Ok(_) => {
                    println!("Relation {} opened successfully", rel_name.iter().collect());
                    ret
                }
                Err(_) => ret,
            }
        }
        fn attrToTruncatedArray(s: &str, relname: &str) {}
        fn closeHandler(&mut self) -> Result<(), ErrorType> {}
        fn createTableHandler(&mut self) -> Result<(), ErrorType> {}
        fn dropTableHandler(&mut self) -> Result<(), ErrorType> {}
        fn createIndexHandler(&mut self) -> Result<(), ErrorType> {}
        fn dropIndexHandler(&mut self) -> Result<(), ErrorType> {}
        fn renameTableHandler(&mut self) -> Result<(), ErrorType> {}
        fn renameColumnHandler(&mut self) -> Result<(), ErrorType> {}
        fn insertSingleHandler(&mut self) -> Result<(), ErrorType> {}
        fn insertFromFileHandler(&mut self) -> Result<(), ErrorType> {}
        fn selectFromHandler(&mut self) -> Result<(), ErrorType> {}
        fn selectFromFileHandler(&mut self) -> Result<(), ErrorType> {}
        fn selectFromWhereHandler(&mut self) -> Result<(), ErrorType> {}
        fn selectAttrFromHandler(&mut self) -> Result<(), ErrorType> {}
        fn selectAttrFromWhereHandler(&mut self) -> Result<(), ErrorType> {}
        fn selectAttrFromJoinHandler(&mut self) -> Result<(), ErrorType> {}
        fn selectFromJoinHandler(&mut self) -> Result<(), ErrorType> {}
        fn customFunctionHandler(&mut self) -> Result<(), ErrorType> {}
        fn handler() -> Result<(), ErrorType> {}
    }

    pub fn printHelp() {
        println!("CREATE TABLE tablename(attr1_name attr1_type ,attr2_name attr2_type....); \n\t -create a relation with given attribute names");
        println!("DROP TABLE tablename;\n\t-delete the relation\n ");
        println!("OPEN TABLE tablename;\n\t-open the relation \n");
        println!("CLOSE TABLE tablename;\n\t-close the relation \n ");
        println!(
            "CREATE INDEX ON tablename.attributename;\n\t-create an index on a given attribute. \n",
        );
        println!("DROP INDEX ON tablename.attributename; \n\t-delete the index. \n");
        println!("ALTER TABLE RENAME tablename TO new_tablename;\n\t-rename an existing relation to a given new name. \n");
        println!("ALTER TABLE RENAME tablename COLUMN column_name TO new_column_name;\n\t-rename an attribute of an existing relation.\n");
        println!("INSERT INTO tablename VALUES ( value1,value2,value3,... );\n\t-insert a single record into the given relation. \n");
        println!("INSERT INTO tablename VALUES FROM filepath; \n\t-insert multiple records from a csv file \n");
        println!("SELECT * FROM source_relation INTO target_relation; \n\t-creates a relation with the same attributes and records as of source relation\n");
        println!("SELECT Attribute1,Attribute2,....FROM source_relation INTO target_relation; \n\t-creates a relation with attributes specified and all records\n");
        println!("SELECT * FROM source_relation INTO target_relation WHERE attrname OP value; \n\t-retrieve records based on a condition and insert them into a target relation\n");
        println!("SELECT Attribute1,Attribute2,....FROM source_relation INTO target_relation;\n\t-creates a relation with the attributes specified and inserts those records which satisfy the given condition.\n");
        println!("SELECT * FROM source_relation1 JOIN source_relation2 INTO target_relation WHERE source_relation1.attribute1 = source_relation2.attribute2; \n\t-creates a new relation with by equi-join of both the source relations\n");
        println!("SELECT Attribute1,Attribute2,.. FROM source_relation1 JOIN source_relation2 INTO target_relation WHERE source_relation1.attribute1 = source_relation2.attribute2; \n\t-creates a new relation by equi-join of both the source relations with the attributes specified \n");
        println!("echo <any message> \n\t -echo back the given string. \n");
        println!("run <filename> \n\t -run commands from an input file in sequence. \n");
        println!("exit \n\t-Exit the interface");
    }
}
