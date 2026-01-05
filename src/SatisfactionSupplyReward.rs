//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{ExcelSheet, ColumnData, ExcelRowKind, ExcelSingleRow},
    common::Language,
};
pub struct SatisfactionSupplyRewardDataElement<'a> {
    pub RewardCurrency: &'a ColumnData,
    pub QuantityLow: &'a ColumnData,
    pub QuantityMid: &'a ColumnData,
    pub QuantityHigh: &'a ColumnData,
}
#[derive(Debug, Clone)]
pub struct SatisfactionSupplyRewardSheet {
    sheet: ExcelSheet,
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
    fn read_row(&self, row: &ExcelSingleRow) -> Option<SatisfactionSupplyRewardRow> {
        let column_defs = &self.sheet.exh.column_definitions;
        let mut zipped: Vec<_> = row
            .columns
            .clone()
            .into_iter()
            .zip(column_defs)
            .collect();
        zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
        let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition>) = zipped
            .into_iter()
            .unzip();
        Some(SatisfactionSupplyRewardRow {
            columns,
        })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<SatisfactionSupplyRewardRow> {
        let row = &self.sheet.get_row(row_id)?;
        let row = match row {
            ExcelRowKind::SingleRow(row) => row,
            ExcelRowKind::SubRows(rows) => &rows.first()?.1,
        };
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn get_subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<SatisfactionSupplyRewardRow> {
        let row = &self.sheet.get_row(row_id)?;
        let row = match row {
            ExcelRowKind::SingleRow(row) => return None,
            ExcelRowKind::SubRows(subrows) => {
                &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1
            }
        };
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
pub struct SatisfactionSupplyRewardRow {
    columns: Vec<ColumnData>,
}
impl SatisfactionSupplyRewardRow {
    pub fn SatisfactionSupplyRewardData<'a>(
        &'a self,
    ) -> [SatisfactionSupplyRewardDataElement<'a>; 2] {
        [
            SatisfactionSupplyRewardDataElement {
                RewardCurrency: &self.columns[0],
                QuantityLow: &self.columns[1],
                QuantityMid: &self.columns[2],
                QuantityHigh: &self.columns[3],
            },
            SatisfactionSupplyRewardDataElement {
                RewardCurrency: &self.columns[4],
                QuantityLow: &self.columns[5],
                QuantityMid: &self.columns[6],
                QuantityHigh: &self.columns[7],
            },
        ]
    }
    pub fn SatisfactionLow<'a>(&'a self) -> &'a ColumnData {
        &self.columns[8]
    }
    pub fn SatisfactionMid<'a>(&'a self) -> &'a ColumnData {
        &self.columns[9]
    }
    pub fn SatisfactionHigh<'a>(&'a self) -> &'a ColumnData {
        &self.columns[10]
    }
    pub fn GilLow<'a>(&'a self) -> &'a ColumnData {
        &self.columns[11]
    }
    pub fn GilMid<'a>(&'a self) -> &'a ColumnData {
        &self.columns[12]
    }
    pub fn GilHigh<'a>(&'a self) -> &'a ColumnData {
        &self.columns[13]
    }
    pub fn BonusMultiplier<'a>(&'a self) -> &'a ColumnData {
        &self.columns[14]
    }
    /// 0 == current cap
    pub fn MinLevelForSecondReward<'a>(&'a self) -> &'a ColumnData {
        &self.columns[15]
    }
}
