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
pub struct CharaMakeClassEquipSheet {
    sheet: Sheet,
}
impl CharaMakeClassEquipSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("CharaMakeClassEquip")?;
        let sheet = resolver.read_excel_sheet(&exh, "CharaMakeClassEquip", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<CharaMakeClassEquipRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<CharaMakeClassEquipRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for CharaMakeClassEquipSheet {
    type Row = CharaMakeClassEquipRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Helmet: row
                .columns[0]
                .into_u64()
                .copied()
                .expect("Expected column 0 to be a uint64!"),
            Top: row
                .columns[1]
                .into_u64()
                .copied()
                .expect("Expected column 1 to be a uint64!"),
            Glove: row
                .columns[2]
                .into_u64()
                .copied()
                .expect("Expected column 2 to be a uint64!"),
            Down: row
                .columns[3]
                .into_u64()
                .copied()
                .expect("Expected column 3 to be a uint64!"),
            Shoes: row
                .columns[4]
                .into_u64()
                .copied()
                .expect("Expected column 4 to be a uint64!"),
            Weapon: row
                .columns[5]
                .into_u64()
                .copied()
                .expect("Expected column 5 to be a uint64!"),
            SubWeapon: row
                .columns[6]
                .into_u64()
                .copied()
                .expect("Expected column 6 to be a uint64!"),
            Class: row
                .columns[7]
                .into_i32()
                .copied()
                .expect("Expected column 7 to be a int32!"),
        })
    }
}
impl<'a> IntoIterator for &'a CharaMakeClassEquipSheet {
    type Item = (u32, Vec<(u16, CharaMakeClassEquipRow)>);
    type IntoIter = StructuredSheetIterator<'a, CharaMakeClassEquipSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, CharaMakeClassEquipSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CharaMakeClassEquipRow {
    ///""
    pub Helmet: u64,
    ///""
    pub Top: u64,
    ///""
    pub Glove: u64,
    ///""
    pub Down: u64,
    ///""
    pub Shoes: u64,
    ///""
    pub Weapon: u64,
    ///""
    pub SubWeapon: u64,
    ///""
    pub Class: i32,
}
