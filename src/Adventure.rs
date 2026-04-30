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
pub struct AdventureSheet {
    sheet: Sheet,
}
impl AdventureSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Adventure")?;
        let sheet = resolver.read_excel_sheet(&exh, "Adventure", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<AdventureRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<AdventureRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for AdventureSheet {
    type Row = AdventureRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Name: row
                .columns[13]
                .into_string()
                .cloned()
                .expect("Expected column 13 to be a string!"),
            Impression: row
                .columns[14]
                .into_string()
                .cloned()
                .expect("Expected column 14 to be a string!"),
            Description: row
                .columns[15]
                .into_string()
                .cloned()
                .expect("Expected column 15 to be a string!"),
            Level: row
                .columns[0]
                .into_i32()
                .copied()
                .expect("Expected column 0 to be a int32!"),
            MinLevel: row
                .columns[1]
                .into_i32()
                .copied()
                .expect("Expected column 1 to be a int32!"),
            PlaceName: row
                .columns[8]
                .into_i32()
                .copied()
                .expect("Expected column 8 to be a int32!"),
            IconList: row
                .columns[9]
                .into_i32()
                .copied()
                .expect("Expected column 9 to be a int32!"),
            IconDiscovered: row
                .columns[10]
                .into_i32()
                .copied()
                .expect("Expected column 10 to be a int32!"),
            IconUndiscovered: row
                .columns[11]
                .into_i32()
                .copied()
                .expect("Expected column 11 to be a int32!"),
            Emote: row
                .columns[3]
                .into_u16()
                .copied()
                .expect("Expected column 3 to be a uint16!"),
            MinTime: row
                .columns[6]
                .into_u16()
                .copied()
                .expect("Expected column 6 to be a uint16!"),
            MaxTime: row
                .columns[7]
                .into_u16()
                .copied()
                .expect("Expected column 7 to be a uint16!"),
            MaxLevel: row
                .columns[2]
                .into_u8()
                .copied()
                .expect("Expected column 2 to be a uint8!"),
            IsInitial: row
                .columns[4]
                .into_u8()
                .copied()
                .expect("Expected column 4 to be a uint8!"),
            Unknown0: row
                .columns[5]
                .into_u8()
                .copied()
                .expect("Expected column 5 to be a uint8!"),
            Unknown1: row
                .columns[12]
                .into_bool()
                .copied()
                .expect("Expected column 12 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a AdventureSheet {
    type Item = (u32, Vec<(u16, AdventureRow)>);
    type IntoIter = StructuredSheetIterator<'a, AdventureSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, AdventureSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AdventureRow {
    ///""
    pub Name: String,
    ///""
    pub Impression: String,
    ///""
    pub Description: String,
    ///""
    pub Level: i32,
    ///""
    pub MinLevel: i32,
    ///""
    pub PlaceName: i32,
    ///""
    pub IconList: i32,
    ///""
    pub IconDiscovered: i32,
    ///""
    pub IconUndiscovered: i32,
    ///""
    pub Emote: u16,
    ///""
    pub MinTime: u16,
    ///""
    pub MaxTime: u16,
    ///""
    pub MaxLevel: u8,
    ///""
    pub IsInitial: u8,
    ///""
    pub Unknown0: u8,
    ///""
    pub Unknown1: bool,
}
