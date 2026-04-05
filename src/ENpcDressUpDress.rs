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
pub struct ENpcDressUpDressSheet {
    sheet: Sheet,
}
impl ENpcDressUpDressSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("ENpcDressUpDress")?;
        let sheet = resolver.read_excel_sheet(&exh, "ENpcDressUpDress", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<ENpcDressUpDressRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<ENpcDressUpDressRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for ENpcDressUpDressSheet {
    type Row = ENpcDressUpDressRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a ENpcDressUpDressSheet {
    type Item = (u32, Vec<(u16, ENpcDressUpDressRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, ENpcDressUpDressSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, ENpcDressUpDressSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ENpcDressUpDressRow<'a> {
    row: &'a Row,
}
impl<'a> ENpcDressUpDressRow<'a> {
    pub fn ModelMainHand(&'a self) -> u64 {
        self.row.columns[37].into_u64().copied().unwrap()
    }
    pub fn ModelOffHand(&'a self) -> u64 {
        self.row.columns[40].into_u64().copied().unwrap()
    }
    pub fn Unknown0(&'a self) -> u32 {
        self.row.columns[0].into_u32().copied().unwrap()
    }
    pub fn ENpc(&'a self) -> u32 {
        self.row.columns[7].into_u32().copied().unwrap()
    }
    pub fn ModelHead(&'a self) -> u32 {
        self.row.columns[43].into_u32().copied().unwrap()
    }
    pub fn ModelBody(&'a self) -> u32 {
        self.row.columns[46].into_u32().copied().unwrap()
    }
    pub fn ModelHands(&'a self) -> u32 {
        self.row.columns[49].into_u32().copied().unwrap()
    }
    pub fn ModelLegs(&'a self) -> u32 {
        self.row.columns[52].into_u32().copied().unwrap()
    }
    pub fn ModelFeet(&'a self) -> u32 {
        self.row.columns[55].into_u32().copied().unwrap()
    }
    pub fn Unknown1(&'a self) -> u32 {
        self.row.columns[58].into_u32().copied().unwrap()
    }
    pub fn Unknown2(&'a self) -> u32 {
        self.row.columns[61].into_u32().copied().unwrap()
    }
    pub fn Unknown3(&'a self) -> u32 {
        self.row.columns[64].into_u32().copied().unwrap()
    }
    pub fn Unknown4(&'a self) -> u32 {
        self.row.columns[67].into_u32().copied().unwrap()
    }
    pub fn Unknown5(&'a self) -> u32 {
        self.row.columns[70].into_u32().copied().unwrap()
    }
    pub fn Unknown6(&'a self) -> u16 {
        self.row.columns[8].into_u16().copied().unwrap()
    }
    pub fn Behavior(&'a self) -> u16 {
        self.row.columns[9].into_u16().copied().unwrap()
    }
    pub fn Unknown7(&'a self) -> u16 {
        self.row.columns[10].into_u16().copied().unwrap()
    }
    pub fn Unknown8(&'a self) -> u8 {
        self.row.columns[6].into_u8().copied().unwrap()
    }
    pub fn Unknown9(&'a self) -> u8 {
        self.row.columns[11].into_u8().copied().unwrap()
    }
    pub fn Unknown10(&'a self) -> u8 {
        self.row.columns[12].into_u8().copied().unwrap()
    }
    pub fn Unknown11(&'a self) -> u8 {
        self.row.columns[13].into_u8().copied().unwrap()
    }
    pub fn Unknown12(&'a self) -> u8 {
        self.row.columns[14].into_u8().copied().unwrap()
    }
    pub fn Unknown13(&'a self) -> u8 {
        self.row.columns[15].into_u8().copied().unwrap()
    }
    pub fn Unknown14(&'a self) -> u8 {
        self.row.columns[16].into_u8().copied().unwrap()
    }
    pub fn Unknown15(&'a self) -> u8 {
        self.row.columns[17].into_u8().copied().unwrap()
    }
    pub fn Unknown16(&'a self) -> u8 {
        self.row.columns[18].into_u8().copied().unwrap()
    }
    pub fn Unknown17(&'a self) -> u8 {
        self.row.columns[19].into_u8().copied().unwrap()
    }
    pub fn Unknown18(&'a self) -> u8 {
        self.row.columns[20].into_u8().copied().unwrap()
    }
    pub fn Unknown19(&'a self) -> u8 {
        self.row.columns[21].into_u8().copied().unwrap()
    }
    pub fn Unknown20(&'a self) -> u8 {
        self.row.columns[22].into_u8().copied().unwrap()
    }
    pub fn Unknown21(&'a self) -> u8 {
        self.row.columns[23].into_u8().copied().unwrap()
    }
    pub fn Unknown22(&'a self) -> u8 {
        self.row.columns[24].into_u8().copied().unwrap()
    }
    pub fn Unknown23(&'a self) -> u8 {
        self.row.columns[25].into_u8().copied().unwrap()
    }
    pub fn Unknown24(&'a self) -> u8 {
        self.row.columns[26].into_u8().copied().unwrap()
    }
    pub fn Unknown25(&'a self) -> u8 {
        self.row.columns[27].into_u8().copied().unwrap()
    }
    pub fn Unknown26(&'a self) -> u8 {
        self.row.columns[28].into_u8().copied().unwrap()
    }
    pub fn Unknown27(&'a self) -> u8 {
        self.row.columns[29].into_u8().copied().unwrap()
    }
    pub fn Unknown28(&'a self) -> u8 {
        self.row.columns[30].into_u8().copied().unwrap()
    }
    pub fn Unknown29(&'a self) -> u8 {
        self.row.columns[31].into_u8().copied().unwrap()
    }
    pub fn Unknown30(&'a self) -> u8 {
        self.row.columns[32].into_u8().copied().unwrap()
    }
    pub fn Unknown31(&'a self) -> u8 {
        self.row.columns[33].into_u8().copied().unwrap()
    }
    pub fn Unknown32(&'a self) -> u8 {
        self.row.columns[34].into_u8().copied().unwrap()
    }
    pub fn Unknown33(&'a self) -> u8 {
        self.row.columns[35].into_u8().copied().unwrap()
    }
    pub fn Unknown34(&'a self) -> u8 {
        self.row.columns[36].into_u8().copied().unwrap()
    }
    pub fn DyeMainHand(&'a self) -> u8 {
        self.row.columns[38].into_u8().copied().unwrap()
    }
    pub fn Dye2MainHand(&'a self) -> u8 {
        self.row.columns[39].into_u8().copied().unwrap()
    }
    pub fn DyeOffHand(&'a self) -> u8 {
        self.row.columns[41].into_u8().copied().unwrap()
    }
    pub fn Dye2OffHand(&'a self) -> u8 {
        self.row.columns[42].into_u8().copied().unwrap()
    }
    pub fn DyeHead(&'a self) -> u8 {
        self.row.columns[44].into_u8().copied().unwrap()
    }
    pub fn DyeBody(&'a self) -> u8 {
        self.row.columns[47].into_u8().copied().unwrap()
    }
    pub fn DyeHands(&'a self) -> u8 {
        self.row.columns[50].into_u8().copied().unwrap()
    }
    pub fn DyeLegs(&'a self) -> u8 {
        self.row.columns[53].into_u8().copied().unwrap()
    }
    pub fn DyeFeet(&'a self) -> u8 {
        self.row.columns[56].into_u8().copied().unwrap()
    }
    pub fn DyeEars(&'a self) -> u8 {
        self.row.columns[59].into_u8().copied().unwrap()
    }
    pub fn DyeNeck(&'a self) -> u8 {
        self.row.columns[62].into_u8().copied().unwrap()
    }
    pub fn DyeWrists(&'a self) -> u8 {
        self.row.columns[65].into_u8().copied().unwrap()
    }
    pub fn DyeLeftRing(&'a self) -> u8 {
        self.row.columns[68].into_u8().copied().unwrap()
    }
    pub fn DyeRightRing(&'a self) -> u8 {
        self.row.columns[71].into_u8().copied().unwrap()
    }
    pub fn Dye2Head(&'a self) -> u8 {
        self.row.columns[45].into_u8().copied().unwrap()
    }
    pub fn Dye2Body(&'a self) -> u8 {
        self.row.columns[48].into_u8().copied().unwrap()
    }
    pub fn Dye2Hands(&'a self) -> u8 {
        self.row.columns[51].into_u8().copied().unwrap()
    }
    pub fn Dye2Legs(&'a self) -> u8 {
        self.row.columns[54].into_u8().copied().unwrap()
    }
    pub fn Dye2Feet(&'a self) -> u8 {
        self.row.columns[57].into_u8().copied().unwrap()
    }
    pub fn Dye2Ears(&'a self) -> u8 {
        self.row.columns[60].into_u8().copied().unwrap()
    }
    pub fn Dye2Neck(&'a self) -> u8 {
        self.row.columns[63].into_u8().copied().unwrap()
    }
    pub fn Dye2Wrists(&'a self) -> u8 {
        self.row.columns[66].into_u8().copied().unwrap()
    }
    pub fn Dye2LeftRing(&'a self) -> u8 {
        self.row.columns[69].into_u8().copied().unwrap()
    }
    pub fn Dye2RightRing(&'a self) -> u8 {
        self.row.columns[72].into_u8().copied().unwrap()
    }
    pub fn Unknown40(&'a self) -> bool {
        self.row.columns[1].into_bool().copied().unwrap()
    }
    pub fn Unknown41(&'a self) -> bool {
        self.row.columns[2].into_bool().copied().unwrap()
    }
    pub fn Unknown42(&'a self) -> bool {
        self.row.columns[3].into_bool().copied().unwrap()
    }
    pub fn Unknown43(&'a self) -> bool {
        self.row.columns[4].into_bool().copied().unwrap()
    }
    pub fn Unknown44(&'a self) -> bool {
        self.row.columns[5].into_bool().copied().unwrap()
    }
}
