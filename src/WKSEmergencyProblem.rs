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
pub struct WKSEmergencyProblemSheet {
    sheet: Sheet,
}
impl WKSEmergencyProblemSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("WKSEmergencyProblem")?;
        let sheet = resolver.read_excel_sheet(&exh, "WKSEmergencyProblem", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<WKSEmergencyProblemRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<WKSEmergencyProblemRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for WKSEmergencyProblemSheet {
    type Row = WKSEmergencyProblemRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            EmergencyInfoTeleportText1: row
                .columns[0]
                .into_u32()
                .copied()
                .expect("Expected column 0 to be a uint32!"),
            EmergencyInfoTeleportText2: row
                .columns[1]
                .into_u32()
                .copied()
                .expect("Expected column 1 to be a uint32!"),
            EventLayout: row
                .columns[2]
                .into_u32()
                .copied()
                .expect("Expected column 2 to be a uint32!"),
            RequiredProgress: row
                .columns[4]
                .into_u32()
                .copied()
                .expect("Expected column 4 to be a uint32!"),
            Unknown2: row
                .columns[3]
                .into_u8()
                .copied()
                .expect("Expected column 3 to be a uint8!"),
        })
    }
}
impl<'a> IntoIterator for &'a WKSEmergencyProblemSheet {
    type Item = (u32, Vec<(u16, WKSEmergencyProblemRow)>);
    type IntoIter = StructuredSheetIterator<'a, WKSEmergencyProblemSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, WKSEmergencyProblemSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct WKSEmergencyProblemRow {
    ///"The first option to teleport"
    pub EmergencyInfoTeleportText1: u32,
    ///"The second option to teleport"
    pub EmergencyInfoTeleportText2: u32,
    ///""
    pub EventLayout: u32,
    ///""
    pub RequiredProgress: u32,
    ///""
    pub Unknown2: u8,
}
