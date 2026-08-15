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
pub struct SubmarineExplorationSheet {
    sheet: Sheet,
}
impl SubmarineExplorationSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("SubmarineExploration")?;
        let sheet = resolver.read_excel_sheet(&exh, "SubmarineExploration", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<SubmarineExplorationRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<SubmarineExplorationRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for SubmarineExplorationSheet {
    type Row = SubmarineExplorationRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Destination: row
                .columns[0]
                .into_string()
                .cloned()
                .expect("Expected column 0 to be a string!"),
            Location: row
                .columns[1]
                .into_string()
                .cloned()
                .expect("Expected column 1 to be a string!"),
            ExpReward: row
                .columns[12]
                .into_u32()
                .copied()
                .expect("Expected column 12 to be a uint32!"),
            SurveyDurationmin: row
                .columns[10]
                .into_u16()
                .copied()
                .expect("Expected column 10 to be a uint16!"),
            X: row
                .columns[2]
                .into_i16()
                .copied()
                .expect("Expected column 2 to be a int16!"),
            Y: row
                .columns[3]
                .into_i16()
                .copied()
                .expect("Expected column 3 to be a int16!"),
            Z: row
                .columns[4]
                .into_i16()
                .copied()
                .expect("Expected column 4 to be a int16!"),
            Map: row
                .columns[5]
                .into_u8()
                .copied()
                .expect("Expected column 5 to be a uint8!"),
            Stars: row
                .columns[7]
                .into_u8()
                .copied()
                .expect("Expected column 7 to be a uint8!"),
            RankReq: row
                .columns[8]
                .into_u8()
                .copied()
                .expect("Expected column 8 to be a uint8!"),
            CeruleumTankReq: row
                .columns[9]
                .into_u8()
                .copied()
                .expect("Expected column 9 to be a uint8!"),
            SurveyDistance: row
                .columns[11]
                .into_u8()
                .copied()
                .expect("Expected column 11 to be a uint8!"),
            StartingPoint: row
                .columns[6]
                .into_bool()
                .copied()
                .expect("Expected column 6 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a SubmarineExplorationSheet {
    type Item = (u32, Vec<(u16, SubmarineExplorationRow)>);
    type IntoIter = StructuredSheetIterator<'a, SubmarineExplorationSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, SubmarineExplorationSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SubmarineExplorationRow {
    ///""
    pub Destination: String,
    ///""
    pub Location: String,
    ///""
    pub ExpReward: u32,
    ///""
    pub SurveyDurationmin: u16,
    ///""
    pub X: i16,
    ///""
    pub Y: i16,
    ///""
    pub Z: i16,
    ///""
    pub Map: u8,
    ///""
    pub Stars: u8,
    ///""
    pub RankReq: u8,
    ///""
    pub CeruleumTankReq: u8,
    ///""
    pub SurveyDistance: u8,
    ///""
    pub StartingPoint: bool,
}
