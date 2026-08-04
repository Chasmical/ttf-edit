use std::fs;
use ttf_view::tables::TableDirectoryRepr;

fn main() {
    let path = r#"D:\repos\flag-emojis-for-windows\build\Segoe.UI.Emoji.with.Twemoji.Flags.ttf"#;

    let font_data = fs::read(path).unwrap();

    let dir = unsafe { TableDirectoryRepr::new_unchecked(&font_data) };

    // println!("{:#?}", dir);

    // println!("{:#?}", dir.head());
    // println!("{:#?}", dir.hhea());
    // println!("{:#?}", dir.maxp());
    println!("{:#?}", dir.name());
}
