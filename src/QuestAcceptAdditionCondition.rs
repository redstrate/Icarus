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
pub struct QuestAcceptAdditionConditionSheet {
    sheet: Sheet,
}
impl QuestAcceptAdditionConditionSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 65961u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("QuestAcceptAdditionCondition")?;
        let sheet = resolver
            .read_excel_sheet(&exh, "QuestAcceptAdditionCondition", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<QuestAcceptAdditionConditionRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<QuestAcceptAdditionConditionRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for QuestAcceptAdditionConditionSheet {
    type Row = QuestAcceptAdditionConditionRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Requirement0: row
                .columns[1]
                .into_u32()
                .copied()
                .expect("Expected column 1 to be a uint32!"),
            Requirement1: row
                .columns[2]
                .into_u32()
                .copied()
                .expect("Expected column 2 to be a uint32!"),
            Unknown0: row
                .columns[3]
                .into_u32()
                .copied()
                .expect("Expected column 3 to be a uint32!"),
            Unknown2: row
                .columns[0]
                .into_bool()
                .copied()
                .expect("Expected column 0 to be a bool!"),
            Unknown1: row
                .columns[4]
                .into_bool()
                .copied()
                .expect("Expected column 4 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a QuestAcceptAdditionConditionSheet {
    type Item = (u32, Vec<(u16, QuestAcceptAdditionConditionRow)>);
    type IntoIter = StructuredSheetIterator<'a, QuestAcceptAdditionConditionSheet>;
    fn into_iter(
        self,
    ) -> StructuredSheetIterator<'a, QuestAcceptAdditionConditionSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct QuestAcceptAdditionConditionRow {
    ///""
    pub Requirement0: u32,
    ///""
    pub Requirement1: u32,
    ///""
    pub Unknown0: u32,
    ///""
    pub Unknown2: bool,
    ///""
    pub Unknown1: bool,
}
