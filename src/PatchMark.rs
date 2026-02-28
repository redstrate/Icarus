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
pub struct PatchMarkSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl PatchMarkSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("PatchMark")?;
        let sheet = resolver.read_excel_sheet(&exh, "PatchMark", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<PatchMarkRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<PatchMarkRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for PatchMarkSheet {
    type Row = PatchMarkRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a PatchMarkSheet {
    type Item = (u32, Vec<(u16, PatchMarkRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, PatchMarkSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, PatchMarkSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct PatchMarkRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> PatchMarkRow<'a> {
    pub fn Requirement(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[0]]
    }
    pub fn MarkID(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1]]
    }
    pub fn SubCategory(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[2]]
    }
    pub fn Unknown1(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[3]]
    }
    pub fn SubCategoryType(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[4]]
    }
    /// 1 = Some Housing Permission 1
    /// 2 = Some Housing Permission 2
    /// 3 = Quest
    /// 4 = UnlockLink
    /// 5 = InstanceContent 50001 unlocked, but 50003 not
    /// 6 = LocalPlayer has a FreeCompanyTag
    ///
    pub fn RequirementType(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[5]]
    }
    pub fn Version(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[6]]
    }
    pub fn Category(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[7]]
    }
}
