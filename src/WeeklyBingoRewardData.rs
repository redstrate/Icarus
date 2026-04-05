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
impl<'a> StructuredSheet<'a> for WeeklyBingoRewardDataSheet {
    type Row = WeeklyBingoRewardDataRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a WeeklyBingoRewardDataSheet {
    type Item = (u32, Vec<(u16, WeeklyBingoRewardDataRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, WeeklyBingoRewardDataSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, WeeklyBingoRewardDataSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct WeeklyBingoRewardDataRow<'a> {
    row: &'a Row,
}
impl<'a> WeeklyBingoRewardDataRow<'a> {
    pub fn RewardItem1(&'a self) -> u32 {
        self.row.columns[1].into_u32().copied().unwrap()
    }
    pub fn RewardItem2(&'a self) -> u32 {
        self.row.columns[6].into_u32().copied().unwrap()
    }
    pub fn RewardItem3(&'a self) -> u32 {
        self.row.columns[10].into_u32().copied().unwrap()
    }
    pub fn RewardQuantity1(&'a self) -> u16 {
        self.row.columns[3].into_u16().copied().unwrap()
    }
    pub fn RewardQuantity2(&'a self) -> u16 {
        self.row.columns[8].into_u16().copied().unwrap()
    }
    pub fn RewardQuantity3(&'a self) -> u16 {
        self.row.columns[12].into_u16().copied().unwrap()
    }
    pub fn RewardType1(&'a self) -> u8 {
        self.row.columns[0].into_u8().copied().unwrap()
    }
    pub fn Unknown0(&'a self) -> u8 {
        self.row.columns[4].into_u8().copied().unwrap()
    }
    pub fn RewardType2(&'a self) -> u8 {
        self.row.columns[5].into_u8().copied().unwrap()
    }
    pub fn RewardType3(&'a self) -> u8 {
        self.row.columns[9].into_u8().copied().unwrap()
    }
    pub fn RewardHq2(&'a self) -> bool {
        self.row.columns[7].into_bool().copied().unwrap()
    }
    pub fn RewardHq3(&'a self) -> bool {
        self.row.columns[11].into_bool().copied().unwrap()
    }
    pub fn RewardHq1(&'a self) -> bool {
        self.row.columns[2].into_bool().copied().unwrap()
    }
}
