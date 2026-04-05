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
impl<'a> StructuredSheet<'a> for AirshipExplorationPointSheet {
    type Row = AirshipExplorationPointRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a AirshipExplorationPointSheet {
    type Item = (u32, Vec<(u16, AirshipExplorationPointRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, AirshipExplorationPointSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, AirshipExplorationPointSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct AirshipExplorationPointRow<'a> {
    row: &'a Row,
}
impl<'a> AirshipExplorationPointRow<'a> {
    pub fn Name(&'a self) -> &'a str {
        self.row.columns[0].into_string().unwrap()
    }
    pub fn NameShort(&'a self) -> &'a str {
        self.row.columns[1].into_string().unwrap()
    }
    pub fn ExpReward(&'a self) -> u32 {
        self.row.columns[13].into_u32().copied().unwrap()
    }
    pub fn CeruleumTankReq(&'a self) -> u16 {
        self.row.columns[6].into_u16().copied().unwrap()
    }
    pub fn SurveyDurationmin(&'a self) -> u16 {
        self.row.columns[7].into_u16().copied().unwrap()
    }
    pub fn SurveyDistance(&'a self) -> u16 {
        self.row.columns[8].into_u16().copied().unwrap()
    }
    pub fn X(&'a self) -> i16 {
        self.row.columns[3].into_i16().copied().unwrap()
    }
    pub fn Y(&'a self) -> i16 {
        self.row.columns[4].into_i16().copied().unwrap()
    }
    pub fn RankReq(&'a self) -> u8 {
        self.row.columns[5].into_u8().copied().unwrap()
    }
    pub fn Unknown0(&'a self) -> u8 {
        self.row.columns[9].into_u8().copied().unwrap()
    }
    pub fn Unknown1(&'a self) -> u8 {
        self.row.columns[11].into_u8().copied().unwrap()
    }
    pub fn SurveillanceReq(&'a self) -> u8 {
        self.row.columns[10].into_u8().copied().unwrap()
    }
    pub fn Unknown2(&'a self) -> u8 {
        self.row.columns[12].into_u8().copied().unwrap()
    }
    pub fn Passengers(&'a self) -> bool {
        self.row.columns[2].into_bool().copied().unwrap()
    }
}
