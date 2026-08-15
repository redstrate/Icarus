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
pub struct GilShopItemSheet {
    sheet: Sheet,
}
impl GilShopItemSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 262144u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("GilShopItem")?;
        let sheet = resolver.read_excel_sheet(&exh, "GilShopItem", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<GilShopItemRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<GilShopItemRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for GilShopItemSheet {
    type Row = GilShopItemRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Item: row
                .columns[0]
                .into_i32()
                .copied()
                .expect("Expected column 0 to be a int32!"),
            QuestRequired: [
                row
                    .columns[4]
                    .into_i32()
                    .copied()
                    .expect("Expected column 4 to be a int32!"),
                row
                    .columns[5]
                    .into_i32()
                    .copied()
                    .expect("Expected column 5 to be a int32!"),
            ],
            AchievementRequired: row
                .columns[6]
                .into_i32()
                .copied()
                .expect("Expected column 6 to be a int32!"),
            StateRequired: row
                .columns[8]
                .into_u16()
                .copied()
                .expect("Expected column 8 to be a uint16!"),
            Patch: row
                .columns[9]
                .into_u16()
                .copied()
                .expect("Expected column 9 to be a uint16!"),
            Unknown_70_1: row
                .columns[2]
                .into_u8()
                .copied()
                .expect("Expected column 2 to be a uint8!"),
            Unknown_70_2: row
                .columns[3]
                .into_u8()
                .copied()
                .expect("Expected column 3 to be a uint8!"),
            Unknown1: row
                .columns[7]
                .into_u8()
                .copied()
                .expect("Expected column 7 to be a uint8!"),
            IsHQ: row
                .columns[1]
                .into_bool()
                .copied()
                .expect("Expected column 1 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a GilShopItemSheet {
    type Item = (u32, Vec<(u16, GilShopItemRow)>);
    type IntoIter = StructuredSheetIterator<'a, GilShopItemSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, GilShopItemSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct GilShopItemRow {
    ///""
    pub Item: i32,
    ///""
    pub QuestRequired: [i32; 2],
    ///""
    pub AchievementRequired: i32,
    ///""
    pub StateRequired: u16,
    ///""
    pub Patch: u16,
    ///""
    pub Unknown_70_1: u8,
    ///""
    pub Unknown_70_2: u8,
    ///""
    pub Unknown1: u8,
    ///""
    pub IsHQ: bool,
}
