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
pub struct BGMSheet {
    sheet: Sheet,
}
impl BGMSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("BGM")?;
        let sheet = resolver.read_excel_sheet(&exh, "BGM", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<BGMRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<BGMRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for BGMSheet {
    type Row = BGMRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            File: row
                .columns[0]
                .into_string()
                .cloned()
                .expect("Expected column 0 to be a string!"),
            DisableRestartResetTime: row
                .columns[5]
                .into_f32()
                .copied()
                .expect("Expected column 5 to be a float32!"),
            Priority: row
                .columns[1]
                .into_u8()
                .copied()
                .expect("Expected column 1 to be a uint8!"),
            SpecialMode: row
                .columns[6]
                .into_u8()
                .copied()
                .expect("Expected column 6 to be a uint8!"),
            DisableRestartTimeOut: row
                .columns[2]
                .into_bool()
                .copied()
                .expect("Expected column 2 to be a bool!"),
            DisableRestart: row
                .columns[3]
                .into_bool()
                .copied()
                .expect("Expected column 3 to be a bool!"),
            PassEnd: row
                .columns[4]
                .into_bool()
                .copied()
                .expect("Expected column 4 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a BGMSheet {
    type Item = (u32, Vec<(u16, BGMRow)>);
    type IntoIter = StructuredSheetIterator<'a, BGMSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, BGMSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BGMRow {
    ///""
    pub File: String,
    ///""
    pub DisableRestartResetTime: f32,
    ///""
    pub Priority: u8,
    ///""
    pub SpecialMode: u8,
    ///""
    pub DisableRestartTimeOut: bool,
    ///""
    pub DisableRestart: bool,
    ///""
    pub PassEnd: bool,
}
