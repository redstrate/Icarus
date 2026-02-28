//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
pub struct SatisfactionSupplyRewardDataElement<'a> {
    pub RewardCurrency: &'a Field,
    pub QuantityLow: &'a Field,
    pub QuantityMid: &'a Field,
    pub QuantityHigh: &'a Field,
}
#[derive(Debug, Clone)]
pub struct SatisfactionSupplyRewardSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
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
        let mut index_mapping: Vec<(usize, &ExcelColumnDefinition)> = sheet
            .exh
            .column_definitions
            .iter()
            .enumerate()
            .collect();
        index_mapping.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
        let index_mapping: Vec<usize> = index_mapping
            .iter()
            .map(|(index, _)| *index)
            .collect();
        Ok(Self { sheet, index_mapping })
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
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
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
    index_mapping: Vec<usize>,
}
impl<'a> SatisfactionSupplyRewardRow<'a> {
    pub fn SatisfactionSupplyRewardData(
        &'a self,
    ) -> [SatisfactionSupplyRewardDataElement<'a>; 2] {
        [
            SatisfactionSupplyRewardDataElement {
                RewardCurrency: &self.row.columns[self.index_mapping[0]],
                QuantityLow: &self.row.columns[self.index_mapping[1]],
                QuantityMid: &self.row.columns[self.index_mapping[2]],
                QuantityHigh: &self.row.columns[self.index_mapping[3]],
            },
            SatisfactionSupplyRewardDataElement {
                RewardCurrency: &self.row.columns[self.index_mapping[4]],
                QuantityLow: &self.row.columns[self.index_mapping[5]],
                QuantityMid: &self.row.columns[self.index_mapping[6]],
                QuantityHigh: &self.row.columns[self.index_mapping[7]],
            },
        ]
    }
    pub fn SatisfactionLow(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[8]]
    }
    pub fn SatisfactionMid(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[9]]
    }
    pub fn SatisfactionHigh(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[10]]
    }
    pub fn GilLow(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[11]]
    }
    pub fn GilMid(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[12]]
    }
    pub fn GilHigh(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[13]]
    }
    pub fn BonusMultiplier(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[14]]
    }
    /// 0 == current cap
    pub fn MinLevelForSecondReward(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[15]]
    }
}
