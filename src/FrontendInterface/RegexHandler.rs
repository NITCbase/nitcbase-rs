use regex::Regex;
use regex::RegexBuilder;

/* External File System Commands */
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

type HandlerFunction = fn(&mut RegexHandler) -> i32;

pub struct RegexHandler {
    m: Regex::Captures,
    handlers: Vec<(Regex, HandlerFunction)>,
}

impl RegexHandler {
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
        RegexHandler {
            m: Regex::Captures::default(),
            handlers: vec![],
        }
    }
    fn helpHandler(&mut self) -> i32 {}
    fn exitHandler(&mut self) -> i32 {}
    fn echoHandler(&mut self) -> i32 {}
    fn runHandler(&mut self) -> i32 {}
    fn openHandler(&mut self) -> i32 {}
    fn closeHandler(&mut self) -> i32 {}
    fn createTableHandler(&mut self) -> i32 {}
    fn dropTableHandler(&mut self) -> i32 {}
    fn createIndexHandler(&mut self) -> i32 {}
    fn dropIndexHandler(&mut self) -> i32 {}
    fn renameTableHandler(&mut self) -> i32 {}
    fn renameColumnHandler(&mut self) -> i32 {}
    fn insertSingleHandler(&mut self) -> i32 {}
    fn insertFromFileHandler(&mut self) -> i32 {}
    fn selectFromFileHandler(&mut self) -> i32 {}
    fn selectFromWhereHandler(&mut self) -> i32 {}
    fn selectAttrFromHandler(&mut self) -> i32 {}
    fn selectAttrFromWhereHandler(&mut self) -> i32 {}
    fn selectAttrFromJoinHandler(&mut self) -> i32 {}
    fn selectFromJoinHandler(&mut self) -> i32 {}
    fn customFunctionHandler(&mut self) -> i32 {}
}
