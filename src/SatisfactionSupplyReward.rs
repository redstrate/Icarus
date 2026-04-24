//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl<'a> StructuredSheet<'a> for SatisfactionSupplyRewardSheet {
    type Row = SatisfactionSupplyRewardRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a SatisfactionSupplyRewardSheet {
    type Item = (u32, Vec<(u16, SatisfactionSupplyRewardRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, SatisfactionSupplyRewardSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, SatisfactionSupplyRewardSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct SatisfactionSupplyRewardRow<'a> {
    row: &'a Row,
}
impl<'a> SatisfactionSupplyRewardRow<'a> {
    pub fn SatisfactionSupplyRewardData(
        &'a self,
    ) -> [SatisfactionSupplyRewardDataElement; 2] {
        [
            SatisfactionSupplyRewardDataElement {
                RewardCurrency: self.row.columns[1].into_u16().copied().unwrap(),
                QuantityLow: self.row.columns[2].into_u16().copied().unwrap(),
                QuantityMid: self.row.columns[3].into_u16().copied().unwrap(),
                QuantityHigh: self.row.columns[4].into_u16().copied().unwrap(),
            },
            SatisfactionSupplyRewardDataElement {
                RewardCurrency: self.row.columns[5].into_u16().copied().unwrap(),
                QuantityLow: self.row.columns[6].into_u16().copied().unwrap(),
                QuantityMid: self.row.columns[7].into_u16().copied().unwrap(),
                QuantityHigh: self.row.columns[8].into_u16().copied().unwrap(),
            },
        ]
    }
    pub fn SatisfactionLow(&'a self) -> u16 {
        self.row.columns[10].into_u16().copied().unwrap()
    }
    pub fn SatisfactionMid(&'a self) -> u16 {
        self.row.columns[11].into_u16().copied().unwrap()
    }
    pub fn SatisfactionHigh(&'a self) -> u16 {
        self.row.columns[12].into_u16().copied().unwrap()
    }
    pub fn GilLow(&'a self) -> u16 {
        self.row.columns[13].into_u16().copied().unwrap()
    }
    pub fn GilMid(&'a self) -> u16 {
        self.row.columns[14].into_u16().copied().unwrap()
    }
    pub fn GilHigh(&'a self) -> u16 {
        self.row.columns[15].into_u16().copied().unwrap()
    }
    pub fn BonusMultiplier(&'a self) -> u8 {
        self.row.columns[0].into_u8().copied().unwrap()
    }
    /// 0 == current cap
    pub fn MinLevelForSecondReward(&'a self) -> u8 {
        self.row.columns[9].into_u8().copied().unwrap()
    }
}
