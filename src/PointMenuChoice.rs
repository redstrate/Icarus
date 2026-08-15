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
pub struct PointMenuChoiceSheet {
    sheet: Sheet,
}
impl PointMenuChoiceSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("PointMenuChoice")?;
        let sheet = resolver.read_excel_sheet(&exh, "PointMenuChoice", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<PointMenuChoiceRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<PointMenuChoiceRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for PointMenuChoiceSheet {
    type Row = PointMenuChoiceRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Unknown0: row
                .columns[0]
                .into_f32()
                .copied()
                .expect("Expected column 0 to be a float32!"),
            Unknown1: row
                .columns[1]
                .into_f32()
                .copied()
                .expect("Expected column 1 to be a float32!"),
            Unknown2: row
                .columns[4]
                .into_f32()
                .copied()
                .expect("Expected column 4 to be a float32!"),
            Unknown3: row
                .columns[5]
                .into_f32()
                .copied()
                .expect("Expected column 5 to be a float32!"),
            Unknown4: row
                .columns[6]
                .into_f32()
                .copied()
                .expect("Expected column 6 to be a float32!"),
            Unknown5: row
                .columns[7]
                .into_f32()
                .copied()
                .expect("Expected column 7 to be a float32!"),
            Unknown6: row
                .columns[12]
                .into_u16()
                .copied()
                .expect("Expected column 12 to be a uint16!"),
            Unknown7: row
                .columns[2]
                .into_u8()
                .copied()
                .expect("Expected column 2 to be a uint8!"),
            Unknown8: row
                .columns[3]
                .into_u8()
                .copied()
                .expect("Expected column 3 to be a uint8!"),
            Unknown9: row
                .columns[8]
                .into_u8()
                .copied()
                .expect("Expected column 8 to be a uint8!"),
            Unknown10: row
                .columns[9]
                .into_u8()
                .copied()
                .expect("Expected column 9 to be a uint8!"),
            Unknown11: row
                .columns[10]
                .into_u8()
                .copied()
                .expect("Expected column 10 to be a uint8!"),
            Unknown12: row
                .columns[11]
                .into_u8()
                .copied()
                .expect("Expected column 11 to be a uint8!"),
        })
    }
}
impl<'a> IntoIterator for &'a PointMenuChoiceSheet {
    type Item = (u32, Vec<(u16, PointMenuChoiceRow)>);
    type IntoIter = StructuredSheetIterator<'a, PointMenuChoiceSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, PointMenuChoiceSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PointMenuChoiceRow {
    ///""
    pub Unknown0: f32,
    ///""
    pub Unknown1: f32,
    ///""
    pub Unknown2: f32,
    ///""
    pub Unknown3: f32,
    ///""
    pub Unknown4: f32,
    ///""
    pub Unknown5: f32,
    ///""
    pub Unknown6: u16,
    ///""
    pub Unknown7: u8,
    ///""
    pub Unknown8: u8,
    ///""
    pub Unknown9: u8,
    ///""
    pub Unknown10: u8,
    ///""
    pub Unknown11: u8,
    ///""
    pub Unknown12: u8,
}
