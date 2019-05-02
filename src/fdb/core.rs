use std::collections::BTreeMap;

/// Value datatypes used in the database
#[derive(Debug)]
pub enum ValueType {
    Nothing,
    Integer,
    Float,
    Text,
    Boolean,
    BigInt,
    VarChar,
    Unknown(u32),
}

impl From<ValueType> for u32 {
    fn from(value_type: ValueType) -> u32 {
        match value_type {
            ValueType::Nothing => 0,
            ValueType::Integer => 1,
            ValueType::Float => 3,
            ValueType::Text => 4,
            ValueType::Boolean => 5,
            ValueType::BigInt => 6,
            ValueType::VarChar => 8,
            ValueType::Unknown(key) => key,
        }
    }
}

impl From<u32> for ValueType {
    fn from(value_type: u32) -> ValueType {
        match value_type {
            0 => ValueType::Nothing,
            1 => ValueType::Integer,
            3 => ValueType::Float,
            4 => ValueType::Text,
            5 => ValueType::Boolean,
            6 => ValueType::BigInt,
            8 => ValueType::VarChar,
            k => ValueType::Unknown(k),
        }
    }
}

/// A database single field
#[derive(Debug)]
pub enum Field {
    Nothing,
    Integer(i32),
    Float(f32),
    Text(String),
    Boolean(bool),
    BigInt(i64),
    VarChar(String),
}

impl From<Field> for ValueType {
    fn from(val: Field) -> Self {
        match val {
            Field::Nothing => ValueType::Nothing,
            Field::Integer(_) => ValueType::Integer,
            Field::Float(_) => ValueType::Float,
            Field::Text(_) => ValueType::Text,
            Field::Boolean(_) => ValueType::Boolean,
            Field::BigInt(_) => ValueType::BigInt,
            Field::VarChar(_) => ValueType::VarChar,
        }
    }
}

/// A sequence of fields
#[derive(Debug)]
pub struct Row(Vec<Field>);

impl From<Vec<Field>> for Row {
    fn from(fields: Vec<Field>) -> Self {
        Row(fields)
    }
}

impl Row {
    #[allow(dead_code)]
    pub fn new() -> Row {
        Row(Vec::new())
    }

    pub fn fields(self) -> Vec<Field> {
        self.0
    }
}

/// A container of rows with the same hash value
pub struct Bucket(pub Vec<Row>);

impl Bucket {
    pub fn new() -> Bucket {
        Bucket(Vec::new())
    }

    pub fn rows(self) -> Vec<Row> {
        self.0
    }
}

/// Name and default type for one field in each row
#[derive(Debug)]
pub struct Column {
    name: String,
    field_type: ValueType,
}

impl From<(&str, ValueType)> for Column {
    fn from(data: (&str, ValueType)) -> Self {
        Column { name: String::from(data.0), field_type: data.1 }
    }
}

/// A list of buckets and thus collection of rows with a name
#[allow(dead_code)]
pub struct Table {
    name: String,
    buckets: Vec<Bucket>,
    columns: Vec<Column>,
}

impl Table {
    pub fn new(name: String, buckets: Vec<Bucket>, columns: Vec<Column>) -> Self {
        Table {
            name: name,
            buckets: buckets,
            columns: columns,
        }
    }

    pub fn buckets(self) -> Vec<Bucket> {
        self.buckets
    }

    pub fn columns(self) -> Vec<Column> {
        self.columns
    }

    pub fn name(self) -> String {
        self.name
    }
}

/// A collection of tables
pub struct Schema {
    tables: BTreeMap<String, Table>,
}

impl Schema {
    pub fn new() -> Schema {
        Schema {
            tables: BTreeMap::new(),
        }
    }

    pub fn table(&self, name: &str) -> Option<&Table> {
        self.tables.get(name)
    }
}

impl From<Vec<Table>> for Schema {
    fn from(tables: Vec<Table>) -> Self {
        let mut tree = BTreeMap::new();
        for table in tables {
            tree.insert(table.name.clone(), table);
        }
        Schema { tables: tree }
    }
}
