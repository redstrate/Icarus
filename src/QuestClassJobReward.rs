//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    common::Language,
};
#[derive(Debug, Clone)]
pub struct QuestClassJobRewardSheet {
    sheet: Sheet,
}
impl QuestClassJobRewardSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("QuestClassJobReward")?;
        let sheet = resolver.read_excel_sheet(&exh, "QuestClassJobReward", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<QuestClassJobRewardRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<QuestClassJobRewardRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for QuestClassJobRewardSheet {
    type Row = QuestClassJobRewardRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        let column_defs = &self.sheet.exh.column_definitions;
        let mut zipped: Vec<_> = row
            .columns
            .clone()
            .into_iter()
            .zip(column_defs)
            .collect();
        zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
        let (columns, _): (Vec<Field>, Vec<ExcelColumnDefinition>) = zipped
            .into_iter()
            .unzip();
        Some(Self::Row { columns })
    }
}
impl<'a> IntoIterator for &'a QuestClassJobRewardSheet {
    type Item = (u32, Vec<(u16, QuestClassJobRewardRow)>);
    type IntoIter = StructuredSheetIterator<'a, QuestClassJobRewardSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, QuestClassJobRewardSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct QuestClassJobRewardRow {
    columns: Vec<Field>,
}
impl QuestClassJobRewardRow {
    pub fn RewardItem<'a>(&'a self) -> [&'a Field; 4] {
        [&self.columns[0], &self.columns[1], &self.columns[2], &self.columns[3]]
    }
    pub fn RequiredItem<'a>(&'a self) -> [&'a Field; 4] {
        [&self.columns[4], &self.columns[5], &self.columns[6], &self.columns[7]]
    }
    pub fn Unknown8<'a>(&'a self) -> &'a Field {
        &self.columns[8]
    }
    pub fn Unknown9<'a>(&'a self) -> &'a Field {
        &self.columns[9]
    }
    pub fn ClassJobCategory<'a>(&'a self) -> &'a Field {
        &self.columns[10]
    }
    pub fn RewardAmount<'a>(&'a self) -> [&'a Field; 4] {
        [&self.columns[11], &self.columns[12], &self.columns[13], &self.columns[14]]
    }
    pub fn RequiredAmount<'a>(&'a self) -> [&'a Field; 4] {
        [&self.columns[15], &self.columns[16], &self.columns[17], &self.columns[18]]
    }
    pub fn Unknown10<'a>(&'a self) -> &'a Field {
        &self.columns[19]
    }
    pub fn Unknown11<'a>(&'a self) -> &'a Field {
        &self.columns[20]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a Field {
        &self.columns[21]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a Field {
        &self.columns[22]
    }
    pub fn Unknown2<'a>(&'a self) -> &'a Field {
        &self.columns[23]
    }
    pub fn Unknown3<'a>(&'a self) -> &'a Field {
        &self.columns[24]
    }
    pub fn Unknown4<'a>(&'a self) -> &'a Field {
        &self.columns[25]
    }
    pub fn Unknown5<'a>(&'a self) -> &'a Field {
        &self.columns[26]
    }
    pub fn Unknown6<'a>(&'a self) -> &'a Field {
        &self.columns[27]
    }
    pub fn Unknown7<'a>(&'a self) -> &'a Field {
        &self.columns[28]
    }
    pub fn Unknown12<'a>(&'a self) -> &'a Field {
        &self.columns[29]
    }
    pub fn Unknown13<'a>(&'a self) -> &'a Field {
        &self.columns[30]
    }
    pub fn Unknown14<'a>(&'a self) -> &'a Field {
        &self.columns[31]
    }
    pub fn Unknown15<'a>(&'a self) -> &'a Field {
        &self.columns[32]
    }
}
