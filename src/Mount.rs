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
pub struct MountSheet {
    sheet: Sheet,
}
impl MountSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Mount")?;
        let sheet = resolver.read_excel_sheet(&exh, "Mount", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<MountRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<MountRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for MountSheet {
    type Row = MountRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a MountSheet {
    type Item = (u32, Vec<(u16, MountRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, MountSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, MountSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct MountRow<'a> {
    row: &'a Row,
}
impl<'a> MountRow<'a> {
    pub fn Singular(&'a self) -> &'a str {
        self.row.columns[0].into_string().unwrap()
    }
    pub fn Plural(&'a self) -> &'a str {
        self.row.columns[2].into_string().unwrap()
    }
    pub fn Adjective(&'a self) -> i8 {
        self.row.columns[1].into_i8().copied().unwrap()
    }
    pub fn PossessivePronoun(&'a self) -> i8 {
        self.row.columns[3].into_i8().copied().unwrap()
    }
    pub fn StartsWithVowel(&'a self) -> i8 {
        self.row.columns[4].into_i8().copied().unwrap()
    }
    pub fn Unknown0(&'a self) -> i8 {
        self.row.columns[5].into_i8().copied().unwrap()
    }
    pub fn Pronoun(&'a self) -> i8 {
        self.row.columns[6].into_i8().copied().unwrap()
    }
    pub fn Article(&'a self) -> i8 {
        self.row.columns[7].into_i8().copied().unwrap()
    }
    pub fn Unknown1(&'a self) -> &'a str {
        self.row.columns[18].into_string().unwrap()
    }
    pub fn Unknown2(&'a self) -> &'a str {
        self.row.columns[19].into_string().unwrap()
    }
    pub fn Unknown3(&'a self) -> &'a str {
        self.row.columns[20].into_string().unwrap()
    }
    pub fn ModelChara(&'a self) -> i32 {
        self.row.columns[8].into_i32().copied().unwrap()
    }
    pub fn EquipHead(&'a self) -> i32 {
        self.row.columns[25].into_i32().copied().unwrap()
    }
    pub fn EquipBody(&'a self) -> i32 {
        self.row.columns[26].into_i32().copied().unwrap()
    }
    pub fn EquipLeg(&'a self) -> i32 {
        self.row.columns[27].into_i32().copied().unwrap()
    }
    pub fn EquipFoot(&'a self) -> i32 {
        self.row.columns[28].into_i32().copied().unwrap()
    }
    pub fn MoveControl(&'a self) -> u16 {
        self.row.columns[9].into_u16().copied().unwrap()
    }
    pub fn RideBGM(&'a self) -> u16 {
        self.row.columns[17].into_u16().copied().unwrap()
    }
    pub fn Icon(&'a self) -> u16 {
        self.row.columns[30].into_u16().copied().unwrap()
    }
    pub fn UIPriority(&'a self) -> u16 {
        self.row.columns[31].into_u16().copied().unwrap()
    }
    pub fn MountAction(&'a self) -> u16 {
        self.row.columns[38].into_u16().copied().unwrap()
    }
    pub fn Unknown_70_1(&'a self) -> u16 {
        self.row.columns[48].into_u16().copied().unwrap()
    }
    pub fn Unknown_70_2(&'a self) -> u16 {
        self.row.columns[49].into_u16().copied().unwrap()
    }
    pub fn Unknown16(&'a self) -> u16 {
        self.row.columns[50].into_u16().copied().unwrap()
    }
    pub fn Unknown17(&'a self) -> u16 {
        self.row.columns[52].into_u16().copied().unwrap()
    }
    pub fn Order(&'a self) -> i16 {
        self.row.columns[29].into_i16().copied().unwrap()
    }
    pub fn FlyingCondition(&'a self) -> u8 {
        self.row.columns[10].into_u8().copied().unwrap()
    }
    pub fn Unknown5(&'a self) -> u8 {
        self.row.columns[11].into_u8().copied().unwrap()
    }
    pub fn Unknown6(&'a self) -> u8 {
        self.row.columns[12].into_u8().copied().unwrap()
    }
    pub fn Unknown7(&'a self) -> u8 {
        self.row.columns[13].into_u8().copied().unwrap()
    }
    pub fn IsFlying(&'a self) -> u8 {
        self.row.columns[14].into_u8().copied().unwrap()
    }
    pub fn Unknown8(&'a self) -> u8 {
        self.row.columns[15].into_u8().copied().unwrap()
    }
    pub fn MountCustomize(&'a self) -> u8 {
        self.row.columns[16].into_u8().copied().unwrap()
    }
    pub fn ExitMoveDist(&'a self) -> u8 {
        self.row.columns[21].into_u8().copied().unwrap()
    }
    pub fn ExitMoveSpeed(&'a self) -> u8 {
        self.row.columns[22].into_u8().copied().unwrap()
    }
    pub fn RadiusRate(&'a self) -> u8 {
        self.row.columns[32].into_u8().copied().unwrap()
    }
    pub fn BaseMotionSpeed_Run(&'a self) -> u8 {
        self.row.columns[34].into_u8().copied().unwrap()
    }
    pub fn BaseMotionSpeed_Walk(&'a self) -> u8 {
        self.row.columns[35].into_u8().copied().unwrap()
    }
    pub fn Unknown9(&'a self) -> u8 {
        self.row.columns[36].into_u8().copied().unwrap()
    }
    pub fn ExtraSeats(&'a self) -> u8 {
        self.row.columns[37].into_u8().copied().unwrap()
    }
    pub fn Unknown10(&'a self) -> u8 {
        self.row.columns[44].into_u8().copied().unwrap()
    }
    pub fn Unknown11(&'a self) -> u8 {
        self.row.columns[45].into_u8().copied().unwrap()
    }
    pub fn Unknown12(&'a self) -> bool {
        self.row.columns[23].into_bool().copied().unwrap()
    }
    pub fn IsEmote(&'a self) -> bool {
        self.row.columns[24].into_bool().copied().unwrap()
    }
    pub fn Unknown20(&'a self) -> bool {
        self.row.columns[33].into_bool().copied().unwrap()
    }
    pub fn IsAirborne(&'a self) -> bool {
        self.row.columns[39].into_bool().copied().unwrap()
    }
    pub fn ExHotbarEnableConfig(&'a self) -> bool {
        self.row.columns[40].into_bool().copied().unwrap()
    }
    pub fn UseEP(&'a self) -> bool {
        self.row.columns[41].into_bool().copied().unwrap()
    }
    pub fn Unknown13(&'a self) -> bool {
        self.row.columns[42].into_bool().copied().unwrap()
    }
    pub fn IsImmobile(&'a self) -> bool {
        self.row.columns[43].into_bool().copied().unwrap()
    }
    pub fn Unknown14(&'a self) -> bool {
        self.row.columns[46].into_bool().copied().unwrap()
    }
    pub fn HideHeadgear(&'a self) -> bool {
        self.row.columns[47].into_bool().copied().unwrap()
    }
    pub fn Unknown18(&'a self) -> bool {
        self.row.columns[51].into_bool().copied().unwrap()
    }
    pub fn Unknown19(&'a self) -> bool {
        self.row.columns[53].into_bool().copied().unwrap()
    }
}
