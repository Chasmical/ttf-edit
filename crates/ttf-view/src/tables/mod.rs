use crate::{
    tables::cmap::CmapTableRepr,
    types::{Offset32, Tag, tags, uint16, uint32},
};

pub mod cmap;

#[repr(C)]
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

    pub const fn tables(&self) -> &[TableRecordRepr] {
        unsafe {
            std::slice::from_raw_parts(self.table_records.as_ptr(), self.num_tables.get() as _)
        }
    }

    pub(crate) const unsafe fn find_table<T>(&self, tag: Tag) -> Option<&T> {
        let tables = self.tables();
        let mut idx = 0;
        while idx < tables.len() {
            let table = &tables[idx];
            if table.table_tag == tag {
                return Some(unsafe { table.data_cast(self) });
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
