use gray_matter::engine::YAML;
use gray_matter::{Matter, ParsedEntity, Pod};
use notesmng::notes::Notes;

pub struct ReturnData {
    pub result: ParsedEntity,
    pub notes: Notes,
}

pub fn parse(input: &str) -> ReturnData {
    let mut matter = Matter::<YAML>::new();
    matter.delimiter = "***".to_owned();

    let mut result = matter.parse(input);
    let data = result.data.take().unwrap().deserialize();

    ReturnData {
        result,
        notes: data.unwrap(),
    }
}
