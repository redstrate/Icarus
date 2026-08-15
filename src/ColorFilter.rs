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
pub struct ColorFilterSheet {
    sheet: Sheet,
}
impl ColorFilterSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("ColorFilter")?;
        let sheet = resolver.read_excel_sheet(&exh, "ColorFilter", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<ColorFilterRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<ColorFilterRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for ColorFilterSheet {
    type Row = ColorFilterRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Unknown0: row
                .columns[0]
                .into_string()
                .cloned()
                .expect("Expected column 0 to be a string!"),
            Unknown1: row
                .columns[2]
                .into_f32()
                .copied()
                .expect("Expected column 2 to be a float32!"),
            Unknown2: row
                .columns[3]
                .into_f32()
                .copied()
                .expect("Expected column 3 to be a float32!"),
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
                .columns[8]
                .into_f32()
                .copied()
                .expect("Expected column 8 to be a float32!"),
            Unknown7: row
                .columns[9]
                .into_f32()
                .copied()
                .expect("Expected column 9 to be a float32!"),
            Unknown8: row
                .columns[10]
                .into_f32()
                .copied()
                .expect("Expected column 10 to be a float32!"),
            Unknown9: row
                .columns[11]
                .into_f32()
                .copied()
                .expect("Expected column 11 to be a float32!"),
            Unknown10: row
                .columns[12]
                .into_f32()
                .copied()
                .expect("Expected column 12 to be a float32!"),
            Unknown11: row
                .columns[13]
                .into_f32()
                .copied()
                .expect("Expected column 13 to be a float32!"),
            Unknown12: row
                .columns[14]
                .into_f32()
                .copied()
                .expect("Expected column 14 to be a float32!"),
            Unknown13: row
                .columns[15]
                .into_f32()
                .copied()
                .expect("Expected column 15 to be a float32!"),
            Unknown14: row
                .columns[1]
                .into_u8()
                .copied()
                .expect("Expected column 1 to be a uint8!"),
            Unknown15: row
                .columns[4]
                .into_bool()
                .copied()
                .expect("Expected column 4 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a ColorFilterSheet {
    type Item = (u32, Vec<(u16, ColorFilterRow)>);
    type IntoIter = StructuredSheetIterator<'a, ColorFilterSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, ColorFilterSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ColorFilterRow {
    ///""
    pub Unknown0: String,
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
    pub Unknown6: f32,
    ///""
    pub Unknown7: f32,
    ///""
    pub Unknown8: f32,
    ///""
    pub Unknown9: f32,
    ///""
    pub Unknown10: f32,
    ///""
    pub Unknown11: f32,
    ///""
    pub Unknown12: f32,
    ///""
    pub Unknown13: f32,
    ///""
    pub Unknown14: u8,
    ///""
    pub Unknown15: bool,
}
