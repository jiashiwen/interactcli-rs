use serde_yaml::{from_str, Value};
use std::fs;
use yaml_rust::{yaml, YamlEmitter, YamlLoader};

fn main() {
    let contents =
        fs::read_to_string("Settings.yml").expect("Something went wrong reading the file");

    let docs = YamlLoader::load_from_str(contents.as_str()).unwrap();

    // Multi document support, doc is a yaml::Yaml
    let doc = &docs[0];

    // Debug support
    println!("{:?}", docs);
    println!("{:?}", doc);
    println!("{:?}", doc["services"]["redissyncer-monitor"]);

    // Dump the YAML object
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter
            .dump(&doc["services"]["redissyncer-monitor"])
            .unwrap(); // dump the YAML object to a String
    }
    println!("{}", out_str);
    let s = r#"{ A: 65, B: 66, C: 67 }"#;
    let object = from_str::<Value>(s).unwrap();
    let x = object.get("A").unwrap();
    println!("{:?}", x);
}
