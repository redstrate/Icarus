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
}
impl ClassJobActionUISheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("ClassJobActionUI")?;
        let sheet = resolver.read_excel_sheet(&exh, "ClassJobActionUI", language)?;
        Ok(Self { sheet })
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
        Some(Self::Row { row })
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
}
impl<'a> ClassJobActionUIRow<'a> {
    pub fn UpgradeAction(&'a self) -> u32 {
        self.row.columns[0].into_u32().copied().unwrap()
    }
    pub fn BaseAction(&'a self) -> u32 {
        self.row.columns[3].into_u32().copied().unwrap()
    }
    /// Used to position the action within a combo tree diagram. Non-zero digits identify branches within the combo (eg, Hakaze -> Shifu -> Kasha is 100 -> 120 -> 121)
    pub fn ComboTreeLayout(&'a self) -> u16 {
        self.row.columns[4].into_u16().copied().unwrap()
    }
    pub fn Unknown3(&'a self) -> u8 {
        self.row.columns[1].into_u8().copied().unwrap()
    }
    pub fn Unknown4(&'a self) -> u8 {
        self.row.columns[2].into_u8().copied().unwrap()
    }
    pub fn Unknown5(&'a self) -> u8 {
        self.row.columns[6].into_u8().copied().unwrap()
    }
    /// Currently only used for MNK actions; displays a set of actions in a shared rectangular cell instead of a tree.
    pub fn GroupedCell(&'a self) -> bool {
        self.row.columns[5].into_bool().copied().unwrap()
    }
}
