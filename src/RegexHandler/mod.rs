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
    type HandlerFunction<'a> = fn(&'a RegexHandler<'a>) -> Result<(), ErrorType>;
    type HandlerFunctionMut<'a> = fn(&'a mut RegexHandler<'a>) -> Result<(), ErrorType>;
    enum HandlerType<'a> {
        Ref(HandlerFunction<'a>),
        MutRef(HandlerFunctionMut<'a>),
    }

    pub struct RegexHandler<'a> {
        m: Vec<String>,
        handlers: Vec<(Regex, HandlerType<'a>)>,
    }

    impl<'a> RegexHandler<'_> {
        fn new() -> Self {
            let handlers: Vec<(Regex, HandlerType)> = vec![
                (regex!(HELP_CMD), HandlerType::Ref(Self::helpHandler)),
                (regex!(EXIT_CMD), HandlerType::Ref(Self::exitHandler)),
                (regex!(ECHO_CMD), HandlerType::Ref(Self::echoHandler)),
                (regex!(RUN_CMD), HandlerType::MutRef(Self::runHandler)),
                (regex!(OPEN_TABLE_CMD), HandlerType::Ref(Self::openHandler)),
                (
                    regex!(CLOSE_TABLE_CMD),
                    HandlerType::Ref(Self::closeHandler),
                ),
                (
                    regex!(CREATE_TABLE_CMD),
                    HandlerType::Ref(Self::createTableHandler),
                ),
                (
                    regex!(DROP_TABLE_CMD),
                    HandlerType::Ref(Self::dropTableHandler),
                ),
                (
                    regex!(CREATE_INDEX_CMD),
                    HandlerType::Ref(Self::createIndexHandler),
                ),
                //(regex!(DROP_INDEX_CMD), HandlerType::Ref(Self::dropIndexHandler)),
                //(regex!(RENAME_TABLE_CMD), HandlerType::Ref(Self::renameTableHandler)),
                //(regex!(RENAME_COLUMN_CMD), HandlerType::Ref(Self::renameColumnHandler)),
                //(regex!(INSERT_SINGLE_CMD), HandlerType::Ref(Self::insertSingleHandler)),
                //(regex!(INSERT_MULTIPLE_CMD), HandlerType::Ref(Self::insertFromFileHandler)),
                //(regex!(SELECT_FROM_CMD), HandlerType::Ref(Self::selectFromHandler)),
                //(regex!(SELECT_FROM_WHERE_CMD), HandlerType::Ref(Self::selectFromWhereHandler)),
                //(regex!(SELECT_ATTR_FROM_CMD), HandlerType::Ref(Self::selectAttrFromHandler)),
                //(
                //    regex!(SELECT_ATTR_FROM_WHERE_CMD),
                //    HandlerType::Ref(Self::selectAttrFromWhereHandler,)
                //),
                //(regex!(SELECT_FROM_JOIN_CMD), HandlerType::Ref(Self::selectFromJoinHandler)),
                //(
                //    regex!(SELECT_ATTR_FROM_JOIN_CMD),
                //    HandlerType::Ref(Self::selectAttrFromJoinHandler,)
                //),
                //(regex!(CUSTOM_CMD), HandlerType::Ref(Self::customFunctionHandler)),
            ];
            RegexHandler {
                m: vec![],
                handlers,
            }
        }
        fn runHandler(&mut self) -> Result<(), ErrorType> {
            let fileName: &str = &self.m[1];
            let filePath: &str = BATCH_FILES_PATH;
            let commands_file = match OpenOptions::new()
                .read(true)
                .open(filePath.to_string() + fileName)
            {
                Ok(file) => file,
                Err(_) => {
                    println!("The file {} does not exist", fileName);
                    return Err(ErrorType::FAILURE);
                }
            };

            let mut line_number = 1;
            let reader = BufReader::new(commands_file);
            for line in reader.lines() {
                let ret = Self::handle(self, &line.map_err(|e| ErrorType::IO_ERROR(e))?);
                match ret {
                    Ok(_) => {}
                    Err(ErrorType::EXIT) => return ret,
                    _ => {
                        println!("Executed up till line {}\n. Error at line number {}. Subsequent lines will be skipped.", line_number - 1, line_number);
                        return ret;
                    }
                };
                line_number += 1;
            }
            //std::mem::drop(commands_file);
            Ok(())
        }
        fn handle(&mut self, command: &String) -> Result<(), ErrorType> {
            for (test_command, handler) in self.handlers.iter() {
                if test_command.is_match(&command) {
                    let caps = test_command.captures(command.as_str()).unwrap();
                    self.m.clear();
                    for i in 0..caps.len() {
                        self.m.push(String::from(&caps[i]));
                    }

                    let status = match handler {
                        HandlerType::Ref(func) => func(&*self),
                        HandlerType::MutRef(func) => func(&mut *self),
                    };
                    match status {
                        Ok(_) | Err(ErrorType::EXIT) => {
                            return status;
                        }
                        Err(e) => {
                            e.print_error();
                            return Err(ErrorType::FAILURE);
                        }
                    }
                }
            }
            Ok(())
        }
        fn helpHandler(&self) -> Result<(), ErrorType> {
            printHelp();
            Ok(())
        }
        fn exitHandler(&self) -> Result<(), ErrorType> {
            Err(ErrorType::EXIT)
        }
        fn echoHandler(&self) -> Result<(), ErrorType> {
            let messg = &self.m[1];
            println!("{}", messg);
            Ok(())
        }
        fn openHandler(&self) -> Result<(), ErrorType> {
            let mut rel_name: String = String::from('\0'.to_string().repeat(16));
            let st = &self.m[1];
            Self::attrToTruncatedArray(&st, &mut rel_name);
            let ret = Frontend::open_table(&rel_name);
            match ret {
                Ok(_) => {
                    println!("Relation {} opened successfully", rel_name);
                    ret
                }
                Err(_) => ret,
            }
        }
        fn attrToTruncatedArray(nameString: &str, nameArray: &mut String) {
            *nameArray = String::from(&nameString[0..ATTR_SIZE - 1]);
            if nameString.len() >= ATTR_SIZE {
                println!("(warning: '{}' truncated to '{}')", nameString, nameArray);
            }
        }
        fn closeHandler(&self) -> Result<(), ErrorType> {
            let mut relName: String = String::from('\0').to_string().repeat(16);
            let st = &self.m[1];
            Self::attrToTruncatedArray(st, &mut relName);

            let ret = Frontend::close_table(&relName);
            match ret {
                Ok(_) => {
                    println!("Relation {} opened successfully", relName);
                    ret
                }
                Err(_) => ret,
            }
        }
        fn extractTokens(input: &str) -> Vec<&str> {
            let re = Regex::new(r"\s*,\s*|\s+").unwrap();
            let tokens: Vec<&str> = re.split(input).collect();
            tokens
        }
        fn createTableHandler(&self) -> Result<(), ErrorType> {
            let mut relName: String = String::from('\0').to_string().repeat(16);
            Self::attrToTruncatedArray(self.m[1].as_ref(), &mut relName);

            let mut words: Vec<&str> = Self::extractTokens(&self.m[2]);
            let attrCount = words.len() / 2;
            if attrCount > 125 {
                return Err(ErrorType::MAXATTRS);
            }

            let mut attrNames: Vec<String> =
                vec![String::from('\0'.to_string().repeat(ATTR_SIZE)); attrCount];
            let mut attrTypes: Vec<AttributeType> = vec![AttributeType::NUMBER; attrCount];

            let mut k = 0;
            for i in 0..attrCount {
                Self::attrToTruncatedArray(words[k], &mut attrNames[i]);
                if words[k + 1] == "STR" {
                    attrTypes[i] = AttributeType::STRING;
                } else if words[k + 1] == "NUM" {
                    attrTypes[i] = AttributeType::NUMBER;
                }
                k += 2;
            }

            let ret = Frontend::create_table(&relName, attrCount, &attrNames, &attrTypes);
            match ret {
                Ok(_) => {
                    println!("Relation {} created successfully", relName);
                    ret
                }
                Err(_) => ret,
            }
        }
        fn dropTableHandler(&self) -> Result<(), ErrorType> {
            let mut relName: String = String::from('\0').to_string().repeat(16);
            Self::attrToTruncatedArray(&self.m[1], &mut relName);

            let ret = Frontend::drop_table(&relName);
            match ret {
                Ok(_) => {
                    println!("Relation {} deleted successfully", relName);
                    ret
                }
                Err(_) => ret,
            }
        }
        fn createIndexHandler(&self) -> Result<(), ErrorType> {
            let mut relName: String = String::from('\0').to_string().repeat(16);
            let mut attrName: String = String::from('\0').to_string().repeat(16);
            Self::attrToTruncatedArray(&self.m[1], &mut relName);
            Self::attrToTruncatedArray(&self.m[2], &mut attrName);

            let ret = Frontend::create_index(&relName, &attrName);
            match ret {
                Ok(_) => {
                    println!("Index created successfully");
                    ret
                }
                Err(_) => ret,
            }
        }
        //fn dropIndexHandler(&self) -> Result<(), ErrorType> {}
        //fn renameTableHandler(&self) -> Result<(), ErrorType> {}
        //fn renameColumnHandler(&self) -> Result<(), ErrorType> {}
        //fn insertSingleHandler(&self) -> Result<(), ErrorType> {}
        //fn insertFromFileHandler(&self) -> Result<(), ErrorType> {}
        //fn selectFromHandler(&self) -> Result<(), ErrorType> {}
        //fn selectFromFileHandler(&self) -> Result<(), ErrorType> {}
        //fn selectFromWhereHandler(&self) -> Result<(), ErrorType> {}
        //fn selectAttrFromHandler(&self) -> Result<(), ErrorType> {}
        //fn selectAttrFromWhereHandler(&self) -> Result<(), ErrorType> {}
        //fn selectAttrFromJoinHandler(&self) -> Result<(), ErrorType> {}
        //fn selectFromJoinHandler(&self) -> Result<(), ErrorType> {}
        //fn customFunctionHandler(&self) -> Result<(), ErrorType> {}
        //fn handler() -> Result<(), ErrorType> {}
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
