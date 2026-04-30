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
pub struct WKSFunctionSheet {
    sheet: Sheet,
}
impl WKSFunctionSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("WKSFunction")?;
        let sheet = resolver.read_excel_sheet(&exh, "WKSFunction", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<WKSFunctionRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<WKSFunctionRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for WKSFunctionSheet {
    type Row = WKSFunctionRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            RequiredQuests0: [
                row
                    .columns[8]
                    .into_u32()
                    .copied()
                    .expect("Expected column 8 to be a uint32!"),
                row
                    .columns[9]
                    .into_u32()
                    .copied()
                    .expect("Expected column 9 to be a uint32!"),
                row
                    .columns[10]
                    .into_u32()
                    .copied()
                    .expect("Expected column 10 to be a uint32!"),
                row
                    .columns[11]
                    .into_u32()
                    .copied()
                    .expect("Expected column 11 to be a uint32!"),
            ],
            RequiredQuests1: [
                row
                    .columns[12]
                    .into_u32()
                    .copied()
                    .expect("Expected column 12 to be a uint32!"),
                row
                    .columns[13]
                    .into_u32()
                    .copied()
                    .expect("Expected column 13 to be a uint32!"),
                row
                    .columns[14]
                    .into_u32()
                    .copied()
                    .expect("Expected column 14 to be a uint32!"),
                row
                    .columns[15]
                    .into_u32()
                    .copied()
                    .expect("Expected column 15 to be a uint32!"),
            ],
            RequiredQuests2: [
                row
                    .columns[16]
                    .into_u32()
                    .copied()
                    .expect("Expected column 16 to be a uint32!"),
                row
                    .columns[17]
                    .into_u32()
                    .copied()
                    .expect("Expected column 17 to be a uint32!"),
                row
                    .columns[18]
                    .into_u32()
                    .copied()
                    .expect("Expected column 18 to be a uint32!"),
                row
                    .columns[19]
                    .into_u32()
                    .copied()
                    .expect("Expected column 19 to be a uint32!"),
            ],
            RequiredDevGrade: [
                row
                    .columns[0]
                    .into_u8()
                    .copied()
                    .expect("Expected column 0 to be a uint8!"),
                row
                    .columns[1]
                    .into_u8()
                    .copied()
                    .expect("Expected column 1 to be a uint8!"),
                row
                    .columns[2]
                    .into_u8()
                    .copied()
                    .expect("Expected column 2 to be a uint8!"),
                row
                    .columns[3]
                    .into_u8()
                    .copied()
                    .expect("Expected column 3 to be a uint8!"),
            ],
            Unknown12: [
                row
                    .columns[4]
                    .into_u8()
                    .copied()
                    .expect("Expected column 4 to be a uint8!"),
                row
                    .columns[5]
                    .into_u8()
                    .copied()
                    .expect("Expected column 5 to be a uint8!"),
                row
                    .columns[6]
                    .into_u8()
                    .copied()
                    .expect("Expected column 6 to be a uint8!"),
                row
                    .columns[7]
                    .into_u8()
                    .copied()
                    .expect("Expected column 7 to be a uint8!"),
            ],
        })
    }
}
impl<'a> IntoIterator for &'a WKSFunctionSheet {
    type Item = (u32, Vec<(u16, WKSFunctionRow)>);
    type IntoIter = StructuredSheetIterator<'a, WKSFunctionSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, WKSFunctionSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct WKSFunctionRow {
    ///"Required quest needed for function to return true"
    pub RequiredQuests0: [u32; 4],
    ///"Required quest needed for function to return true"
    pub RequiredQuests1: [u32; 4],
    ///"Best guess for now as no references are easily found in the game files"
    pub RequiredQuests2: [u32; 4],
    ///"Needs this grade to be completed for function to be valid. Game checks for WKSManager.DevGrade < RequiredDevGrade[]"
    pub RequiredDevGrade: [u8; 4],
    ///"Seems to only control one function tree for now relating to something in WKSManager but is also slated to check for '- 2 >= 2' and fail"
    pub Unknown12: [u8; 4],
}
