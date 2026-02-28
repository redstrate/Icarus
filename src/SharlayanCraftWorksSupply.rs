//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
pub struct ItemElement<'a> {
    pub ItemId: &'a Field,
    pub XPReward: &'a Field,
    pub CollectabilityMid: &'a Field,
    pub CollectabilityHigh: &'a Field,
    pub GilReward: &'a Field,
    pub Level: &'a Field,
    pub HighXPMultiplier: &'a Field,
    pub HighGilMultiplier: &'a Field,
    pub Unknown8: &'a Field,
    pub ScripReward: &'a Field,
    pub HighScripMultiplier: &'a Field,
}
#[derive(Debug, Clone)]
pub struct SharlayanCraftWorksSupplySheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl SharlayanCraftWorksSupplySheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("SharlayanCraftWorksSupply")?;
        let sheet = resolver
            .read_excel_sheet(&exh, "SharlayanCraftWorksSupply", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<SharlayanCraftWorksSupplyRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<SharlayanCraftWorksSupplyRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for SharlayanCraftWorksSupplySheet {
    type Row = SharlayanCraftWorksSupplyRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a SharlayanCraftWorksSupplySheet {
    type Item = (u32, Vec<(u16, SharlayanCraftWorksSupplyRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, SharlayanCraftWorksSupplySheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, SharlayanCraftWorksSupplySheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct SharlayanCraftWorksSupplyRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> SharlayanCraftWorksSupplyRow<'a> {
    pub fn Item(&'a self) -> [ItemElement<'a>; 4] {
        [
            ItemElement {
                ItemId: &self.row.columns[self.index_mapping[0]],
                XPReward: &self.row.columns[self.index_mapping[1]],
                CollectabilityMid: &self.row.columns[self.index_mapping[2]],
                CollectabilityHigh: &self.row.columns[self.index_mapping[3]],
                GilReward: &self.row.columns[self.index_mapping[4]],
                Level: &self.row.columns[self.index_mapping[5]],
                HighXPMultiplier: &self.row.columns[self.index_mapping[6]],
                HighGilMultiplier: &self.row.columns[self.index_mapping[7]],
                Unknown8: &self.row.columns[self.index_mapping[8]],
                ScripReward: &self.row.columns[self.index_mapping[9]],
                HighScripMultiplier: &self.row.columns[self.index_mapping[10]],
            },
            ItemElement {
                ItemId: &self.row.columns[self.index_mapping[11]],
                XPReward: &self.row.columns[self.index_mapping[12]],
                CollectabilityMid: &self.row.columns[self.index_mapping[13]],
                CollectabilityHigh: &self.row.columns[self.index_mapping[14]],
                GilReward: &self.row.columns[self.index_mapping[15]],
                Level: &self.row.columns[self.index_mapping[16]],
                HighXPMultiplier: &self.row.columns[self.index_mapping[17]],
                HighGilMultiplier: &self.row.columns[self.index_mapping[18]],
                Unknown8: &self.row.columns[self.index_mapping[19]],
                ScripReward: &self.row.columns[self.index_mapping[20]],
                HighScripMultiplier: &self.row.columns[self.index_mapping[21]],
            },
            ItemElement {
                ItemId: &self.row.columns[self.index_mapping[22]],
                XPReward: &self.row.columns[self.index_mapping[23]],
                CollectabilityMid: &self.row.columns[self.index_mapping[24]],
                CollectabilityHigh: &self.row.columns[self.index_mapping[25]],
                GilReward: &self.row.columns[self.index_mapping[26]],
                Level: &self.row.columns[self.index_mapping[27]],
                HighXPMultiplier: &self.row.columns[self.index_mapping[28]],
                HighGilMultiplier: &self.row.columns[self.index_mapping[29]],
                Unknown8: &self.row.columns[self.index_mapping[30]],
                ScripReward: &self.row.columns[self.index_mapping[31]],
                HighScripMultiplier: &self.row.columns[self.index_mapping[32]],
            },
            ItemElement {
                ItemId: &self.row.columns[self.index_mapping[33]],
                XPReward: &self.row.columns[self.index_mapping[34]],
                CollectabilityMid: &self.row.columns[self.index_mapping[35]],
                CollectabilityHigh: &self.row.columns[self.index_mapping[36]],
                GilReward: &self.row.columns[self.index_mapping[37]],
                Level: &self.row.columns[self.index_mapping[38]],
                HighXPMultiplier: &self.row.columns[self.index_mapping[39]],
                HighGilMultiplier: &self.row.columns[self.index_mapping[40]],
                Unknown8: &self.row.columns[self.index_mapping[41]],
                ScripReward: &self.row.columns[self.index_mapping[42]],
                HighScripMultiplier: &self.row.columns[self.index_mapping[43]],
            },
        ]
    }
}
