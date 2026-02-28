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
pub struct PlayerSearchSubLocationSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl PlayerSearchSubLocationSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("PlayerSearchSubLocation")?;
        let sheet = resolver
            .read_excel_sheet(&exh, "PlayerSearchSubLocation", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<PlayerSearchSubLocationRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<PlayerSearchSubLocationRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for PlayerSearchSubLocationSheet {
    type Row = PlayerSearchSubLocationRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a PlayerSearchSubLocationSheet {
    type Item = (u32, Vec<(u16, PlayerSearchSubLocationRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, PlayerSearchSubLocationSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, PlayerSearchSubLocationSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct PlayerSearchSubLocationRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> PlayerSearchSubLocationRow<'a> {
    pub fn Name0(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[0]]
    }
    pub fn Name1(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1]]
    }
    pub fn Name2(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[2]]
    }
    pub fn PlaceName(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[3]]
    }
    /// The UI category this appears in.
    pub fn Location(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[4]]
    }
    pub fn SortKey(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[5]]
    }
}
