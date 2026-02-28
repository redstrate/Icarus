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
pub struct WKSEmergencyMissionSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl WKSEmergencyMissionSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("WKSEmergencyMission")?;
        let sheet = resolver.read_excel_sheet(&exh, "WKSEmergencyMission", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<WKSEmergencyMissionRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<WKSEmergencyMissionRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for WKSEmergencyMissionSheet {
    type Row = WKSEmergencyMissionRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a WKSEmergencyMissionSheet {
    type Item = (u32, Vec<(u16, WKSEmergencyMissionRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, WKSEmergencyMissionSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, WKSEmergencyMissionSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct WKSEmergencyMissionRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> WKSEmergencyMissionRow<'a> {
    /// As of Patch 7.31 this can always be direct queried with subrow 0 to get the follow chain to WKSMissionUnit from the sheet
    pub fn WKSEmergencyMissionGroupRowId(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[0]]
    }
}
