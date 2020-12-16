use assembly_core::displaydoc::Display;
use color_eyre::eyre::WrapErr;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};
use thiserror::Error;

use quick_xml::{events::Event, Reader};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// Prints the names of all tables and their columns
struct Options {
    /// The FDB file
    file: PathBuf,
}

fn expect_decl<B: BufRead>(xml: &mut Reader<B>, buf: &mut Vec<u8>) -> quick_xml::Result<()> {
    loop {
        match xml.read_event(buf)? {
            Event::Text(_) => {}
            Event::Decl(_) => {
                buf.clear();
                break Ok(());
            }
            _ => panic!(),
        }
        buf.clear();
    }
}

#[derive(Debug, Display, Error)]
enum XmlError {
    /// Failed to read the next XML event
    Reader(#[from] quick_xml::Error),
    /// Reached EOF while searching for {0}
    EofWhileExpecting(&'static str),
}

fn expect_database<B: BufRead>(
    xml: &mut Reader<B>,
    buf: &mut Vec<u8>,
) -> Result<Option<String>, XmlError> {
    expect_named_elem(xml, buf, "database", None)
}

fn expect_table<B: BufRead>(
    xml: &mut Reader<B>,
    buf: &mut Vec<u8>,
) -> Result<Option<String>, XmlError> {
    expect_named_elem(xml, buf, "table", Some("database"))
}

fn expect_columns<B: BufRead>(xml: &mut Reader<B>, buf: &mut Vec<u8>) -> Result<(), XmlError> {
    expect_elem(xml, buf, "columns")
}

fn expect_rows<B: BufRead>(xml: &mut Reader<B>, buf: &mut Vec<u8>) -> Result<(), XmlError> {
    expect_elem(xml, buf, "rows")
}

struct Column {
    name: String,
    data_type: String,
}

fn expect_column_or_end_columns<B: BufRead>(
    xml: &mut Reader<B>,
    buf: &mut Vec<u8>,
) -> Result<Option<Column>, XmlError> {
    loop {
        match xml.read_event(buf)? {
            Event::Text(_) => {}
            Event::Empty(start) => {
                if start.name() == b"column" {
                    let mut name = None;
                    let mut data_type = None;
                    for attr in start.attributes() {
                        let attr = attr?;
                        if attr.key == b"name" {
                            name = Some(xml.decode(&attr.value).into_owned());
                        }

                        if attr.key == b"type" {
                            data_type = Some(xml.decode(&attr.value).into_owned());
                        }
                    }
                    buf.clear();
                    break Ok(Some(Column {
                        name: name.unwrap(),
                        data_type: data_type.unwrap(),
                    }));
                } else {
                    todo!();
                }
            }
            Event::End(v) => {
                assert_eq!(v.name(), b"columns");
                return Ok(None);
            }
            Event::Eof => return Err(XmlError::EofWhileExpecting("column")),
            x => panic!("What? {:?}", x),
        }
        buf.clear();
    }
}

fn expect_row_or_end_rows<B: BufRead>(
    xml: &mut Reader<B>,
    buf: &mut Vec<u8>,
) -> Result<Option<HashMap<String, String>>, XmlError> {
    loop {
        match xml.read_event(buf)? {
            Event::Text(_) => {}
            Event::Empty(start) => {
                if start.name() == b"row" {
                    let /*mut*/ map = HashMap::new();
                    /*for attr in start.attributes() {
                        let attr = attr?;
                        let key = xml.decode(&attr.key).into_owned();
                        let value = xml.decode(&attr.value).into_owned();
                        map.insert(key, value);
                    }*/
                    buf.clear();
                    break Ok(Some(map));
                } else {
                    todo!();
                }
            }
            Event::End(v) => {
                assert_eq!(v.name(), b"rows");
                return Ok(None);
            }
            Event::Eof => return Err(XmlError::EofWhileExpecting("row")),
            x => panic!("What? {:?}", x),
        }
        buf.clear();
    }
}

fn expect_named_elem<B: BufRead>(
    xml: &mut Reader<B>,
    buf: &mut Vec<u8>,
    key: &'static str,
    parent: Option<&'static str>,
) -> Result<Option<String>, XmlError> {
    loop {
        match xml.read_event(buf)? {
            Event::Text(_) => {}
            Event::Start(start) => {
                if start.name() == key.as_bytes() {
                    let mut name = String::new();
                    for attr in start.attributes() {
                        let attr = attr?;
                        if attr.key == b"name" {
                            name = xml.decode(&attr.value).into_owned();
                            break;
                        }
                    }
                    buf.clear();
                    break Ok(Some(name));
                } else {
                    todo!();
                }
            }
            Event::End(e) => {
                assert_eq!(e.name(), parent.unwrap().as_bytes());
                buf.clear();
                return Ok(None);
            }
            Event::Eof => return Err(XmlError::EofWhileExpecting(key)),
            _ => panic!(),
        }
        buf.clear();
    }
}

fn expect_elem<B: BufRead>(
    xml: &mut Reader<B>,
    buf: &mut Vec<u8>,
    key: &'static str,
) -> Result<(), XmlError> {
    loop {
        match xml.read_event(buf)? {
            Event::Text(_) => {}
            Event::Start(start) => {
                if start.name() == key.as_bytes() {
                    buf.clear();
                    break Ok(());
                } else {
                    todo!();
                }
            }
            Event::Eof => return Err(XmlError::EofWhileExpecting(key)),
            _ => panic!(),
        }
        buf.clear();
    }
}

fn expect_end<B: BufRead>(
    xml: &mut Reader<B>,
    buf: &mut Vec<u8>,
    key: &'static str,
) -> Result<(), XmlError> {
    loop {
        match xml.read_event(buf)? {
            Event::Text(_) => {}
            Event::End(end) => {
                if end.name() == key.as_bytes() {
                    buf.clear();
                    break Ok(());
                } else {
                    todo!();
                }
            }
            Event::Eof => return Err(XmlError::EofWhileExpecting(key)),
            _ => panic!(),
        }
        buf.clear();
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let opts = Options::from_args();

    let src_file = File::open(&opts.file)
        .wrap_err_with(|| format!("Failed to open input file '{}'", opts.file.display()))?;
    let reader = BufReader::new(src_file);

    let mut xml = Reader::from_reader(reader);
    let xml = &mut xml;

    let mut buf = Vec::new();
    let buf = &mut buf;

    expect_decl(xml, buf)?;
    let db_name = expect_database(xml, buf)?.unwrap();
    println!("Loading database: '{}'", db_name);

    while let Some(table_name) = expect_table(xml, buf)? {
        println!("table '{}'", table_name);

        expect_columns(xml, buf)?;

        while let Some(col) = expect_column_or_end_columns(xml, buf)? {
            println!("column '{}' ({})", col.name, col.data_type);
        }

        expect_rows(xml, buf)?;

        while let Some(_row) = expect_row_or_end_rows(xml, buf)? {
            //println!("row {:?}", row);
        }

        expect_end(xml, buf, "table")?;
    }

    Ok(())
}
