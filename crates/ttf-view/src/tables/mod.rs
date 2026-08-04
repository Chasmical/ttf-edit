use crate::{
    tables::cmap::CmapTableRepr,
    types::{Offset32, Tag, tags, uint16, uint32},
};
use std::fmt;

pub mod cmap;
pub mod head;
pub mod hhea;
pub mod maxp;

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

    pub(crate) const unsafe fn find_table<T>(&self, tag: Tag) -> Option<&T> {
        let tables = self.table_records();
        let mut idx = 0;
        while idx < tables.len() {
            let table = &tables[idx];
            if table.table_tag == tag {
                return Some(unsafe { table.data_cast::<T>(self) });
            }
            idx += 1;
        }
        None
    }

    pub const fn cmap(&self) -> &CmapTableRepr {
        unsafe { self.find_table(tags::cmap).unwrap() }
    }
}

impl TableRecordRepr {
    pub const fn data<'a>(&'a self, dir: &'a TableDirectoryRepr) -> &'a [u8] {
        unsafe {
            let start = dir.table_data.as_ptr().add(self.offset.get() as _);
            std::slice::from_raw_parts(start, self.length.get() as _)
        }
    }
    pub const unsafe fn data_cast<'a, T>(&'a self, dir: &'a TableDirectoryRepr) -> &'a T {
        unsafe { &*dir.table_data.as_ptr().add(self.offset.get() as _).cast() }
    }
}

impl fmt::Debug for TableDirectoryRepr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TableDirectoryRepr")
            .field_with("sfnt_version", |f| write!(f, "{:#010X}", self.sfnt_version))
            .field("num_tables", &self.num_tables.get())
            .field("search_range", &self.search_range.get())
            .field("entry_selector", &self.entry_selector.get())
            .field("range_shift", &self.range_shift.get())
            .field_with("table_records", |f| {
                let mut list = f.debug_list();

                for table in self.table_records() {
                    list.entry_with(|f| {
                        table.fmt(&mut f.with_options(*f.options().alternate(false)))
                    });
                }
                list.finish()
            })
            .finish()
    }
}

impl fmt::Debug for TableRecordRepr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TableRecordRepr")
            .field("table_tag", &self.table_tag)
            .field_with("checksum", |f| write!(f, "{:#010X}", self.checksum))
            .field_with("offset", |f| write!(f, "{:#010X}", self.offset))
            .field_with("length", |f| write!(f, "{:#010X}", self.length))
            .finish()
    }
}
