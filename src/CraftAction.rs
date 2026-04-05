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
pub struct CraftActionSheet {
    sheet: Sheet,
}
impl CraftActionSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("CraftAction")?;
        let sheet = resolver.read_excel_sheet(&exh, "CraftAction", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<CraftActionRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<CraftActionRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for CraftActionSheet {
    type Row = CraftActionRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a CraftActionSheet {
    type Item = (u32, Vec<(u16, CraftActionRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, CraftActionSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, CraftActionSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct CraftActionRow<'a> {
    row: &'a Row,
}
impl<'a> CraftActionRow<'a> {
    pub fn Name(&'a self) -> &'a str {
        self.row.columns[0].into_string().unwrap()
    }
    pub fn Description(&'a self) -> &'a str {
        self.row.columns[1].into_string().unwrap()
    }
    pub fn QuestRequirement(&'a self) -> u32 {
        self.row.columns[8].into_u32().copied().unwrap()
    }
    pub fn CRP(&'a self) -> i32 {
        self.row.columns[12].into_i32().copied().unwrap()
    }
    pub fn BSM(&'a self) -> i32 {
        self.row.columns[13].into_i32().copied().unwrap()
    }
    pub fn ARM(&'a self) -> i32 {
        self.row.columns[14].into_i32().copied().unwrap()
    }
    pub fn GSM(&'a self) -> i32 {
        self.row.columns[15].into_i32().copied().unwrap()
    }
    pub fn LTW(&'a self) -> i32 {
        self.row.columns[16].into_i32().copied().unwrap()
    }
    pub fn WVR(&'a self) -> i32 {
        self.row.columns[17].into_i32().copied().unwrap()
    }
    pub fn ALC(&'a self) -> i32 {
        self.row.columns[18].into_i32().copied().unwrap()
    }
    pub fn CUL(&'a self) -> i32 {
        self.row.columns[19].into_i32().copied().unwrap()
    }
    pub fn AnimationStart(&'a self) -> u16 {
        self.row.columns[2].into_u16().copied().unwrap()
    }
    pub fn AnimationEnd(&'a self) -> u16 {
        self.row.columns[3].into_u16().copied().unwrap()
    }
    pub fn Icon(&'a self) -> u16 {
        self.row.columns[4].into_u16().copied().unwrap()
    }
    pub fn RequiredStatus(&'a self) -> u16 {
        self.row.columns[10].into_u16().copied().unwrap()
    }
    pub fn ClassJobCategory(&'a self) -> u8 {
        self.row.columns[6].into_u8().copied().unwrap()
    }
    pub fn ClassJobLevel(&'a self) -> u8 {
        self.row.columns[7].into_u8().copied().unwrap()
    }
    pub fn Cost(&'a self) -> u8 {
        self.row.columns[11].into_u8().copied().unwrap()
    }
    pub fn ClassJob(&'a self) -> i8 {
        self.row.columns[5].into_i8().copied().unwrap()
    }
    pub fn Specialist(&'a self) -> bool {
        self.row.columns[9].into_bool().copied().unwrap()
    }
}
