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
pub struct ActivityFeedImagesSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl ActivityFeedImagesSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("ActivityFeedImages")?;
        let sheet = resolver.read_excel_sheet(&exh, "ActivityFeedImages", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<ActivityFeedImagesRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<ActivityFeedImagesRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for ActivityFeedImagesSheet {
    type Row = ActivityFeedImagesRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a ActivityFeedImagesSheet {
    type Item = (u32, Vec<(u16, ActivityFeedImagesRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, ActivityFeedImagesSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, ActivityFeedImagesSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ActivityFeedImagesRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> ActivityFeedImagesRow<'a> {
    pub fn ExpansionImage(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[0]]
    }
    pub fn ActivityFeedJA(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1]]
    }
    pub fn ActivityFeedEN(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[2]]
    }
    pub fn ActivityFeedDE(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[3]]
    }
    pub fn ActivityFeedFR(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[4]]
    }
}
