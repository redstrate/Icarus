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
pub struct SatisfactionSupplySheet {
    sheet: Sheet,
}
impl SatisfactionSupplySheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("SatisfactionSupply")?;
        let sheet = resolver.read_excel_sheet(&exh, "SatisfactionSupply", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<SatisfactionSupplyRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<SatisfactionSupplyRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for SatisfactionSupplySheet {
    type Row = SatisfactionSupplyRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Item: row
                .columns[2]
                .into_i32()
                .copied()
                .expect("Expected column 2 to be a int32!"),
            CollectabilityLow: row
                .columns[3]
                .into_u16()
                .copied()
                .expect("Expected column 3 to be a uint16!"),
            CollectabilityMid: row
                .columns[4]
                .into_u16()
                .copied()
                .expect("Expected column 4 to be a uint16!"),
            CollectabilityHigh: row
                .columns[5]
                .into_u16()
                .copied()
                .expect("Expected column 5 to be a uint16!"),
            Reward: row
                .columns[6]
                .into_u16()
                .copied()
                .expect("Expected column 6 to be a uint16!"),
            FishingSpotId: row
                .columns[8]
                .into_u16()
                .copied()
                .expect("Expected column 8 to be a uint16!"),
            SpearFishingSpotId: row
                .columns[9]
                .into_u16()
                .copied()
                .expect("Expected column 9 to be a uint16!"),
            Slot: row
                .columns[0]
                .into_u8()
                .copied()
                .expect("Expected column 0 to be a uint8!"),
            ProbabilityPercent: row
                .columns[1]
                .into_u8()
                .copied()
                .expect("Expected column 1 to be a uint8!"),
            IsBonus: row
                .columns[7]
                .into_bool()
                .copied()
                .expect("Expected column 7 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a SatisfactionSupplySheet {
    type Item = (u32, Vec<(u16, SatisfactionSupplyRow)>);
    type IntoIter = StructuredSheetIterator<'a, SatisfactionSupplySheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, SatisfactionSupplySheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SatisfactionSupplyRow {
    ///""
    pub Item: i32,
    ///""
    pub CollectabilityLow: u16,
    ///""
    pub CollectabilityMid: u16,
    ///""
    pub CollectabilityHigh: u16,
    ///""
    pub Reward: u16,
    ///""
    pub FishingSpotId: u16,
    ///""
    pub SpearFishingSpotId: u16,
    ///""
    pub Slot: u8,
    ///""
    pub ProbabilityPercent: u8,
    ///""
    pub IsBonus: bool,
}
