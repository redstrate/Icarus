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
pub struct SnipePerformanceCameraSheet {
    sheet: Sheet,
}
impl SnipePerformanceCameraSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 1u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("SnipePerformanceCamera")?;
        let sheet = resolver.read_excel_sheet(&exh, "SnipePerformanceCamera", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<SnipePerformanceCameraRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<SnipePerformanceCameraRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for SnipePerformanceCameraSheet {
    type Row = SnipePerformanceCameraRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Unknown0: row
                .columns[5]
                .into_u16()
                .copied()
                .expect("Expected column 5 to be a uint16!"),
            Unknown1: row
                .columns[8]
                .into_u16()
                .copied()
                .expect("Expected column 8 to be a uint16!"),
            Unknown2: row
                .columns[9]
                .into_u16()
                .copied()
                .expect("Expected column 9 to be a uint16!"),
            Unknown3: row
                .columns[1]
                .into_i16()
                .copied()
                .expect("Expected column 1 to be a int16!"),
            Unknown4: row
                .columns[2]
                .into_i16()
                .copied()
                .expect("Expected column 2 to be a int16!"),
            Unknown5: row
                .columns[3]
                .into_i16()
                .copied()
                .expect("Expected column 3 to be a int16!"),
            Unknown6: row
                .columns[4]
                .into_i16()
                .copied()
                .expect("Expected column 4 to be a int16!"),
            Unknown7: row
                .columns[6]
                .into_i16()
                .copied()
                .expect("Expected column 6 to be a int16!"),
            Unknown8: row
                .columns[7]
                .into_i16()
                .copied()
                .expect("Expected column 7 to be a int16!"),
            Unknown9: row
                .columns[0]
                .into_u8()
                .copied()
                .expect("Expected column 0 to be a uint8!"),
        })
    }
}
impl<'a> IntoIterator for &'a SnipePerformanceCameraSheet {
    type Item = (u32, Vec<(u16, SnipePerformanceCameraRow)>);
    type IntoIter = StructuredSheetIterator<'a, SnipePerformanceCameraSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, SnipePerformanceCameraSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SnipePerformanceCameraRow {
    ///""
    pub Unknown0: u16,
    ///""
    pub Unknown1: u16,
    ///""
    pub Unknown2: u16,
    ///""
    pub Unknown3: i16,
    ///""
    pub Unknown4: i16,
    ///""
    pub Unknown5: i16,
    ///""
    pub Unknown6: i16,
    ///""
    pub Unknown7: i16,
    ///""
    pub Unknown8: i16,
    ///""
    pub Unknown9: u8,
}
