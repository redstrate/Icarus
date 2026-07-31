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
pub struct TraitSheet {
    sheet: Sheet,
}
impl TraitSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Trait")?;
        let sheet = resolver.read_excel_sheet(&exh, "Trait", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<TraitRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<TraitRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for TraitSheet {
    type Row = TraitRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Name: row
                .columns[0]
                .into_string()
                .cloned()
                .expect("Expected column 0 to be a string!"),
            Quest: row
                .columns[6]
                .into_u32()
                .copied()
                .expect("Expected column 6 to be a uint32!"),
            Icon: row
                .columns[1]
                .into_i32()
                .copied()
                .expect("Expected column 1 to be a int32!"),
            Value: row
                .columns[7]
                .into_i16()
                .copied()
                .expect("Expected column 7 to be a int16!"),
            ClassJob: row
                .columns[3]
                .into_u8()
                .copied()
                .expect("Expected column 3 to be a uint8!"),
            ClassJobCategory: row
                .columns[4]
                .into_u8()
                .copied()
                .expect("Expected column 4 to be a uint8!"),
            Level: row
                .columns[5]
                .into_u8()
                .copied()
                .expect("Expected column 5 to be a uint8!"),
            Unknown2: row
                .columns[8]
                .into_u8()
                .copied()
                .expect("Expected column 8 to be a uint8!"),
            Unknown0: row
                .columns[9]
                .into_u8()
                .copied()
                .expect("Expected column 9 to be a uint8!"),
            Unknown1: row
                .columns[2]
                .into_bool()
                .copied()
                .expect("Expected column 2 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a TraitSheet {
    type Item = (u32, Vec<(u16, TraitRow)>);
    type IntoIter = StructuredSheetIterator<'a, TraitSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, TraitSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TraitRow {
    ///""
    pub Name: String,
    ///""
    pub Quest: u32,
    ///""
    pub Icon: i32,
    ///""
    pub Value: i16,
    ///""
    pub ClassJob: u8,
    ///""
    pub ClassJobCategory: u8,
    ///""
    pub Level: u8,
    ///""
    pub Unknown2: u8,
    ///""
    pub Unknown0: u8,
    ///""
    pub Unknown1: bool,
}
