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
pub struct ItemDataElement {
    pub Item: u32,
    pub Cost: u32,
    pub FCRankRequired: u8,
}
#[derive(Debug, Clone)]
pub struct FccShopSheet {
    sheet: Sheet,
}
impl FccShopSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("FccShop")?;
        let sheet = resolver.read_excel_sheet(&exh, "FccShop", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<FccShopRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<FccShopRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for FccShopSheet {
    type Row = FccShopRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Name: row
                .columns[0]
                .into_string()
                .cloned()
                .expect("Expected column 0 to be a string!"),
            ItemData: [
                ItemDataElement {
                    Item: row
                        .columns[1]
                        .into_u32()
                        .copied()
                        .expect("Expected column 1 to be a uint32!"),
                    Cost: row
                        .columns[11]
                        .into_u32()
                        .copied()
                        .expect("Expected column 11 to be a uint32!"),
                    FCRankRequired: row
                        .columns[21]
                        .into_u8()
                        .copied()
                        .expect("Expected column 21 to be a uint8!"),
                },
                ItemDataElement {
                    Item: row
                        .columns[2]
                        .into_u32()
                        .copied()
                        .expect("Expected column 2 to be a uint32!"),
                    Cost: row
                        .columns[12]
                        .into_u32()
                        .copied()
                        .expect("Expected column 12 to be a uint32!"),
                    FCRankRequired: row
                        .columns[22]
                        .into_u8()
                        .copied()
                        .expect("Expected column 22 to be a uint8!"),
                },
                ItemDataElement {
                    Item: row
                        .columns[3]
                        .into_u32()
                        .copied()
                        .expect("Expected column 3 to be a uint32!"),
                    Cost: row
                        .columns[13]
                        .into_u32()
                        .copied()
                        .expect("Expected column 13 to be a uint32!"),
                    FCRankRequired: row
                        .columns[23]
                        .into_u8()
                        .copied()
                        .expect("Expected column 23 to be a uint8!"),
                },
                ItemDataElement {
                    Item: row
                        .columns[4]
                        .into_u32()
                        .copied()
                        .expect("Expected column 4 to be a uint32!"),
                    Cost: row
                        .columns[14]
                        .into_u32()
                        .copied()
                        .expect("Expected column 14 to be a uint32!"),
                    FCRankRequired: row
                        .columns[24]
                        .into_u8()
                        .copied()
                        .expect("Expected column 24 to be a uint8!"),
                },
                ItemDataElement {
                    Item: row
                        .columns[5]
                        .into_u32()
                        .copied()
                        .expect("Expected column 5 to be a uint32!"),
                    Cost: row
                        .columns[15]
                        .into_u32()
                        .copied()
                        .expect("Expected column 15 to be a uint32!"),
                    FCRankRequired: row
                        .columns[25]
                        .into_u8()
                        .copied()
                        .expect("Expected column 25 to be a uint8!"),
                },
                ItemDataElement {
                    Item: row
                        .columns[6]
                        .into_u32()
                        .copied()
                        .expect("Expected column 6 to be a uint32!"),
                    Cost: row
                        .columns[16]
                        .into_u32()
                        .copied()
                        .expect("Expected column 16 to be a uint32!"),
                    FCRankRequired: row
                        .columns[26]
                        .into_u8()
                        .copied()
                        .expect("Expected column 26 to be a uint8!"),
                },
                ItemDataElement {
                    Item: row
                        .columns[7]
                        .into_u32()
                        .copied()
                        .expect("Expected column 7 to be a uint32!"),
                    Cost: row
                        .columns[17]
                        .into_u32()
                        .copied()
                        .expect("Expected column 17 to be a uint32!"),
                    FCRankRequired: row
                        .columns[27]
                        .into_u8()
                        .copied()
                        .expect("Expected column 27 to be a uint8!"),
                },
                ItemDataElement {
                    Item: row
                        .columns[8]
                        .into_u32()
                        .copied()
                        .expect("Expected column 8 to be a uint32!"),
                    Cost: row
                        .columns[18]
                        .into_u32()
                        .copied()
                        .expect("Expected column 18 to be a uint32!"),
                    FCRankRequired: row
                        .columns[28]
                        .into_u8()
                        .copied()
                        .expect("Expected column 28 to be a uint8!"),
                },
                ItemDataElement {
                    Item: row
                        .columns[9]
                        .into_u32()
                        .copied()
                        .expect("Expected column 9 to be a uint32!"),
                    Cost: row
                        .columns[19]
                        .into_u32()
                        .copied()
                        .expect("Expected column 19 to be a uint32!"),
                    FCRankRequired: row
                        .columns[29]
                        .into_u8()
                        .copied()
                        .expect("Expected column 29 to be a uint8!"),
                },
                ItemDataElement {
                    Item: row
                        .columns[10]
                        .into_u32()
                        .copied()
                        .expect("Expected column 10 to be a uint32!"),
                    Cost: row
                        .columns[20]
                        .into_u32()
                        .copied()
                        .expect("Expected column 20 to be a uint32!"),
                    FCRankRequired: row
                        .columns[30]
                        .into_u8()
                        .copied()
                        .expect("Expected column 30 to be a uint8!"),
                },
            ],
        })
    }
}
impl<'a> IntoIterator for &'a FccShopSheet {
    type Item = (u32, Vec<(u16, FccShopRow)>);
    type IntoIter = StructuredSheetIterator<'a, FccShopSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, FccShopSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FccShopRow {
    ///""
    pub Name: String,
    ///""
    pub ItemData: [ItemDataElement; 10],
}
