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
pub struct AirshipExplorationPointSheet {
    sheet: Sheet,
}
impl AirshipExplorationPointSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("AirshipExplorationPoint")?;
        let sheet = resolver
            .read_excel_sheet(&exh, "AirshipExplorationPoint", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<AirshipExplorationPointRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<AirshipExplorationPointRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for AirshipExplorationPointSheet {
    type Row = AirshipExplorationPointRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Name: row
                .columns[0]
                .into_string()
                .cloned()
                .expect("Expected column 0 to be a string!"),
            NameShort: row
                .columns[1]
                .into_string()
                .cloned()
                .expect("Expected column 1 to be a string!"),
            ExpReward: row
                .columns[13]
                .into_u32()
                .copied()
                .expect("Expected column 13 to be a uint32!"),
            CeruleumTankReq: row
                .columns[6]
                .into_u16()
                .copied()
                .expect("Expected column 6 to be a uint16!"),
            SurveyDurationmin: row
                .columns[7]
                .into_u16()
                .copied()
                .expect("Expected column 7 to be a uint16!"),
            SurveyDistance: row
                .columns[8]
                .into_u16()
                .copied()
                .expect("Expected column 8 to be a uint16!"),
            X: row
                .columns[3]
                .into_i16()
                .copied()
                .expect("Expected column 3 to be a int16!"),
            Y: row
                .columns[4]
                .into_i16()
                .copied()
                .expect("Expected column 4 to be a int16!"),
            RankReq: row
                .columns[5]
                .into_u8()
                .copied()
                .expect("Expected column 5 to be a uint8!"),
            Unknown0: row
                .columns[9]
                .into_u8()
                .copied()
                .expect("Expected column 9 to be a uint8!"),
            Unknown1: row
                .columns[11]
                .into_u8()
                .copied()
                .expect("Expected column 11 to be a uint8!"),
            SurveillanceReq: row
                .columns[10]
                .into_u8()
                .copied()
                .expect("Expected column 10 to be a uint8!"),
            Unknown2: row
                .columns[12]
                .into_u8()
                .copied()
                .expect("Expected column 12 to be a uint8!"),
            Passengers: row
                .columns[2]
                .into_bool()
                .copied()
                .expect("Expected column 2 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a AirshipExplorationPointSheet {
    type Item = (u32, Vec<(u16, AirshipExplorationPointRow)>);
    type IntoIter = StructuredSheetIterator<'a, AirshipExplorationPointSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, AirshipExplorationPointSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AirshipExplorationPointRow {
    ///""
    pub Name: String,
    ///""
    pub NameShort: String,
    ///""
    pub ExpReward: u32,
    ///""
    pub CeruleumTankReq: u16,
    ///""
    pub SurveyDurationmin: u16,
    ///""
    pub SurveyDistance: u16,
    ///""
    pub X: i16,
    ///""
    pub Y: i16,
    ///""
    pub RankReq: u8,
    ///""
    pub Unknown0: u8,
    ///""
    pub Unknown1: u8,
    ///""
    pub SurveillanceReq: u8,
    ///""
    pub Unknown2: u8,
    ///""
    pub Passengers: bool,
}
