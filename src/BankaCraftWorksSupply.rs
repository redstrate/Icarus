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
pub struct ItemElement {
    pub ItemId: u32,
    pub XPReward: u32,
    pub Collectability: u16,
    pub GilReward: u16,
    pub Level: u8,
    pub HighXPMultiplier: u8,
    pub HighGilMultiplier: u8,
    pub Unknown8: u8,
    pub ScripReward: u8,
    pub HighScripMultiplier: u8,
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
impl<'a> StructuredSheet<'a> for BankaCraftWorksSupplySheet {
    type Row = BankaCraftWorksSupplyRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a BankaCraftWorksSupplySheet {
    type Item = (u32, Vec<(u16, BankaCraftWorksSupplyRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, BankaCraftWorksSupplySheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, BankaCraftWorksSupplySheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct BankaCraftWorksSupplyRow<'a> {
    row: &'a Row,
}
impl<'a> BankaCraftWorksSupplyRow<'a> {
    pub fn Item(&'a self) -> [ItemElement; 4] {
        [
            ItemElement {
                ItemId: self.row.columns[4].into_u32().copied().unwrap(),
                XPReward: self.row.columns[12].into_u32().copied().unwrap(),
                Collectability: self.row.columns[8].into_u16().copied().unwrap(),
                GilReward: self.row.columns[20].into_u16().copied().unwrap(),
                Level: self.row.columns[0].into_u8().copied().unwrap(),
                HighXPMultiplier: self.row.columns[16].into_u8().copied().unwrap(),
                HighGilMultiplier: self.row.columns[24].into_u8().copied().unwrap(),
                Unknown8: self.row.columns[28].into_u8().copied().unwrap(),
                ScripReward: self.row.columns[32].into_u8().copied().unwrap(),
                HighScripMultiplier: self.row.columns[36].into_u8().copied().unwrap(),
            },
            ItemElement {
                ItemId: self.row.columns[5].into_u32().copied().unwrap(),
                XPReward: self.row.columns[13].into_u32().copied().unwrap(),
                Collectability: self.row.columns[9].into_u16().copied().unwrap(),
                GilReward: self.row.columns[21].into_u16().copied().unwrap(),
                Level: self.row.columns[1].into_u8().copied().unwrap(),
                HighXPMultiplier: self.row.columns[17].into_u8().copied().unwrap(),
                HighGilMultiplier: self.row.columns[25].into_u8().copied().unwrap(),
                Unknown8: self.row.columns[29].into_u8().copied().unwrap(),
                ScripReward: self.row.columns[33].into_u8().copied().unwrap(),
                HighScripMultiplier: self.row.columns[37].into_u8().copied().unwrap(),
            },
            ItemElement {
                ItemId: self.row.columns[6].into_u32().copied().unwrap(),
                XPReward: self.row.columns[14].into_u32().copied().unwrap(),
                Collectability: self.row.columns[10].into_u16().copied().unwrap(),
                GilReward: self.row.columns[22].into_u16().copied().unwrap(),
                Level: self.row.columns[2].into_u8().copied().unwrap(),
                HighXPMultiplier: self.row.columns[18].into_u8().copied().unwrap(),
                HighGilMultiplier: self.row.columns[26].into_u8().copied().unwrap(),
                Unknown8: self.row.columns[30].into_u8().copied().unwrap(),
                ScripReward: self.row.columns[34].into_u8().copied().unwrap(),
                HighScripMultiplier: self.row.columns[38].into_u8().copied().unwrap(),
            },
            ItemElement {
                ItemId: self.row.columns[7].into_u32().copied().unwrap(),
                XPReward: self.row.columns[15].into_u32().copied().unwrap(),
                Collectability: self.row.columns[11].into_u16().copied().unwrap(),
                GilReward: self.row.columns[23].into_u16().copied().unwrap(),
                Level: self.row.columns[3].into_u8().copied().unwrap(),
                HighXPMultiplier: self.row.columns[19].into_u8().copied().unwrap(),
                HighGilMultiplier: self.row.columns[27].into_u8().copied().unwrap(),
                Unknown8: self.row.columns[31].into_u8().copied().unwrap(),
                ScripReward: self.row.columns[35].into_u8().copied().unwrap(),
                HighScripMultiplier: self.row.columns[39].into_u8().copied().unwrap(),
            },
        ]
    }
}
