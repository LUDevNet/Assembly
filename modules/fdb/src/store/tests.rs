use crate::mem;

use super::super::core;
use super::*;

#[test]
fn test_write_empty() {
    let mut out = Vec::new();
    let db = Database::new();
    db.write(&mut out).unwrap();
    let cmp: &[u8] = &[0, 0, 0, 0, 8, 0, 0, 0];
    assert_eq!(&out[..], cmp);
}

#[test]
fn test_write_table_without_columns() {
    let mut out = Vec::new();
    let mut db = Database::new();
    db.push_table(Latin1String::encode("Foobar"), Table::new(0));
    db.write(&mut out).unwrap();
    let cmp: &'static [u8] = &[
        1, 0, 0, 0, 8, 0, 0, 0, // FDBHeader
        16, 0, 0, 0, 36, 0, 0, 0, // FDBTableHeader
        0, 0, 0, 0, 28, 0, 0, 0, 28, 0, 0, 0, // FDBTableDefHeader
        b'F', b'o', b'o', b'b', b'a', b'r', 0, 0, // Name
        0, 0, 0, 0, 44, 0, 0, 0, // FDBTableDataHeader
    ];
    assert_eq!(&out[..], cmp);

    let odb = mem::Database::new(&out);
    let otb = odb.tables().unwrap();
    assert_eq!(otb.len(), 1);
    let foobar = otb.get(0).expect("table #0").expect("table load");
    assert_eq!(foobar.column_count(), 0);
    assert_eq!(foobar.name(), "Foobar");
}

#[test]
fn test_write_table_with_column() {
    let mut out = Vec::new();
    let mut table = Table::new(0);
    table.push_column(Latin1String::encode("foo"), ValueType::Integer);
    let mut db = Database::new();
    db.push_table(Latin1String::encode("Foobar"), table);
    db.write(&mut out).unwrap();
    let cmp: &'static [u8] = &[
        1, 0, 0, 0, 8, 0, 0, 0, // FDBHeader
        16, 0, 0, 0, 48, 0, 0, 0, // FDBTableHeader
        1, 0, 0, 0, 36, 0, 0, 0, 28, 0, 0, 0, // FDBTableDefHeader
        1, 0, 0, 0, 44, 0, 0, 0, // FDBColumnHeader
        b'F', b'o', b'o', b'b', b'a', b'r', 0, 0, // table `Foobar`
        b'f', b'o', b'o', 0, // column `Foobar`.`foo`
        0, 0, 0, 0, 56, 0, 0, 0, // FDBTableDataHeader
    ];
    assert_eq!(&out[..], cmp);

    let odb = mem::Database::new(&out);
    let otb = odb.tables().unwrap();
    assert_eq!(otb.len(), 1);
    let foobar = otb.get(0).expect("table #0").expect("table load");
    assert_eq!(foobar.name(), "Foobar");
    let columns: Vec<_> = foobar.column_iter().collect();
    assert_eq!(columns.len(), 1);
    assert_eq!(columns[0].name(), "foo");
}

#[test]
fn test_write_table_with_columns() {
    let mut out = Vec::new();
    let mut table = Table::new(0);
    table.push_column(Latin1String::encode("foo"), ValueType::Integer);
    table.push_column(Latin1String::encode("bar"), ValueType::Boolean);
    let mut db = Database::new();
    db.push_table(Latin1String::encode("Foobar"), table);
    db.write(&mut out).unwrap();
    let cmp: &'static [u8] = &[
        1, 0, 0, 0, 8, 0, 0, 0, // FDBHeader
        16, 0, 0, 0, 60, 0, 0, 0, // FDBTableHeader
        2, 0, 0, 0, 44, 0, 0, 0, 28, 0, 0, 0, // FDBTableDefHeader
        1, 0, 0, 0, 52, 0, 0, 0, // FDBColumnHeader
        5, 0, 0, 0, 56, 0, 0, 0, // FDBColumnHeader
        b'F', b'o', b'o', b'b', b'a', b'r', 0, 0, // table `Foobar`
        b'f', b'o', b'o', 0, // column `Foobar`.`foo`
        b'b', b'a', b'r', 0, // column `Foobar`.`bar`
        0, 0, 0, 0, 68, 0, 0, 0, // FDBTableDataHeader
    ];
    assert_eq!(&out[..], cmp);

    let odb = mem::Database::new(&out);
    let otb = odb.tables().unwrap();
    assert_eq!(otb.len(), 1);
    let foobar = otb.get(0).expect("table #0").expect("table load");
    assert_eq!(foobar.name(), "Foobar");
    let columns: Vec<_> = foobar.column_iter().collect();
    assert_eq!(columns.len(), 2);
    assert_eq!(columns[0].name(), "foo");
    assert_eq!(columns[1].name(), "bar");
}

