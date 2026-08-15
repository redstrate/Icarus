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
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
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
impl StructuredSheet for WKSMissionRewardSheet {
    type Row = WKSMissionRewardRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Item: row
                .columns[16]
                .into_u32()
                .copied()
                .expect("Expected column 16 to be a uint32!"),
            ExpModifier: [
                row
                    .columns[0]
                    .into_u16()
                    .copied()
                    .expect("Expected column 0 to be a uint16!"),
                row
                    .columns[1]
                    .into_u16()
                    .copied()
                    .expect("Expected column 1 to be a uint16!"),
                row
                    .columns[2]
                    .into_u16()
                    .copied()
                    .expect("Expected column 2 to be a uint16!"),
            ],
            CosmoCredits: row
                .columns[3]
                .into_u16()
                .copied()
                .expect("Expected column 3 to be a uint16!"),
            PlanetCredits: row
                .columns[4]
                .into_u16()
                .copied()
                .expect("Expected column 4 to be a uint16!"),
            BaseDronebits: row
                .columns[5]
                .into_u16()
                .copied()
                .expect("Expected column 5 to be a uint16!"),
            ResearchReward: [
                row
                    .columns[9]
                    .into_u16()
                    .copied()
                    .expect("Expected column 9 to be a uint16!"),
                row
                    .columns[12]
                    .into_u16()
                    .copied()
                    .expect("Expected column 12 to be a uint16!"),
                row
                    .columns[15]
                    .into_u16()
                    .copied()
                    .expect("Expected column 15 to be a uint16!"),
            ],
            ItemCount: row
                .columns[17]
                .into_u16()
                .copied()
                .expect("Expected column 17 to be a uint16!"),
            Unknown19: row
                .columns[6]
                .into_u8()
                .copied()
                .expect("Expected column 6 to be a uint8!"),
            Tool: [
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
            ],
            TypeIndex: [
                row
                    .columns[8]
                    .into_u8()
                    .copied()
                    .expect("Expected column 8 to be a uint8!"),
                row
                    .columns[11]
                    .into_u8()
                    .copied()
                    .expect("Expected column 11 to be a uint8!"),
                row
                    .columns[14]
                    .into_u8()
                    .copied()
                    .expect("Expected column 14 to be a uint8!"),
            ],
        })
    }
}
impl<'a> IntoIterator for &'a WKSMissionRewardSheet {
    type Item = (u32, Vec<(u16, WKSMissionRewardRow)>);
    type IntoIter = StructuredSheetIterator<'a, WKSMissionRewardSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, WKSMissionRewardSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct WKSMissionRewardRow {
    ///""
    pub Item: u32,
    ///"ExpReward = ExpToNex * (lvl < 50 ? ExpModifier[0] : lvl < 90 ? ExpModifier[1] : ExpModifier[2]) / 100"
    pub ExpModifier: [u16; 3],
    ///""
    pub CosmoCredits: u16,
    ///""
    pub PlanetCredits: u16,
    ///"Doesn't take Gold bonus in to account"
    pub BaseDronebits: u16,
    ///""
    pub ResearchReward: [u16; 3],
    ///""
    pub ItemCount: u16,
    ///"Needs to match WKSEmergencyProblem.Unknown2 to be active?"
    pub Unknown19: u8,
    ///""
    pub Tool: [u8; 3],
    ///""
    pub TypeIndex: [u8; 3],
}
