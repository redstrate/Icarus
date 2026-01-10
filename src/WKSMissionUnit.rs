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
}
impl WKSMissionUnitSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("WKSMissionUnit")?;
        let sheet = resolver.read_excel_sheet(&exh, "WKSMissionUnit", language)?;
        Ok(Self { sheet })
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
impl StructuredSheet for WKSMissionUnitSheet {
    type Row = WKSMissionUnitRow;
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
impl<'a> IntoIterator for &'a WKSMissionUnitSheet {
    type Item = (u32, Vec<(u16, WKSMissionUnitRow)>);
    type IntoIter = StructuredSheetIterator<'a, WKSMissionUnitSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, WKSMissionUnitSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct WKSMissionUnitRow {
    columns: Vec<Field>,
}
impl WKSMissionUnitRow {
    pub fn Name<'a>(&'a self) -> &'a Field {
        &self.columns[0]
    }
    pub fn WKSMissionText<'a>(&'a self) -> &'a Field {
        &self.columns[1]
    }
    pub fn ClassJobCategory<'a>(&'a self) -> [&'a Field; 2] {
        [&self.columns[2], &self.columns[3]]
    }
    pub fn MissionTime<'a>(&'a self) -> &'a Field {
        &self.columns[4]
    }
    pub fn MissionReward<'a>(&'a self) -> &'a Field {
        &self.columns[5]
    }
    pub fn SilverStarRequirement<'a>(&'a self) -> &'a Field {
        &self.columns[6]
    }
    pub fn GoldStarRequirement<'a>(&'a self) -> &'a Field {
        &self.columns[7]
    }
    pub fn MissionToDo<'a>(&'a self) -> [&'a Field; 3] {
        [&self.columns[8], &self.columns[9], &self.columns[10]]
    }
    pub fn LockedBehind<'a>(&'a self) -> &'a Field {
        &self.columns[11]
    }
    pub fn WKSMissionSupplyItem<'a>(&'a self) -> &'a Field {
        &self.columns[12]
    }
    pub fn WKSMissionRecipe<'a>(&'a self) -> &'a Field {
        &self.columns[13]
    }
    pub fn PlaceName<'a>(&'a self) -> &'a Field {
        &self.columns[14]
    }
    pub fn SortKey<'a>(&'a self) -> &'a Field {
        &self.columns[15]
    }
    pub fn WKSFunction<'a>(&'a self) -> &'a Field {
        &self.columns[16]
    }
    pub fn LevelGroup<'a>(&'a self) -> &'a Field {
        &self.columns[17]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a Field {
        &self.columns[18]
    }
    pub fn WKSMissionLotterySpecialCond<'a>(&'a self) -> &'a Field {
        &self.columns[19]
    }
    pub fn IsSynced<'a>(&'a self) -> &'a Field {
        &self.columns[20]
    }
    pub fn IsSpecialQuest<'a>(&'a self) -> &'a Field {
        &self.columns[21]
    }
}