#[test]
fn test_write_tables_with_columns() {
    let mut out = Vec::new();

    // Create first table
    let mut table0 = Table::new(0);
    table0.push_column(Latin1String::encode("foo"), ValueType::Integer);
    table0.push_column(Latin1String::encode("bar"), ValueType::Boolean);

    // Create second table
    let mut table1 = Table::new(0);
    table1.push_column(Latin1String::encode("ID"), ValueType::Integer);
    table1.push_column(Latin1String::encode("displayName"), ValueType::Text);

    // Create the database
    let mut db = Database::new();
    db.push_table(Latin1String::encode("Foobar"), table0);
    db.push_table(Latin1String::encode("Players"), table1);

    db.write(&mut out).unwrap();
    let cmp: &'static [u8] = &[
        2, 0, 0, 0, 8, 0, 0, 0, // FDBHeader
        24, 0, 0, 0, 68, 0, 0, 0, // FDBTableHeader
        76, 0, 0, 0, 128, 0, 0, 0, // FDBTableHeader
        2, 0, 0, 0, 52, 0, 0, 0, 36, 0, 0, 0, // FDBTableDefHeader
        1, 0, 0, 0, 60, 0, 0, 0, // FDBColumnHeader
        5, 0, 0, 0, 64, 0, 0, 0, // FDBColumnHeader
        b'F', b'o', b'o', b'b', b'a', b'r', 0, 0, // table `Foobar`
        b'f', b'o', b'o', 0, // column `Foobar`.`foo`
        b'b', b'a', b'r', 0, // column `Foobar`.`bar`
        0, 0, 0, 0, 76, 0, 0, 0, // FDBTableDataHeader
        2, 0, 0, 0, 104, 0, 0, 0, 88, 0, 0, 0, // FDBTableDefHeader
        1, 0, 0, 0, 112, 0, 0, 0, // FDBColumnHeader
        4, 0, 0, 0, 116, 0, 0, 0, // FDBColumnHeader
        b'P', b'l', b'a', b'y', b'e', b'r', b's', 0, // table `Players`
        b'I', b'D', 0, 0, // column `Foobar`.`foo`
        b'd', b'i', b's', b'p', b'l', b'a', b'y', b'N', b'a', b'm', b'e',
        0, // column `Foobar`.`displayName`
        0, 0, 0, 0, 136, 0, 0, 0, // FDBTableDataHeader
    ];
    assert_eq!(&out[..], cmp);

    let odb = mem::Database::new(&out);
    let otb = odb.tables().unwrap();
    assert_eq!(otb.len(), 2);

    let foobar = otb.get(0).expect("table #0").expect("table load");
    assert_eq!(foobar.name(), "Foobar");
    let columns: Vec<_> = foobar.column_iter().collect();
    assert_eq!(columns.len(), 2);
    assert_eq!(columns[0].name(), "foo");
    assert_eq!(columns[1].name(), "bar");

    let players = otb.get(1).expect("table #1").expect("table load");
    assert_eq!(players.name(), "Players");
    let columns: Vec<_> = players.column_iter().collect();
    assert_eq!(columns.len(), 2);
    assert_eq!(columns[0].name(), "ID");
    assert_eq!(columns[1].name(), "displayName");
}

#[test]
fn test_write_table_with_data() {
    let mut out = Vec::new();

    // Create first table
    let mut table0 = Table::new(2);
    table0.push_column(Latin1String::encode("foo"), ValueType::Integer);
    table0.push_column(Latin1String::encode("bar"), ValueType::Boolean);

    table0.push_row(10, &[core::Field::Integer(200), core::Field::Boolean(true)]);
    table0.push_row(12, &[core::Field::Integer(250), core::Field::Boolean(true)]);
    table0.push_row(
        14,
        &[core::Field::Integer(100), core::Field::Boolean(false)],
    );

    table0.push_row(
        17,
        &[core::Field::Integer(123), core::Field::Boolean(false)],
    );
    table0.push_row(21, &[core::Field::Integer(456), core::Field::Boolean(true)]);

    let mut db = Database::new();
    db.push_table(Latin1String::encode("Foobar"), table0);
    db.write(&mut out).unwrap();

    let odb = mem::Database::new(&out);
    let otb = odb.tables().unwrap();
    assert_eq!(otb.len(), 1);

    let foobar = otb.get(0).expect("table #0").expect("table load");
    assert_eq!(foobar.name(), "Foobar");
    let columns: Vec<_> = foobar.column_iter().collect();
    assert_eq!(columns.len(), 2);
    assert_eq!(columns[0].name(), "foo");
    assert_eq!(columns[1].name(), "bar");

    let mut buckets = foobar.bucket_iter();

    fn map_row(r: mem::Row) -> Vec<mem::Field> {
        r.field_iter().collect::<Vec<_>>()
    }

    let bucket0 = buckets.next().expect("bucket#0");
    let mut rows0 = bucket0.row_iter();
    assert_eq!(
        rows0.next().map(map_row),
        Some(vec![mem::Field::Integer(200), mem::Field::Boolean(true)])
    );
    assert_eq!(
        rows0.next().map(map_row),
        Some(vec![mem::Field::Integer(250), mem::Field::Boolean(true)])
    );
    assert_eq!(
        rows0.next().map(map_row),
        Some(vec![mem::Field::Integer(100), mem::Field::Boolean(false)])
    );
    assert!(rows0.next().is_none());

    let bucket1 = buckets.next().expect("bucket#1");
    let mut rows1 = bucket1.row_iter();
    assert_eq!(
        rows1.next().map(map_row),
        Some(vec![mem::Field::Integer(123), mem::Field::Boolean(false)])
    );
    assert_eq!(
        rows1.next().map(map_row),
        Some(vec![mem::Field::Integer(456), mem::Field::Boolean(true)])
    );
    assert!(rows1.next().is_none());

    assert!(buckets.next().is_none());
}

