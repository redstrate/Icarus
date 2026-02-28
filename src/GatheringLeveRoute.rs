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
pub struct GatheringLeveRouteSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl GatheringLeveRouteSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("GatheringLeveRoute")?;
        let sheet = resolver.read_excel_sheet(&exh, "GatheringLeveRoute", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<GatheringLeveRouteRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<GatheringLeveRouteRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for GatheringLeveRouteSheet {
    type Row = GatheringLeveRouteRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a GatheringLeveRouteSheet {
    type Item = (u32, Vec<(u16, GatheringLeveRouteRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, GatheringLeveRouteSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, GatheringLeveRouteSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct GatheringLeveRouteRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> GatheringLeveRouteRow<'a> {
    pub fn GatheringPoint(&'a self) -> [&'a Field; 12] {
        [
            &self.row.columns[self.index_mapping[0]],
            &self.row.columns[self.index_mapping[1]],
            &self.row.columns[self.index_mapping[2]],
            &self.row.columns[self.index_mapping[3]],
            &self.row.columns[self.index_mapping[4]],
            &self.row.columns[self.index_mapping[5]],
            &self.row.columns[self.index_mapping[6]],
            &self.row.columns[self.index_mapping[7]],
            &self.row.columns[self.index_mapping[8]],
            &self.row.columns[self.index_mapping[9]],
            &self.row.columns[self.index_mapping[10]],
            &self.row.columns[self.index_mapping[11]],
        ]
    }
    pub fn PopRange(&'a self) -> [&'a Field; 12] {
        [
            &self.row.columns[self.index_mapping[12]],
            &self.row.columns[self.index_mapping[13]],
            &self.row.columns[self.index_mapping[14]],
            &self.row.columns[self.index_mapping[15]],
            &self.row.columns[self.index_mapping[16]],
            &self.row.columns[self.index_mapping[17]],
            &self.row.columns[self.index_mapping[18]],
            &self.row.columns[self.index_mapping[19]],
            &self.row.columns[self.index_mapping[20]],
            &self.row.columns[self.index_mapping[21]],
            &self.row.columns[self.index_mapping[22]],
            &self.row.columns[self.index_mapping[23]],
        ]
    }
}
