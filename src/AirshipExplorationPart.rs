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
pub struct AirshipExplorationPartSheet {
    sheet: Sheet,
}
impl AirshipExplorationPartSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("AirshipExplorationPart")?;
        let sheet = resolver.read_excel_sheet(&exh, "AirshipExplorationPart", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<AirshipExplorationPartRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<AirshipExplorationPartRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for AirshipExplorationPartSheet {
    type Row = AirshipExplorationPartRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a AirshipExplorationPartSheet {
    type Item = (u32, Vec<(u16, AirshipExplorationPartRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, AirshipExplorationPartSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, AirshipExplorationPartSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct AirshipExplorationPartRow<'a> {
    row: &'a Row,
}
impl<'a> AirshipExplorationPartRow<'a> {
    pub fn Class(&'a self) -> u16 {
        self.row.columns[8].into_u16().copied().unwrap()
    }
    pub fn Surveillance(&'a self) -> i16 {
        self.row.columns[3].into_i16().copied().unwrap()
    }
    pub fn Retrieval(&'a self) -> i16 {
        self.row.columns[4].into_i16().copied().unwrap()
    }
    pub fn Speed(&'a self) -> i16 {
        self.row.columns[5].into_i16().copied().unwrap()
    }
    pub fn Range(&'a self) -> i16 {
        self.row.columns[6].into_i16().copied().unwrap()
    }
    pub fn Favor(&'a self) -> i16 {
        self.row.columns[7].into_i16().copied().unwrap()
    }
    pub fn Slot(&'a self) -> u8 {
        self.row.columns[0].into_u8().copied().unwrap()
    }
    pub fn Rank(&'a self) -> u8 {
        self.row.columns[1].into_u8().copied().unwrap()
    }
    pub fn Components(&'a self) -> u8 {
        self.row.columns[2].into_u8().copied().unwrap()
    }
    pub fn RepairMaterials(&'a self) -> u8 {
        self.row.columns[9].into_u8().copied().unwrap()
    }
}
