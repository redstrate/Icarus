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
pub struct ContentGaugeSheet {
    sheet: Sheet,
}
impl ContentGaugeSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("ContentGauge")?;
        let sheet = resolver.read_excel_sheet(&exh, "ContentGauge", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<ContentGaugeRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<ContentGaugeRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for ContentGaugeSheet {
    type Row = ContentGaugeRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Name: row
                .columns[1]
                .into_string()
                .cloned()
                .expect("Expected column 1 to be a string!"),
            TextString: row
                .columns[4]
                .into_string()
                .cloned()
                .expect("Expected column 4 to be a string!"),
            Unknown0: row
                .columns[5]
                .into_i16()
                .copied()
                .expect("Expected column 5 to be a int16!"),
            Unknown1: row
                .columns[8]
                .into_i16()
                .copied()
                .expect("Expected column 8 to be a int16!"),
            Unknown2: row
                .columns[11]
                .into_i16()
                .copied()
                .expect("Expected column 11 to be a int16!"),
            Unknown3: row
                .columns[0]
                .into_u8()
                .copied()
                .expect("Expected column 0 to be a uint8!"),
            Color: row
                .columns[2]
                .into_u8()
                .copied()
                .expect("Expected column 2 to be a uint8!"),
            Unknown4: row
                .columns[7]
                .into_u8()
                .copied()
                .expect("Expected column 7 to be a uint8!"),
            Unknown5: row
                .columns[10]
                .into_u8()
                .copied()
                .expect("Expected column 10 to be a uint8!"),
            Unknown6: row
                .columns[6]
                .into_i8()
                .copied()
                .expect("Expected column 6 to be a int8!"),
            Unknown7: row
                .columns[9]
                .into_i8()
                .copied()
                .expect("Expected column 9 to be a int8!"),
            Unknown8: row
                .columns[3]
                .into_bool()
                .copied()
                .expect("Expected column 3 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a ContentGaugeSheet {
    type Item = (u32, Vec<(u16, ContentGaugeRow)>);
    type IntoIter = StructuredSheetIterator<'a, ContentGaugeSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, ContentGaugeSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ContentGaugeRow {
    ///""
    pub Name: String,
    ///""
    pub TextString: String,
    ///""
    pub Unknown0: i16,
    ///""
    pub Unknown1: i16,
    ///""
    pub Unknown2: i16,
    ///""
    pub Unknown3: u8,
    ///""
    pub Color: u8,
    ///""
    pub Unknown4: u8,
    ///""
    pub Unknown5: u8,
    ///""
    pub Unknown6: i8,
    ///""
    pub Unknown7: i8,
    ///""
    pub Unknown8: bool,
}
