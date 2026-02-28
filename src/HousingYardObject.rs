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
pub struct HousingYardObjectSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl HousingYardObjectSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("HousingYardObject")?;
        let sheet = resolver.read_excel_sheet(&exh, "HousingYardObject", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<HousingYardObjectRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<HousingYardObjectRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for HousingYardObjectSheet {
    type Row = HousingYardObjectRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a HousingYardObjectSheet {
    type Item = (u32, Vec<(u16, HousingYardObjectRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, HousingYardObjectSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, HousingYardObjectSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct HousingYardObjectRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> HousingYardObjectRow<'a> {
    pub fn UsageParameter(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[0]]
    }
    pub fn CustomTalk(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1]]
    }
    pub fn Item(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[2]]
    }
    pub fn ModelKey(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[3]]
    }
    pub fn HousingItemCategory(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[4]]
    }
    pub fn UsageType(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[5]]
    }
    pub fn Unknown0(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[6]]
    }
    pub fn Unknown1(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[7]]
    }
    pub fn Unknown2(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[8]]
    }
    pub fn Unknown3(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[9]]
    }
    pub fn DestroyOnRemoval(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[10]]
    }
    pub fn Unknown4(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[11]]
    }
    pub fn Unknown5(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[12]]
    }
    pub fn Unknown6(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[13]]
    }
    pub fn Unknown7(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[14]]
    }
}
