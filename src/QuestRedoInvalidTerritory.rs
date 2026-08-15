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
pub struct QuestRedoInvalidTerritorySheet {
    sheet: Sheet,
}
impl QuestRedoInvalidTerritorySheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 1297u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("QuestRedoInvalidTerritory")?;
        let sheet = resolver
            .read_excel_sheet(&exh, "QuestRedoInvalidTerritory", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<QuestRedoInvalidTerritoryRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<QuestRedoInvalidTerritoryRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for QuestRedoInvalidTerritorySheet {
    type Row = QuestRedoInvalidTerritoryRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Unknown0: row
                .columns[0]
                .into_bool()
                .copied()
                .expect("Expected column 0 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a QuestRedoInvalidTerritorySheet {
    type Item = (u32, Vec<(u16, QuestRedoInvalidTerritoryRow)>);
    type IntoIter = StructuredSheetIterator<'a, QuestRedoInvalidTerritorySheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, QuestRedoInvalidTerritorySheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct QuestRedoInvalidTerritoryRow {
    ///""
    pub Unknown0: bool,
}
