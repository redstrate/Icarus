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
pub struct GameConditionValueArraySheet {
    sheet: Sheet,
}
impl GameConditionValueArraySheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("GameConditionValueArray")?;
        let sheet = resolver
            .read_excel_sheet(&exh, "GameConditionValueArray", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<GameConditionValueArrayRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<GameConditionValueArrayRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for GameConditionValueArraySheet {
    type Row = GameConditionValueArrayRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Unknown0: row
                .columns[0]
                .into_u32()
                .copied()
                .expect("Expected column 0 to be a uint32!"),
            Unknown1: row
                .columns[1]
                .into_u32()
                .copied()
                .expect("Expected column 1 to be a uint32!"),
            Unknown2: row
                .columns[2]
                .into_u32()
                .copied()
                .expect("Expected column 2 to be a uint32!"),
            Unknown3: row
                .columns[3]
                .into_u32()
                .copied()
                .expect("Expected column 3 to be a uint32!"),
            Unknown4: row
                .columns[4]
                .into_u32()
                .copied()
                .expect("Expected column 4 to be a uint32!"),
            Unknown5: row
                .columns[5]
                .into_u32()
                .copied()
                .expect("Expected column 5 to be a uint32!"),
            Unknown6: row
                .columns[6]
                .into_u32()
                .copied()
                .expect("Expected column 6 to be a uint32!"),
            Unknown7: row
                .columns[7]
                .into_u32()
                .copied()
                .expect("Expected column 7 to be a uint32!"),
        })
    }
}
impl<'a> IntoIterator for &'a GameConditionValueArraySheet {
    type Item = (u32, Vec<(u16, GameConditionValueArrayRow)>);
    type IntoIter = StructuredSheetIterator<'a, GameConditionValueArraySheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, GameConditionValueArraySheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct GameConditionValueArrayRow {
    ///""
    pub Unknown0: u32,
    ///""
    pub Unknown1: u32,
    ///""
    pub Unknown2: u32,
    ///""
    pub Unknown3: u32,
    ///""
    pub Unknown4: u32,
    ///""
    pub Unknown5: u32,
    ///""
    pub Unknown6: u32,
    ///""
    pub Unknown7: u32,
}
