//! # The manifest (`*.txt`) files

mod lines;

use std::collections::BTreeMap;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

#[cfg(feature = "async")]
use std::io;

#[cfg(feature = "async")]
use futures_util::{TryStream, TryStreamExt};
use nom_supreme::final_parser::Location;
use thiserror::Error;

use self::lines::{file_line, version_line};
pub use self::lines::{FileLine, VersionLine};
pub use crate::common::FileMeta;

#[cfg(feature = "md5sum")]
use crate::{common::FileMetaPair, md5::MD5Sum};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// A section of the manifest
pub enum Section {
    /// The `[version]` section
    Version,
    /// The `[files]` section
    Files,
}

impl Section {
    fn as_header(&self) -> &'static str {
        match self {
            Self::Files => "[files]",
            Self::Version => "[version]",
        }
    }
}

#[derive(Debug, Error)]
/// An error from parsing a manifest
pub enum Error {
    /// Unexpected EOF, expected a header
    MissingHeader(&'static str),
    /// Unexpected EOF, expected version line
    MissingVersionLine,
    /// Expected a header but found something else
    ExpectedHeader(&'static str, String),
    /// An IO error
    IO(#[from] std::io::Error),
    /// Failed to parse a line
    Nom(#[from] nom::error::Error<Location>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingHeader(h) => write!(f, "Missing '{}' header", h),
            Self::ExpectedHeader(h, line) => write!(f, "Expected '{}' header, got {:?}", h, line),
            Self::MissingVersionLine => write!(f, "Missing version line"),
            Self::IO(_e) => write!(f, "I/O error"),
            Self::Nom(_e) => write!(f, "Parser error"),
        }
    }
}

/// The result type for this module
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(feature = "async")]
async fn expect_header<L>(lines: &mut L, section: Section) -> Result<()>
where
    L: TryStream<Ok = String, Error = io::Error> + Unpin,
{
    let header = section.as_header();
    let res = lines.try_next().await?;
    let line = res.ok_or(Error::MissingHeader(header))?;
    if line != header {
        return Err(Error::ExpectedHeader(header, line));
    }

    Ok(())
}

#[cfg(feature = "async")]
async fn read_index_version<L>(lines: &mut L) -> Result<VersionLine>
where
    L: TryStream<Ok = String, Error = io::Error> + Unpin,
{
    expect_header(lines, Section::Version).await?;
    let line = lines.try_next().await?.ok_or(Error::MissingVersionLine)?;
    let version = version_line(&line)?;
    Ok(version)
}

/// The line ending to use when writing a manifest
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LineEnding {
    /// `\n`
    Lf,
    /// `\r\n`, like the manifests served to the original patcher
    CrLf,
}

impl LineEnding {
    fn as_str(self) -> &'static str {
        match self {
            Self::Lf => "\n",
            Self::CrLf => "\r\n",
        }
    }
}

/// A manifest file in-memory
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Manifest {
    /// The parsed version line
    pub version: VersionLine,
    /// The parsed, sorted list of files
    pub files: BTreeMap<String, FileLine>,
}

fn find_version_line<B: BufRead>(buffer: &mut String, reader: &mut B) -> Result<VersionLine> {
    buffer.clear();
    loop {
        let len = reader.read_line(buffer)?;
        assert_ne!(len, 0);

        if buffer.trim() == Section::Version.as_header() {
            buffer.clear();
            reader.read_line(buffer)?;
            //panic!("{:?}", buffer);
            let line = version_line(buffer.trim())?;
            //panic!("{:?}", &line);
            break Ok(line);
        }
        buffer.clear();
    }
}

fn find_files_lines<B: BufRead>(buffer: &mut String, reader: &mut B) -> Result<()> {
    buffer.clear();
    loop {
        let len = reader.read_line(buffer)?;
        assert_ne!(len, 0);

        if buffer.trim() == Section::Files.as_header() {
            buffer.clear();
            break Ok(());
        }
        buffer.clear();
    }
}

impl Manifest {
    /// Read a manifest from a [BufRead] implementation
    ///
    /// ```
    /// use std::{io::Cursor, collections::BTreeMap};
    /// use assembly_pack::{md5::MD5Sum, txt::{Manifest, VersionLine}};
    ///
    /// let hash = MD5Sum::compute("32");
    /// let text = format!("[version]\n32,{},Name\n[files]\n", hash);
    /// let res = Manifest::from_buf_read(&mut Cursor::new(text));
    ///
    /// match res {
    ///     Err(e) => panic!("{}", e),
    ///     Ok(m) => {
    ///         assert_eq!(m, Manifest {
    ///             version: VersionLine::new(32, String::from("Name")),
    ///             files: BTreeMap::new(),
    ///         });
    ///     }
    /// }
    /// ```
    pub fn from_buf_read<B: BufRead>(reader: &mut B) -> Result<Self> {
        let mut buffer = String::new();
        let mut files = BTreeMap::new();
        let version = find_version_line(&mut buffer, reader)?;
        find_files_lines(&mut buffer, reader)?;

        while reader.read_line(&mut buffer)? > 0 {
            let (name, meta, hash) = file_line(buffer.trim())?;
            files.insert(name.to_string(), (meta, hash));
            buffer.clear();
        }

        Ok(Self { version, files })
    }

