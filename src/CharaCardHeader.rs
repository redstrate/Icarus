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
pub struct CharaCardHeaderSheet {
    sheet: Sheet,
}
impl CharaCardHeaderSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("CharaCardHeader")?;
        let sheet = resolver.read_excel_sheet(&exh, "CharaCardHeader", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<CharaCardHeaderRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<CharaCardHeaderRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for CharaCardHeaderSheet {
    type Row = CharaCardHeaderRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Name: row
                .columns[12]
                .into_string()
                .cloned()
                .expect("Expected column 12 to be a string!"),
            TopImage: row
                .columns[0]
                .into_i32()
                .copied()
                .expect("Expected column 0 to be a int32!"),
            BottomImage: row
                .columns[1]
                .into_i32()
                .copied()
                .expect("Expected column 1 to be a int32!"),
            UnlockCondition: row
                .columns[6]
                .into_u16()
                .copied()
                .expect("Expected column 6 to be a uint16!"),
            Unknown_70_1: row
                .columns[7]
                .into_u16()
                .copied()
                .expect("Expected column 7 to be a uint16!"),
            Unknown_70_2: row
                .columns[8]
                .into_u16()
                .copied()
                .expect("Expected column 8 to be a uint16!"),
            Unknown4: row
                .columns[9]
                .into_u16()
                .copied()
                .expect("Expected column 9 to be a uint16!"),
            Unknown5: row
                .columns[11]
                .into_u16()
                .copied()
                .expect("Expected column 11 to be a uint16!"),
            FontColor: row
                .columns[2]
                .into_u8()
                .copied()
                .expect("Expected column 2 to be a uint8!"),
            Unknown2: row
                .columns[3]
                .into_u8()
                .copied()
                .expect("Expected column 3 to be a uint8!"),
            Unknown1: row
                .columns[4]
                .into_u8()
                .copied()
                .expect("Expected column 4 to be a uint8!"),
            Unknown0: row
                .columns[5]
                .into_u8()
                .copied()
                .expect("Expected column 5 to be a uint8!"),
            Category: row
                .columns[10]
                .into_u8()
                .copied()
                .expect("Expected column 10 to be a uint8!"),
        })
    }
}
impl<'a> IntoIterator for &'a CharaCardHeaderSheet {
    type Item = (u32, Vec<(u16, CharaCardHeaderRow)>);
    type IntoIter = StructuredSheetIterator<'a, CharaCardHeaderSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, CharaCardHeaderSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CharaCardHeaderRow {
    ///""
    pub Name: String,
    ///""
    pub TopImage: i32,
    ///""
    pub BottomImage: i32,
    ///""
    pub UnlockCondition: u16,
    ///""
    pub Unknown_70_1: u16,
    ///""
    pub Unknown_70_2: u16,
    ///""
    pub Unknown4: u16,
    ///""
    pub Unknown5: u16,
    ///""
    pub FontColor: u8,
    ///""
    pub Unknown2: u8,
    ///""
    pub Unknown1: u8,
    ///""
    pub Unknown0: u8,
    ///""
    pub Category: u8,
}
