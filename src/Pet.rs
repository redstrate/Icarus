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
pub struct PetSheet {
    sheet: Sheet,
}
impl PetSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Pet")?;
        let sheet = resolver.read_excel_sheet(&exh, "Pet", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<PetRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<PetRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for PetSheet {
    type Row = PetRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a PetSheet {
    type Item = (u32, Vec<(u16, PetRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, PetSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, PetSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct PetRow<'a> {
    row: &'a Row,
}
impl<'a> PetRow<'a> {
    pub fn Name(&'a self) -> &'a str {
        self.row.columns[0].into_string().unwrap()
    }
    pub fn Abilities(&'a self) -> [u16; 4] {
        [
            self.row.columns[1].into_u16().copied().unwrap(),
            self.row.columns[2].into_u16().copied().unwrap(),
            self.row.columns[3].into_u16().copied().unwrap(),
            self.row.columns[4].into_u16().copied().unwrap(),
        ]
    }
    pub fn AutoAction(&'a self) -> u16 {
        self.row.columns[5].into_u16().copied().unwrap()
    }
    pub fn SmallScalePercentage(&'a self) -> u8 {
        self.row.columns[9].into_u8().copied().unwrap()
    }
    pub fn MediumScalePercentage(&'a self) -> u8 {
        self.row.columns[10].into_u8().copied().unwrap()
    }
    pub fn LargeScalePercentage(&'a self) -> u8 {
        self.row.columns[11].into_u8().copied().unwrap()
    }
    pub fn Unknown8(&'a self) -> u8 {
        self.row.columns[13].into_u8().copied().unwrap()
    }
    pub fn Unknown9(&'a self) -> u8 {
        self.row.columns[14].into_u8().copied().unwrap()
    }
    pub fn Unknown10(&'a self) -> u8 {
        self.row.columns[15].into_u8().copied().unwrap()
    }
    pub fn Unknown11(&'a self) -> u8 {
        self.row.columns[16].into_u8().copied().unwrap()
    }
    pub fn Unknown12(&'a self) -> u8 {
        self.row.columns[17].into_u8().copied().unwrap()
    }
    pub fn Unknown13(&'a self) -> u8 {
        self.row.columns[18].into_u8().copied().unwrap()
    }
    pub fn Unknown14(&'a self) -> i8 {
        self.row.columns[12].into_i8().copied().unwrap()
    }
    pub fn Unknown15(&'a self) -> bool {
        self.row.columns[6].into_bool().copied().unwrap()
    }
    pub fn NonCombatSummon(&'a self) -> bool {
        self.row.columns[7].into_bool().copied().unwrap()
    }
    pub fn Unknown17(&'a self) -> bool {
        self.row.columns[8].into_bool().copied().unwrap()
    }
}
