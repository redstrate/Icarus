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
pub struct FateProgressUISheet {
    sheet: Sheet,
}
impl FateProgressUISheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("FateProgressUI")?;
        let sheet = resolver.read_excel_sheet(&exh, "FateProgressUI", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<FateProgressUIRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<FateProgressUIRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for FateProgressUISheet {
    type Row = FateProgressUIRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a FateProgressUISheet {
    type Item = (u32, Vec<(u16, FateProgressUIRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, FateProgressUISheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, FateProgressUISheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct FateProgressUIRow<'a> {
    row: &'a Row,
}
impl<'a> FateProgressUIRow<'a> {
    pub fn Location(&'a self) -> i32 {
        self.row.columns[0].into_i32().copied().unwrap()
    }
    pub fn ReqFatesToRank2(&'a self) -> u8 {
        self.row.columns[1].into_u8().copied().unwrap()
    }
    pub fn ReqFatesToRank3(&'a self) -> u8 {
        self.row.columns[2].into_u8().copied().unwrap()
    }
    pub fn ReqFatesToRank4(&'a self) -> u8 {
        self.row.columns[3].into_u8().copied().unwrap()
    }
    pub fn DisplayOrder(&'a self) -> u8 {
        self.row.columns[5].into_u8().copied().unwrap()
    }
    pub fn Unknown0(&'a self) -> i8 {
        self.row.columns[4].into_i8().copied().unwrap()
    }
}
