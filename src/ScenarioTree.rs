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
pub struct ScenarioTreeSheet {
    sheet: Sheet,
}
impl ScenarioTreeSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 65564u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("ScenarioTree")?;
        let sheet = resolver.read_excel_sheet(&exh, "ScenarioTree", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<ScenarioTreeRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<ScenarioTreeRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for ScenarioTreeSheet {
    type Row = ScenarioTreeRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Name: row
                .columns[4]
                .into_string()
                .cloned()
                .expect("Expected column 4 to be a string!"),
            Addon: row
                .columns[2]
                .into_u32()
                .copied()
                .expect("Expected column 2 to be a uint32!"),
            QuestChapter: row
                .columns[3]
                .into_u32()
                .copied()
                .expect("Expected column 3 to be a uint32!"),
            Unknown0: row
                .columns[6]
                .into_u32()
                .copied()
                .expect("Expected column 6 to be a uint32!"),
            Unknown1: row
                .columns[5]
                .into_i32()
                .copied()
                .expect("Expected column 5 to be a int32!"),
            Unknown2: row
                .columns[1]
                .into_u16()
                .copied()
                .expect("Expected column 1 to be a uint16!"),
            Type: row
                .columns[0]
                .into_u8()
                .copied()
                .expect("Expected column 0 to be a uint8!"),
        })
    }
}
impl<'a> IntoIterator for &'a ScenarioTreeSheet {
    type Item = (u32, Vec<(u16, ScenarioTreeRow)>);
    type IntoIter = StructuredSheetIterator<'a, ScenarioTreeSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, ScenarioTreeSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ScenarioTreeRow {
    ///""
    pub Name: String,
    ///""
    pub Addon: u32,
    ///""
    pub QuestChapter: u32,
    ///""
    pub Unknown0: u32,
    ///""
    pub Unknown1: i32,
    ///""
    pub Unknown2: u16,
    ///""
    pub Type: u8,
}
