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
pub struct UDS_EventSheet {
    sheet: Sheet,
}
impl UDS_EventSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("UDS_Event")?;
        let sheet = resolver.read_excel_sheet(&exh, "UDS_Event", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<UDS_EventRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<UDS_EventRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for UDS_EventSheet {
    type Row = UDS_EventRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Text: row
                .columns[0]
                .into_string()
                .cloned()
                .expect("Expected column 0 to be a string!"),
            Type: row
                .columns[1]
                .into_string()
                .cloned()
                .expect("Expected column 1 to be a string!"),
            Property: [
                row
                    .columns[2]
                    .into_i32()
                    .copied()
                    .expect("Expected column 2 to be a int32!"),
                row
                    .columns[3]
                    .into_i32()
                    .copied()
                    .expect("Expected column 3 to be a int32!"),
                row
                    .columns[4]
                    .into_i32()
                    .copied()
                    .expect("Expected column 4 to be a int32!"),
                row
                    .columns[5]
                    .into_i32()
                    .copied()
                    .expect("Expected column 5 to be a int32!"),
                row
                    .columns[6]
                    .into_i32()
                    .copied()
                    .expect("Expected column 6 to be a int32!"),
                row
                    .columns[7]
                    .into_i32()
                    .copied()
                    .expect("Expected column 7 to be a int32!"),
                row
                    .columns[8]
                    .into_i32()
                    .copied()
                    .expect("Expected column 8 to be a int32!"),
                row
                    .columns[9]
                    .into_i32()
                    .copied()
                    .expect("Expected column 9 to be a int32!"),
                row
                    .columns[10]
                    .into_i32()
                    .copied()
                    .expect("Expected column 10 to be a int32!"),
                row
                    .columns[11]
                    .into_i32()
                    .copied()
                    .expect("Expected column 11 to be a int32!"),
                row
                    .columns[12]
                    .into_i32()
                    .copied()
                    .expect("Expected column 12 to be a int32!"),
                row
                    .columns[13]
                    .into_i32()
                    .copied()
                    .expect("Expected column 13 to be a int32!"),
                row
                    .columns[14]
                    .into_i32()
                    .copied()
                    .expect("Expected column 14 to be a int32!"),
                row
                    .columns[15]
                    .into_i32()
                    .copied()
                    .expect("Expected column 15 to be a int32!"),
                row
                    .columns[16]
                    .into_i32()
                    .copied()
                    .expect("Expected column 16 to be a int32!"),
                row
                    .columns[17]
                    .into_i32()
                    .copied()
                    .expect("Expected column 17 to be a int32!"),
                row
                    .columns[18]
                    .into_i32()
                    .copied()
                    .expect("Expected column 18 to be a int32!"),
                row
                    .columns[19]
                    .into_i32()
                    .copied()
                    .expect("Expected column 19 to be a int32!"),
                row
                    .columns[20]
                    .into_i32()
                    .copied()
                    .expect("Expected column 20 to be a int32!"),
                row
                    .columns[21]
                    .into_i32()
                    .copied()
                    .expect("Expected column 21 to be a int32!"),
                row
                    .columns[22]
                    .into_i32()
                    .copied()
                    .expect("Expected column 22 to be a int32!"),
                row
                    .columns[23]
                    .into_i32()
                    .copied()
                    .expect("Expected column 23 to be a int32!"),
                row
                    .columns[24]
                    .into_i32()
                    .copied()
                    .expect("Expected column 24 to be a int32!"),
                row
                    .columns[25]
                    .into_i32()
                    .copied()
                    .expect("Expected column 25 to be a int32!"),
                row
                    .columns[26]
                    .into_i32()
                    .copied()
                    .expect("Expected column 26 to be a int32!"),
                row
                    .columns[27]
                    .into_i32()
                    .copied()
                    .expect("Expected column 27 to be a int32!"),
                row
                    .columns[28]
                    .into_i32()
                    .copied()
                    .expect("Expected column 28 to be a int32!"),
                row
                    .columns[29]
                    .into_i32()
                    .copied()
                    .expect("Expected column 29 to be a int32!"),
                row
                    .columns[30]
                    .into_i32()
                    .copied()
                    .expect("Expected column 30 to be a int32!"),
                row
                    .columns[31]
                    .into_i32()
                    .copied()
                    .expect("Expected column 31 to be a int32!"),
                row
                    .columns[32]
                    .into_i32()
                    .copied()
                    .expect("Expected column 32 to be a int32!"),
                row
                    .columns[33]
                    .into_i32()
                    .copied()
                    .expect("Expected column 33 to be a int32!"),
            ],
        })
    }
}
impl<'a> IntoIterator for &'a UDS_EventSheet {
    type Item = (u32, Vec<(u16, UDS_EventRow)>);
    type IntoIter = StructuredSheetIterator<'a, UDS_EventSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, UDS_EventSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UDS_EventRow {
    ///""
    pub Text: String,
    ///""
    pub Type: String,
    ///""
    pub Property: [i32; 32],
}
