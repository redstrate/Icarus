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
impl StructuredSheet for WKSFunctionSheet {
    type Row = WKSFunctionRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        let column_defs = &self.sheet.exh.column_definitions;
        let mut zipped: Vec<_> = row
            .columns
            .clone()
            .into_iter()
            .zip(column_defs)
            .collect();
        zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
        let (columns, _): (Vec<Field>, Vec<ExcelColumnDefinition>) = zipped
            .into_iter()
            .unzip();
        Some(Self::Row { columns })
    }
}
impl<'a> IntoIterator for &'a WKSFunctionSheet {
    type Item = (u32, Vec<(u16, WKSFunctionRow)>);
    type IntoIter = StructuredSheetIterator<'a, WKSFunctionSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, WKSFunctionSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct WKSFunctionRow {
    columns: Vec<Field>,
}
impl WKSFunctionRow {
    /// Required quest needed for function to return true
    pub fn RequiredQuests0<'a>(&'a self) -> [&'a Field; 4] {
        [&self.columns[0], &self.columns[1], &self.columns[2], &self.columns[3]]
    }
    /// Required quest needed for function to return true
    pub fn RequiredQuests1<'a>(&'a self) -> [&'a Field; 4] {
        [&self.columns[4], &self.columns[5], &self.columns[6], &self.columns[7]]
    }
    /// Best guess for now as no references are easily found in the game files
    pub fn RequiredQuests2<'a>(&'a self) -> [&'a Field; 4] {
        [&self.columns[8], &self.columns[9], &self.columns[10], &self.columns[11]]
    }
    /// Needs this grade to be completed for function to be valid. Game checks for WKSManager.DevGrade < RequiredDevGrade[]
    pub fn RequiredDevGrade<'a>(&'a self) -> [&'a Field; 4] {
        [&self.columns[12], &self.columns[13], &self.columns[14], &self.columns[15]]
    }
    /// Seems to only control one function tree for now relating to something in WKSManager but is also slated to check for '- 2 >= 2' and fail
    pub fn Unknown12<'a>(&'a self) -> [&'a Field; 4] {
        [&self.columns[16], &self.columns[17], &self.columns[18], &self.columns[19]]
    }
}
