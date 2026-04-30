//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
#[derive(Clone, Debug, PartialEq)]
pub struct SatisfactionSupplyRewardDataElement {
    pub RewardCurrency: u16,
    pub QuantityLow: u16,
    pub QuantityMid: u16,
    pub QuantityHigh: u16,
}
#[derive(Debug, Clone)]
pub struct SatisfactionSupplyRewardSheet {
    sheet: Sheet,
}
impl SatisfactionSupplyRewardSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("SatisfactionSupplyReward")?;
        let sheet = resolver
            .read_excel_sheet(&exh, "SatisfactionSupplyReward", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<SatisfactionSupplyRewardRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<SatisfactionSupplyRewardRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for SatisfactionSupplyRewardSheet {
    type Row = SatisfactionSupplyRewardRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            SatisfactionSupplyRewardData: [
                SatisfactionSupplyRewardDataElement {
                    RewardCurrency: row
                        .columns[1]
                        .into_u16()
                        .copied()
                        .expect("Expected column 1 to be a uint16!"),
                    QuantityLow: row
                        .columns[2]
                        .into_u16()
                        .copied()
                        .expect("Expected column 2 to be a uint16!"),
                    QuantityMid: row
                        .columns[3]
                        .into_u16()
                        .copied()
                        .expect("Expected column 3 to be a uint16!"),
                    QuantityHigh: row
                        .columns[4]
                        .into_u16()
                        .copied()
                        .expect("Expected column 4 to be a uint16!"),
                },
                SatisfactionSupplyRewardDataElement {
                    RewardCurrency: row
                        .columns[5]
                        .into_u16()
                        .copied()
                        .expect("Expected column 5 to be a uint16!"),
                    QuantityLow: row
                        .columns[6]
                        .into_u16()
                        .copied()
                        .expect("Expected column 6 to be a uint16!"),
                    QuantityMid: row
                        .columns[7]
                        .into_u16()
                        .copied()
                        .expect("Expected column 7 to be a uint16!"),
                    QuantityHigh: row
                        .columns[8]
                        .into_u16()
                        .copied()
                        .expect("Expected column 8 to be a uint16!"),
                },
            ],
            SatisfactionLow: row
                .columns[10]
                .into_u16()
                .copied()
                .expect("Expected column 10 to be a uint16!"),
            SatisfactionMid: row
                .columns[11]
                .into_u16()
                .copied()
                .expect("Expected column 11 to be a uint16!"),
            SatisfactionHigh: row
                .columns[12]
                .into_u16()
                .copied()
                .expect("Expected column 12 to be a uint16!"),
            GilLow: row
                .columns[13]
                .into_u16()
                .copied()
                .expect("Expected column 13 to be a uint16!"),
            GilMid: row
                .columns[14]
                .into_u16()
                .copied()
                .expect("Expected column 14 to be a uint16!"),
            GilHigh: row
                .columns[15]
                .into_u16()
                .copied()
                .expect("Expected column 15 to be a uint16!"),
            BonusMultiplier: row
                .columns[0]
                .into_u8()
                .copied()
                .expect("Expected column 0 to be a uint8!"),
            MinLevelForSecondReward: row
                .columns[9]
                .into_u8()
                .copied()
                .expect("Expected column 9 to be a uint8!"),
        })
    }
}
impl<'a> IntoIterator for &'a SatisfactionSupplyRewardSheet {
    type Item = (u32, Vec<(u16, SatisfactionSupplyRewardRow)>);
    type IntoIter = StructuredSheetIterator<'a, SatisfactionSupplyRewardSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, SatisfactionSupplyRewardSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SatisfactionSupplyRewardRow {
    ///""
    pub SatisfactionSupplyRewardData: [SatisfactionSupplyRewardDataElement; 2],
    ///""
    pub SatisfactionLow: u16,
    ///""
    pub SatisfactionMid: u16,
    ///""
    pub SatisfactionHigh: u16,
    ///""
    pub GilLow: u16,
    ///""
    pub GilMid: u16,
    ///""
    pub GilHigh: u16,
    ///""
    pub BonusMultiplier: u8,
    ///"0 == current cap"
    pub MinLevelForSecondReward: u8,
}
