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
pub struct CompanionTransientSheet {
    sheet: Sheet,
}
impl CompanionTransientSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("CompanionTransient")?;
        let sheet = resolver.read_excel_sheet(&exh, "CompanionTransient", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<CompanionTransientRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<CompanionTransientRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for CompanionTransientSheet {
    type Row = CompanionTransientRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a CompanionTransientSheet {
    type Item = (u32, Vec<(u16, CompanionTransientRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, CompanionTransientSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, CompanionTransientSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct CompanionTransientRow<'a> {
    row: &'a Row,
}
impl<'a> CompanionTransientRow<'a> {
    pub fn Description(&'a self) -> &'a str {
        self.row.columns[0].into_string().unwrap()
    }
    pub fn DescriptionEnhanced(&'a self) -> &'a str {
        self.row.columns[1].into_string().unwrap()
    }
    pub fn Tooltip(&'a self) -> &'a str {
        self.row.columns[2].into_string().unwrap()
    }
    pub fn SpecialActionName(&'a self) -> &'a str {
        self.row.columns[3].into_string().unwrap()
    }
    pub fn SpecialActionDescription(&'a self) -> &'a str {
        self.row.columns[4].into_string().unwrap()
    }
    pub fn Attack(&'a self) -> u8 {
        self.row.columns[5].into_u8().copied().unwrap()
    }
    pub fn Defense(&'a self) -> u8 {
        self.row.columns[6].into_u8().copied().unwrap()
    }
    pub fn Speed(&'a self) -> u8 {
        self.row.columns[7].into_u8().copied().unwrap()
    }
    pub fn MinionSkillType(&'a self) -> u8 {
        self.row.columns[13].into_u8().copied().unwrap()
    }
    pub fn HasAreaAttack(&'a self) -> bool {
        self.row.columns[8].into_bool().copied().unwrap()
    }
    pub fn StrengthGate(&'a self) -> bool {
        self.row.columns[9].into_bool().copied().unwrap()
    }
    pub fn StrengthEye(&'a self) -> bool {
        self.row.columns[10].into_bool().copied().unwrap()
    }
    pub fn StrengthShield(&'a self) -> bool {
        self.row.columns[11].into_bool().copied().unwrap()
    }
    pub fn StrengthArcana(&'a self) -> bool {
        self.row.columns[12].into_bool().copied().unwrap()
    }
}
