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
pub struct PhysicsOffGroupSheet {
    sheet: Sheet,
}
impl PhysicsOffGroupSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("PhysicsOffGroup")?;
        let sheet = resolver.read_excel_sheet(&exh, "PhysicsOffGroup", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<PhysicsOffGroupRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<PhysicsOffGroupRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for PhysicsOffGroupSheet {
    type Row = PhysicsOffGroupRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a PhysicsOffGroupSheet {
    type Item = (u32, Vec<(u16, PhysicsOffGroupRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, PhysicsOffGroupSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, PhysicsOffGroupSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct PhysicsOffGroupRow<'a> {
    row: &'a Row,
}
impl<'a> PhysicsOffGroupRow<'a> {
    pub fn Unknown0(&'a self) -> i8 {
        self.row.columns[0].into_i8().copied().unwrap()
    }
    pub fn Unknown1(&'a self) -> i8 {
        self.row.columns[1].into_i8().copied().unwrap()
    }
    pub fn Unknown_70_1(&'a self) -> i8 {
        self.row.columns[2].into_i8().copied().unwrap()
    }
    pub fn Unknown_70_2(&'a self) -> i8 {
        self.row.columns[3].into_i8().copied().unwrap()
    }
    pub fn Unknown_70_3(&'a self) -> i8 {
        self.row.columns[4].into_i8().copied().unwrap()
    }
    pub fn Unknown_70_4(&'a self) -> i8 {
        self.row.columns[5].into_i8().copied().unwrap()
    }
    pub fn Unknown_70_5(&'a self) -> i8 {
        self.row.columns[6].into_i8().copied().unwrap()
    }
    pub fn Unknown_70_6(&'a self) -> i8 {
        self.row.columns[7].into_i8().copied().unwrap()
    }
    pub fn Unknown_70_7(&'a self) -> i8 {
        self.row.columns[8].into_i8().copied().unwrap()
    }
    pub fn Unknown_70_8(&'a self) -> i8 {
        self.row.columns[9].into_i8().copied().unwrap()
    }
    pub fn Unknown_70_9(&'a self) -> i8 {
        self.row.columns[10].into_i8().copied().unwrap()
    }
    pub fn Unknown_70_10(&'a self) -> i8 {
        self.row.columns[11].into_i8().copied().unwrap()
    }
    pub fn Unknown_70_11(&'a self) -> i8 {
        self.row.columns[12].into_i8().copied().unwrap()
    }
    pub fn Unknown_70_12(&'a self) -> i8 {
        self.row.columns[13].into_i8().copied().unwrap()
    }
    pub fn Unknown_70_13(&'a self) -> i8 {
        self.row.columns[14].into_i8().copied().unwrap()
    }
    pub fn Unknown_70_14(&'a self) -> i8 {
        self.row.columns[15].into_i8().copied().unwrap()
    }
}
