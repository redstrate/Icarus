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
pub struct ConditionSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl ConditionSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Condition")?;
        let sheet = resolver.read_excel_sheet(&exh, "Condition", language)?;
        let mut index_mapping: Vec<(usize, &ExcelColumnDefinition)> = sheet
            .exh
            .column_definitions
            .iter()
            .enumerate()
            .collect();
        index_mapping.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
        let index_mapping: Vec<usize> = index_mapping
            .iter()
            .map(|(index, _)| *index)
            .collect();
        Ok(Self { sheet, index_mapping })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<ConditionRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<ConditionRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for ConditionSheet {
    type Row = ConditionRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a ConditionSheet {
    type Item = (u32, Vec<(u16, ConditionRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, ConditionSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, ConditionSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ConditionRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> ConditionRow<'a> {
    pub fn LogMessage(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[0]]
    }
    pub fn Permission(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1]]
    }
    /// The lower this number is, the higher the priority. Used to determine which conditions LogMessage should be printed.
    pub fn LogMessagePriority(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[2]]
    }
    /// Allows this Condition to be set by a packet received from the server.
    pub fn IsNetworked(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[3]]
    }
}
