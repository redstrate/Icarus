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
pub struct WKSMissionRewardSheet {
    sheet: Sheet,
}
impl WKSMissionRewardSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("WKSMissionReward")?;
        let sheet = resolver.read_excel_sheet(&exh, "WKSMissionReward", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<WKSMissionRewardRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<WKSMissionRewardRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for WKSMissionRewardSheet {
    type Row = WKSMissionRewardRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a WKSMissionRewardSheet {
    type Item = (u32, Vec<(u16, WKSMissionRewardRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, WKSMissionRewardSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, WKSMissionRewardSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct WKSMissionRewardRow<'a> {
    row: &'a Row,
}
impl<'a> WKSMissionRewardRow<'a> {
    pub fn Item(&'a self) -> u32 {
        self.row.columns[16].into_u32().copied().unwrap()
    }
    /// ExpReward = ExpToNex * (lvl < 50 ? ExpModifier[0] : lvl < 90 ? ExpModifier[1] : ExpModifier[2]) / 100
    pub fn ExpModifier(&'a self) -> [u16; 3] {
        [
            self.row.columns[0].into_u16().copied().unwrap(),
            self.row.columns[1].into_u16().copied().unwrap(),
            self.row.columns[2].into_u16().copied().unwrap(),
        ]
    }
    pub fn CosmoCredits(&'a self) -> u16 {
        self.row.columns[3].into_u16().copied().unwrap()
    }
    pub fn PlanetCredits(&'a self) -> u16 {
        self.row.columns[4].into_u16().copied().unwrap()
    }
    pub fn Unknown20(&'a self) -> u16 {
        self.row.columns[5].into_u16().copied().unwrap()
    }
    pub fn ResearchReward(&'a self) -> [u16; 3] {
        [
            self.row.columns[9].into_u16().copied().unwrap(),
            self.row.columns[12].into_u16().copied().unwrap(),
            self.row.columns[15].into_u16().copied().unwrap(),
        ]
    }
    pub fn ItemCount(&'a self) -> u16 {
        self.row.columns[17].into_u16().copied().unwrap()
    }
    /// Needs to match WKSEmergencyProblem.Unknown2 to be active?
    pub fn Unknown19(&'a self) -> u8 {
        self.row.columns[6].into_u8().copied().unwrap()
    }
    pub fn Tool(&'a self) -> [u8; 3] {
        [
            self.row.columns[7].into_u8().copied().unwrap(),
            self.row.columns[10].into_u8().copied().unwrap(),
            self.row.columns[13].into_u8().copied().unwrap(),
        ]
    }
    pub fn TypeIndex(&'a self) -> [u8; 3] {
        [
            self.row.columns[8].into_u8().copied().unwrap(),
            self.row.columns[11].into_u8().copied().unwrap(),
            self.row.columns[14].into_u8().copied().unwrap(),
        ]
    }
}
