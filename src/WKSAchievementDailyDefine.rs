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
pub struct WKSAchievementDailyDefineSheet {
    sheet: Sheet,
}
impl WKSAchievementDailyDefineSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("WKSAchievementDailyDefine")?;
        let sheet = resolver
            .read_excel_sheet(&exh, "WKSAchievementDailyDefine", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<WKSAchievementDailyDefineRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<WKSAchievementDailyDefineRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for WKSAchievementDailyDefineSheet {
    type Row = WKSAchievementDailyDefineRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a WKSAchievementDailyDefineSheet {
    type Item = (u32, Vec<(u16, WKSAchievementDailyDefineRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, WKSAchievementDailyDefineSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, WKSAchievementDailyDefineSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct WKSAchievementDailyDefineRow<'a> {
    row: &'a Row,
}
impl<'a> WKSAchievementDailyDefineRow<'a> {
    pub fn Unknown0(&'a self) -> u32 {
        self.row.columns[1].into_u32().copied().unwrap()
    }
    pub fn Unknown1(&'a self) -> u32 {
        self.row.columns[4].into_u32().copied().unwrap()
    }
    pub fn Unknown2(&'a self) -> u32 {
        self.row.columns[7].into_u32().copied().unwrap()
    }
    pub fn Unknown3(&'a self) -> u32 {
        self.row.columns[10].into_u32().copied().unwrap()
    }
    pub fn RewardItem(&'a self) -> [u32; 2] {
        [
            self.row.columns[2].into_u32().copied().unwrap(),
            self.row.columns[5].into_u32().copied().unwrap(),
        ]
    }
    pub fn Unknown4(&'a self) -> u32 {
        self.row.columns[8].into_u32().copied().unwrap()
    }
    pub fn Unknown5(&'a self) -> u32 {
        self.row.columns[11].into_u32().copied().unwrap()
    }
    pub fn SuccessPointsRequired(&'a self) -> [u32; 2] {
        [
            self.row.columns[3].into_u32().copied().unwrap(),
            self.row.columns[6].into_u32().copied().unwrap(),
        ]
    }
    pub fn Unknown6(&'a self) -> u32 {
        self.row.columns[9].into_u32().copied().unwrap()
    }
    pub fn Unknown7(&'a self) -> u32 {
        self.row.columns[12].into_u32().copied().unwrap()
    }
    pub fn RewardQuantity(&'a self) -> u16 {
        self.row.columns[0].into_u16().copied().unwrap()
    }
}
