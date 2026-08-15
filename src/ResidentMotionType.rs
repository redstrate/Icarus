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
pub struct ResidentMotionTypeSheet {
    sheet: Sheet,
}
impl ResidentMotionTypeSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("ResidentMotionType")?;
        let sheet = resolver.read_excel_sheet(&exh, "ResidentMotionType", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<ResidentMotionTypeRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<ResidentMotionTypeRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for ResidentMotionTypeSheet {
    type Row = ResidentMotionTypeRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Unknown0: row
                .columns[0]
                .into_string()
                .cloned()
                .expect("Expected column 0 to be a string!"),
            Unknown1: row
                .columns[1]
                .into_string()
                .cloned()
                .expect("Expected column 1 to be a string!"),
            Unknown2: row
                .columns[2]
                .into_string()
                .cloned()
                .expect("Expected column 2 to be a string!"),
            Unknown3: row
                .columns[3]
                .into_string()
                .cloned()
                .expect("Expected column 3 to be a string!"),
            Unknown4: row
                .columns[4]
                .into_string()
                .cloned()
                .expect("Expected column 4 to be a string!"),
            Unknown5: row
                .columns[5]
                .into_string()
                .cloned()
                .expect("Expected column 5 to be a string!"),
            Unknown6: row
                .columns[6]
                .into_string()
                .cloned()
                .expect("Expected column 6 to be a string!"),
            Unknown7: row
                .columns[7]
                .into_string()
                .cloned()
                .expect("Expected column 7 to be a string!"),
        })
    }
}
impl<'a> IntoIterator for &'a ResidentMotionTypeSheet {
    type Item = (u32, Vec<(u16, ResidentMotionTypeRow)>);
    type IntoIter = StructuredSheetIterator<'a, ResidentMotionTypeSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, ResidentMotionTypeSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ResidentMotionTypeRow {
    ///""
    pub Unknown0: String,
    ///""
    pub Unknown1: String,
    ///""
    pub Unknown2: String,
    ///""
    pub Unknown3: String,
    ///""
    pub Unknown4: String,
    ///""
    pub Unknown5: String,
    ///""
    pub Unknown6: String,
    ///""
    pub Unknown7: String,
}
