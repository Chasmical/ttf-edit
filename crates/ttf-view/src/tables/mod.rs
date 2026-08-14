use crate::types::Tag;

mod directory;
pub use directory::*;

pub mod cmap;
pub mod head;
pub mod hhea;
pub mod hmtx;
pub mod maxp;
pub mod name;
pub mod os_2;

pub trait Table {
    const TAG: Tag;
    type Handle<'a>: TableHandle<'a>;
    fn in_directory(dir: &TableDirectoryRepr) -> Option<Self::Handle<'_>> {
        Self::Handle::in_directory(dir)
    }
}
pub trait TableHandle<'a>: Sized {
    fn in_directory(dir: &'a TableDirectoryRepr) -> Option<Self>;
}
impl<'a, T: Table> TableHandle<'a> for &'a T {
    fn in_directory(dir: &'a TableDirectoryRepr) -> Option<Self> {
        dir.table_raw::<T>()
    }
}

impl TableDirectoryRepr {
    // Note: These are all required tables, so we'll panic on their absence.
    pub fn cmap(&self) -> &cmap::CmapTableRepr {
        self.table::<cmap::CmapTableRepr>().unwrap()
    }
    pub fn head(&self) -> &head::HeadTableRepr {
        self.table::<head::HeadTableRepr>().unwrap()
    }
    pub fn hhea(&self) -> &hhea::HheaTableRepr {
        self.table::<hhea::HheaTableRepr>().unwrap()
    }
    pub fn hmtx(&self) -> hmtx::HmtxTableHandle<'_> {
        self.table::<hmtx::HmtxTableRepr>().unwrap()
    }
    pub fn maxp(&self) -> &maxp::MaxpTableRepr {
        self.table::<maxp::MaxpTableRepr>().unwrap()
    }
    pub fn name(&self) -> &name::NameTableRepr {
        self.table::<name::NameTableRepr>().unwrap()
    }
    pub fn os_2(&self) -> os_2::Os_2TableHandle<'_> {
        self.table::<os_2::Os_2TableRepr>().unwrap()
    }
}
