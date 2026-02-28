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
pub struct WKSMissionUnitSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl WKSMissionUnitSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("WKSMissionUnit")?;
        let sheet = resolver.read_excel_sheet(&exh, "WKSMissionUnit", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<WKSMissionUnitRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<WKSMissionUnitRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for WKSMissionUnitSheet {
    type Row = WKSMissionUnitRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a WKSMissionUnitSheet {
    type Item = (u32, Vec<(u16, WKSMissionUnitRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, WKSMissionUnitSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, WKSMissionUnitSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct WKSMissionUnitRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> WKSMissionUnitRow<'a> {
    pub fn Name(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[0]]
    }
    pub fn WKSMissionText(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1]]
    }
    pub fn ClassJobCategory(&'a self) -> [&'a Field; 2] {
        [
            &self.row.columns[self.index_mapping[2]],
            &self.row.columns[self.index_mapping[3]],
        ]
    }
    pub fn MissionTime(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[4]]
    }
    pub fn MissionReward(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[5]]
    }
    pub fn SilverStarRequirement(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[6]]
    }
    pub fn GoldStarRequirement(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[7]]
    }
    pub fn MissionToDo(&'a self) -> [&'a Field; 3] {
        [
            &self.row.columns[self.index_mapping[8]],
            &self.row.columns[self.index_mapping[9]],
            &self.row.columns[self.index_mapping[10]],
        ]
    }
    pub fn LockedBehind(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[11]]
    }
    pub fn WKSMissionSupplyItem(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[12]]
    }
    pub fn WKSMissionRecipe(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[13]]
    }
    pub fn PlaceName(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[14]]
    }
    pub fn SortKey(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[15]]
    }
    pub fn WKSFunction(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[16]]
    }
    pub fn LevelGroup(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[17]]
    }
    pub fn Unknown0(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[18]]
    }
    pub fn WKSMissionLotterySpecialCond(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[19]]
    }
    pub fn IsSynced(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[20]]
    }
    pub fn IsSpecialQuest(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[21]]
    }
}
