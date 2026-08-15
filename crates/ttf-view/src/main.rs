use color_print::{ceprint, cstr};
use serde::Serialize;
use std::{
    error::Error,
    fmt::Debug,
    io::{Write, stdout},
    process,
};
use ttf_view::{
    tables::{TableDirectoryRepr, TableRecordRepr},
    types::{Tag, tags},
};

enum Action {
    Help,
    Version,
    ListTables,
    Dump,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Format {
    Debug,
    Binary,
    Json,
}

macro_rules! error_exit {
    ($($arg:tt)*) => {{
        ceprint!("<s,r!>error</>: ");
        eprintln!($($arg)*);
        process::exit(1);
    }};
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut font_path = None;
    let mut action = Action::Help;
    let mut format = Format::Debug;
    let mut table_tag = None;

    let mut args = std::env::args();
    args.next(); // skip executable

    while let Some(arg) = args.next() {
        match &*arg {
            "-h" | "--help" => {
                action = Action::Help;
            },
            "-V" | "--version" => {
                action = Action::Version;
            },
            "--list-tables" => {
                action = Action::ListTables;
            },
            "-f" | "--format" => {
                match args.next() {
                    Some(arg) => match &*arg {
                        "dbg" | "debug" => format = Format::Debug,
                        "bin" | "binary" => format = Format::Binary,
                        "json" => format = Format::Json,
                        format => error_exit!("Got an unknown format identifier '{format}'"),
                    },
                    None => error_exit!("Expected a format identifier after '-f'/'--format'"),
                };
            },
            "-t" | "--table" => {
                match args.next() {
                    Some(arg) => match Tag::from_str(&arg) {
                        Ok(tag) => table_tag = Some(tag),
                        Err(tag_error) => error_exit!("Got an invalid table tag ({tag_error})"),
                    },
                    None => error_exit!("Expected a table tag after '-t'/'--table'"),
                };
            },
            thing => {
                if let Some(x) = font_path {
                    error_exit!("Got more than one path in arguments ('{}', '{}')", &x, thing);
                }
                font_path = Some(thing.to_owned());
                action = Action::Dump;
            },
        };
    }

    match action {
        Action::Version => {
            print_version();
            process::exit(0);
        },
        Action::Help => {
            print_help();
            process::exit(0);
        },
        Action::ListTables => {
            print_tables();
            process::exit(0);
        },
        Action::Dump => {
            let Some(font_path) = font_path else { error_exit!("The font file was not specified") };

            let font_data = std::fs::read(font_path).unwrap();
            let dir = unsafe { TableDirectoryRepr::new_unchecked(&font_data) };

            match format {
                Format::Binary => {
                    let data = dump_binary(&font_data, dir, table_tag);
                    stdout().write_all(data).unwrap();
                },
                _ => {
                    let mut writer = stdout().lock();
                    dump_format(dir, table_tag, format, &mut writer)?;
                },
            };
        },
    };

    Ok(())
}

fn dump_binary<'a>(data: &'a Vec<u8>, dir: &'a TableDirectoryRepr, tag: Option<Tag>) -> &'a [u8] {
    match tag {
        Some(tag) => dir.table_record(tag).map_or_default(|t| t.table_as_bytes()),
        None => {
            let dir_size = size_of::<TableDirectoryRepr>()
                + dir.table_records().len() * size_of::<TableRecordRepr>();
            &data[..dir_size]
        },
    }
}

fn dump_format(
    dir: &TableDirectoryRepr,
    tag: Option<Tag>,
    format: Format,
    mut w: impl Write,
) -> Result<(), Box<dyn Error>> {
    let table: &dyn SerializeSpec = match tag {
        None => dir,

        // Some(tags::cmap) => dir.cmap(),
        Some(tags::head) => dir.head(),
        Some(tags::hhea) => dir.hhea(),
        Some(tags::hmtx) => &dir.hmtx(),
        Some(tags::maxp) => dir.maxp(),
        Some(tags::name) => dir.name(),
        Some(tags::OS_2) => &dir.os_2(),

        Some(table_tag @ _) => {
            if Tag::KNOWN_TAGS.contains(&table_tag) {
                error_exit!("error: Dumping the table '{table_tag}' is not supported yet")
            } else {
                error_exit!("error: Could not find a table with tag '{table_tag}'")
            }
        },
    };

    match format {
        Format::Binary => {},
        Format::Debug => {
            write!(w, "{:#?}", table)?;
        },
        Format::Json => {
            table.serialize_json(&mut w)?;
        },
    };
    Ok(())
}

trait SerializeSpec: std::fmt::Debug {
    fn serialize_json(&self, w: &mut dyn Write) -> serde_json::Result<()>;
}
impl<T: Serialize + std::fmt::Debug> SerializeSpec for T {
    fn serialize_json(&self, w: &mut dyn Write) -> serde_json::Result<()> {
        self.serialize(&mut serde_json::Serializer::pretty(w))
    }
}

fn print_version() {
    println!("ttf-view {}", env!("CARGO_PKG_VERSION"));
}

fn print_help() {
    const HELP: &'static str = cstr!(r#"

A TrueType/OpenType font parsing/viewing Rust library, and also a CLI tool.
The project's GitHub repository: https://github.com/Chasmical/ttf-edit

<s><u>Usage:</u> ttf-view</s> [OPTIONS] <<FONT>>

<s,u>Arguments:</>
  <<FONT>>  Path to the OpenType font file to view (.ttf, .otf)

<s,u>Options:</>
  <s>-f, --format</> <<FORMAT>>  The format to dump the table data in [default: debug] (possible values: dbg/debug, json, bin/binary)
  <s>-t, --table</> <<TAG>>      The table to dump (omit to dump the table directory)
  <s>    --list-tables</>      List all supported OpenType tables (binary format always works)
  <s>-h, --help</>             Print help
  <s>-V, --version</>          Print version

"#).trim_ascii();

    eprintln!("{}", HELP);
}

fn print_tables() {
    const TABLES: &'static str = cstr!(
        r#"

Currently only the following OpenType tables can be exported:

<s>head</>  Font Header Table
<s>hhea</>  Horizontal Header Table
<s>hmtx</>  Horizontal Metrics Table
<s>maxp</>  Maximum Profile
<s>name</>  Naming Table
<s>OS/2</>  OS/2 and Windows Metrics Table

Note: <s>bin</> format is always available for any tables.

"#
    )
    .trim_ascii();

// <s>cmap</>  Character Mapping Table

    eprintln!("{}", TABLES);
}
