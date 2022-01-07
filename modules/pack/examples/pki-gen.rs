use argh::FromArgs;
use assembly_pack::{
    pki::{self, writer::write_pki_file},
    txt::gen::push_command,
};
use color_eyre::eyre::Context;
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter},
    path::PathBuf,
};

#[derive(FromArgs)]
/// print the entry for a specific CRC in the PKI
struct Args {
    /// the PKI file
    #[argh(positional)]
    generator_config: PathBuf,
}

fn main() -> color_eyre::Result<()> {
    let args: Args = argh::from_env();

    let cfg_file = File::open(&args.generator_config).wrap_err_with(|| {
        format!(
            "Failed to load generator_config file ({})",
            args.generator_config.display()
        )
    })?;

    let directory = std::env::current_dir().unwrap();
    let output = directory.join("primary.pki");
    let manifest = directory.join("trunk.txt");
    let mut config = pki::gen::Config {
        directory,
        output,
        manifest,
        prefix: "client\\res\\".to_string(),
        pack_files: vec![],
    };
    let cfg_reader = BufReader::new(cfg_file);
    for next_line in cfg_reader.lines() {
        let line = next_line.wrap_err("failed to read config line")?;
        if let Some(cmd) = assembly_pack::txt::gen::parse_line(&line) {
            push_command(&mut config, cmd);
        }
    }

    let output = config.output.clone();
    let pki = config.run();

    dbg!(pki.archives.len());
    dbg!(pki.files.len());

    let file =
        File::create(&output).with_context(|| format!("Failed to create {}", output.display()))?;
    let mut writer = BufWriter::new(file);
    write_pki_file(&mut writer, &pki).context("Failed to write PKI file")?;

    Ok(())
}
