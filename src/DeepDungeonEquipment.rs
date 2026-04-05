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
pub struct DeepDungeonEquipmentSheet {
    sheet: Sheet,
}
impl DeepDungeonEquipmentSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("DeepDungeonEquipment")?;
        let sheet = resolver.read_excel_sheet(&exh, "DeepDungeonEquipment", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<DeepDungeonEquipmentRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<DeepDungeonEquipmentRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for DeepDungeonEquipmentSheet {
    type Row = DeepDungeonEquipmentRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a DeepDungeonEquipmentSheet {
    type Item = (u32, Vec<(u16, DeepDungeonEquipmentRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, DeepDungeonEquipmentSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, DeepDungeonEquipmentSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct DeepDungeonEquipmentRow<'a> {
    row: &'a Row,
}
impl<'a> DeepDungeonEquipmentRow<'a> {
    pub fn Singular(&'a self) -> &'a str {
        self.row.columns[1].into_string().unwrap()
    }
    pub fn Plural(&'a self) -> &'a str {
        self.row.columns[3].into_string().unwrap()
    }
    pub fn Name(&'a self) -> &'a str {
        self.row.columns[9].into_string().unwrap()
    }
    pub fn Description(&'a self) -> &'a str {
        self.row.columns[10].into_string().unwrap()
    }
    pub fn Adjective(&'a self) -> i8 {
        self.row.columns[2].into_i8().copied().unwrap()
    }
    pub fn PossessivePronoun(&'a self) -> i8 {
        self.row.columns[4].into_i8().copied().unwrap()
    }
    pub fn StartsWithVowel(&'a self) -> i8 {
        self.row.columns[5].into_i8().copied().unwrap()
    }
    pub fn Unknown0(&'a self) -> i8 {
        self.row.columns[6].into_i8().copied().unwrap()
    }
    pub fn Pronoun(&'a self) -> i8 {
        self.row.columns[7].into_i8().copied().unwrap()
    }
    pub fn Article(&'a self) -> i8 {
        self.row.columns[8].into_i8().copied().unwrap()
    }
    pub fn Icon(&'a self) -> u32 {
        self.row.columns[0].into_u32().copied().unwrap()
    }
}
