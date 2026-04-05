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
pub struct RaceSheet {
    sheet: Sheet,
}
impl RaceSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Race")?;
        let sheet = resolver.read_excel_sheet(&exh, "Race", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<RaceRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<RaceRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for RaceSheet {
    type Row = RaceRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a RaceSheet {
    type Item = (u32, Vec<(u16, RaceRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, RaceSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, RaceSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct RaceRow<'a> {
    row: &'a Row,
}
impl<'a> RaceRow<'a> {
    pub fn Masculine(&'a self) -> &'a str {
        self.row.columns[0].into_string().unwrap()
    }
    pub fn Feminine(&'a self) -> &'a str {
        self.row.columns[1].into_string().unwrap()
    }
    pub fn RSEMBody(&'a self) -> i32 {
        self.row.columns[2].into_i32().copied().unwrap()
    }
    pub fn RSEFBody(&'a self) -> i32 {
        self.row.columns[6].into_i32().copied().unwrap()
    }
    pub fn RSEMHands(&'a self) -> i32 {
        self.row.columns[3].into_i32().copied().unwrap()
    }
    pub fn RSEFHands(&'a self) -> i32 {
        self.row.columns[7].into_i32().copied().unwrap()
    }
    pub fn RSEMLegs(&'a self) -> i32 {
        self.row.columns[4].into_i32().copied().unwrap()
    }
    pub fn RSEFLegs(&'a self) -> i32 {
        self.row.columns[8].into_i32().copied().unwrap()
    }
    pub fn RSEMFeet(&'a self) -> i32 {
        self.row.columns[5].into_i32().copied().unwrap()
    }
    pub fn RSEFFeet(&'a self) -> i32 {
        self.row.columns[9].into_i32().copied().unwrap()
    }
    pub fn Unknown0(&'a self) -> u8 {
        self.row.columns[10].into_u8().copied().unwrap()
    }
    pub fn ExPac(&'a self) -> u8 {
        self.row.columns[11].into_u8().copied().unwrap()
    }
}
