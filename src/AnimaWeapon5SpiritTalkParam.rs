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
pub struct AnimaWeapon5SpiritTalkParamSheet {
    sheet: Sheet,
}
impl AnimaWeapon5SpiritTalkParamSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("AnimaWeapon5SpiritTalkParam")?;
        let sheet = resolver
            .read_excel_sheet(&exh, "AnimaWeapon5SpiritTalkParam", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<AnimaWeapon5SpiritTalkParamRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<AnimaWeapon5SpiritTalkParamRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for AnimaWeapon5SpiritTalkParamSheet {
    type Row = AnimaWeapon5SpiritTalkParamRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Prologue: row
                .columns[0]
                .into_string()
                .cloned()
                .expect("Expected column 0 to be a string!"),
            Epilogue: row
                .columns[1]
                .into_string()
                .cloned()
                .expect("Expected column 1 to be a string!"),
        })
    }
}
impl<'a> IntoIterator for &'a AnimaWeapon5SpiritTalkParamSheet {
    type Item = (u32, Vec<(u16, AnimaWeapon5SpiritTalkParamRow)>);
    type IntoIter = StructuredSheetIterator<'a, AnimaWeapon5SpiritTalkParamSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, AnimaWeapon5SpiritTalkParamSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AnimaWeapon5SpiritTalkParamRow {
    ///""
    pub Prologue: String,
    ///""
    pub Epilogue: String,
}
