use assembly_xml::universe_config::Environment;

fn main() {
    let mut args = std::env::args();
    let _self = args.next().unwrap();
    let file = args.next().unwrap();
    println!("{}", file);
    let xml = std::fs::read_to_string(&file).unwrap();
    let data: Environment = quick_xml::de::from_str(&xml).unwrap();
    println!("{:#?}", data);
}
