mod yaml_parser;

pub struct DbNode {
    parent: Option<Box<DbNode>>,
    children: Vec<DbNode>,
    name: String,
    columns: Vec<DbColumn>,
}

pub struct DbColumn {
    name: String,
    data_type: ColumnType,
}

pub enum ColumnType {
    PrimaryKey,
    String,
    Number,
}

pub enum DbParseResult {
    ValidTree(DbNode),
    InvalidTree(String)
}

pub trait DbParser {
    fn parse(data: String) -> DbParseResult;
}

