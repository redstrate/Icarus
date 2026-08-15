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
pub struct WKSCosmoToolNameSheet {
    sheet: Sheet,
}
impl WKSCosmoToolNameSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("WKSCosmoToolName")?;
        let sheet = resolver.read_excel_sheet(&exh, "WKSCosmoToolName", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<WKSCosmoToolNameRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<WKSCosmoToolNameRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for WKSCosmoToolNameSheet {
    type Row = WKSCosmoToolNameRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Name: row
                .columns[0]
                .into_string()
                .cloned()
                .expect("Expected column 0 to be a string!"),
            Unknown1: row
                .columns[2]
                .into_string()
                .cloned()
                .expect("Expected column 2 to be a string!"),
            Unknown2: row
                .columns[1]
                .into_i8()
                .copied()
                .expect("Expected column 1 to be a int8!"),
            Unknown3: row
                .columns[3]
                .into_i8()
                .copied()
                .expect("Expected column 3 to be a int8!"),
            Unknown4: row
                .columns[4]
                .into_i8()
                .copied()
                .expect("Expected column 4 to be a int8!"),
            Unknown5: row
                .columns[5]
                .into_i8()
                .copied()
                .expect("Expected column 5 to be a int8!"),
            Unknown6: row
                .columns[6]
                .into_i8()
                .copied()
                .expect("Expected column 6 to be a int8!"),
            Unknown7: row
                .columns[7]
                .into_i8()
                .copied()
                .expect("Expected column 7 to be a int8!"),
        })
    }
}
impl<'a> IntoIterator for &'a WKSCosmoToolNameSheet {
    type Item = (u32, Vec<(u16, WKSCosmoToolNameRow)>);
    type IntoIter = StructuredSheetIterator<'a, WKSCosmoToolNameSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, WKSCosmoToolNameSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct WKSCosmoToolNameRow {
    ///""
    pub Name: String,
    ///""
    pub Unknown1: String,
    ///""
    pub Unknown2: i8,
    ///""
    pub Unknown3: i8,
    ///""
    pub Unknown4: i8,
    ///""
    pub Unknown5: i8,
    ///""
    pub Unknown6: i8,
    ///""
    pub Unknown7: i8,
}
