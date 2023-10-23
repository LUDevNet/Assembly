use nom::{
    bytes::complete::take,
    combinator::{map, map_opt},
    multi::count,
    number::complete::{le_u32, le_u8},
    IResult,
};

use crate::parse_u32_bytes_wstring_nt;

bitflags::bitflags! {
    /// VS Data Services DsRef Type Enum
    ///
    /// See: <https://learn.microsoft.com/en-us/dotnet/api/microsoft.visualstudio.data.services.supportentities.interop.__dsreftype>
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct DsRefType: u32 {
        /// Specifies a collection.
        const COLLECTION = 1;

        /// Specifies a database object.
        const DATABASE = 2048;

        /// Specifies a data source root.
        const DATASOURCEROOT = 16;

        /// Specifies an extended type.
        const EXTENDED = 16384;

        /// Specifies a data field.
        const FIELD = 256;

        /// Specifies a database function.
        const FUNCTION = 16777216;

        /// Specifies that the DSRef object has a first child.
        const HASFIRSTCHILD = 65536;

        /// Specifies that the DSRef object has a moniker.
        const HASMONIKER = 524288;

        /// Specifies that the DSRef object has a name.
        const HASNAME = 262144;

        /// Specifies that the DSRef object has a sibling that can be accessed next.
        const HASNEXTSIBLING = 131072;

        /// Specifies that the DSRef object has an owner.
        const HASOWNER = 2097152;

        /// Specifies that the DSRef object has properties.
        const HASPROP = 4194304;

        /// Specifies a database index.
        const INDEX = 268435456;

        /// Specifies the DSRef object supports mixed mode
        const MIXED = 4;

        /// Specifies a multiple DSRef object.
        const MULTIPLE = 2;

        /// Specifies a generic node.
        const NODE = 0xFF90FF00;  //-7274752;

        /// Specifies a null value (0).
        const NULL = 0;

        /// Specifies a package.
        const PACKAGE = 33554432;

        /// Specifies a package body.
        const PACKAGEBODY = 67108864;

        /// Specifies a query.
        const QUERY = 1024;

        /// Specifies a database relationship object.
        const RELATIONSHIP = 134217728;

        /// The DSRef object.
        const SCHEMADIAGRAM = 32768;

        /// Specifies a stored procedure.
        const STOREDPROCEDURE = 8192;

        /// Specifies a synonym.
        const SYNONYM = 8388608;

        /// Specifies a table.
        const TABLE = 512;

        /// Specifies a trigger.
        const TRIGGER = 4096;

        /// Specifies a user-defined type.
        const USERDEFINEDTYPE = 536870912;

        /// The DSRef object.
        const VIEW = 1048576;

        /// Specifies a database view index.
        const VIEWINDEX = 0x80000000; //-2147483648;

        /// Specifies a database view trigger.
        const VIEWTRIGGER = 1073741824;

    }
}

#[derive(Debug, Clone)]
pub struct DSRefSchemaEntry {
    pub ref_type: DsRefType,
    pub table: String,
    pub schema: String,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct DSRefSchemaContents {
    pub _a0: u32,
    pub _a1: u32,
    pub _d1: Vec<u8>,
    pub connection: String,
    pub(crate) ref_type: DsRefType,
    pub name: String,
    pub tables: Vec<DSRefSchemaEntry>,
    pub(crate) _d4: Vec<u8>,
    pub guid: String,
    //pub settings: HashMap<String, String>,
}

fn parse_entry(input: &[u8]) -> IResult<&[u8], DSRefSchemaEntry> {
    let (input, ref_type) = map_opt(le_u32, DsRefType::from_bits)(input)?;
    let (input, table) = parse_u32_bytes_wstring_nt(input)?;
    let (input, schema) = parse_u32_bytes_wstring_nt(input)?;
    Ok((
        input,
        DSRefSchemaEntry {
            ref_type,
            table,
            schema,
        },
    ))
}

pub fn parse_dsref_schema_contents(input: &[u8]) -> IResult<&[u8], DSRefSchemaContents> {
    let (input, _a0) = le_u32(input)?;
    let (input, _a1) = le_u32(input)?;
    let (input, _d1) = take(17usize)(input)?;
    let (input, len) = map(le_u8, usize::from)(input)?;
    let (input, _d2) = take(26usize)(input)?;
    let (input, connection) = parse_u32_bytes_wstring_nt(input)?;
    //let (input, settings) = map_res(success(connection.clone()), get_settings)(input)?;
    let (input, ref_type) = map_opt(le_u32, DsRefType::from_bits)(input)?;
    let (input, name) = parse_u32_bytes_wstring_nt(input)?;
    let (input, tables) = count(parse_entry, len)(input)?;
    let (input, _d4) = map(take(22usize), Vec::from)(input)?;
    let (input, guid) = parse_u32_bytes_wstring_nt(input)?;
    Ok((
        input,
        DSRefSchemaContents {
            _a0,
            _a1,
            _d1: _d1.to_owned(),
            connection,
            ref_type,
            name,
            tables,
            _d4,
            guid,
        },
    ))
}
