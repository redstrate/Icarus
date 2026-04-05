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
pub struct AOZContentBriefingBNpcSheet {
    sheet: Sheet,
}
impl AOZContentBriefingBNpcSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("AOZContentBriefingBNpc")?;
        let sheet = resolver.read_excel_sheet(&exh, "AOZContentBriefingBNpc", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<AOZContentBriefingBNpcRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<AOZContentBriefingBNpcRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for AOZContentBriefingBNpcSheet {
    type Row = AOZContentBriefingBNpcRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a AOZContentBriefingBNpcSheet {
    type Item = (u32, Vec<(u16, AOZContentBriefingBNpcRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, AOZContentBriefingBNpcSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, AOZContentBriefingBNpcSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct AOZContentBriefingBNpcRow<'a> {
    row: &'a Row,
}
impl<'a> AOZContentBriefingBNpcRow<'a> {
    pub fn BNpcName(&'a self) -> u32 {
        self.row.columns[0].into_u32().copied().unwrap()
    }
    pub fn TargetSmall(&'a self) -> u32 {
        self.row.columns[1].into_u32().copied().unwrap()
    }
    pub fn TargetLarge(&'a self) -> u32 {
        self.row.columns[2].into_u32().copied().unwrap()
    }
    pub fn Endurance(&'a self) -> u8 {
        self.row.columns[4].into_u8().copied().unwrap()
    }
    pub fn Fire(&'a self) -> u8 {
        self.row.columns[5].into_u8().copied().unwrap()
    }
    pub fn Ice(&'a self) -> u8 {
        self.row.columns[6].into_u8().copied().unwrap()
    }
    pub fn Wind(&'a self) -> u8 {
        self.row.columns[7].into_u8().copied().unwrap()
    }
    pub fn Earth(&'a self) -> u8 {
        self.row.columns[8].into_u8().copied().unwrap()
    }
    pub fn Thunder(&'a self) -> u8 {
        self.row.columns[9].into_u8().copied().unwrap()
    }
    pub fn Water(&'a self) -> u8 {
        self.row.columns[10].into_u8().copied().unwrap()
    }
    pub fn Slashing(&'a self) -> u8 {
        self.row.columns[11].into_u8().copied().unwrap()
    }
    pub fn Piercing(&'a self) -> u8 {
        self.row.columns[12].into_u8().copied().unwrap()
    }
    pub fn Blunt(&'a self) -> u8 {
        self.row.columns[13].into_u8().copied().unwrap()
    }
    pub fn Magic(&'a self) -> u8 {
        self.row.columns[14].into_u8().copied().unwrap()
    }
    pub fn HideStats(&'a self) -> bool {
        self.row.columns[3].into_bool().copied().unwrap()
    }
    pub fn SlowVuln(&'a self) -> bool {
        self.row.columns[15].into_bool().copied().unwrap()
    }
    pub fn PetrificationVuln(&'a self) -> bool {
        self.row.columns[16].into_bool().copied().unwrap()
    }
    pub fn ParalysisVuln(&'a self) -> bool {
        self.row.columns[17].into_bool().copied().unwrap()
    }
    pub fn InterruptionVuln(&'a self) -> bool {
        self.row.columns[18].into_bool().copied().unwrap()
    }
    pub fn BlindVuln(&'a self) -> bool {
        self.row.columns[19].into_bool().copied().unwrap()
    }
    pub fn StunVuln(&'a self) -> bool {
        self.row.columns[20].into_bool().copied().unwrap()
    }
    pub fn SleepVuln(&'a self) -> bool {
        self.row.columns[21].into_bool().copied().unwrap()
    }
    pub fn BindVuln(&'a self) -> bool {
        self.row.columns[22].into_bool().copied().unwrap()
    }
    pub fn HeavyVuln(&'a self) -> bool {
        self.row.columns[23].into_bool().copied().unwrap()
    }
    pub fn FlatOrDeathVuln(&'a self) -> bool {
        self.row.columns[24].into_bool().copied().unwrap()
    }
}