#[test]
fn test_write_tables_with_data() {
    let mut out = Vec::new();

    // Create first table
    let mut table0 = Table::new(2);
    table0.push_column(Latin1String::encode("foo"), ValueType::Integer);
    table0.push_column(Latin1String::encode("bar"), ValueType::Boolean);

    table0.push_row(10, &[core::Field::Integer(200), core::Field::Boolean(true)]);
    table0.push_row(12, &[core::Field::Integer(250), core::Field::Boolean(true)]);
    table0.push_row(
        14,
        &[core::Field::Integer(100), core::Field::Boolean(false)],
    );

    table0.push_row(
        17,
        &[core::Field::Integer(123), core::Field::Boolean(false)],
    );
    table0.push_row(21, &[core::Field::BigInt(456), core::Field::Boolean(true)]);

    // Create a second table
    let mut table1 = Table::new(4);
    table1.push_column(Latin1String::encode("ID"), ValueType::Integer);
    table1.push_column(Latin1String::encode("displayName"), ValueType::Text);

    table1.push_row(
        3,
        &[
            core::Field::Integer(3),
            core::Field::Text(String::from("Hello World!")),
        ],
    );

    // Create the database
    let mut db = Database::new();
    db.push_table(Latin1String::encode("Foobar"), table0);
    db.push_table(Latin1String::encode("Players"), table1);

    // Write to the buffer
    db.write(&mut out).unwrap();

    // Test the result
    let odb = mem::Database::new(&out);
    let otb = odb.tables().unwrap();
    assert_eq!(otb.len(), 2);

    let foobar = otb.get(0).expect("table #0").expect("table load");
    assert_eq!(foobar.name(), "Foobar");
    let columns: Vec<_> = foobar.column_iter().collect();
    assert_eq!(columns.len(), 2);
    assert_eq!(columns[0].name(), "foo");
    assert_eq!(columns[1].name(), "bar");

    let mut buckets = foobar.bucket_iter();

    fn map_row(r: mem::Row) -> Vec<mem::Field> {
        r.field_iter().collect::<Vec<_>>()
    }

    let bucket0 = buckets.next().expect("bucket#0");
    let mut rows0 = bucket0.row_iter();
    assert_eq!(
        rows0.next().map(map_row),
        Some(vec![mem::Field::Integer(200), mem::Field::Boolean(true)])
    );
    assert_eq!(
        rows0.next().map(map_row),
        Some(vec![mem::Field::Integer(250), mem::Field::Boolean(true)])
    );
    assert_eq!(
        rows0.next().map(map_row),
        Some(vec![mem::Field::Integer(100), mem::Field::Boolean(false)])
    );
    assert!(rows0.next().is_none());

    let bucket1 = buckets.next().expect("bucket#1");
    let mut rows1 = bucket1.row_iter();
    assert_eq!(
        rows1.next().map(map_row),
        Some(vec![mem::Field::Integer(123), mem::Field::Boolean(false)])
    );
    assert_eq!(
        rows1.next().map(map_row),
        Some(vec![mem::Field::BigInt(456), mem::Field::Boolean(true)])
    );
    assert!(rows1.next().is_none());

    assert!(buckets.next().is_none());

    let players = otb.get(1).expect("table #1").expect("table load");
    assert_eq!(players.name(), "Players");
    let columns: Vec<_> = players.column_iter().collect();
    assert_eq!(columns.len(), 2);
    assert_eq!(columns[0].name(), "ID");
    assert_eq!(columns[1].name(), "displayName");

    assert_eq!(4, players.bucket_count());
    assert_eq!(Some(true), players.bucket_at(0).map(|b| b.is_empty()));
    assert_eq!(Some(true), players.bucket_at(1).map(|b| b.is_empty()));
    assert_eq!(Some(true), players.bucket_at(2).map(|b| b.is_empty()));
    let bucket3 = players.bucket_at(3).unwrap();
    assert!(!bucket3.is_empty());
    let mut rows3 = bucket3.row_iter();
    assert_eq!(
        rows3.next().map(map_row),
        Some(vec![
            mem::Field::Integer(3),
            mem::Field::Text(unsafe { Latin1Str::from_bytes_unchecked(b"Hello World!") }),
        ])
    );
    assert!(rows3.next().is_none());
}
