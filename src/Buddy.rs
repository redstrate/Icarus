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
pub struct BuddySheet {
    sheet: Sheet,
}
impl BuddySheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Buddy")?;
        let sheet = resolver.read_excel_sheet(&exh, "Buddy", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<BuddyRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<BuddyRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for BuddySheet {
    type Row = BuddyRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            SoundEffect4: row
                .columns[4]
                .into_string()
                .cloned()
                .expect("Expected column 4 to be a string!"),
            SoundEffect3: row
                .columns[5]
                .into_string()
                .cloned()
                .expect("Expected column 5 to be a string!"),
            SoundEffect2: row
                .columns[6]
                .into_string()
                .cloned()
                .expect("Expected column 6 to be a string!"),
            SoundEffect1: row
                .columns[7]
                .into_string()
                .cloned()
                .expect("Expected column 7 to be a string!"),
            QuestRequirement2: row
                .columns[1]
                .into_i32()
                .copied()
                .expect("Expected column 1 to be a int32!"),
            QuestRequirement1: row
                .columns[2]
                .into_i32()
                .copied()
                .expect("Expected column 2 to be a int32!"),
            BaseEquip: row
                .columns[3]
                .into_i32()
                .copied()
                .expect("Expected column 3 to be a int32!"),
            Base: row
                .columns[0]
                .into_u8()
                .copied()
                .expect("Expected column 0 to be a uint8!"),
        })
    }
}
impl<'a> IntoIterator for &'a BuddySheet {
    type Item = (u32, Vec<(u16, BuddyRow)>);
    type IntoIter = StructuredSheetIterator<'a, BuddySheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, BuddySheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BuddyRow {
    ///""
    pub SoundEffect4: String,
    ///""
    pub SoundEffect3: String,
    ///""
    pub SoundEffect2: String,
    ///""
    pub SoundEffect1: String,
    ///""
    pub QuestRequirement2: i32,
    ///""
    pub QuestRequirement1: i32,
    ///""
    pub BaseEquip: i32,
    ///""
    pub Base: u8,
}
