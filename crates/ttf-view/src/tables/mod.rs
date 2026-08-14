use crate::{
    tables::{
        cmap::CmapTableRepr,
        head::HeadTableRepr,
        hhea::HheaTableRepr,
        hmtx::{HmtxTableHandle, HmtxTableRepr},
        maxp::MaxpTableRepr,
        name::NameTableRepr,
        os_2::{Os_2TableHandle, Os_2TableRepr},
    },
    types::{Offset32, Tag, tags, uint16, uint32},
    util::{Describe, Describer, StructDescriber, describe, describe_impl},
};

pub mod cmap;
pub mod head;
pub mod hhea;
pub mod hmtx;
pub mod maxp;
pub mod name;
pub mod os_2;

#[repr(C)]
#[non_exhaustive]
pub struct TableDirectoryRepr {
    table_data: [u8; 0],
    pub sfnt_version: uint32,
    pub num_tables: uint16,
    pub search_range: uint16,
    pub entry_selector: uint16,
    pub range_shift: uint16,
    table_records: [TableRecordRepr; 0],
}

#[repr(C)]
pub struct TableRecordRepr {
    pub table_tag: Tag,
    pub checksum: uint32,
    pub offset: Offset32,
    pub length: uint32,
}

impl TableDirectoryRepr {
    pub const unsafe fn new_unchecked(bytes: &[u8]) -> &Self {
        unsafe { &*bytes.as_ptr().cast() }
    }

    pub const fn table_records(&self) -> &[TableRecordRepr] {
        let len = self.num_tables.get() as usize;
        unsafe { std::slice::from_raw_parts(self.table_records.as_ptr(), len) }
    }

    pub fn table_record(&self, tag: Tag) -> Option<&TableRecordRepr> {
        self.table_records().iter().find(|t| t.table_tag == tag)
    }
    pub fn table<T: Table>(&self) -> Option<&T> {
        self.table_records().iter().find_map(|t| t.get_as::<T>(self))
    }

    // Note: These are all required tables, so we'll panic on their absence.
    pub fn cmap(&self) -> &CmapTableRepr {
        self.table().unwrap()
    }
    pub fn head(&self) -> &HeadTableRepr {
        self.table().unwrap()
    }
    pub fn hhea(&self) -> &HheaTableRepr {
        self.table().unwrap()
    }
    pub fn hmtx(&self) -> HmtxTableHandle<'_> {
        HmtxTableHandle::new(self)
    }
    pub fn maxp(&self) -> &MaxpTableRepr {
        self.table().unwrap()
    }
    pub fn name(&self) -> &NameTableRepr {
        self.table().unwrap()
    }
    pub fn os_2(&self) -> Os_2TableHandle<'_> {
        Os_2TableHandle::new(self)
    }
}

impl TableRecordRepr {
    pub const fn data<'a>(&'a self, dir: &'a TableDirectoryRepr) -> &'a [u8] {
        unsafe {
            let start = dir.table_data.as_ptr().add(self.offset.get() as _);
            std::slice::from_raw_parts(start, self.length.get() as _)
        }
    }

    pub const fn get_as<'a, T: Table>(&'a self, dir: &'a TableDirectoryRepr) -> Option<&'a T> {
        if self.table_tag == T::TAG {
            Some(unsafe { &*dir.table_data.as_ptr().add(self.offset.get() as _).cast() })
        } else {
            None
        }
    }
}

pub trait Table {
    const TAG: Tag;
}

macro_rules! impl_table_trait {
    ($($tag:expr => $table:ty),* $(,)?) => (
        $( impl Table for $table { const TAG: Tag = $tag; } )*
    );
}
impl_table_trait! {
    tags::cmap => CmapTableRepr,
    tags::head => HeadTableRepr,
    tags::hhea => HheaTableRepr,
    tags::hmtx => HmtxTableRepr,
    tags::maxp => MaxpTableRepr,
    tags::name => NameTableRepr,
    tags::OS_2 => Os_2TableRepr,
}

impl Describe for TableDirectoryRepr {
    fn describe<D: Describer>(&self, d: D) -> Result<D::Ok, D::Error> {
        let mut d = d.describe_struct("TableDirectory");

        describe!(d, self {
            sfnt_version: "{:#010X}",
            num_tables,
            search_range,
            entry_selector,
            range_shift,
        });

        d.field_fmt("table_records", self.table_records(), |f, x| {
            let mut list = f.debug_list();

            for table in x {
                list.entry_with(|f| {
                    let mut f = f.with_options(*f.options().alternate(false));
                    std::fmt::Debug::fmt(table, &mut f)
                });
            }
            list.finish()
        });

        d.finish()
    }
}

describe_impl! { Debug, Serialize for TableDirectoryRepr }

impl Describe for TableRecordRepr {
    fn describe<D: Describer>(&self, d: D) -> Result<D::Ok, D::Error> {
        describe!(d, self as "TableRecord" {
            table_tag,
            checksum: "{:#010X}",
            offset: "{:#010X}",
            length: "{:#010X}",
        })
    }
}

describe_impl! { Debug, Serialize for TableRecordRepr }
