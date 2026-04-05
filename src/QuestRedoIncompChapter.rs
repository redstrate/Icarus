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
pub struct QuestRedoIncompChapterSheet {
    sheet: Sheet,
}
impl QuestRedoIncompChapterSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("QuestRedoIncompChapter")?;
        let sheet = resolver.read_excel_sheet(&exh, "QuestRedoIncompChapter", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<QuestRedoIncompChapterRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<QuestRedoIncompChapterRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for QuestRedoIncompChapterSheet {
    type Row = QuestRedoIncompChapterRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a QuestRedoIncompChapterSheet {
    type Item = (u32, Vec<(u16, QuestRedoIncompChapterRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, QuestRedoIncompChapterSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, QuestRedoIncompChapterSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct QuestRedoIncompChapterRow<'a> {
    row: &'a Row,
}
impl<'a> QuestRedoIncompChapterRow<'a> {
    pub fn Chapter(&'a self) -> u16 {
        self.row.columns[0].into_u16().copied().unwrap()
    }
}