    /// Read a Manifest from a file
    pub fn from_file(path: &Path) -> Result<Self> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        Self::from_buf_read(&mut reader)
    }

    /// Add a file, calculating the checksum of its line
    ///
    /// The checksum covers the path as given, so this is also how to change a
    /// path's form (e.g. from `\\` to `/` separators).
    #[cfg(feature = "md5sum")]
    pub fn add_file(&mut self, path: String, meta: FileMetaPair) {
        let line_hash = MD5Sum::compute(&format!("{path},{meta}"));
        self.files.insert(path, (meta, line_hash));
    }

    /// Write this manifest, which [`Manifest::from_buf_read`] reads back
    ///
    /// Every line ends with the line ending, like the original manifests. Files
    /// are ordered by their path with `\\` separators, whichever form the paths
    /// are in.
    ///
    /// ```
    /// use std::{collections::BTreeMap, io::Cursor};
    /// use assembly_pack::txt::{LineEnding, Manifest, VersionLine};
    ///
    /// let manifest = Manifest {
    ///     version: VersionLine::new(32, String::from("Name")),
    ///     files: BTreeMap::new(),
    /// };
    /// let mut text = Vec::new();
    /// manifest.write(&mut text, LineEnding::CrLf).unwrap();
    /// assert_eq!(
    ///     String::from_utf8(text.clone()).unwrap(),
    ///     format!("[version]\r\n{}\r\n[files]\r\n", manifest.version)
    /// );
    /// assert_eq!(Manifest::from_buf_read(&mut Cursor::new(text)).unwrap(), manifest);
    /// ```
    pub fn write<W: Write>(&self, writer: &mut W, ending: LineEnding) -> std::io::Result<()> {
        let eol = ending.as_str();
        let version = Section::Version.as_header();
        let files = Section::Files.as_header();
        write!(writer, "{version}{eol}{}{eol}{files}{eol}", self.version)?;
        let mut lines: Vec<_> = self.files.iter().collect();
        lines.sort_by_cached_key(|(path, _)| path.replace('/', "\\"));
        for (path, (meta, line_hash)) in lines {
            write!(writer, "{path},{meta},{line_hash}{eol}")?;
        }
        Ok(())
    }
}

/// Load the manifest from a stream of lines
#[cfg(feature = "async")]
pub async fn load_manifest<L>(lines: &mut L) -> Result<Manifest>
where
    L: TryStream<Ok = String, Error = io::Error> + Unpin,
{
    let mut files = BTreeMap::new();

    let version = read_index_version(lines).await?;
    expect_header(lines, Section::Files).await?;
    while let Some(item) = lines.try_next().await? {
        let line = item;
        let (filename, meta, hash) = file_line(&line)?;
        files.insert(filename.to_owned(), (meta, hash));
    }

    Ok(Manifest { version, files })
}

#[cfg(all(test, feature = "md5sum"))]
mod write_tests {
    use super::{LineEnding, Manifest, VersionLine};
    use crate::{
        common::{FileMeta, FileMetaPair},
        md5::MD5Sum,
    };
    use std::{collections::BTreeMap, io::Cursor};

    fn meta(text: &str) -> FileMeta {
        FileMeta { size: text.len() as u32, hash: MD5Sum::compute(text) }
    }

    fn sample() -> Manifest {
        let mut manifest = Manifest {
            version: VersionLine::new(82, String::from("1.2.3")),
            files: BTreeMap::new(),
        };
        manifest.add_file("client/b.txt".into(), FileMetaPair::new(meta("b"), meta("bb")));
        manifest.add_file("client/a.txt".into(), FileMetaPair::new(meta("a"), meta("aa")));
        manifest
    }

    #[test]
    fn roundtrips_with_both_line_endings() {
        for ending in [LineEnding::Lf, LineEnding::CrLf] {
            let manifest = sample();
            let mut text = Vec::new();
            manifest.write(&mut text, ending).unwrap();
            let read = Manifest::from_buf_read(&mut Cursor::new(text)).unwrap();
            assert_eq!(read, manifest);
        }
    }

    #[test]
    fn order_does_not_depend_on_the_separator() {
        let order = |a: &str, b: &str| {
            let mut manifest = sample();
            manifest.files.clear();
            manifest.add_file(a.into(), FileMetaPair::new(meta("a"), meta("aa")));
            manifest.add_file(b.into(), FileMetaPair::new(meta("b"), meta("bb")));
            let mut text = Vec::new();
            manifest.write(&mut text, LineEnding::Lf).unwrap();
            let text = String::from_utf8(text).unwrap();
            text.lines().nth(3).unwrap().split(',').next().unwrap().to_owned()
        };
        // '/' < '1' < '\\', so a plain sort differs between the two forms
        assert_eq!(order("client/a/b.txt", "client/a1/c.txt"), "client/a1/c.txt");
        assert_eq!(order("client\\a\\b.txt", "client\\a1\\c.txt"), "client\\a1\\c.txt");
    }

    #[test]
    fn line_checksum_covers_the_first_five_fields_and_lines_are_sorted() {
        let mut text = Vec::new();
        sample().write(&mut text, LineEnding::CrLf).unwrap();
        let text = String::from_utf8(text).unwrap();
        assert!(text.ends_with("\r\n"));
        let lines: Vec<&str> = text.trim_end().split("\r\n").collect();
        assert_eq!(lines.len(), 5);
        assert!(lines[3].starts_with("client/a.txt,"));
        for line in &lines[3..] {
            let (content, sum) = line.rsplit_once(',').unwrap();
            assert_eq!(format!("{}", MD5Sum::compute(content)), sum);
        }
    }
}
