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
pub struct AnimaWeapon5TradeItemSheet {
    sheet: Sheet,
}
impl AnimaWeapon5TradeItemSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("AnimaWeapon5TradeItem")?;
        let sheet = resolver.read_excel_sheet(&exh, "AnimaWeapon5TradeItem", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<AnimaWeapon5TradeItemRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<AnimaWeapon5TradeItemRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for AnimaWeapon5TradeItemSheet {
    type Row = AnimaWeapon5TradeItemRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a AnimaWeapon5TradeItemSheet {
    type Item = (u32, Vec<(u16, AnimaWeapon5TradeItemRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, AnimaWeapon5TradeItemSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, AnimaWeapon5TradeItemSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct AnimaWeapon5TradeItemRow<'a> {
    row: &'a Row,
}
impl<'a> AnimaWeapon5TradeItemRow<'a> {
    pub fn CrystalSand(&'a self) -> u32 {
        self.row.columns[1].into_u32().copied().unwrap()
    }
    pub fn Item(&'a self) -> [u32; 8] {
        [
            self.row.columns[3].into_u32().copied().unwrap(),
            self.row.columns[6].into_u32().copied().unwrap(),
            self.row.columns[9].into_u32().copied().unwrap(),
            self.row.columns[12].into_u32().copied().unwrap(),
            self.row.columns[15].into_u32().copied().unwrap(),
            self.row.columns[18].into_u32().copied().unwrap(),
            self.row.columns[21].into_u32().copied().unwrap(),
            self.row.columns[24].into_u32().copied().unwrap(),
        ]
    }
    pub fn Order(&'a self) -> u8 {
        self.row.columns[0].into_u8().copied().unwrap()
    }
    pub fn ReceiveQuantity(&'a self) -> u8 {
        self.row.columns[2].into_u8().copied().unwrap()
    }
    pub fn Quantity(&'a self) -> [u8; 8] {
        [
            self.row.columns[5].into_u8().copied().unwrap(),
            self.row.columns[8].into_u8().copied().unwrap(),
            self.row.columns[11].into_u8().copied().unwrap(),
            self.row.columns[14].into_u8().copied().unwrap(),
            self.row.columns[17].into_u8().copied().unwrap(),
            self.row.columns[20].into_u8().copied().unwrap(),
            self.row.columns[23].into_u8().copied().unwrap(),
            self.row.columns[26].into_u8().copied().unwrap(),
        ]
    }
    pub fn Category(&'a self) -> u8 {
        self.row.columns[27].into_u8().copied().unwrap()
    }
    pub fn IsHQ(&'a self) -> [bool; 8] {
        [
            self.row.columns[4].into_bool().copied().unwrap(),
            self.row.columns[7].into_bool().copied().unwrap(),
            self.row.columns[10].into_bool().copied().unwrap(),
            self.row.columns[13].into_bool().copied().unwrap(),
            self.row.columns[16].into_bool().copied().unwrap(),
            self.row.columns[19].into_bool().copied().unwrap(),
            self.row.columns[22].into_bool().copied().unwrap(),
            self.row.columns[25].into_bool().copied().unwrap(),
        ]
    }
}
