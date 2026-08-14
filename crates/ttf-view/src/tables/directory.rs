use crate::{
    tables::Table,
    types::{Offset32, Tag, tags, uint16, uint32},
    util::{Describe, Describer, StructDescriber, describe, describe_impl, iterator_map},
};

#[repr(C)]
#[non_exhaustive]
pub struct TableDirectoryRepr {
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

    pub const fn table_records_raw(&self) -> &[TableRecordRepr] {
        let len = self.num_tables.get() as usize;
        unsafe { std::slice::from_raw_parts(self.table_records.as_ptr(), len) }
    }
    pub fn table_record_raw(&self, tag: Tag) -> Option<&TableRecordRepr> {
        self.table_records_raw().iter().find(|x| x.table_tag == tag)
    }

    pub const fn table_records(&self) -> TableRecordsIter<'_> {
        TableRecordsIter::new(self)
    }
    pub fn table_record(&self, tag: Tag) -> Option<TableRecordHandle<'_>> {
        Some(TableRecordHandle(self, self.table_record_raw(tag)?))
    }

    pub fn table_raw<T: Table>(&self) -> Option<&T> {
        self.table_record(T::TAG)?.table_as()
    }
    pub fn table<T: Table>(&self) -> Option<T::Handle<'_>> {
        T::in_directory(self)
    }

    // Note: see src/tables/mod.rs for specific table methods
}

#[derive_const(Clone)]
pub struct TableRecordHandle<'a>(&'a TableDirectoryRepr, &'a TableRecordRepr);

const impl<'a> std::ops::Deref for TableRecordHandle<'a> {
    type Target = TableRecordRepr;
    fn deref(&self) -> &Self::Target {
        self.1
    }
}
impl<'a> TableRecordHandle<'a> {
    pub const fn table_as_bytes(&self) -> &'a [u8] {
        unsafe {
            let start = std::ptr::from_ref(self.0).cast::<u8>().add(self.offset.get() as _);
            std::slice::from_raw_parts(start, self.length.get() as _)
        }
    }
    pub const fn table_as<T: Table>(&self) -> Option<&'a T> {
        if self.table_tag == T::TAG { Some(unsafe { self.table_as_unchecked() }) } else { None }
    }
    pub const unsafe fn table_as_unchecked<T: Table>(&self) -> &'a T {
        debug_assert!(self.table_tag == T::TAG);
        unsafe { &*self.table_as_bytes().as_ptr().cast() }
    }

    pub fn calculate_checksum(&self) -> u32 {
        let (uint32s, rest) = self.table_as_bytes().as_chunks::<4>();
        let mut sum: u32 = uint32s.iter().map(|x| u32::from_be_bytes(*x)).sum();

        if !rest.is_empty() {
            let mut buf = [0; 4];
            buf[..rest.len()].copy_from_slice(rest);
            sum += u32::from_be_bytes(buf);
        }

        if self.table_tag == tags::head {
            let checksum_adjustment = u32::from_be_bytes(uint32s[2]);
            sum -= checksum_adjustment;
        }

        sum
    }
}

// TODO: When std::slice::Iter's Clone is constified, replace this with #[derive_const]
#[derive(Clone)]
pub struct TableRecordsIter<'a> {
    dir: &'a TableDirectoryRepr,
    inner: std::slice::Iter<'a, TableRecordRepr>,
}
impl<'a> TableRecordsIter<'a> {
    pub const fn new(dir: &'a TableDirectoryRepr) -> Self {
        Self { dir, inner: dir.table_records_raw().iter() }
    }
}
iterator_map!(TableRecordsIter<'a> {
    type Item = TableRecordHandle<'a>;
    |this, x| TableRecordHandle(this.dir, x)
});

impl Describe for TableDirectoryRepr {
    fn describe<D: Describer>(&self, d: D) -> Result<D::Ok, D::Error> {
        let mut d = d.describe_struct("TableDirectory");

        describe!(d, self {
            sfnt_version ["{:#010X}"],
            num_tables,
            search_range,
            entry_selector,
            range_shift,
        });

        d.field_fmt("table_records", self.table_records_raw(), |f, x| {
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
            checksum ["{:#010X}"],
            offset ["{:#010X}"],
            length ["{:#010X}"],
        })
    }
}

describe_impl! { Debug, Serialize for TableRecordRepr }
