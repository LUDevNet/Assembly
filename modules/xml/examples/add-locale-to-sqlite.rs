use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::BufReader,
    path::PathBuf,
    time::Instant,
};

use argh::FromArgs;
use color_eyre::eyre::WrapErr;
use quick_xml::{events::Event as XmlEvent, Reader as XmlReader};
use rusqlite::{Connection, Transaction, TransactionBehavior};

use assembly_xml::common::exact::{
    expect_attribute, expect_child_or_end, expect_end, expect_start, expect_text,
    expect_text_or_end,
};

const TAG_LOCALIZATION: &str = "localization";
const TAG_LOCALES: &str = "locales";
const TAG_LOCALE: &str = "locale";
const TAG_PHRASES: &str = "phrases";
const TAG_PHRASE: &str = "phrase";
const TAG_TRANSLATION: &str = "translation";

const ATTR_COUNT: &str = "count";
const ATTR_LOCALE: &str = "locale";
const ATTR_ID: &str = "id";

fn find_primary_keys(tx: &Transaction) -> rusqlite::Result<HashMap<String, Vec<String>>> {
    let mut table_pks = HashMap::new();

    let mut stmt = tx.prepare("SELECT t.name, c.name from sqlite_master t, pragma_table_info(t.name) c where t.type == 'table' and c.pk > 0")?;
    let mut rows = stmt.query([])?;
    let mut last_table_name = String::new();
    let mut pks = vec![];
    while let Some(row) = rows.next()? {
        let table_name = row.get::<usize, String>(0)?;
        let pk_name = row.get::<usize, String>(1)?;
        if table_name != last_table_name {
            table_pks.insert(last_table_name, pks);
            last_table_name = table_name;
            pks = vec![];
        }
        pks.push(pk_name);
    }
    Ok(table_pks)
}

/// Try to add locale info to an existing converted cdclient SQLite file
///
/// This function does the following:
///
/// 1. `BEGIN`s a transaction
/// 2. For every locale entry matching a specific format:
///   a. If not already done, run ALTER TABLE ADD COLUMN to add the column to the DB
///   b. Runs UPDATE to add the info to the DB
/// 3. `COMMIT`s the transaction
pub fn try_add_locale(
    conn: &mut Connection,
    mut reader: XmlReader<BufReader<File>>,
) -> color_eyre::Result<()> {
    // All data we will be inserting will be strings, so disable the check for string type for better performance
    conn.pragma_update(None, "ignore_check_constraints", true)?;
    let tx = Transaction::new(conn, TransactionBehavior::Exclusive)?;

    let mut buf = Vec::new();

    // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    if let Ok(XmlEvent::Decl(_)) = reader.read_event(&mut buf) {}
    buf.clear();

    let _ = expect_start(TAG_LOCALIZATION, &mut reader, &mut buf)?;
    buf.clear();

    let e_locales = expect_start(TAG_LOCALES, &mut reader, &mut buf)?;
    let locale_count: usize = expect_attribute(ATTR_COUNT, &reader, &e_locales)?;
    let mut real_locale_count = 0;
    buf.clear();

    while expect_child_or_end(TAG_LOCALE, TAG_LOCALES, &mut reader, &mut buf)?.is_some() {
        buf.clear();

        let _locale = expect_text(&mut reader, &mut buf)?;

        expect_end(TAG_LOCALE, &mut reader, &mut buf)?;
        buf.clear();

        real_locale_count += 1;
    }
    buf.clear();

    if real_locale_count != locale_count {
        println!(
            "locale.xml specifies a locale count of {}, but has {}",
            locale_count, real_locale_count
        );
    }

    let e_phrases = expect_start(TAG_PHRASES, &mut reader, &mut buf)?;
    let phrase_count: usize = expect_attribute(ATTR_COUNT, &reader, &e_phrases)?;
    let mut real_phrase_count = 0;
    buf.clear();

    let mut tables: HashMap<String, HashSet<String>> = HashMap::new();
    let table_pks = find_primary_keys(&tx)?;

    if table_pks.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "could not find PK info in DB - is your DB annotated?",
        )
        .into());
    }

    while let Some(e_phrase) = expect_child_or_end(TAG_PHRASE, TAG_PHRASES, &mut reader, &mut buf)?
    {
        let id: String = expect_attribute(ATTR_ID, &reader, &e_phrase)?;
        buf.clear();

        while let Some(e_translation) =
            expect_child_or_end(TAG_TRANSLATION, TAG_PHRASE, &mut reader, &mut buf)?
        {
            let locale: String = expect_attribute(ATTR_LOCALE, &reader, &e_translation)?;
            buf.clear();

            let trans = expect_text_or_end(TAG_TRANSLATION, &mut reader, &mut buf)?;

            let (table, rest) = if let Some((table, rest)) = id.split_once("_") {
                (table, rest)
            } else {
                continue;
            };

            let pks = if let Some(x) = table_pks.get(table) {
                x
            } else {
                continue;
            };

            let mut parts = rest.splitn(pks.len() + 1, "_");
            let mut params = vec![trans.as_ref()];
            for _ in 0..pks.len() {
                params.push(if let Some(x) = parts.next() {
                    x
                } else {
                    continue;
                });
            }
            let column = if let Some(x) = parts.next() {
                x
            } else {
                continue;
            };

            let columns = match tables.get_mut(table) {
                Some(x) => x,
                None => tables.entry(table.to_owned()).or_default(),
            };
            let col_loc = format!("{}_{}", column, locale);
            let update = format!(
                r#"UPDATE "{}" SET "{}" = ? WHERE {}"#,
                table,
                col_loc,
                pks.into_iter()
                    .map(|x| format!(r#""{}" = ?"#, x))
                    .collect::<Vec<_>>()
                    .join(" AND ")
            );

            if !columns.contains(&col_loc) {
                let sql = format!(
                    r#"ALTER TABLE "{}" ADD COLUMN "{}" TEXT4 CHECK (TYPEOF("{}") in ('text', 'null'))"#,
                    table, col_loc, col_loc
                );
                tx.execute(&sql, rusqlite::params![])?;
                columns.insert(col_loc);
            }
            tx.execute(&update, rusqlite::params_from_iter(params))?;
            buf.clear();
        }
        buf.clear();

        real_phrase_count += 1;
    }
    buf.clear();

    if phrase_count != real_phrase_count {
        println!(
            "locale.xml specifies a count of {} phrases, but has {}",
            phrase_count, real_phrase_count
        );
    }
    tx.commit()?;
    Ok(())
}

#[derive(FromArgs)]
/// Turns an FDB file into an equivalent SQLite file
struct Options {
    /// the locale.xml file
    #[argh(positional)]
    locale: PathBuf,
    /// the SQLite DB to augment
    #[argh(positional)]
    sqlite: PathBuf,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let opts: Options = argh::from_env();
    let start = Instant::now();

    println!("Copying data, this may take a few seconds...");

    let mut conn = Connection::open(opts.sqlite)?;

    let file = File::open(&opts.locale)?;
    let file = BufReader::new(file);

    let mut reader = XmlReader::from_reader(file);
    reader.trim_text(true);

    try_add_locale(&mut conn, reader).wrap_err("Failed to add locale info to sqlite")?;

    let duration = start.elapsed();
    println!(
        "Finished in {}.{}s",
        duration.as_secs(),
        duration.subsec_millis()
    );

    Ok(())
}
