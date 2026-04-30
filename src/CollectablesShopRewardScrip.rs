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
pub struct CollectablesShopRewardScripSheet {
    sheet: Sheet,
}
impl CollectablesShopRewardScripSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("CollectablesShopRewardScrip")?;
        let sheet = resolver
            .read_excel_sheet(&exh, "CollectablesShopRewardScrip", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<CollectablesShopRewardScripRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<CollectablesShopRewardScripRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for CollectablesShopRewardScripSheet {
    type Row = CollectablesShopRewardScripRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Currency: row
                .columns[0]
                .into_u16()
                .copied()
                .expect("Expected column 0 to be a uint16!"),
            LowReward: row
                .columns[1]
                .into_u16()
                .copied()
                .expect("Expected column 1 to be a uint16!"),
            MidReward: row
                .columns[2]
                .into_u16()
                .copied()
                .expect("Expected column 2 to be a uint16!"),
            HighReward: row
                .columns[3]
                .into_u16()
                .copied()
                .expect("Expected column 3 to be a uint16!"),
            ExpRatioLow: row
                .columns[4]
                .into_u16()
                .copied()
                .expect("Expected column 4 to be a uint16!"),
            ExpRatioMid: row
                .columns[5]
                .into_u16()
                .copied()
                .expect("Expected column 5 to be a uint16!"),
            ExpRatioHigh: row
                .columns[6]
                .into_u16()
                .copied()
                .expect("Expected column 6 to be a uint16!"),
        })
    }
}
impl<'a> IntoIterator for &'a CollectablesShopRewardScripSheet {
    type Item = (u32, Vec<(u16, CollectablesShopRewardScripRow)>);
    type IntoIter = StructuredSheetIterator<'a, CollectablesShopRewardScripSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, CollectablesShopRewardScripSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CollectablesShopRewardScripRow {
    ///""
    pub Currency: u16,
    ///""
    pub LowReward: u16,
    ///""
    pub MidReward: u16,
    ///""
    pub HighReward: u16,
    ///""
    pub ExpRatioLow: u16,
    ///""
    pub ExpRatioMid: u16,
    ///""
    pub ExpRatioHigh: u16,
}
