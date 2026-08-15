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
pub struct RaceSheet {
    sheet: Sheet,
}
impl RaceSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Race")?;
        let sheet = resolver.read_excel_sheet(&exh, "Race", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<RaceRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<RaceRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for RaceSheet {
    type Row = RaceRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Masculine: row
                .columns[0]
                .into_string()
                .cloned()
                .expect("Expected column 0 to be a string!"),
            Feminine: row
                .columns[1]
                .into_string()
                .cloned()
                .expect("Expected column 1 to be a string!"),
            RSEMBody: row
                .columns[2]
                .into_i32()
                .copied()
                .expect("Expected column 2 to be a int32!"),
            RSEFBody: row
                .columns[6]
                .into_i32()
                .copied()
                .expect("Expected column 6 to be a int32!"),
            RSEMHands: row
                .columns[3]
                .into_i32()
                .copied()
                .expect("Expected column 3 to be a int32!"),
            RSEFHands: row
                .columns[7]
                .into_i32()
                .copied()
                .expect("Expected column 7 to be a int32!"),
            RSEMLegs: row
                .columns[4]
                .into_i32()
                .copied()
                .expect("Expected column 4 to be a int32!"),
            RSEFLegs: row
                .columns[8]
                .into_i32()
                .copied()
                .expect("Expected column 8 to be a int32!"),
            RSEMFeet: row
                .columns[5]
                .into_i32()
                .copied()
                .expect("Expected column 5 to be a int32!"),
            RSEFFeet: row
                .columns[9]
                .into_i32()
                .copied()
                .expect("Expected column 9 to be a int32!"),
            Unknown0: row
                .columns[10]
                .into_u8()
                .copied()
                .expect("Expected column 10 to be a uint8!"),
            ExPac: row
                .columns[11]
                .into_u8()
                .copied()
                .expect("Expected column 11 to be a uint8!"),
        })
    }
}
impl<'a> IntoIterator for &'a RaceSheet {
    type Item = (u32, Vec<(u16, RaceRow)>);
    type IntoIter = StructuredSheetIterator<'a, RaceSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, RaceSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RaceRow {
    ///""
    pub Masculine: String,
    ///""
    pub Feminine: String,
    ///""
    pub RSEMBody: i32,
    ///""
    pub RSEFBody: i32,
    ///""
    pub RSEMHands: i32,
    ///""
    pub RSEFHands: i32,
    ///""
    pub RSEMLegs: i32,
    ///""
    pub RSEFLegs: i32,
    ///""
    pub RSEMFeet: i32,
    ///""
    pub RSEFFeet: i32,
    ///""
    pub Unknown0: u8,
    ///""
    pub ExPac: u8,
}
