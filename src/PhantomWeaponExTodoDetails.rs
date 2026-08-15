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
pub struct PhantomWeaponExTodoDetailsSheet {
    sheet: Sheet,
}
impl PhantomWeaponExTodoDetailsSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("PhantomWeaponExTodoDetails")?;
        let sheet = resolver
            .read_excel_sheet(&exh, "PhantomWeaponExTodoDetails", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<PhantomWeaponExTodoDetailsRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<PhantomWeaponExTodoDetailsRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for PhantomWeaponExTodoDetailsSheet {
    type Row = PhantomWeaponExTodoDetailsRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            LogMessage: row
                .columns[0]
                .into_u32()
                .copied()
                .expect("Expected column 0 to be a uint32!"),
            ExtendedQuestCounter: row
                .columns[1]
                .into_u32()
                .copied()
                .expect("Expected column 1 to be a uint32!"),
            DetailText: row
                .columns[2]
                .into_u32()
                .copied()
                .expect("Expected column 2 to be a uint32!"),
            ContentFinderCondition: row
                .columns[3]
                .into_u32()
                .copied()
                .expect("Expected column 3 to be a uint32!"),
            TerritoryType: row
                .columns[5]
                .into_u32()
                .copied()
                .expect("Expected column 5 to be a uint32!"),
            Unknown5: row
                .columns[4]
                .into_u16()
                .copied()
                .expect("Expected column 4 to be a uint16!"),
        })
    }
}
impl<'a> IntoIterator for &'a PhantomWeaponExTodoDetailsSheet {
    type Item = (u32, Vec<(u16, PhantomWeaponExTodoDetailsRow)>);
    type IntoIter = StructuredSheetIterator<'a, PhantomWeaponExTodoDetailsSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, PhantomWeaponExTodoDetailsSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PhantomWeaponExTodoDetailsRow {
    ///""
    pub LogMessage: u32,
    ///""
    pub ExtendedQuestCounter: u32,
    ///""
    pub DetailText: u32,
    ///""
    pub ContentFinderCondition: u32,
    ///""
    pub TerritoryType: u32,
    ///""
    pub Unknown5: u16,
}
