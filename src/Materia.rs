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
pub struct MateriaSheet {
    sheet: Sheet,
}
impl MateriaSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Materia")?;
        let sheet = resolver.read_excel_sheet(&exh, "Materia", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<MateriaRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<MateriaRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for MateriaSheet {
    type Row = MateriaRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a MateriaSheet {
    type Item = (u32, Vec<(u16, MateriaRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, MateriaSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, MateriaSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct MateriaRow<'a> {
    row: &'a Row,
}
impl<'a> MateriaRow<'a> {
    pub fn Item(&'a self) -> [i32; 16] {
        [
            self.row.columns[0].into_i32().copied().unwrap(),
            self.row.columns[1].into_i32().copied().unwrap(),
            self.row.columns[2].into_i32().copied().unwrap(),
            self.row.columns[3].into_i32().copied().unwrap(),
            self.row.columns[4].into_i32().copied().unwrap(),
            self.row.columns[5].into_i32().copied().unwrap(),
            self.row.columns[6].into_i32().copied().unwrap(),
            self.row.columns[7].into_i32().copied().unwrap(),
            self.row.columns[8].into_i32().copied().unwrap(),
            self.row.columns[9].into_i32().copied().unwrap(),
            self.row.columns[10].into_i32().copied().unwrap(),
            self.row.columns[11].into_i32().copied().unwrap(),
            self.row.columns[12].into_i32().copied().unwrap(),
            self.row.columns[13].into_i32().copied().unwrap(),
            self.row.columns[14].into_i32().copied().unwrap(),
            self.row.columns[15].into_i32().copied().unwrap(),
        ]
    }
    pub fn Value(&'a self) -> [i16; 16] {
        [
            self.row.columns[17].into_i16().copied().unwrap(),
            self.row.columns[18].into_i16().copied().unwrap(),
            self.row.columns[19].into_i16().copied().unwrap(),
            self.row.columns[20].into_i16().copied().unwrap(),
            self.row.columns[21].into_i16().copied().unwrap(),
            self.row.columns[22].into_i16().copied().unwrap(),
            self.row.columns[23].into_i16().copied().unwrap(),
            self.row.columns[24].into_i16().copied().unwrap(),
            self.row.columns[25].into_i16().copied().unwrap(),
            self.row.columns[26].into_i16().copied().unwrap(),
            self.row.columns[27].into_i16().copied().unwrap(),
            self.row.columns[28].into_i16().copied().unwrap(),
            self.row.columns[29].into_i16().copied().unwrap(),
            self.row.columns[30].into_i16().copied().unwrap(),
            self.row.columns[31].into_i16().copied().unwrap(),
            self.row.columns[32].into_i16().copied().unwrap(),
        ]
    }
    pub fn BaseParam(&'a self) -> u8 {
        self.row.columns[16].into_u8().copied().unwrap()
    }
}
