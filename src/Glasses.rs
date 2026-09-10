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
pub struct GlassesSheet {
    sheet: Sheet,
}
impl GlassesSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Glasses")?;
        let sheet = resolver.read_excel_sheet(&exh, "Glasses", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<GlassesRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<GlassesRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for GlassesSheet {
    type Row = GlassesRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Singular: row
                .columns[4]
                .into_string()
                .cloned()
                .expect("Expected column 4 to be a string!"),
            Plural: row
                .columns[6]
                .into_string()
                .cloned()
                .expect("Expected column 6 to be a string!"),
            Description: row
                .columns[12]
                .into_string()
                .cloned()
                .expect("Expected column 12 to be a string!"),
            Name: row
                .columns[13]
                .into_string()
                .cloned()
                .expect("Expected column 13 to be a string!"),
            Adjective: row
                .columns[5]
                .into_i8()
                .copied()
                .expect("Expected column 5 to be a int8!"),
            PossessivePronoun: row
                .columns[7]
                .into_i8()
                .copied()
                .expect("Expected column 7 to be a int8!"),
            StartsWithVowel: row
                .columns[8]
                .into_i8()
                .copied()
                .expect("Expected column 8 to be a int8!"),
            Countability: row
                .columns[9]
                .into_i8()
                .copied()
                .expect("Expected column 9 to be a int8!"),
            Pronoun: row
                .columns[10]
                .into_i8()
                .copied()
                .expect("Expected column 10 to be a int8!"),
            Article: row
                .columns[11]
                .into_i8()
                .copied()
                .expect("Expected column 11 to be a int8!"),
            Model: row
                .columns[0]
                .into_u32()
                .copied()
                .expect("Expected column 0 to be a uint32!"),
            Icon: row
                .columns[2]
                .into_i32()
                .copied()
                .expect("Expected column 2 to be a int32!"),
            Unknown_70_8: row
                .columns[3]
                .into_u16()
                .copied()
                .expect("Expected column 3 to be a uint16!"),
            Style: row
                .columns[1]
                .into_i16()
                .copied()
                .expect("Expected column 1 to be a int16!"),
        })
    }
}
impl<'a> IntoIterator for &'a GlassesSheet {
    type Item = (u32, Vec<(u16, GlassesRow)>);
    type IntoIter = StructuredSheetIterator<'a, GlassesSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, GlassesSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct GlassesRow {
    ///""
    pub Singular: String,
    ///""
    pub Plural: String,
    ///""
    pub Description: String,
    ///""
    pub Name: String,
    ///""
    pub Adjective: i8,
    ///""
    pub PossessivePronoun: i8,
    ///""
    pub StartsWithVowel: i8,
    ///""
    pub Countability: i8,
    ///""
    pub Pronoun: i8,
    ///""
    pub Article: i8,
    ///""
    pub Model: u32,
    ///""
    pub Icon: i32,
    ///""
    pub Unknown_70_8: u16,
    ///""
    pub Style: i16,
}
