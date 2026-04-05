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
pub struct StatusSheet {
    sheet: Sheet,
}
impl StatusSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Status")?;
        let sheet = resolver.read_excel_sheet(&exh, "Status", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<StatusRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<StatusRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for StatusSheet {
    type Row = StatusRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a StatusSheet {
    type Item = (u32, Vec<(u16, StatusRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, StatusSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, StatusSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct StatusRow<'a> {
    row: &'a Row,
}
impl<'a> StatusRow<'a> {
    pub fn Name(&'a self) -> &'a str {
        self.row.columns[0].into_string().unwrap()
    }
    pub fn Description(&'a self) -> &'a str {
        self.row.columns[1].into_string().unwrap()
    }
    pub fn Icon(&'a self) -> u32 {
        self.row.columns[2].into_u32().copied().unwrap()
    }
    pub fn ParamModifier(&'a self) -> i32 {
        self.row.columns[22].into_i32().copied().unwrap()
    }
    pub fn VFX(&'a self) -> u16 {
        self.row.columns[8].into_u16().copied().unwrap()
    }
    pub fn Log(&'a self) -> u16 {
        self.row.columns[25].into_u16().copied().unwrap()
    }
    pub fn Unknown0(&'a self) -> u8 {
        self.row.columns[3].into_u8().copied().unwrap()
    }
    pub fn MaxStacks(&'a self) -> u8 {
        self.row.columns[4].into_u8().copied().unwrap()
    }
    pub fn ClassJobCategory(&'a self) -> u8 {
        self.row.columns[5].into_u8().copied().unwrap()
    }
    pub fn StatusCategory(&'a self) -> u8 {
        self.row.columns[6].into_u8().copied().unwrap()
    }
    pub fn HitEffect(&'a self) -> u8 {
        self.row.columns[7].into_u8().copied().unwrap()
    }
    pub fn PartyListPriority(&'a self) -> u8 {
        self.row.columns[18].into_u8().copied().unwrap()
    }
    pub fn CanIncreaseRewards(&'a self) -> u8 {
        self.row.columns[19].into_u8().copied().unwrap()
    }
    pub fn ParamEffect(&'a self) -> u8 {
        self.row.columns[23].into_u8().copied().unwrap()
    }
    pub fn TargetType(&'a self) -> u8 {
        self.row.columns[29].into_u8().copied().unwrap()
    }
    /// actually an index of the flag
    pub fn Flags(&'a self) -> u8 {
        self.row.columns[30].into_u8().copied().unwrap()
    }
    pub fn Flag2(&'a self) -> u8 {
        self.row.columns[31].into_u8().copied().unwrap()
    }
    pub fn Unknown_70_1(&'a self) -> u8 {
        self.row.columns[35].into_u8().copied().unwrap()
    }
    pub fn Unknown2(&'a self) -> i8 {
        self.row.columns[27].into_i8().copied().unwrap()
    }
    pub fn LockMovement(&'a self) -> bool {
        self.row.columns[9].into_bool().copied().unwrap()
    }
    pub fn Unknown3(&'a self) -> bool {
        self.row.columns[10].into_bool().copied().unwrap()
    }
    pub fn LockActions(&'a self) -> bool {
        self.row.columns[11].into_bool().copied().unwrap()
    }
    pub fn LockControl(&'a self) -> bool {
        self.row.columns[12].into_bool().copied().unwrap()
    }
    pub fn Transfiguration(&'a self) -> bool {
        self.row.columns[13].into_bool().copied().unwrap()
    }
    pub fn IsGaze(&'a self) -> bool {
        self.row.columns[14].into_bool().copied().unwrap()
    }
    pub fn CanDispel(&'a self) -> bool {
        self.row.columns[15].into_bool().copied().unwrap()
    }
    pub fn InflictedByActor(&'a self) -> bool {
        self.row.columns[16].into_bool().copied().unwrap()
    }
    pub fn IsPermanent(&'a self) -> bool {
        self.row.columns[17].into_bool().copied().unwrap()
    }
    pub fn NoLogVfx(&'a self) -> bool {
        self.row.columns[20].into_bool().copied().unwrap()
    }
    pub fn Unknown5(&'a self) -> bool {
        self.row.columns[21].into_bool().copied().unwrap()
    }
    pub fn CanStatusOff(&'a self) -> bool {
        self.row.columns[24].into_bool().copied().unwrap()
    }
    pub fn IsFcBuff(&'a self) -> bool {
        self.row.columns[26].into_bool().copied().unwrap()
    }
    pub fn Invisibility(&'a self) -> bool {
        self.row.columns[28].into_bool().copied().unwrap()
    }
    pub fn Unknown6(&'a self) -> bool {
        self.row.columns[32].into_bool().copied().unwrap()
    }
    pub fn Unknown_70_2(&'a self) -> bool {
        self.row.columns[33].into_bool().copied().unwrap()
    }
    pub fn Unknown7(&'a self) -> bool {
        self.row.columns[34].into_bool().copied().unwrap()
    }
}
