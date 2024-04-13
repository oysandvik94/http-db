use super::DbParser;

pub struct YamlParser {}

impl DbParser for YamlParser {
    fn parse(data: String) -> super::DbParseResult {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::db_tree::{yaml_parser::YamlParser, DbParseResult, DbParser};

    #[test]
    fn single_table_db() {
        let table_yaml = "
            position:
              id: primary_key
              name: string
            ";

        let result = YamlParser::parse(table_yaml.to_string());
        match result {
            DbParseResult::ValidTree(table) => {
                assert_eq!(table.name, "position");
                assert_eq!(table.columns.len(), 2); 
                assert_eq!(table.columns[0].name, "id");
                assert_eq!(table.columns[1].name, "name");
            }
            DbParseResult::InvalidTree(err) => panic!("{err}"),
        }
    }
}
