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
pub struct LeveRewardItemGroupSheet {
    sheet: Sheet,
}
impl LeveRewardItemGroupSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("LeveRewardItemGroup")?;
        let sheet = resolver.read_excel_sheet(&exh, "LeveRewardItemGroup", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<LeveRewardItemGroupRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<LeveRewardItemGroupRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for LeveRewardItemGroupSheet {
    type Row = LeveRewardItemGroupRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Item: [
                row
                    .columns[0]
                    .into_i32()
                    .copied()
                    .expect("Expected column 0 to be a int32!"),
                row
                    .columns[3]
                    .into_i32()
                    .copied()
                    .expect("Expected column 3 to be a int32!"),
                row
                    .columns[6]
                    .into_i32()
                    .copied()
                    .expect("Expected column 6 to be a int32!"),
                row
                    .columns[9]
                    .into_i32()
                    .copied()
                    .expect("Expected column 9 to be a int32!"),
                row
                    .columns[12]
                    .into_i32()
                    .copied()
                    .expect("Expected column 12 to be a int32!"),
                row
                    .columns[15]
                    .into_i32()
                    .copied()
                    .expect("Expected column 15 to be a int32!"),
                row
                    .columns[18]
                    .into_i32()
                    .copied()
                    .expect("Expected column 18 to be a int32!"),
                row
                    .columns[21]
                    .into_i32()
                    .copied()
                    .expect("Expected column 21 to be a int32!"),
                row
                    .columns[24]
                    .into_i32()
                    .copied()
                    .expect("Expected column 24 to be a int32!"),
            ],
            Count: [
                row
                    .columns[1]
                    .into_u8()
                    .copied()
                    .expect("Expected column 1 to be a uint8!"),
                row
                    .columns[4]
                    .into_u8()
                    .copied()
                    .expect("Expected column 4 to be a uint8!"),
                row
                    .columns[7]
                    .into_u8()
                    .copied()
                    .expect("Expected column 7 to be a uint8!"),
                row
                    .columns[10]
                    .into_u8()
                    .copied()
                    .expect("Expected column 10 to be a uint8!"),
                row
                    .columns[13]
                    .into_u8()
                    .copied()
                    .expect("Expected column 13 to be a uint8!"),
                row
                    .columns[16]
                    .into_u8()
                    .copied()
                    .expect("Expected column 16 to be a uint8!"),
                row
                    .columns[19]
                    .into_u8()
                    .copied()
                    .expect("Expected column 19 to be a uint8!"),
                row
                    .columns[22]
                    .into_u8()
                    .copied()
                    .expect("Expected column 22 to be a uint8!"),
                row
                    .columns[25]
                    .into_u8()
                    .copied()
                    .expect("Expected column 25 to be a uint8!"),
            ],
            IsHQ: [
                row
                    .columns[2]
                    .into_bool()
                    .copied()
                    .expect("Expected column 2 to be a bool!"),
                row
                    .columns[5]
                    .into_bool()
                    .copied()
                    .expect("Expected column 5 to be a bool!"),
                row
                    .columns[8]
                    .into_bool()
                    .copied()
                    .expect("Expected column 8 to be a bool!"),
                row
                    .columns[11]
                    .into_bool()
                    .copied()
                    .expect("Expected column 11 to be a bool!"),
                row
                    .columns[14]
                    .into_bool()
                    .copied()
                    .expect("Expected column 14 to be a bool!"),
                row
                    .columns[17]
                    .into_bool()
                    .copied()
                    .expect("Expected column 17 to be a bool!"),
                row
                    .columns[20]
                    .into_bool()
                    .copied()
                    .expect("Expected column 20 to be a bool!"),
                row
                    .columns[23]
                    .into_bool()
                    .copied()
                    .expect("Expected column 23 to be a bool!"),
                row
                    .columns[26]
                    .into_bool()
                    .copied()
                    .expect("Expected column 26 to be a bool!"),
            ],
        })
    }
}
impl<'a> IntoIterator for &'a LeveRewardItemGroupSheet {
    type Item = (u32, Vec<(u16, LeveRewardItemGroupRow)>);
    type IntoIter = StructuredSheetIterator<'a, LeveRewardItemGroupSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, LeveRewardItemGroupSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LeveRewardItemGroupRow {
    ///""
    pub Item: [i32; 9],
    ///""
    pub Count: [u8; 9],
    ///""
    pub IsHQ: [bool; 9],
}
