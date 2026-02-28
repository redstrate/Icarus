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
pub struct ClassJobActionUISheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl ClassJobActionUISheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("ClassJobActionUI")?;
        let sheet = resolver.read_excel_sheet(&exh, "ClassJobActionUI", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<ClassJobActionUIRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<ClassJobActionUIRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for ClassJobActionUISheet {
    type Row = ClassJobActionUIRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a ClassJobActionUISheet {
    type Item = (u32, Vec<(u16, ClassJobActionUIRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, ClassJobActionUISheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, ClassJobActionUISheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ClassJobActionUIRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> ClassJobActionUIRow<'a> {
    pub fn UpgradeAction(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[0]]
    }
    pub fn BaseAction(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1]]
    }
    /// Used to position the action within a combo tree diagram. Non-zero digits identify branches within the combo (eg, Hakaze -> Shifu -> Kasha is 100 -> 120 -> 121)
    pub fn ComboTreeLayout(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[2]]
    }
    pub fn Unknown3(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[3]]
    }
    pub fn Unknown4(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[4]]
    }
    pub fn Unknown5(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[5]]
    }
    /// Currently only used for MNK actions; displays a set of actions in a shared rectangular cell instead of a tree.
    pub fn GroupedCell(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[6]]
    }
}
