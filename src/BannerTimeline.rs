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
pub struct BannerTimelineSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl BannerTimelineSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("BannerTimeline")?;
        let sheet = resolver.read_excel_sheet(&exh, "BannerTimeline", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<BannerTimelineRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<BannerTimelineRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for BannerTimelineSheet {
    type Row = BannerTimelineRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a BannerTimelineSheet {
    type Item = (u32, Vec<(u16, BannerTimelineRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, BannerTimelineSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, BannerTimelineSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct BannerTimelineRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> BannerTimelineRow<'a> {
    pub fn Name(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[0]]
    }
    pub fn AdditionalData(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1]]
    }
    pub fn Icon(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[2]]
    }
    pub fn UnlockCondition(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[3]]
    }
    pub fn Unknown_70_1(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[4]]
    }
    pub fn Unknown_70_2(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[5]]
    }
    pub fn Unknown0(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[6]]
    }
    pub fn SortKey(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[7]]
    }
    pub fn Type(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[8]]
    }
    pub fn AcceptClassJobCategory(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[9]]
    }
    pub fn Category(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[10]]
    }
}
