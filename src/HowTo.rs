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
pub struct HowToSheet {
    sheet: Sheet,
}
impl HowToSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("HowTo")?;
        let sheet = resolver.read_excel_sheet(&exh, "HowTo", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<HowToRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<HowToRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for HowToSheet {
    type Row = HowToRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a HowToSheet {
    type Item = (u32, Vec<(u16, HowToRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, HowToSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, HowToSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct HowToRow<'a> {
    row: &'a Row,
}
impl<'a> HowToRow<'a> {
    pub fn Name(&'a self) -> &'a str {
        self.row.columns[0].into_string().unwrap()
    }
    pub fn HowToPagePC(&'a self) -> [i16; 5] {
        [
            self.row.columns[2].into_i16().copied().unwrap(),
            self.row.columns[3].into_i16().copied().unwrap(),
            self.row.columns[4].into_i16().copied().unwrap(),
            self.row.columns[5].into_i16().copied().unwrap(),
            self.row.columns[6].into_i16().copied().unwrap(),
        ]
    }
    pub fn HowToPageController(&'a self) -> [i16; 5] {
        [
            self.row.columns[7].into_i16().copied().unwrap(),
            self.row.columns[8].into_i16().copied().unwrap(),
            self.row.columns[9].into_i16().copied().unwrap(),
            self.row.columns[10].into_i16().copied().unwrap(),
            self.row.columns[11].into_i16().copied().unwrap(),
        ]
    }
    pub fn Sort(&'a self) -> u8 {
        self.row.columns[13].into_u8().copied().unwrap()
    }
    pub fn Category(&'a self) -> i8 {
        self.row.columns[12].into_i8().copied().unwrap()
    }
    pub fn Announce(&'a self) -> bool {
        self.row.columns[1].into_bool().copied().unwrap()
    }
}
