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
pub struct CutsceneSheet {
    sheet: Sheet,
}
impl CutsceneSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Cutscene")?;
        let sheet = resolver.read_excel_sheet(&exh, "Cutscene", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<CutsceneRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<CutsceneRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for CutsceneSheet {
    type Row = CutsceneRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Path: row
                .columns[0]
                .into_string()
                .cloned()
                .expect("Expected column 0 to be a string!"),
            Unknown0: row
                .columns[4]
                .into_i32()
                .copied()
                .expect("Expected column 4 to be a int32!"),
            Unknown1: row
                .columns[5]
                .into_i32()
                .copied()
                .expect("Expected column 5 to be a int32!"),
            Unknown2: row
                .columns[6]
                .into_i32()
                .copied()
                .expect("Expected column 6 to be a int32!"),
            Unknown3: row
                .columns[7]
                .into_i32()
                .copied()
                .expect("Expected column 7 to be a int32!"),
            Unknown4: row
                .columns[8]
                .into_u16()
                .copied()
                .expect("Expected column 8 to be a uint16!"),
            Unknown5: row
                .columns[1]
                .into_u8()
                .copied()
                .expect("Expected column 1 to be a uint8!"),
            Unknown6: row
                .columns[2]
                .into_u8()
                .copied()
                .expect("Expected column 2 to be a uint8!"),
            Unknown7: row
                .columns[3]
                .into_bool()
                .copied()
                .expect("Expected column 3 to be a bool!"),
            Unknown8: row
                .columns[9]
                .into_bool()
                .copied()
                .expect("Expected column 9 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a CutsceneSheet {
    type Item = (u32, Vec<(u16, CutsceneRow)>);
    type IntoIter = StructuredSheetIterator<'a, CutsceneSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, CutsceneSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CutsceneRow {
    ///""
    pub Path: String,
    ///""
    pub Unknown0: i32,
    ///""
    pub Unknown1: i32,
    ///""
    pub Unknown2: i32,
    ///""
    pub Unknown3: i32,
    ///""
    pub Unknown4: u16,
    ///""
    pub Unknown5: u8,
    ///""
    pub Unknown6: u8,
    ///""
    pub Unknown7: bool,
    ///""
    pub Unknown8: bool,
}
