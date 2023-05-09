#[allow(non_snake_case)]
#[allow(unused)]
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
    type HandlerFunction<'a> = fn(&RegexHandler<'a>) -> Result<(), ErrorType>;
    type HandlerFunctionMut<'a> = fn(&mut RegexHandler<'a>) -> Result<(), ErrorType>;
    enum HandlerType<'a> {
        Ref(HandlerFunction<'a>),
        MutRef(HandlerFunctionMut<'a>),
    }

    pub struct RegexHandler<'a> {
        m: Vec<String>,
        handlers: Vec<(Regex, HandlerType<'a>)>,
    }

    impl<'a> RegexHandler<'_> {
        pub fn new() -> Self {
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
                (
                    regex!(DROP_INDEX_CMD),
                    HandlerType::Ref(Self::dropIndexHandler),
                ),
                (
                    regex!(RENAME_TABLE_CMD),
                    HandlerType::Ref(Self::renameTableHandler),
                ),
                (
                    regex!(RENAME_COLUMN_CMD),
                    HandlerType::Ref(Self::renameColumnHandler),
                ),
                (
                    regex!(INSERT_SINGLE_CMD),
                    HandlerType::Ref(Self::insertSingleHandler),
                ),
                (
                    regex!(INSERT_MULTIPLE_CMD),
                    HandlerType::MutRef(Self::insertFromFileHandler),
                ),
                (
                    regex!(SELECT_FROM_CMD),
                    HandlerType::Ref(Self::selectFromHandler),
                ),
                (
                    regex!(SELECT_FROM_WHERE_CMD),
                    HandlerType::Ref(Self::selectFromWhereHandler),
                ),
                (
                    regex!(SELECT_ATTR_FROM_CMD),
                    HandlerType::Ref(Self::selectAttrFromHandler),
                ),
                (
                    regex!(SELECT_ATTR_FROM_WHERE_CMD),
                    HandlerType::Ref(Self::selectAttrFromWhereHandler),
                ),
                (
                    regex!(SELECT_FROM_JOIN_CMD),
                    HandlerType::Ref(Self::selectFromJoinHandler),
                ),
                (
                    regex!(SELECT_ATTR_FROM_JOIN_CMD),
                    HandlerType::Ref(Self::selectAttrFromJoinHandler),
                ),
                (
                    regex!(CUSTOM_CMD),
                    HandlerType::Ref(Self::customFunctionHandler),
                ),
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
        pub fn handle(&mut self, command: &String) -> Result<(), ErrorType> {
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
            println!("Query not recognized!");
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
            let relName = Self::attrToTruncatedArray(&self.m[1]);
            let ret = Frontend::open_table(relName);
            match ret {
                Ok(_) => {
                    println!("Relation {} opened successfully", relName);
                    ret
                }
                Err(_) => ret,
            }
        }
        fn attrToTruncatedArray(nameString: &str) -> &str {
            if nameString.len() >= ATTR_SIZE {
                println!(
                    "(warning: '{}' truncated to '{}')",
                    nameString,
                    &nameString[0..ATTR_SIZE]
                );
                &nameString[0..ATTR_SIZE]
            } else {
                nameString
            }
        }
        fn closeHandler(&self) -> Result<(), ErrorType> {
            let relName = Self::attrToTruncatedArray(&self.m[1]);

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
            let relName = Self::attrToTruncatedArray(&self.m[1]);

            let words: Vec<&str> = Self::extractTokens(&self.m[2]);
            let attrCount = words.len() / 2;
            if attrCount > 125 {
                return Err(ErrorType::MAXATTRS);
            }

            let mut attrNames: Vec<String> =
                vec![String::from('\0'.to_string().repeat(ATTR_SIZE)); attrCount];
            let mut attrTypes: Vec<AttributeType> = vec![AttributeType::NUMBER; attrCount];

            let mut k = 0;
            for i in 0..attrCount {
                attrNames[i] = Self::attrToTruncatedArray(words[k]).to_string();
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
            let relName = Self::attrToTruncatedArray(&self.m[1]);

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
            let relName = Self::attrToTruncatedArray(&self.m[1]);
            let attrName = Self::attrToTruncatedArray(&self.m[2]);

            let ret = Frontend::create_index(&relName, &attrName);
            match ret {
                Ok(_) => {
                    println!("Index created successfully");
                    ret
                }
                Err(_) => ret,
            }
        }
        fn dropIndexHandler(&self) -> Result<(), ErrorType> {
            let relName = Self::attrToTruncatedArray(&self.m[1]);
            let attrName = Self::attrToTruncatedArray(&self.m[2]);
            let ret = Frontend::drop_index(&relName, &attrName);
            match ret {
                Ok(_) => {
                    println!("Index deleted successfully");
                    ret
                }
                Err(_) => ret,
            }
        }
        fn renameTableHandler(&self) -> Result<(), ErrorType> {
            let oldRelName = Self::attrToTruncatedArray(&self.m[1]);
            let newRelName = Self::attrToTruncatedArray(&self.m[2]);

            let ret = Frontend::alter_table_rename(&oldRelName, &newRelName);
            match ret {
                Ok(_) => {
                    println!("Renamed Relation successfully");
                    ret
                }
                Err(_) => ret,
            }
        }
        fn renameColumnHandler(&self) -> Result<(), ErrorType> {
            let relName = Self::attrToTruncatedArray(&self.m[1]);
            let oldColName = Self::attrToTruncatedArray(&self.m[2]);
            let newColName = Self::attrToTruncatedArray(&self.m[3]);

            let ret = Frontend::alter_table_rename_column(&relName, &oldColName, &newColName);
            match ret {
                Ok(_) => {
                    println!("Renamed Attribute successfully");
                    ret
                }
                Err(_) => ret,
            }
        }

        fn insertSingleHandler(&self) -> Result<(), ErrorType> {
            let relName = Self::attrToTruncatedArray(&self.m[1]);
            let words: Vec<&str> = Self::extractTokens(self.m[2].as_str());

            let attrCount: usize = words.len();
            let mut attrValues: Vec<String> =
                vec![String::from('\0'.to_string().repeat(ATTR_SIZE)); attrCount];
            for i in 0..attrCount {
                attrValues[i] = Self::attrToTruncatedArray(words[i]).to_string();
            }

            let ret = Frontend::insert_into_table_values(&relName, attrCount, &attrValues);
            match ret {
                Ok(_) => {
                    println!("Inserted successfully");
                    ret
                }
                Err(_) => ret,
            }
        }
        fn insertFromFileHandler(&mut self) -> Result<(), ErrorType> {
            let relName = Self::attrToTruncatedArray(&self.m[1]);
            let filePath: String = INPUT_FILES_PATH.to_string() + self.m[2].as_str();

            println!("File path: {}", filePath);

            let file = match File::open(&filePath) {
                Ok(file) => file,
                Err(err) => {
                    println!("Invalid file path or file does not exist");
                    return Err(ErrorType::IO_ERROR(err));
                }
            };

            let mut error_msg = String::new();
            let mut columnCount = 128;
            let mut lineNumber = 1;

            let mut ret = Ok(());
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let file_line = line.map_err(|e| ErrorType::IO_ERROR(e))?;
                let mut row: Vec<String> = Vec::new();
                for item in file_line.split(',') {
                    if item.is_empty() {
                        error_msg += "Null values not allowed in attribute values\n";
                        return Err(ErrorType::FAILURE);
                    }
                    row.push(item.to_string());
                }
                if columnCount == 128 {
                    columnCount = row.len();
                } else if columnCount != row.len() {
                    error_msg += "Mismatch in number of attributes\n";
                    ret = Err(ErrorType::FAILURE);
                    break;
                }
                let mut rowArray: Vec<String> =
                    vec![String::from('\0'.to_string().repeat(ATTR_SIZE)); columnCount];
                for (i, item) in row.iter().enumerate() {
                    rowArray[i] = Self::attrToTruncatedArray(item).to_string();
                }

                ret = Frontend::insert_into_table_values(relName, columnCount, &rowArray);
                match ret {
                    Ok(_) => {}
                    Err(_) => {
                        break;
                    }
                };

                lineNumber += 1;
            }
            if let Ok(()) = ret {
                println!("{} rows inserted successfully", lineNumber);
                ret
            } else {
                if lineNumber > 1 {
                    println!("Rows till line {} successfully inserted", lineNumber - 1);
                }
                println!("Insertion error at line {} in file", lineNumber);
                println!("Subsequent lines will be skipped");
                println!("Error: {}", error_msg);
                ret
            }
        }

        fn selectFromHandler(&self) -> Result<(), ErrorType> {
            let sourceRelName = Self::attrToTruncatedArray(&self.m[1]);
            let targetRelName = Self::attrToTruncatedArray(&self.m[2]);

            let ret = Frontend::select_from_table(&sourceRelName, &targetRelName);
            match ret {
                Ok(_) => {
                    println!("Selected successfully into {}", targetRelName);
                    ret
                }
                Err(_) => ret,
            }
        }
        fn selectFromWhereHandler(&self) -> Result<(), ErrorType> {
            let sourceRelName = Self::attrToTruncatedArray(&self.m[1]);
            let targetRelName = Self::attrToTruncatedArray(&self.m[2]);
            let attribute = Self::attrToTruncatedArray(&self.m[3]);
            let op = getOperator(&self.m[4].as_str());
            let valueStr = Self::attrToTruncatedArray(&self.m[5]);

            let ret = Frontend::select_from_table_where(
                sourceRelName,
                targetRelName,
                attribute,
                &op,
                valueStr,
            );
            match ret {
                Ok(_) => {
                    println!("Selected successfully into {}", targetRelName);
                    ret
                }
                Err(_) => ret,
            }
        }
        fn selectAttrFromHandler(&self) -> Result<(), ErrorType> {
            let sourceRelName = Self::attrToTruncatedArray(&self.m[1]);
            let targetRelName = Self::attrToTruncatedArray(&self.m[2]);

            let words: Vec<&str> = Self::extractTokens(&self.m[1]);
            let attrCount = words.len();

            let mut attrNames: Vec<String> =
                vec![String::from('\0'.to_string().repeat(ATTR_SIZE)); attrCount];

            for i in 0..attrCount {
                attrNames[i] = Self::attrToTruncatedArray(words[i]).to_string();
            }

            let ret = Frontend::select_attrlist_from_table(
                sourceRelName,
                targetRelName,
                attrCount,
                &attrNames,
            );
            match ret {
                Ok(_) => {
                    println!("Selected successfully into {}", targetRelName);
                    ret
                }
                Err(_) => ret,
            }
        }
        fn selectAttrFromWhereHandler(&self) -> Result<(), ErrorType> {
            let sourceRelName = Self::attrToTruncatedArray(&self.m[2]);
            let targetRelName = Self::attrToTruncatedArray(&self.m[3]);
            let attribute = Self::attrToTruncatedArray(&self.m[4]);
            let op = getOperator(&self.m[5].as_str());
            let valueStr = Self::attrToTruncatedArray(&self.m[6]);

            let attrTokens: Vec<&str> = Self::extractTokens(&self.m[1]);
            let attrCount = attrTokens.len();
            let mut attrNames: Vec<String> =
                vec![String::from('\0'.to_string().repeat(ATTR_SIZE)); attrCount];
            for i in 0..attrCount {
                attrNames[i] = Self::attrToTruncatedArray(attrTokens[i]).to_string();
            }
            let ret = Frontend::select_attrlist_from_table_where(
                sourceRelName,
                targetRelName,
                attrCount,
                &attrNames,
                attribute,
                &op,
                valueStr,
            );
            match ret {
                Ok(_) => {
                    println!("Selected successfully into {}", targetRelName);
                    ret
                }
                Err(_) => ret,
            }
        }
        fn selectFromJoinHandler(&self) -> Result<(), ErrorType> {
            let sourceRelOneName = Self::attrToTruncatedArray(&self.m[1]);
            let sourceRelTwoName = Self::attrToTruncatedArray(&self.m[2]);
            let targetRelName = Self::attrToTruncatedArray(&self.m[3]);

            let joinAttributeOne;
            let joinAttributeTwo;

            if self.m[1] == self.m[4] && self.m[2] == self.m[6] {
                joinAttributeOne = Self::attrToTruncatedArray(&self.m[5]);
                joinAttributeTwo = Self::attrToTruncatedArray(&self.m[7]);
            } else if self.m[1] == self.m[6] && self.m[2] == self.m[4] {
                joinAttributeOne = Self::attrToTruncatedArray(&self.m[7]);
                joinAttributeTwo = Self::attrToTruncatedArray(&self.m[5]);
            } else {
                println!("Syntax Error: Relation names do not match");
                return Err(ErrorType::FAILURE);
            }
            let ret = Frontend::select_from_join_where(
                sourceRelOneName,
                sourceRelTwoName,
                targetRelName,
                joinAttributeOne,
                joinAttributeTwo,
            );
            match ret {
                Ok(_) => {
                    println!("Selected successfully into {}", targetRelName);
                    ret
                }
                Err(_) => ret,
            }
        }
        fn selectAttrFromJoinHandler(&self) -> Result<(), ErrorType> {
            let sourceRelOneName = Self::attrToTruncatedArray(&self.m[2]);
            let sourceRelTwoName = Self::attrToTruncatedArray(&self.m[3]);
            let targetRelName = Self::attrToTruncatedArray(&self.m[4]);

            let joinAttributeOne;
            let joinAttributeTwo;

            if self.m[2] == self.m[5] && self.m[3] == self.m[7] {
                joinAttributeOne = Self::attrToTruncatedArray(&self.m[6]);
                joinAttributeTwo = Self::attrToTruncatedArray(&self.m[8]);
            } else if self.m[2] == self.m[7] && self.m[3] == self.m[5] {
                joinAttributeOne = Self::attrToTruncatedArray(&self.m[8]);
                joinAttributeTwo = Self::attrToTruncatedArray(&self.m[6]);
            } else {
                println!("Syntax Error: Relation names do not match");
                return Err(ErrorType::FAILURE);
            }

            //TODO
            //joinAttributeOne = Self::attrToTruncatedArray(&self.m[6]);
            //joinAttributeTwo = Self::attrToTruncatedArray(&self.m[8]);

            let attrTokens: Vec<&str> = Self::extractTokens(&self.m[1]);
            let attrCount: usize = attrTokens.len();
            let mut attrNames: Vec<String> =
                vec![String::from('\0'.to_string().repeat(ATTR_SIZE)); attrCount];

            for i in 0..attrCount {
                attrNames[i] = Self::attrToTruncatedArray(attrTokens[i]).to_string();
            }
            let ret = Frontend::select_attrlist_from_join_where(
                sourceRelOneName,
                sourceRelTwoName,
                targetRelName,
                joinAttributeOne,
                joinAttributeTwo,
                attrCount,
                &attrNames,
            );
            match ret {
                Ok(_) => {
                    println!("Selected successfully into {}", targetRelName);
                    ret
                }
                Err(_) => ret,
            }
        }
        fn customFunctionHandler(&self) -> Result<(), ErrorType> {
            let attrTokens: Vec<&str> = Self::extractTokens(&self.m[1]);
            let attrCount: usize = attrTokens.len();
            let mut attrNames: Vec<String> =
                vec![String::from('\0'.to_string().repeat(ATTR_SIZE)); attrCount];

            for i in 0..attrCount {
                attrNames[i] = Self::attrToTruncatedArray(attrTokens[i]).to_string();
            }
            let ret = Frontend::custom_function(attrCount, &attrNames);
            ret
        }
    }
    fn getOperator(opStr: &str) -> ConditionalOperators {
        match opStr {
            "=" => ConditionalOperators::EQ,
            "<=" => ConditionalOperators::LE,
            ">=" => ConditionalOperators::GE,
            "<" => ConditionalOperators::LT,
            ">" => ConditionalOperators::GT,
            "!=" => ConditionalOperators::NE,
            _ => ConditionalOperators::EQ,
        }
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
pub use self::RegexHandler::*;
