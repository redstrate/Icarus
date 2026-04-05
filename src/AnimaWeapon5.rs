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
pub struct AnimaWeapon5Sheet {
    sheet: Sheet,
}
impl AnimaWeapon5Sheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("AnimaWeapon5")?;
        let sheet = resolver.read_excel_sheet(&exh, "AnimaWeapon5", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<AnimaWeapon5Row> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<AnimaWeapon5Row> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for AnimaWeapon5Sheet {
    type Row = AnimaWeapon5Row<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a AnimaWeapon5Sheet {
    type Item = (u32, Vec<(u16, AnimaWeapon5Row<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, AnimaWeapon5Sheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, AnimaWeapon5Sheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct AnimaWeapon5Row<'a> {
    row: &'a Row,
}
impl<'a> AnimaWeapon5Row<'a> {
    pub fn Item(&'a self) -> i32 {
        self.row.columns[0].into_i32().copied().unwrap()
    }
    pub fn Unknown0(&'a self) -> u8 {
        self.row.columns[1].into_u8().copied().unwrap()
    }
    pub fn SecondaryStatTotal(&'a self) -> u8 {
        self.row.columns[2].into_u8().copied().unwrap()
    }
    pub fn Parameter(&'a self) -> [u8; 5] {
        [
            self.row.columns[3].into_u8().copied().unwrap(),
            self.row.columns[4].into_u8().copied().unwrap(),
            self.row.columns[5].into_u8().copied().unwrap(),
            self.row.columns[6].into_u8().copied().unwrap(),
            self.row.columns[7].into_u8().copied().unwrap(),
        ]
    }
}
