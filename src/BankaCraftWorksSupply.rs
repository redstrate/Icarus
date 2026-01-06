//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    common::Language,
};
pub struct ItemElement<'a> {
    pub ItemId: &'a Field,
    pub XPReward: &'a Field,
    pub Collectability: &'a Field,
    pub GilReward: &'a Field,
    pub Level: &'a Field,
    pub HighXPMultiplier: &'a Field,
    pub HighGilMultiplier: &'a Field,
    pub Unknown8: &'a Field,
    pub ScripReward: &'a Field,
    pub HighScripMultiplier: &'a Field,
}
#[derive(Debug, Clone)]
pub struct BankaCraftWorksSupplySheet {
    sheet: Sheet,
}
impl BankaCraftWorksSupplySheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("BankaCraftWorksSupply")?;
        let sheet = resolver.read_excel_sheet(&exh, "BankaCraftWorksSupply", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<BankaCraftWorksSupplyRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<BankaCraftWorksSupplyRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for BankaCraftWorksSupplySheet {
    type Row = BankaCraftWorksSupplyRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        let column_defs = &self.sheet.exh.column_definitions;
        let mut zipped: Vec<_> = row
            .columns
            .clone()
            .into_iter()
            .zip(column_defs)
            .collect();
        zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
        let (columns, _): (Vec<Field>, Vec<ExcelColumnDefinition>) = zipped
            .into_iter()
            .unzip();
        Some(Self::Row { columns })
    }
}
impl<'a> IntoIterator for &'a BankaCraftWorksSupplySheet {
    type Item = (u32, Vec<(u16, BankaCraftWorksSupplyRow)>);
    type IntoIter = StructuredSheetIterator<'a, BankaCraftWorksSupplySheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, BankaCraftWorksSupplySheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct BankaCraftWorksSupplyRow {
    columns: Vec<Field>,
}
impl BankaCraftWorksSupplyRow {
    pub fn Item<'a>(&'a self) -> [ItemElement<'a>; 4] {
        [
            ItemElement {
                ItemId: &self.columns[0],
                XPReward: &self.columns[1],
                Collectability: &self.columns[2],
                GilReward: &self.columns[3],
                Level: &self.columns[4],
                HighXPMultiplier: &self.columns[5],
                HighGilMultiplier: &self.columns[6],
                Unknown8: &self.columns[7],
                ScripReward: &self.columns[8],
                HighScripMultiplier: &self.columns[9],
            },
            ItemElement {
                ItemId: &self.columns[10],
                XPReward: &self.columns[11],
                Collectability: &self.columns[12],
                GilReward: &self.columns[13],
                Level: &self.columns[14],
                HighXPMultiplier: &self.columns[15],
                HighGilMultiplier: &self.columns[16],
                Unknown8: &self.columns[17],
                ScripReward: &self.columns[18],
                HighScripMultiplier: &self.columns[19],
            },
            ItemElement {
                ItemId: &self.columns[20],
                XPReward: &self.columns[21],
                Collectability: &self.columns[22],
                GilReward: &self.columns[23],
                Level: &self.columns[24],
                HighXPMultiplier: &self.columns[25],
                HighGilMultiplier: &self.columns[26],
                Unknown8: &self.columns[27],
                ScripReward: &self.columns[28],
                HighScripMultiplier: &self.columns[29],
            },
            ItemElement {
                ItemId: &self.columns[30],
                XPReward: &self.columns[31],
                Collectability: &self.columns[32],
                GilReward: &self.columns[33],
                Level: &self.columns[34],
                HighXPMultiplier: &self.columns[35],
                HighGilMultiplier: &self.columns[36],
                Unknown8: &self.columns[37],
                ScripReward: &self.columns[38],
                HighScripMultiplier: &self.columns[39],
            },
        ]
    }
}
