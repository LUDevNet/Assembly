use argh::FromArgs;
use assembly_pack::{
    common::fs::{scan_dir, FileInfo, FsVisitor},
    crc::calculate_crc,
    sd0::fs::Converter,
    txt::VersionLine,
};
use color_eyre::eyre::Context;
use std::{ffi::OsStr, path::PathBuf};

#[derive(FromArgs)]
/// print the entry for a specific CRC in the PKI
struct Args {
    /// the directory to scan for files
    #[argh(positional)]
    path: PathBuf,

    /// the directory of the cache
    #[argh(option)]
    output: Option<PathBuf>,

    /// a prefix to names
    #[argh(option, default = "String::new()")]
    prefix: String,

    /// name of the patcher directory
    #[argh(option, default = "String::from(\"luclient\")")]
    patcherdir: String,

    /// don't ignore pk files
    #[argh(switch, short = 'i')]
    include_pk: bool,
}

struct Visitor {
    conv: Converter,
    output: PathBuf,
    ignore_pk: bool,
}

impl FsVisitor for Visitor {
    fn visit_file(&mut self, info: FileInfo) {
        let input = info.real();
        if self.ignore_pk && input.extension() == Some(OsStr::new("pk")) {
            return;
        }

        let path = info.path();
        let crc = calculate_crc(path.as_bytes());

        let mut output = self.output.join(crc.to_string());
        output.set_extension("sd0");
        let line = self
            .conv
            .convert_file(input, &output)
            .wrap_err_with(|| {
                format!(
                    "Error converting {} to {}",
                    input.display(),
                    output.display()
                )
            })
            .unwrap();

        let outpath = self.output.join(line.to_path());
        std::fs::create_dir_all(outpath.parent().unwrap()).unwrap();

        std::fs::rename(&output, &outpath).unwrap();

        println!("{},{}", path, line);
    }
}

fn main() -> color_eyre::Result<()> {
    let args: Args = argh::from_env();
    let mut output = args
        .output
        .unwrap_or_else(|| std::env::current_dir().unwrap());
    output.push(args.patcherdir);

    std::fs::create_dir_all(&output).wrap_err("Failed to create output dir")?;

    let mut visitor = Visitor {
        conv: Converter {
            generate_segment_index: false,
        },
        output,
        ignore_pk: !args.include_pk,
    };

    println!("[version]");
    let version = 90;
    let vline = VersionLine::new(version, format!("LUX.{}", version));
    println!("{}", vline);

    println!("[files]");
    scan_dir(&mut visitor, args.prefix, &args.path, true);
    Ok(())
}
