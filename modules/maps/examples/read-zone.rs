//use assembly::fdb::core::Schema;
use assembly_maps::luz::core::ZoneFile;
use assembly_maps::luz::paths::parser::parse_zone_paths;
use std::convert::TryFrom;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() <= 1 {
        println!("USAGE: {} FILE", args[0]);
    } else {
        let path = &args[1][..];
        let zone_file = ZoneFile::try_from(path);
        match zone_file {
            Ok(zone) => {
                println!("Name:        {}", zone.map_name);
                println!("Description: {}", zone.map_description);
                println!("Terrain:     {}", zone.map_filename);

                match zone.path_data {
                    Some(data) => {
                        println!("{}", data.len());
                        match parse_zone_paths(&data) {
                            Ok((_, paths)) => {
                                for path in paths.paths {
                                    println!("{:?}", path);
                                }
                            }
                            Err(e) => println!("{:?}", e),
                        }
                    }
                    None => println!("No Data"),
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }
}
