//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{ExcelSheet, ColumnData, ExcelRowKind, ExcelSingleRow},
    common::Language,
};
pub struct ItemElement<'a> {
    pub ItemId: &'a ColumnData,
    pub XPReward: &'a ColumnData,
    pub CollectabilityMid: &'a ColumnData,
    pub CollectabilityHigh: &'a ColumnData,
    pub GilReward: &'a ColumnData,
    pub Level: &'a ColumnData,
    pub HighXPMultiplier: &'a ColumnData,
    pub HighGilMultiplier: &'a ColumnData,
    pub Unknown8: &'a ColumnData,
    pub ScripReward: &'a ColumnData,
    pub HighScripMultiplier: &'a ColumnData,
}
pub struct SharlayanCraftWorksSupplySheet {
    sheet: ExcelSheet,
}
impl SharlayanCraftWorksSupplySheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("SharlayanCraftWorksSupply")?;
        let sheet = resolver
            .read_excel_sheet(exh, "SharlayanCraftWorksSupply", language)?;
        Ok(Self { sheet })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<SharlayanCraftWorksSupplyRow> {
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
        Some(SharlayanCraftWorksSupplyRow {
            columns,
        })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<SharlayanCraftWorksSupplyRow> {
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
    ) -> Option<SharlayanCraftWorksSupplyRow> {
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
pub struct SharlayanCraftWorksSupplyRow {
    columns: Vec<ColumnData>,
}
impl SharlayanCraftWorksSupplyRow {
    pub fn Item<'a>(&'a self) -> [ItemElement<'a>; 4] {
        [
            ItemElement {
                ItemId: &self.columns[0],
                XPReward: &self.columns[1],
                CollectabilityMid: &self.columns[2],
                CollectabilityHigh: &self.columns[3],
                GilReward: &self.columns[4],
                Level: &self.columns[5],
                HighXPMultiplier: &self.columns[6],
                HighGilMultiplier: &self.columns[7],
                Unknown8: &self.columns[8],
                ScripReward: &self.columns[9],
                HighScripMultiplier: &self.columns[10],
            },
            ItemElement {
                ItemId: &self.columns[11],
                XPReward: &self.columns[12],
                CollectabilityMid: &self.columns[13],
                CollectabilityHigh: &self.columns[14],
                GilReward: &self.columns[15],
                Level: &self.columns[16],
                HighXPMultiplier: &self.columns[17],
                HighGilMultiplier: &self.columns[18],
                Unknown8: &self.columns[19],
                ScripReward: &self.columns[20],
                HighScripMultiplier: &self.columns[21],
            },
            ItemElement {
                ItemId: &self.columns[22],
                XPReward: &self.columns[23],
                CollectabilityMid: &self.columns[24],
                CollectabilityHigh: &self.columns[25],
                GilReward: &self.columns[26],
                Level: &self.columns[27],
                HighXPMultiplier: &self.columns[28],
                HighGilMultiplier: &self.columns[29],
                Unknown8: &self.columns[30],
                ScripReward: &self.columns[31],
                HighScripMultiplier: &self.columns[32],
            },
            ItemElement {
                ItemId: &self.columns[33],
                XPReward: &self.columns[34],
                CollectabilityMid: &self.columns[35],
                CollectabilityHigh: &self.columns[36],
                GilReward: &self.columns[37],
                Level: &self.columns[38],
                HighXPMultiplier: &self.columns[39],
                HighGilMultiplier: &self.columns[40],
                Unknown8: &self.columns[41],
                ScripReward: &self.columns[42],
                HighScripMultiplier: &self.columns[43],
            },
        ]
    }
}
