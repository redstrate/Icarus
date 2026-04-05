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
pub struct SatisfactionSupplyRewardExpSheet {
    sheet: Sheet,
}
impl SatisfactionSupplyRewardExpSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("SatisfactionSupplyRewardExp")?;
        let sheet = resolver
            .read_excel_sheet(&exh, "SatisfactionSupplyRewardExp", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<SatisfactionSupplyRewardExpRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<SatisfactionSupplyRewardExpRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for SatisfactionSupplyRewardExpSheet {
    type Row = SatisfactionSupplyRewardExpRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a SatisfactionSupplyRewardExpSheet {
    type Item = (u32, Vec<(u16, SatisfactionSupplyRewardExpRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, SatisfactionSupplyRewardExpSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, SatisfactionSupplyRewardExpSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct SatisfactionSupplyRewardExpRow<'a> {
    row: &'a Row,
}
impl<'a> SatisfactionSupplyRewardExpRow<'a> {
    pub fn PercentOfLevelLow(&'a self) -> u8 {
        self.row.columns[0].into_u8().copied().unwrap()
    }
    pub fn PercentOfLevelMid(&'a self) -> u8 {
        self.row.columns[1].into_u8().copied().unwrap()
    }
    pub fn PercentOfLevelHigh(&'a self) -> u8 {
        self.row.columns[2].into_u8().copied().unwrap()
    }
}
