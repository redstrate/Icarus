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
pub struct WKSFunctionSheet {
    sheet: Sheet,
}
impl WKSFunctionSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("WKSFunction")?;
        let sheet = resolver.read_excel_sheet(&exh, "WKSFunction", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<WKSFunctionRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<WKSFunctionRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for WKSFunctionSheet {
    type Row = WKSFunctionRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a WKSFunctionSheet {
    type Item = (u32, Vec<(u16, WKSFunctionRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, WKSFunctionSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, WKSFunctionSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct WKSFunctionRow<'a> {
    row: &'a Row,
}
impl<'a> WKSFunctionRow<'a> {
    /// Required quest needed for function to return true
    pub fn RequiredQuests0(&'a self) -> [u32; 4] {
        [
            self.row.columns[8].into_u32().copied().unwrap(),
            self.row.columns[9].into_u32().copied().unwrap(),
            self.row.columns[10].into_u32().copied().unwrap(),
            self.row.columns[11].into_u32().copied().unwrap(),
        ]
    }
    /// Required quest needed for function to return true
    pub fn RequiredQuests1(&'a self) -> [u32; 4] {
        [
            self.row.columns[12].into_u32().copied().unwrap(),
            self.row.columns[13].into_u32().copied().unwrap(),
            self.row.columns[14].into_u32().copied().unwrap(),
            self.row.columns[15].into_u32().copied().unwrap(),
        ]
    }
    /// Best guess for now as no references are easily found in the game files
    pub fn RequiredQuests2(&'a self) -> [u32; 4] {
        [
            self.row.columns[16].into_u32().copied().unwrap(),
            self.row.columns[17].into_u32().copied().unwrap(),
            self.row.columns[18].into_u32().copied().unwrap(),
            self.row.columns[19].into_u32().copied().unwrap(),
        ]
    }
    /// Needs this grade to be completed for function to be valid. Game checks for WKSManager.DevGrade < RequiredDevGrade[]
    pub fn RequiredDevGrade(&'a self) -> [u8; 4] {
        [
            self.row.columns[0].into_u8().copied().unwrap(),
            self.row.columns[1].into_u8().copied().unwrap(),
            self.row.columns[2].into_u8().copied().unwrap(),
            self.row.columns[3].into_u8().copied().unwrap(),
        ]
    }
    /// Seems to only control one function tree for now relating to something in WKSManager but is also slated to check for '- 2 >= 2' and fail
    pub fn Unknown12(&'a self) -> [u8; 4] {
        [
            self.row.columns[4].into_u8().copied().unwrap(),
            self.row.columns[5].into_u8().copied().unwrap(),
            self.row.columns[6].into_u8().copied().unwrap(),
            self.row.columns[7].into_u8().copied().unwrap(),
        ]
    }
}
