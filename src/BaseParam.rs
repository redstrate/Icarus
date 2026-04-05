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
pub struct BaseParamSheet {
    sheet: Sheet,
}
impl BaseParamSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("BaseParam")?;
        let sheet = resolver.read_excel_sheet(&exh, "BaseParam", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<BaseParamRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<BaseParamRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for BaseParamSheet {
    type Row = BaseParamRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a BaseParamSheet {
    type Item = (u32, Vec<(u16, BaseParamRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, BaseParamSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, BaseParamSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct BaseParamRow<'a> {
    row: &'a Row,
}
impl<'a> BaseParamRow<'a> {
    pub fn Name(&'a self) -> &'a str {
        self.row.columns[1].into_string().unwrap()
    }
    pub fn Description(&'a self) -> &'a str {
        self.row.columns[2].into_string().unwrap()
    }
    pub fn OneHandWeaponPercent(&'a self) -> u16 {
        self.row.columns[4].into_u16().copied().unwrap()
    }
    pub fn OffHandPercent(&'a self) -> u16 {
        self.row.columns[5].into_u16().copied().unwrap()
    }
    pub fn HeadPercent(&'a self) -> u16 {
        self.row.columns[6].into_u16().copied().unwrap()
    }
    pub fn ChestPercent(&'a self) -> u16 {
        self.row.columns[7].into_u16().copied().unwrap()
    }
    pub fn HandsPercent(&'a self) -> u16 {
        self.row.columns[8].into_u16().copied().unwrap()
    }
    pub fn WaistPercent(&'a self) -> u16 {
        self.row.columns[9].into_u16().copied().unwrap()
    }
    pub fn LegsPercent(&'a self) -> u16 {
        self.row.columns[10].into_u16().copied().unwrap()
    }
    pub fn FeetPercent(&'a self) -> u16 {
        self.row.columns[11].into_u16().copied().unwrap()
    }
    pub fn EarringPercent(&'a self) -> u16 {
        self.row.columns[12].into_u16().copied().unwrap()
    }
    pub fn NecklacePercent(&'a self) -> u16 {
        self.row.columns[13].into_u16().copied().unwrap()
    }
    pub fn BraceletPercent(&'a self) -> u16 {
        self.row.columns[14].into_u16().copied().unwrap()
    }
    pub fn RingPercent(&'a self) -> u16 {
        self.row.columns[15].into_u16().copied().unwrap()
    }
    pub fn TwoHandWeaponPercent(&'a self) -> u16 {
        self.row.columns[16].into_u16().copied().unwrap()
    }
    pub fn UnderArmorPercent(&'a self) -> u16 {
        self.row.columns[17].into_u16().copied().unwrap()
    }
    pub fn ChestHeadPercent(&'a self) -> u16 {
        self.row.columns[18].into_u16().copied().unwrap()
    }
    pub fn ChestHeadLegsFeetPercent(&'a self) -> u16 {
        self.row.columns[19].into_u16().copied().unwrap()
    }
    pub fn Unknown0(&'a self) -> u16 {
        self.row.columns[20].into_u16().copied().unwrap()
    }
    pub fn LegsFeetPercent(&'a self) -> u16 {
        self.row.columns[21].into_u16().copied().unwrap()
    }
    pub fn HeadChestHandsLegsFeetPercent(&'a self) -> u16 {
        self.row.columns[22].into_u16().copied().unwrap()
    }
    pub fn ChestLegsGlovesPercent(&'a self) -> u16 {
        self.row.columns[23].into_u16().copied().unwrap()
    }
    pub fn ChestLegsFeetPercent(&'a self) -> u16 {
        self.row.columns[24].into_u16().copied().unwrap()
    }
    pub fn Unknown1(&'a self) -> u16 {
        self.row.columns[25].into_u16().copied().unwrap()
    }
    pub fn Unknown3(&'a self) -> u16 {
        self.row.columns[26].into_u16().copied().unwrap()
    }
    pub fn OrderPriority(&'a self) -> u8 {
        self.row.columns[3].into_u8().copied().unwrap()
    }
    pub fn MeldParam(&'a self) -> [u8; 13] {
        [
            self.row.columns[27].into_u8().copied().unwrap(),
            self.row.columns[28].into_u8().copied().unwrap(),
            self.row.columns[29].into_u8().copied().unwrap(),
            self.row.columns[30].into_u8().copied().unwrap(),
            self.row.columns[31].into_u8().copied().unwrap(),
            self.row.columns[32].into_u8().copied().unwrap(),
            self.row.columns[33].into_u8().copied().unwrap(),
            self.row.columns[34].into_u8().copied().unwrap(),
            self.row.columns[35].into_u8().copied().unwrap(),
            self.row.columns[36].into_u8().copied().unwrap(),
            self.row.columns[37].into_u8().copied().unwrap(),
            self.row.columns[38].into_u8().copied().unwrap(),
            self.row.columns[39].into_u8().copied().unwrap(),
        ]
    }
    pub fn PacketIndex(&'a self) -> i8 {
        self.row.columns[0].into_i8().copied().unwrap()
    }
    pub fn Unknown2(&'a self) -> bool {
        self.row.columns[40].into_bool().copied().unwrap()
    }
}
