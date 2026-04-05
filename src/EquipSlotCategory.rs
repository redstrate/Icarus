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
pub struct EquipSlotCategorySheet {
    sheet: Sheet,
}
impl EquipSlotCategorySheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("EquipSlotCategory")?;
        let sheet = resolver.read_excel_sheet(&exh, "EquipSlotCategory", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<EquipSlotCategoryRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<EquipSlotCategoryRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for EquipSlotCategorySheet {
    type Row = EquipSlotCategoryRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a EquipSlotCategorySheet {
    type Item = (u32, Vec<(u16, EquipSlotCategoryRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, EquipSlotCategorySheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, EquipSlotCategorySheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct EquipSlotCategoryRow<'a> {
    row: &'a Row,
}
impl<'a> EquipSlotCategoryRow<'a> {
    pub fn MainHand(&'a self) -> i8 {
        self.row.columns[0].into_i8().copied().unwrap()
    }
    pub fn OffHand(&'a self) -> i8 {
        self.row.columns[1].into_i8().copied().unwrap()
    }
    pub fn Head(&'a self) -> i8 {
        self.row.columns[2].into_i8().copied().unwrap()
    }
    pub fn Body(&'a self) -> i8 {
        self.row.columns[3].into_i8().copied().unwrap()
    }
    pub fn Gloves(&'a self) -> i8 {
        self.row.columns[4].into_i8().copied().unwrap()
    }
    pub fn Waist(&'a self) -> i8 {
        self.row.columns[5].into_i8().copied().unwrap()
    }
    pub fn Legs(&'a self) -> i8 {
        self.row.columns[6].into_i8().copied().unwrap()
    }
    pub fn Feet(&'a self) -> i8 {
        self.row.columns[7].into_i8().copied().unwrap()
    }
    pub fn Ears(&'a self) -> i8 {
        self.row.columns[8].into_i8().copied().unwrap()
    }
    pub fn Neck(&'a self) -> i8 {
        self.row.columns[9].into_i8().copied().unwrap()
    }
    pub fn Wrists(&'a self) -> i8 {
        self.row.columns[10].into_i8().copied().unwrap()
    }
    pub fn FingerL(&'a self) -> i8 {
        self.row.columns[11].into_i8().copied().unwrap()
    }
    pub fn FingerR(&'a self) -> i8 {
        self.row.columns[12].into_i8().copied().unwrap()
    }
    pub fn SoulCrystal(&'a self) -> i8 {
        self.row.columns[13].into_i8().copied().unwrap()
    }
}
