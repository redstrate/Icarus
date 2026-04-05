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
pub struct LeveSheet {
    sheet: Sheet,
}
impl LeveSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Leve")?;
        let sheet = resolver.read_excel_sheet(&exh, "Leve", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<LeveRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<LeveRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for LeveSheet {
    type Row = LeveRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a LeveSheet {
    type Item = (u32, Vec<(u16, LeveRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, LeveSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, LeveSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct LeveRow<'a> {
    row: &'a Row,
}
impl<'a> LeveRow<'a> {
    pub fn Name(&'a self) -> &'a str {
        self.row.columns[0].into_string().unwrap()
    }
    pub fn Description(&'a self) -> &'a str {
        self.row.columns[1].into_string().unwrap()
    }
    pub fn ExpFactor(&'a self) -> f32 {
        self.row.columns[20].into_f32().copied().unwrap()
    }
    pub fn ExpReward(&'a self) -> u32 {
        self.row.columns[21].into_u32().copied().unwrap()
    }
    pub fn GilReward(&'a self) -> u32 {
        self.row.columns[22].into_u32().copied().unwrap()
    }
    pub fn LeveRewardItem(&'a self) -> u16 {
        self.row.columns[23].into_u16().copied().unwrap()
    }
    pub fn JournalGenre(&'a self) -> u32 {
        self.row.columns[14].into_u32().copied().unwrap()
    }
    pub fn LevelLevemete(&'a self) -> u32 {
        self.row.columns[26].into_u32().copied().unwrap()
    }
    pub fn LevelStart(&'a self) -> u32 {
        self.row.columns[29].into_u32().copied().unwrap()
    }
    pub fn LeveClient(&'a self) -> i32 {
        self.row.columns[2].into_i32().copied().unwrap()
    }
    pub fn LeveAssignmentType(&'a self) -> i32 {
        self.row.columns[4].into_i32().copied().unwrap()
    }
    pub fn Town(&'a self) -> i32 {
        self.row.columns[5].into_i32().copied().unwrap()
    }
    pub fn PlaceNameStart(&'a self) -> i32 {
        self.row.columns[9].into_i32().copied().unwrap()
    }
    pub fn PlaceNameIssued(&'a self) -> i32 {
        self.row.columns[10].into_i32().copied().unwrap()
    }
    pub fn PlaceNameStartZone(&'a self) -> i32 {
        self.row.columns[15].into_i32().copied().unwrap()
    }
    pub fn IconCityState(&'a self) -> i32 {
        self.row.columns[16].into_i32().copied().unwrap()
    }
    pub fn DataId(&'a self) -> i32 {
        self.row.columns[17].into_i32().copied().unwrap()
    }
    pub fn IconIssuer(&'a self) -> i32 {
        self.row.columns[27].into_i32().copied().unwrap()
    }
    pub fn ClassJobLevel(&'a self) -> u16 {
        self.row.columns[6].into_u16().copied().unwrap()
    }
    pub fn FishingSpot(&'a self) -> u16 {
        self.row.columns[11].into_u16().copied().unwrap()
    }
    pub fn BGM(&'a self) -> u16 {
        self.row.columns[30].into_u16().copied().unwrap()
    }
    pub fn Unknown1(&'a self) -> u8 {
        self.row.columns[3].into_u8().copied().unwrap()
    }
    pub fn TimeLimit(&'a self) -> u8 {
        self.row.columns[7].into_u8().copied().unwrap()
    }
    pub fn AllowanceCost(&'a self) -> u8 {
        self.row.columns[8].into_u8().copied().unwrap()
    }
    pub fn Unknown2(&'a self) -> u8 {
        self.row.columns[12].into_u8().copied().unwrap()
    }
    pub fn ClassJobCategory(&'a self) -> u8 {
        self.row.columns[13].into_u8().copied().unwrap()
    }
    pub fn MaxDifficulty(&'a self) -> u8 {
        self.row.columns[19].into_u8().copied().unwrap()
    }
    pub fn LeveVfx(&'a self) -> u8 {
        self.row.columns[24].into_u8().copied().unwrap()
    }
    pub fn LeveVfxFrame(&'a self) -> u8 {
        self.row.columns[25].into_u8().copied().unwrap()
    }
    pub fn CanCancel(&'a self) -> bool {
        self.row.columns[18].into_bool().copied().unwrap()
    }
    pub fn LockedLeve(&'a self) -> bool {
        self.row.columns[28].into_bool().copied().unwrap()
    }
}
