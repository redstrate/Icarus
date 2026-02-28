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
    index_mapping: Vec<usize>,
}
impl WKSMissionRewardSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("WKSMissionReward")?;
        let sheet = resolver.read_excel_sheet(&exh, "WKSMissionReward", language)?;
        let mut index_mapping: Vec<(usize, &ExcelColumnDefinition)> = sheet
            .exh
            .column_definitions
            .iter()
            .enumerate()
            .collect();
        index_mapping.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
        let index_mapping: Vec<usize> = index_mapping
            .iter()
            .map(|(index, _)| *index)
            .collect();
        Ok(Self { sheet, index_mapping })
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
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
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
    index_mapping: Vec<usize>,
}
impl<'a> WKSMissionRewardRow<'a> {
    pub fn Item(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[0]]
    }
    /// ExpReward = ExpToNex * (lvl < 50 ? ExpModifier[0] : lvl < 90 ? ExpModifier[1] : ExpModifier[2]) / 100
    pub fn ExpModifier(&'a self) -> [&'a Field; 3] {
        [
            &self.row.columns[self.index_mapping[1]],
            &self.row.columns[self.index_mapping[2]],
            &self.row.columns[self.index_mapping[3]],
        ]
    }
    pub fn CosmoCredits(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[4]]
    }
    pub fn PlanetCredits(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[5]]
    }
    pub fn Unknown20(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[6]]
    }
    pub fn ResearchReward(&'a self) -> [&'a Field; 3] {
        [
            &self.row.columns[self.index_mapping[7]],
            &self.row.columns[self.index_mapping[8]],
            &self.row.columns[self.index_mapping[9]],
        ]
    }
    pub fn ItemCount(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[10]]
    }
    /// Needs to match WKSEmergencyProblem.Unknown2 to be active?
    pub fn Unknown19(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[11]]
    }
    pub fn Tool(&'a self) -> [&'a Field; 3] {
        [
            &self.row.columns[self.index_mapping[12]],
            &self.row.columns[self.index_mapping[13]],
            &self.row.columns[self.index_mapping[14]],
        ]
    }
    pub fn TypeIndex(&'a self) -> [&'a Field; 3] {
        [
            &self.row.columns[self.index_mapping[15]],
            &self.row.columns[self.index_mapping[16]],
            &self.row.columns[self.index_mapping[17]],
        ]
    }
}
