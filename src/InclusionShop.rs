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
pub struct InclusionShopSheet {
    sheet: Sheet,
}
impl InclusionShopSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("InclusionShop")?;
        let sheet = resolver.read_excel_sheet(&exh, "InclusionShop", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<InclusionShopRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<InclusionShopRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for InclusionShopSheet {
    type Row = InclusionShopRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            ShopName: row
                .columns[2]
                .into_string()
                .cloned()
                .expect("Expected column 2 to be a string!"),
            Category: [
                row
                    .columns[3]
                    .into_u16()
                    .copied()
                    .expect("Expected column 3 to be a uint16!"),
                row
                    .columns[4]
                    .into_u16()
                    .copied()
                    .expect("Expected column 4 to be a uint16!"),
                row
                    .columns[5]
                    .into_u16()
                    .copied()
                    .expect("Expected column 5 to be a uint16!"),
                row
                    .columns[6]
                    .into_u16()
                    .copied()
                    .expect("Expected column 6 to be a uint16!"),
                row
                    .columns[7]
                    .into_u16()
                    .copied()
                    .expect("Expected column 7 to be a uint16!"),
                row
                    .columns[8]
                    .into_u16()
                    .copied()
                    .expect("Expected column 8 to be a uint16!"),
                row
                    .columns[9]
                    .into_u16()
                    .copied()
                    .expect("Expected column 9 to be a uint16!"),
                row
                    .columns[10]
                    .into_u16()
                    .copied()
                    .expect("Expected column 10 to be a uint16!"),
                row
                    .columns[11]
                    .into_u16()
                    .copied()
                    .expect("Expected column 11 to be a uint16!"),
                row
                    .columns[12]
                    .into_u16()
                    .copied()
                    .expect("Expected column 12 to be a uint16!"),
                row
                    .columns[13]
                    .into_u16()
                    .copied()
                    .expect("Expected column 13 to be a uint16!"),
                row
                    .columns[14]
                    .into_u16()
                    .copied()
                    .expect("Expected column 14 to be a uint16!"),
                row
                    .columns[15]
                    .into_u16()
                    .copied()
                    .expect("Expected column 15 to be a uint16!"),
                row
                    .columns[16]
                    .into_u16()
                    .copied()
                    .expect("Expected column 16 to be a uint16!"),
                row
                    .columns[17]
                    .into_u16()
                    .copied()
                    .expect("Expected column 17 to be a uint16!"),
                row
                    .columns[18]
                    .into_u16()
                    .copied()
                    .expect("Expected column 18 to be a uint16!"),
                row
                    .columns[19]
                    .into_u16()
                    .copied()
                    .expect("Expected column 19 to be a uint16!"),
                row
                    .columns[20]
                    .into_u16()
                    .copied()
                    .expect("Expected column 20 to be a uint16!"),
                row
                    .columns[21]
                    .into_u16()
                    .copied()
                    .expect("Expected column 21 to be a uint16!"),
                row
                    .columns[22]
                    .into_u16()
                    .copied()
                    .expect("Expected column 22 to be a uint16!"),
                row
                    .columns[23]
                    .into_u16()
                    .copied()
                    .expect("Expected column 23 to be a uint16!"),
                row
                    .columns[24]
                    .into_u16()
                    .copied()
                    .expect("Expected column 24 to be a uint16!"),
                row
                    .columns[25]
                    .into_u16()
                    .copied()
                    .expect("Expected column 25 to be a uint16!"),
                row
                    .columns[26]
                    .into_u16()
                    .copied()
                    .expect("Expected column 26 to be a uint16!"),
                row
                    .columns[27]
                    .into_u16()
                    .copied()
                    .expect("Expected column 27 to be a uint16!"),
                row
                    .columns[28]
                    .into_u16()
                    .copied()
                    .expect("Expected column 28 to be a uint16!"),
                row
                    .columns[29]
                    .into_u16()
                    .copied()
                    .expect("Expected column 29 to be a uint16!"),
                row
                    .columns[30]
                    .into_u16()
                    .copied()
                    .expect("Expected column 30 to be a uint16!"),
                row
                    .columns[31]
                    .into_u16()
                    .copied()
                    .expect("Expected column 31 to be a uint16!"),
                row
                    .columns[32]
                    .into_u16()
                    .copied()
                    .expect("Expected column 32 to be a uint16!"),
            ],
            UnlockQuest: row
                .columns[0]
                .into_u32()
                .copied()
                .expect("Expected column 0 to be a uint32!"),
            InclusionShopWelcom: row
                .columns[1]
                .into_u8()
                .copied()
                .expect("Expected column 1 to be a uint8!"),
        })
    }
}
impl<'a> IntoIterator for &'a InclusionShopSheet {
    type Item = (u32, Vec<(u16, InclusionShopRow)>);
    type IntoIter = StructuredSheetIterator<'a, InclusionShopSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, InclusionShopSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct InclusionShopRow {
    ///""
    pub ShopName: String,
    ///""
    pub Category: [u16; 30],
    ///""
    pub UnlockQuest: u32,
    ///""
    pub InclusionShopWelcom: u8,
}
