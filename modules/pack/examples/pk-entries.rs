use argh::FromArgs;
use assembly_pack::common::CRCTreeVisitor;
use assembly_pack::pk::file::PKEntryData;
use assembly_pack::pk::reader::{PackEntryAccessor, PackFile};
use serde::ser::SerializeMap;
use serde::Serializer;
use serde_json::ser::Formatter;
use std::fs::File;
use std::io::{self, BufReader, Stdout};
use std::ops::ControlFlow;
use std::path::PathBuf;

struct JsonVisitor<'a, W, F>(serde_json::ser::Compound<'a, W, F>);

impl<'a, W, F> CRCTreeVisitor<PKEntryData> for JsonVisitor<'a, W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Break = serde_json::Error;

    fn visit(&mut self, key: u32, value: PKEntryData) -> ControlFlow<Self::Break> {
        match self.0.serialize_entry(&key, &value) {
            Ok(()) => ControlFlow::Continue(()),
            Err(e) => ControlFlow::Break(e),
        }
    }
}

struct PrintVisitor;

impl CRCTreeVisitor<PKEntryData> for PrintVisitor {
    type Break = ();

    fn visit(&mut self, crc: u32, data: PKEntryData) -> ControlFlow<()> {
        println!(
            "{:10} {:9} {:9} {} {} {:08x} {:08x}",
            crc,
            data.orig_file_size,
            data.compr_file_size,
            data.orig_file_hash,
            data.compr_file_hash,
            data.is_compressed,
            data.file_data_addr,
        );
        ControlFlow::Continue(())
    }
}

#[derive(FromArgs)]
/// List the entries in a PK file
struct Args {
    #[argh(positional)]
    /// the PK file
    file: PathBuf,

    // #[argh(option)]
    // /// the manifest to use
    // manifest: Option<PathBuf>,
    #[argh(switch)]
    /// output as json
    json: bool,

    #[argh(switch)]
    /// make the output pretty
    pretty: bool,
}

fn visit_entries_json<F>(
    mut entries: PackEntryAccessor<&mut BufReader<File>>,
    make_ser: impl Fn() -> serde_json::ser::Serializer<Stdout, F>,
) -> color_eyre::Result<()>
where
    F: Formatter,
{
    let mut ser = make_ser();
    let seq = ser.serialize_map(None)?;
    let mut jv = JsonVisitor(seq);
    if let ControlFlow::Break(e) = entries.visit(&mut jv)? {
        return Err(color_eyre::Report::from(e));
    }
    jv.0.end()?;
    println!();
    Ok(())
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let args: Args = argh::from_env();

    let file = File::open(&args.file)?;
    let mut reader = BufReader::new(file);
    let mut pack = PackFile::open(&mut reader);

    let header = pack.get_header()?;

    //if let Some(m) = args.manifest {
    //    println!("Manifest: {}", m.display());
    //}

    let mut entries = pack.get_entry_accessor(header.file_list_base_addr)?;
    if args.json {
        if args.pretty {
            visit_entries_json(
                entries,
                || serde_json::Serializer::pretty(std::io::stdout()),
            )?;
        } else {
            visit_entries_json(entries, || serde_json::Serializer::new(std::io::stdout()))?;
        }
    } else {
        entries.visit(&mut PrintVisitor)?;
    }

    Ok(())
}
