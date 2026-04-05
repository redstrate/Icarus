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
pub struct BNpcPartsSheet {
    sheet: Sheet,
}
impl BNpcPartsSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("BNpcParts")?;
        let sheet = resolver.read_excel_sheet(&exh, "BNpcParts", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<BNpcPartsRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<BNpcPartsRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for BNpcPartsSheet {
    type Row = BNpcPartsRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a BNpcPartsSheet {
    type Item = (u32, Vec<(u16, BNpcPartsRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, BNpcPartsSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, BNpcPartsSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct BNpcPartsRow<'a> {
    row: &'a Row,
}
impl<'a> BNpcPartsRow<'a> {
    pub fn X1(&'a self) -> f32 {
        self.row.columns[7].into_f32().copied().unwrap()
    }
    pub fn X2(&'a self) -> f32 {
        self.row.columns[18].into_f32().copied().unwrap()
    }
    pub fn X3(&'a self) -> f32 {
        self.row.columns[29].into_f32().copied().unwrap()
    }
    pub fn X4(&'a self) -> f32 {
        self.row.columns[40].into_f32().copied().unwrap()
    }
    pub fn X5(&'a self) -> f32 {
        self.row.columns[51].into_f32().copied().unwrap()
    }
    pub fn Unknown0(&'a self) -> f32 {
        self.row.columns[62].into_f32().copied().unwrap()
    }
    pub fn Y1(&'a self) -> f32 {
        self.row.columns[8].into_f32().copied().unwrap()
    }
    pub fn Y2(&'a self) -> f32 {
        self.row.columns[19].into_f32().copied().unwrap()
    }
    pub fn Y3(&'a self) -> f32 {
        self.row.columns[30].into_f32().copied().unwrap()
    }
    pub fn Y4(&'a self) -> f32 {
        self.row.columns[41].into_f32().copied().unwrap()
    }
    pub fn Y5(&'a self) -> f32 {
        self.row.columns[52].into_f32().copied().unwrap()
    }
    pub fn Unknown1(&'a self) -> f32 {
        self.row.columns[63].into_f32().copied().unwrap()
    }
    pub fn Z1(&'a self) -> f32 {
        self.row.columns[9].into_f32().copied().unwrap()
    }
    pub fn Z2(&'a self) -> f32 {
        self.row.columns[20].into_f32().copied().unwrap()
    }
    pub fn Z3(&'a self) -> f32 {
        self.row.columns[31].into_f32().copied().unwrap()
    }
    pub fn Z4(&'a self) -> f32 {
        self.row.columns[42].into_f32().copied().unwrap()
    }
    pub fn Z5(&'a self) -> f32 {
        self.row.columns[53].into_f32().copied().unwrap()
    }
    pub fn Unknown2(&'a self) -> f32 {
        self.row.columns[64].into_f32().copied().unwrap()
    }
    pub fn Scale1(&'a self) -> f32 {
        self.row.columns[11].into_f32().copied().unwrap()
    }
    pub fn Scale2(&'a self) -> f32 {
        self.row.columns[22].into_f32().copied().unwrap()
    }
    pub fn Unknown3(&'a self) -> f32 {
        self.row.columns[33].into_f32().copied().unwrap()
    }
    pub fn Scale4(&'a self) -> f32 {
        self.row.columns[44].into_f32().copied().unwrap()
    }
    pub fn Scale5(&'a self) -> f32 {
        self.row.columns[55].into_f32().copied().unwrap()
    }
    pub fn Unknown4(&'a self) -> f32 {
        self.row.columns[66].into_f32().copied().unwrap()
    }
    pub fn BNpcBase1(&'a self) -> u16 {
        self.row.columns[1].into_u16().copied().unwrap()
    }
    pub fn BNpcBase2(&'a self) -> u16 {
        self.row.columns[12].into_u16().copied().unwrap()
    }
    pub fn BNpcBase3(&'a self) -> u16 {
        self.row.columns[23].into_u16().copied().unwrap()
    }
    pub fn BNpcBase4(&'a self) -> u16 {
        self.row.columns[34].into_u16().copied().unwrap()
    }
    pub fn BNpcBase5(&'a self) -> u16 {
        self.row.columns[45].into_u16().copied().unwrap()
    }
    pub fn Unknown5(&'a self) -> u16 {
        self.row.columns[56].into_u16().copied().unwrap()
    }
    pub fn Unknown6(&'a self) -> i16 {
        self.row.columns[10].into_i16().copied().unwrap()
    }
    pub fn Unknown7(&'a self) -> i16 {
        self.row.columns[21].into_i16().copied().unwrap()
    }
    pub fn Scale3(&'a self) -> i16 {
        self.row.columns[32].into_i16().copied().unwrap()
    }
    pub fn Unknown8(&'a self) -> i16 {
        self.row.columns[43].into_i16().copied().unwrap()
    }
    pub fn Unknown9(&'a self) -> i16 {
        self.row.columns[54].into_i16().copied().unwrap()
    }
    pub fn Unknown10(&'a self) -> i16 {
        self.row.columns[65].into_i16().copied().unwrap()
    }
    pub fn PartSlot1(&'a self) -> u8 {
        self.row.columns[2].into_u8().copied().unwrap()
    }
    pub fn PartSlot2(&'a self) -> u8 {
        self.row.columns[13].into_u8().copied().unwrap()
    }
    pub fn PartSlot3(&'a self) -> u8 {
        self.row.columns[24].into_u8().copied().unwrap()
    }
    pub fn PartSlot4(&'a self) -> u8 {
        self.row.columns[35].into_u8().copied().unwrap()
    }
    pub fn PartSlot5(&'a self) -> u8 {
        self.row.columns[46].into_u8().copied().unwrap()
    }
    pub fn Unknown11(&'a self) -> u8 {
        self.row.columns[57].into_u8().copied().unwrap()
    }
    pub fn Unknown12(&'a self) -> bool {
        self.row.columns[3].into_bool().copied().unwrap()
    }
    pub fn Unknown13(&'a self) -> bool {
        self.row.columns[14].into_bool().copied().unwrap()
    }
    pub fn Unknown14(&'a self) -> bool {
        self.row.columns[25].into_bool().copied().unwrap()
    }
    pub fn Unknown15(&'a self) -> bool {
        self.row.columns[36].into_bool().copied().unwrap()
    }
    pub fn Unknown16(&'a self) -> bool {
        self.row.columns[47].into_bool().copied().unwrap()
    }
    pub fn Unknown17(&'a self) -> bool {
        self.row.columns[58].into_bool().copied().unwrap()
    }
    pub fn Unknown18(&'a self) -> bool {
        self.row.columns[4].into_bool().copied().unwrap()
    }
    pub fn Unknown19(&'a self) -> bool {
        self.row.columns[15].into_bool().copied().unwrap()
    }
    pub fn Unknown20(&'a self) -> bool {
        self.row.columns[26].into_bool().copied().unwrap()
    }
    pub fn Unknown21(&'a self) -> bool {
        self.row.columns[37].into_bool().copied().unwrap()
    }
    pub fn Unknown22(&'a self) -> bool {
        self.row.columns[48].into_bool().copied().unwrap()
    }
    pub fn Unknown23(&'a self) -> bool {
        self.row.columns[59].into_bool().copied().unwrap()
    }
    pub fn Unknown24(&'a self) -> bool {
        self.row.columns[5].into_bool().copied().unwrap()
    }
    pub fn Unknown25(&'a self) -> bool {
        self.row.columns[16].into_bool().copied().unwrap()
    }
    pub fn Unknown26(&'a self) -> bool {
        self.row.columns[27].into_bool().copied().unwrap()
    }
    pub fn Unknown27(&'a self) -> bool {
        self.row.columns[38].into_bool().copied().unwrap()
    }
    pub fn Unknown28(&'a self) -> bool {
        self.row.columns[49].into_bool().copied().unwrap()
    }
    pub fn Unknown29(&'a self) -> bool {
        self.row.columns[60].into_bool().copied().unwrap()
    }
    pub fn Unknown30(&'a self) -> bool {
        self.row.columns[6].into_bool().copied().unwrap()
    }
    pub fn Unknown31(&'a self) -> bool {
        self.row.columns[17].into_bool().copied().unwrap()
    }
    pub fn Unknown32(&'a self) -> bool {
        self.row.columns[28].into_bool().copied().unwrap()
    }
    pub fn Unknown33(&'a self) -> bool {
        self.row.columns[39].into_bool().copied().unwrap()
    }
    pub fn Unknown34(&'a self) -> bool {
        self.row.columns[50].into_bool().copied().unwrap()
    }
    pub fn Unknown35(&'a self) -> bool {
        self.row.columns[61].into_bool().copied().unwrap()
    }
    pub fn Unknown36(&'a self) -> bool {
        self.row.columns[0].into_bool().copied().unwrap()
    }
}
