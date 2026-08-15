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
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
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
impl StructuredSheet for EquipSlotCategorySheet {
    type Row = EquipSlotCategoryRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            MainHand: row
                .columns[0]
                .into_i8()
                .copied()
                .expect("Expected column 0 to be a int8!"),
            OffHand: row
                .columns[1]
                .into_i8()
                .copied()
                .expect("Expected column 1 to be a int8!"),
            Head: row
                .columns[2]
                .into_i8()
                .copied()
                .expect("Expected column 2 to be a int8!"),
            Body: row
                .columns[3]
                .into_i8()
                .copied()
                .expect("Expected column 3 to be a int8!"),
            Gloves: row
                .columns[4]
                .into_i8()
                .copied()
                .expect("Expected column 4 to be a int8!"),
            Waist: row
                .columns[5]
                .into_i8()
                .copied()
                .expect("Expected column 5 to be a int8!"),
            Legs: row
                .columns[6]
                .into_i8()
                .copied()
                .expect("Expected column 6 to be a int8!"),
            Feet: row
                .columns[7]
                .into_i8()
                .copied()
                .expect("Expected column 7 to be a int8!"),
            Ears: row
                .columns[8]
                .into_i8()
                .copied()
                .expect("Expected column 8 to be a int8!"),
            Neck: row
                .columns[9]
                .into_i8()
                .copied()
                .expect("Expected column 9 to be a int8!"),
            Wrists: row
                .columns[10]
                .into_i8()
                .copied()
                .expect("Expected column 10 to be a int8!"),
            FingerL: row
                .columns[11]
                .into_i8()
                .copied()
                .expect("Expected column 11 to be a int8!"),
            FingerR: row
                .columns[12]
                .into_i8()
                .copied()
                .expect("Expected column 12 to be a int8!"),
            SoulCrystal: row
                .columns[13]
                .into_i8()
                .copied()
                .expect("Expected column 13 to be a int8!"),
        })
    }
}
impl<'a> IntoIterator for &'a EquipSlotCategorySheet {
    type Item = (u32, Vec<(u16, EquipSlotCategoryRow)>);
    type IntoIter = StructuredSheetIterator<'a, EquipSlotCategorySheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, EquipSlotCategorySheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EquipSlotCategoryRow {
    ///""
    pub MainHand: i8,
    ///""
    pub OffHand: i8,
    ///""
    pub Head: i8,
    ///""
    pub Body: i8,
    ///""
    pub Gloves: i8,
    ///""
    pub Waist: i8,
    ///""
    pub Legs: i8,
    ///""
    pub Feet: i8,
    ///""
    pub Ears: i8,
    ///""
    pub Neck: i8,
    ///""
    pub Wrists: i8,
    ///""
    pub FingerL: i8,
    ///""
    pub FingerR: i8,
    ///""
    pub SoulCrystal: i8,
}
