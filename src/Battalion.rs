//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
#[derive(Debug, Clone)]
pub struct BattalionSheet {
    sheet: Sheet,
}
impl BattalionSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Battalion")?;
        let sheet = resolver.read_excel_sheet(&exh, "Battalion", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<BattalionRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<BattalionRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for BattalionSheet {
    type Row = BattalionRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a BattalionSheet {
    type Item = (u32, Vec<(u16, BattalionRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, BattalionSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, BattalionSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct BattalionRow<'a> {
    row: &'a Row,
}
impl<'a> BattalionRow<'a> {
    pub fn Unknown0(&'a self) -> bool {
        self.row.columns[0].into_bool().copied().unwrap()
    }
    pub fn Unknown1(&'a self) -> bool {
        self.row.columns[1].into_bool().copied().unwrap()
    }
    pub fn Unknown2(&'a self) -> bool {
        self.row.columns[2].into_bool().copied().unwrap()
    }
    pub fn Unknown3(&'a self) -> bool {
        self.row.columns[3].into_bool().copied().unwrap()
    }
    pub fn Unknown4(&'a self) -> bool {
        self.row.columns[4].into_bool().copied().unwrap()
    }
    pub fn Unknown5(&'a self) -> bool {
        self.row.columns[5].into_bool().copied().unwrap()
    }
    pub fn Unknown6(&'a self) -> bool {
        self.row.columns[6].into_bool().copied().unwrap()
    }
    pub fn Unknown7(&'a self) -> bool {
        self.row.columns[7].into_bool().copied().unwrap()
    }
    pub fn Unknown8(&'a self) -> bool {
        self.row.columns[8].into_bool().copied().unwrap()
    }
    pub fn Unknown9(&'a self) -> bool {
        self.row.columns[9].into_bool().copied().unwrap()
    }
    pub fn Unknown10(&'a self) -> bool {
        self.row.columns[10].into_bool().copied().unwrap()
    }
    pub fn Unknown11(&'a self) -> bool {
        self.row.columns[11].into_bool().copied().unwrap()
    }
    pub fn Unknown12(&'a self) -> bool {
        self.row.columns[12].into_bool().copied().unwrap()
    }
    pub fn Unknown13(&'a self) -> bool {
        self.row.columns[13].into_bool().copied().unwrap()
    }
    pub fn Unknown14(&'a self) -> bool {
        self.row.columns[14].into_bool().copied().unwrap()
    }
}
