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
pub struct WeeklyBingoRewardDataSheet {
    sheet: Sheet,
}
impl WeeklyBingoRewardDataSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("WeeklyBingoRewardData")?;
        let sheet = resolver.read_excel_sheet(&exh, "WeeklyBingoRewardData", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<WeeklyBingoRewardDataRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<WeeklyBingoRewardDataRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for WeeklyBingoRewardDataSheet {
    type Row = WeeklyBingoRewardDataRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            RewardItem1: row
                .columns[1]
                .into_u32()
                .copied()
                .expect("Expected column 1 to be a uint32!"),
            RewardItem2: row
                .columns[6]
                .into_u32()
                .copied()
                .expect("Expected column 6 to be a uint32!"),
            RewardItem3: row
                .columns[10]
                .into_u32()
                .copied()
                .expect("Expected column 10 to be a uint32!"),
            RewardQuantity1: row
                .columns[3]
                .into_u16()
                .copied()
                .expect("Expected column 3 to be a uint16!"),
            RewardQuantity2: row
                .columns[8]
                .into_u16()
                .copied()
                .expect("Expected column 8 to be a uint16!"),
            RewardQuantity3: row
                .columns[12]
                .into_u16()
                .copied()
                .expect("Expected column 12 to be a uint16!"),
            RewardType1: row
                .columns[0]
                .into_u8()
                .copied()
                .expect("Expected column 0 to be a uint8!"),
            Unknown0: row
                .columns[4]
                .into_u8()
                .copied()
                .expect("Expected column 4 to be a uint8!"),
            RewardType2: row
                .columns[5]
                .into_u8()
                .copied()
                .expect("Expected column 5 to be a uint8!"),
            RewardType3: row
                .columns[9]
                .into_u8()
                .copied()
                .expect("Expected column 9 to be a uint8!"),
            RewardHq2: row
                .columns[7]
                .into_bool()
                .copied()
                .expect("Expected column 7 to be a bool!"),
            RewardHq3: row
                .columns[11]
                .into_bool()
                .copied()
                .expect("Expected column 11 to be a bool!"),
            RewardHq1: row
                .columns[2]
                .into_bool()
                .copied()
                .expect("Expected column 2 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a WeeklyBingoRewardDataSheet {
    type Item = (u32, Vec<(u16, WeeklyBingoRewardDataRow)>);
    type IntoIter = StructuredSheetIterator<'a, WeeklyBingoRewardDataSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, WeeklyBingoRewardDataSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct WeeklyBingoRewardDataRow {
    ///""
    pub RewardItem1: u32,
    ///""
    pub RewardItem2: u32,
    ///""
    pub RewardItem3: u32,
    ///""
    pub RewardQuantity1: u16,
    ///""
    pub RewardQuantity2: u16,
    ///""
    pub RewardQuantity3: u16,
    ///""
    pub RewardType1: u8,
    ///""
    pub Unknown0: u8,
    ///""
    pub RewardType2: u8,
    ///""
    pub RewardType3: u8,
    ///""
    pub RewardHq2: bool,
    ///""
    pub RewardHq3: bool,
    ///""
    pub RewardHq1: bool,
}
