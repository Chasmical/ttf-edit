use clap::{CommandFactory, FromArgMatches, Parser, ValueEnum, error::ErrorKind};
use std::{
    fmt::Debug,
    io::{Write, stdout},
    path::PathBuf,
    process,
};
use ttf_view::{
    tables::{TableDirectoryRepr, TableRecordRepr},
    types::{Tag, tags},
};

const ABOUT: &'static str = r#"
A TrueType/OpenType font parsing/viewing Rust library, and also a CLI tool.
The project's GitHub repository: https://github.com/Chasmical/ttf-edit
"#
.trim_ascii();

#[derive(Debug, Parser)]
#[command(name = "ttf-view", version, about = ABOUT, arg_required_else_help = true)]
struct Args {
    /// Path to the OpenType font file to view (.ttf, .otf)
    #[arg(name = "FONT")]
    font_path: PathBuf,

    /// The format to dump the table data in
    #[arg(short, long, value_enum, default_value_t)]
    format: DumpFormat,
    /// The table to dump (omit to dump the table directory)
    #[arg(short, long = "table", name = "TAG")]
    table_tag: Option<Tag>,

    /// List all supported OpenType tables (bin format always works)
    #[arg(long)]
    list_tables: bool,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum DumpFormat {
    #[default]
    #[value(alias("dbg"))]
    Debug,
    #[value(alias("binary"))]
    Bin,
    // TODO: Add json and xml formats, via serde
    // TODO: Add ttx format (that works with fonttools ttx)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args_os: Vec<_> = std::env::args_os().collect();

    // Intercept "--list-tables" here, since clap demands that <FONT> is present,
    // and I don't want the path to the font to be displayed as [FONT] in help.
    if args_os.iter().any(|x| x == "--list-tables") {
        let supported_tables = [
            // (tags::cmap, "Character to Glyph Index Mapping Table"),
            (tags::head, "Font Header Table"),
            (tags::hhea, "Horizontal Header Table"),
            // (tags::hmtx, "Horizontal Metrics Table"),
            (tags::hmtx, "Horizontal Metrics Table"),
            (tags::maxp, "Maximum Profile"),
            (tags::name, "Naming Table"),
        ];
        // TODO: Figure out what formats the tables support via TypeId

        println!(
            "Currently only the following {} OpenType tables can be exported in `debug` format:\n",
            supported_tables.len(),
        );

        for (tag, name) in supported_tables {
            println!("* {:.6} - {}", tag, name);
        }

        println!("\nNote: `bin` format is always available for any tables.");
        return Ok(());
    }

    let mut command = Args::command();

    let args = match command.try_get_matches_from_mut(args_os) {
        Ok(mut matches) => match Args::from_arg_matches_mut(&mut matches) {
            Ok(args) => args,
            Err(err) => err.exit(),
        },
        Err(err) => err.exit(),
    };
    if &args.font_path == "help" {
        command.print_help().unwrap();
        return Ok(());
    }

    let Args { font_path, format, table_tag, .. } = args;

    let font_data = std::fs::read(font_path).unwrap();
    let dir = unsafe { TableDirectoryRepr::new_unchecked(&font_data) };

    if format == DumpFormat::Bin {
        let data = match table_tag {
            Some(tag) => dir.table_record(tag).map_or_default(|t| t.data(dir)),
            None => {
                let dir_size = size_of::<TableDirectoryRepr>()
                    + dir.table_records().len() * size_of::<TableRecordRepr>();
                &font_data[..dir_size]
            },
        };
        stdout().write_all(data).unwrap();
        return Ok(());
    }

    let table: &dyn Debug = match table_tag {
        None => dir,

        // Some(tags::cmap) => dir.cmap(),
        Some(tags::head) => dir.head(),
        Some(tags::hhea) => dir.hhea(),
        // Some(tags::hmtx) => dir.hmtx(),
        Some(tags::maxp) => dir.maxp(),
        Some(tags::name) => dir.name(),

        Some(table_tag @ _) => {
            let msg = if Tag::KNOWN_TAGS.contains(&table_tag) {
                format!("Dumping the table '{}' is not supported yet", table_tag)
            } else {
                format!("Unknown table tag '{}'", table_tag)
            };
            command.error(ErrorKind::InvalidValue, msg).print().unwrap();
            process::exit(1);
        },
    };

    match format {
        DumpFormat::Bin => unreachable!(), // was handled earlier
        DumpFormat::Debug => println!("{:#?}", table),
    };

    Ok(())
}
