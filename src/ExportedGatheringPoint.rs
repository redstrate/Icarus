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
pub struct ExportedGatheringPointSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl ExportedGatheringPointSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("ExportedGatheringPoint")?;
        let sheet = resolver.read_excel_sheet(&exh, "ExportedGatheringPoint", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<ExportedGatheringPointRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<ExportedGatheringPointRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for ExportedGatheringPointSheet {
    type Row = ExportedGatheringPointRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a ExportedGatheringPointSheet {
    type Item = (u32, Vec<(u16, ExportedGatheringPointRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, ExportedGatheringPointSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, ExportedGatheringPointSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ExportedGatheringPointRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> ExportedGatheringPointRow<'a> {
    pub fn X(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[0]]
    }
    pub fn Y(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1]]
    }
    pub fn Radius(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[2]]
    }
    pub fn GatheringType(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[3]]
    }
    pub fn GatheringPointType(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[4]]
    }
}
