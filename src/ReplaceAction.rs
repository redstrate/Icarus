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
pub struct ReplaceActionSheet {
    sheet: Sheet,
}
impl ReplaceActionSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("ReplaceAction")?;
        let sheet = resolver.read_excel_sheet(&exh, "ReplaceAction", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<ReplaceActionRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<ReplaceActionRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for ReplaceActionSheet {
    type Row = ReplaceActionRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a ReplaceActionSheet {
    type Item = (u32, Vec<(u16, ReplaceActionRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, ReplaceActionSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, ReplaceActionSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ReplaceActionRow<'a> {
    row: &'a Row,
}
impl<'a> ReplaceActionRow<'a> {
    pub fn Action(&'a self) -> i32 {
        self.row.columns[0].into_i32().copied().unwrap()
    }
    pub fn ReplaceActions(&'a self) -> [i32; 4] {
        [
            self.row.columns[3].into_i32().copied().unwrap(),
            self.row.columns[6].into_i32().copied().unwrap(),
            self.row.columns[9].into_i32().copied().unwrap(),
            self.row.columns[12].into_i32().copied().unwrap(),
        ]
    }
    pub fn Param1(&'a self) -> i16 {
        self.row.columns[2].into_i16().copied().unwrap()
    }
    pub fn Param2(&'a self) -> i16 {
        self.row.columns[5].into_i16().copied().unwrap()
    }
    pub fn Param3(&'a self) -> i16 {
        self.row.columns[8].into_i16().copied().unwrap()
    }
    pub fn Param4(&'a self) -> i16 {
        self.row.columns[11].into_i16().copied().unwrap()
    }
    pub fn Type1(&'a self) -> i8 {
        self.row.columns[1].into_i8().copied().unwrap()
    }
    pub fn Type2(&'a self) -> i8 {
        self.row.columns[4].into_i8().copied().unwrap()
    }
    pub fn Type3(&'a self) -> i8 {
        self.row.columns[7].into_i8().copied().unwrap()
    }
    pub fn Type4(&'a self) -> i8 {
        self.row.columns[10].into_i8().copied().unwrap()
    }
    pub fn ReplaceSettable(&'a self) -> i8 {
        self.row.columns[13].into_i8().copied().unwrap()
    }
    pub fn Unknown_70(&'a self) -> bool {
        self.row.columns[14].into_bool().copied().unwrap()
    }
}
