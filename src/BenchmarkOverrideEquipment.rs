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
pub struct BenchmarkOverrideEquipmentSheet {
    sheet: Sheet,
}
impl BenchmarkOverrideEquipmentSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("BenchmarkOverrideEquipment")?;
        let sheet = resolver
            .read_excel_sheet(&exh, "BenchmarkOverrideEquipment", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<BenchmarkOverrideEquipmentRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<BenchmarkOverrideEquipmentRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for BenchmarkOverrideEquipmentSheet {
    type Row = BenchmarkOverrideEquipmentRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a BenchmarkOverrideEquipmentSheet {
    type Item = (u32, Vec<(u16, BenchmarkOverrideEquipmentRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, BenchmarkOverrideEquipmentSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, BenchmarkOverrideEquipmentSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct BenchmarkOverrideEquipmentRow<'a> {
    row: &'a Row,
}
impl<'a> BenchmarkOverrideEquipmentRow<'a> {
    pub fn ModelMainHand(&'a self) -> u64 {
        self.row.columns[4].into_u64().copied().unwrap()
    }
    pub fn ModelOffHand(&'a self) -> u64 {
        self.row.columns[6].into_u64().copied().unwrap()
    }
    pub fn Unknown0(&'a self) -> u64 {
        self.row.columns[8].into_u64().copied().unwrap()
    }
    pub fn Unknown1(&'a self) -> u32 {
        self.row.columns[0].into_u32().copied().unwrap()
    }
    pub fn Unknown2(&'a self) -> u32 {
        self.row.columns[1].into_u32().copied().unwrap()
    }
    pub fn ModelHead(&'a self) -> u32 {
        self.row.columns[10].into_u32().copied().unwrap()
    }
    pub fn ModelBody(&'a self) -> u32 {
        self.row.columns[12].into_u32().copied().unwrap()
    }
    pub fn ModelHands(&'a self) -> u32 {
        self.row.columns[14].into_u32().copied().unwrap()
    }
    pub fn ModelLegs(&'a self) -> u32 {
        self.row.columns[16].into_u32().copied().unwrap()
    }
    pub fn ModelFeet(&'a self) -> u32 {
        self.row.columns[18].into_u32().copied().unwrap()
    }
    pub fn ModelEars(&'a self) -> u32 {
        self.row.columns[20].into_u32().copied().unwrap()
    }
    pub fn ModelNeck(&'a self) -> u32 {
        self.row.columns[22].into_u32().copied().unwrap()
    }
    pub fn ModelWrists(&'a self) -> u32 {
        self.row.columns[24].into_u32().copied().unwrap()
    }
    pub fn ModelLeftRing(&'a self) -> u32 {
        self.row.columns[26].into_u32().copied().unwrap()
    }
    pub fn ModelRightRing(&'a self) -> u32 {
        self.row.columns[28].into_u32().copied().unwrap()
    }
    pub fn Unknown3(&'a self) -> u8 {
        self.row.columns[2].into_u8().copied().unwrap()
    }
    pub fn DyeMainHand(&'a self) -> u8 {
        self.row.columns[5].into_u8().copied().unwrap()
    }
    pub fn DyeOffHand(&'a self) -> u8 {
        self.row.columns[7].into_u8().copied().unwrap()
    }
    pub fn Unknown4(&'a self) -> u8 {
        self.row.columns[9].into_u8().copied().unwrap()
    }
    pub fn DyeHead(&'a self) -> u8 {
        self.row.columns[11].into_u8().copied().unwrap()
    }
    pub fn DyeBody(&'a self) -> u8 {
        self.row.columns[13].into_u8().copied().unwrap()
    }
    pub fn DyeHands(&'a self) -> u8 {
        self.row.columns[15].into_u8().copied().unwrap()
    }
    pub fn DyeLegs(&'a self) -> u8 {
        self.row.columns[17].into_u8().copied().unwrap()
    }
    pub fn DyeFeet(&'a self) -> u8 {
        self.row.columns[19].into_u8().copied().unwrap()
    }
    pub fn DyeEars(&'a self) -> u8 {
        self.row.columns[21].into_u8().copied().unwrap()
    }
    pub fn DyeNeck(&'a self) -> u8 {
        self.row.columns[23].into_u8().copied().unwrap()
    }
    pub fn DyeWrists(&'a self) -> u8 {
        self.row.columns[25].into_u8().copied().unwrap()
    }
    pub fn DyeLeftRing(&'a self) -> u8 {
        self.row.columns[27].into_u8().copied().unwrap()
    }
    pub fn DyeRightRing(&'a self) -> u8 {
        self.row.columns[29].into_u8().copied().unwrap()
    }
    pub fn Unknown5(&'a self) -> i8 {
        self.row.columns[3].into_i8().copied().unwrap()
    }
}
