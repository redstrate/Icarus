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
pub struct ItemElement {
    pub ItemId: u32,
    pub XPReward: u32,
    pub CollectabilityMid: u16,
    pub CollectabilityHigh: u16,
    pub GilReward: u16,
    pub Level: u8,
    pub HighXPMultiplier: u8,
    pub HighGilMultiplier: u8,
    pub Unknown8: u8,
    pub ScripReward: u8,
    pub HighScripMultiplier: u8,
}
#[derive(Debug, Clone)]
pub struct SharlayanCraftWorksSupplySheet {
    sheet: Sheet,
}
impl SharlayanCraftWorksSupplySheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("SharlayanCraftWorksSupply")?;
        let sheet = resolver
            .read_excel_sheet(&exh, "SharlayanCraftWorksSupply", language)?;
        Ok(Self { sheet })
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
impl StructuredSheet for SharlayanCraftWorksSupplySheet {
    type Row = SharlayanCraftWorksSupplyRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Item: [
                ItemElement {
                    ItemId: row
                        .columns[4]
                        .into_u32()
                        .copied()
                        .expect("Expected column 4 to be a uint32!"),
                    XPReward: row
                        .columns[16]
                        .into_u32()
                        .copied()
                        .expect("Expected column 16 to be a uint32!"),
                    CollectabilityMid: row
                        .columns[8]
                        .into_u16()
                        .copied()
                        .expect("Expected column 8 to be a uint16!"),
                    CollectabilityHigh: row
                        .columns[12]
                        .into_u16()
                        .copied()
                        .expect("Expected column 12 to be a uint16!"),
                    GilReward: row
                        .columns[24]
                        .into_u16()
                        .copied()
                        .expect("Expected column 24 to be a uint16!"),
                    Level: row
                        .columns[0]
                        .into_u8()
                        .copied()
                        .expect("Expected column 0 to be a uint8!"),
                    HighXPMultiplier: row
                        .columns[20]
                        .into_u8()
                        .copied()
                        .expect("Expected column 20 to be a uint8!"),
                    HighGilMultiplier: row
                        .columns[28]
                        .into_u8()
                        .copied()
                        .expect("Expected column 28 to be a uint8!"),
                    Unknown8: row
                        .columns[32]
                        .into_u8()
                        .copied()
                        .expect("Expected column 32 to be a uint8!"),
                    ScripReward: row
                        .columns[36]
                        .into_u8()
                        .copied()
                        .expect("Expected column 36 to be a uint8!"),
                    HighScripMultiplier: row
                        .columns[40]
                        .into_u8()
                        .copied()
                        .expect("Expected column 40 to be a uint8!"),
                },
                ItemElement {
                    ItemId: row
                        .columns[5]
                        .into_u32()
                        .copied()
                        .expect("Expected column 5 to be a uint32!"),
                    XPReward: row
                        .columns[17]
                        .into_u32()
                        .copied()
                        .expect("Expected column 17 to be a uint32!"),
                    CollectabilityMid: row
                        .columns[9]
                        .into_u16()
                        .copied()
                        .expect("Expected column 9 to be a uint16!"),
                    CollectabilityHigh: row
                        .columns[13]
                        .into_u16()
                        .copied()
                        .expect("Expected column 13 to be a uint16!"),
                    GilReward: row
                        .columns[25]
                        .into_u16()
                        .copied()
                        .expect("Expected column 25 to be a uint16!"),
                    Level: row
                        .columns[1]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1 to be a uint8!"),
                    HighXPMultiplier: row
                        .columns[21]
                        .into_u8()
                        .copied()
                        .expect("Expected column 21 to be a uint8!"),
                    HighGilMultiplier: row
                        .columns[29]
                        .into_u8()
                        .copied()
                        .expect("Expected column 29 to be a uint8!"),
                    Unknown8: row
                        .columns[33]
                        .into_u8()
                        .copied()
                        .expect("Expected column 33 to be a uint8!"),
                    ScripReward: row
                        .columns[37]
                        .into_u8()
                        .copied()
                        .expect("Expected column 37 to be a uint8!"),
                    HighScripMultiplier: row
                        .columns[41]
                        .into_u8()
                        .copied()
                        .expect("Expected column 41 to be a uint8!"),
                },
                ItemElement {
                    ItemId: row
                        .columns[6]
                        .into_u32()
                        .copied()
                        .expect("Expected column 6 to be a uint32!"),
                    XPReward: row
                        .columns[18]
                        .into_u32()
                        .copied()
                        .expect("Expected column 18 to be a uint32!"),
                    CollectabilityMid: row
                        .columns[10]
                        .into_u16()
                        .copied()
                        .expect("Expected column 10 to be a uint16!"),
                    CollectabilityHigh: row
                        .columns[14]
                        .into_u16()
                        .copied()
                        .expect("Expected column 14 to be a uint16!"),
                    GilReward: row
                        .columns[26]
                        .into_u16()
                        .copied()
                        .expect("Expected column 26 to be a uint16!"),
                    Level: row
                        .columns[2]
                        .into_u8()
                        .copied()
                        .expect("Expected column 2 to be a uint8!"),
                    HighXPMultiplier: row
                        .columns[22]
                        .into_u8()
                        .copied()
                        .expect("Expected column 22 to be a uint8!"),
                    HighGilMultiplier: row
                        .columns[30]
                        .into_u8()
                        .copied()
                        .expect("Expected column 30 to be a uint8!"),
                    Unknown8: row
                        .columns[34]
                        .into_u8()
                        .copied()
                        .expect("Expected column 34 to be a uint8!"),
                    ScripReward: row
                        .columns[38]
                        .into_u8()
                        .copied()
                        .expect("Expected column 38 to be a uint8!"),
                    HighScripMultiplier: row
                        .columns[42]
                        .into_u8()
                        .copied()
                        .expect("Expected column 42 to be a uint8!"),
                },
                ItemElement {
                    ItemId: row
                        .columns[7]
                        .into_u32()
                        .copied()
                        .expect("Expected column 7 to be a uint32!"),
                    XPReward: row
                        .columns[19]
                        .into_u32()
                        .copied()
                        .expect("Expected column 19 to be a uint32!"),
                    CollectabilityMid: row
                        .columns[11]
                        .into_u16()
                        .copied()
                        .expect("Expected column 11 to be a uint16!"),
                    CollectabilityHigh: row
                        .columns[15]
                        .into_u16()
                        .copied()
                        .expect("Expected column 15 to be a uint16!"),
                    GilReward: row
                        .columns[27]
                        .into_u16()
                        .copied()
                        .expect("Expected column 27 to be a uint16!"),
                    Level: row
                        .columns[3]
                        .into_u8()
                        .copied()
                        .expect("Expected column 3 to be a uint8!"),
                    HighXPMultiplier: row
                        .columns[23]
                        .into_u8()
                        .copied()
                        .expect("Expected column 23 to be a uint8!"),
                    HighGilMultiplier: row
                        .columns[31]
                        .into_u8()
                        .copied()
                        .expect("Expected column 31 to be a uint8!"),
                    Unknown8: row
                        .columns[35]
                        .into_u8()
                        .copied()
                        .expect("Expected column 35 to be a uint8!"),
                    ScripReward: row
                        .columns[39]
                        .into_u8()
                        .copied()
                        .expect("Expected column 39 to be a uint8!"),
                    HighScripMultiplier: row
                        .columns[43]
                        .into_u8()
                        .copied()
                        .expect("Expected column 43 to be a uint8!"),
                },
            ],
        })
    }
}
impl<'a> IntoIterator for &'a SharlayanCraftWorksSupplySheet {
    type Item = (u32, Vec<(u16, SharlayanCraftWorksSupplyRow)>);
    type IntoIter = StructuredSheetIterator<'a, SharlayanCraftWorksSupplySheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, SharlayanCraftWorksSupplySheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SharlayanCraftWorksSupplyRow {
    ///""
    pub Item: [ItemElement; 4],
}
