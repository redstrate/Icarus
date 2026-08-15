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
pub struct WKSEmergencyInfoSheet {
    sheet: Sheet,
}
impl WKSEmergencyInfoSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("WKSEmergencyInfo")?;
        let sheet = resolver.read_excel_sheet(&exh, "WKSEmergencyInfo", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<WKSEmergencyInfoRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<WKSEmergencyInfoRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for WKSEmergencyInfoSheet {
    type Row = WKSEmergencyInfoRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            EmergencyProblem: [
                row
                    .columns[2]
                    .into_u16()
                    .copied()
                    .expect("Expected column 2 to be a uint16!"),
                row
                    .columns[3]
                    .into_u16()
                    .copied()
                    .expect("Expected column 3 to be a uint16!"),
            ],
            WKSEmergencyMissionRowId: row
                .columns[4]
                .into_u16()
                .copied()
                .expect("Expected column 4 to be a uint16!"),
            Name: row
                .columns[0]
                .into_u8()
                .copied()
                .expect("Expected column 0 to be a uint8!"),
            WKSEmergencyWarningText: row
                .columns[1]
                .into_u8()
                .copied()
                .expect("Expected column 1 to be a uint8!"),
        })
    }
}
impl<'a> IntoIterator for &'a WKSEmergencyInfoSheet {
    type Item = (u32, Vec<(u16, WKSEmergencyInfoRow)>);
    type IntoIter = StructuredSheetIterator<'a, WKSEmergencyInfoSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, WKSEmergencyInfoSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct WKSEmergencyInfoRow {
    ///""
    pub EmergencyProblem: [u16; 2],
    ///"Has between 5 and 6 subrows"
    pub WKSEmergencyMissionRowId: u16,
    ///""
    pub Name: u8,
    ///""
    pub WKSEmergencyWarningText: u8,
}
