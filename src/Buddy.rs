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
impl<'a> StructuredSheet<'a> for BuddySheet {
    type Row = BuddyRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a BuddySheet {
    type Item = (u32, Vec<(u16, BuddyRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, BuddySheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, BuddySheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct BuddyRow<'a> {
    row: &'a Row,
}
impl<'a> BuddyRow<'a> {
    pub fn SoundEffect4(&'a self) -> &'a str {
        self.row.columns[4].into_string().unwrap()
    }
    pub fn SoundEffect3(&'a self) -> &'a str {
        self.row.columns[5].into_string().unwrap()
    }
    pub fn SoundEffect2(&'a self) -> &'a str {
        self.row.columns[6].into_string().unwrap()
    }
    pub fn SoundEffect1(&'a self) -> &'a str {
        self.row.columns[7].into_string().unwrap()
    }
    pub fn QuestRequirement2(&'a self) -> i32 {
        self.row.columns[1].into_i32().copied().unwrap()
    }
    pub fn QuestRequirement1(&'a self) -> i32 {
        self.row.columns[2].into_i32().copied().unwrap()
    }
    pub fn BaseEquip(&'a self) -> i32 {
        self.row.columns[3].into_i32().copied().unwrap()
    }
    pub fn Base(&'a self) -> u8 {
        self.row.columns[0].into_u8().copied().unwrap()
    }
}
