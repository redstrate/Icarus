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
pub struct ENpcBaseSheet {
    sheet: Sheet,
}
impl ENpcBaseSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("ENpcBase")?;
        let sheet = resolver.read_excel_sheet(&exh, "ENpcBase", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<ENpcBaseRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<ENpcBaseRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for ENpcBaseSheet {
    type Row = ENpcBaseRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a ENpcBaseSheet {
    type Item = (u32, Vec<(u16, ENpcBaseRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, ENpcBaseSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, ENpcBaseSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ENpcBaseRow<'a> {
    row: &'a Row,
}
impl<'a> ENpcBaseRow<'a> {
    pub fn ENpcData(&'a self) -> [u32; 32] {
        [
            self.row.columns[2].into_u32().copied().unwrap(),
            self.row.columns[3].into_u32().copied().unwrap(),
            self.row.columns[4].into_u32().copied().unwrap(),
            self.row.columns[5].into_u32().copied().unwrap(),
            self.row.columns[6].into_u32().copied().unwrap(),
            self.row.columns[7].into_u32().copied().unwrap(),
            self.row.columns[8].into_u32().copied().unwrap(),
            self.row.columns[9].into_u32().copied().unwrap(),
            self.row.columns[10].into_u32().copied().unwrap(),
            self.row.columns[11].into_u32().copied().unwrap(),
            self.row.columns[12].into_u32().copied().unwrap(),
            self.row.columns[13].into_u32().copied().unwrap(),
            self.row.columns[14].into_u32().copied().unwrap(),
            self.row.columns[15].into_u32().copied().unwrap(),
            self.row.columns[16].into_u32().copied().unwrap(),
            self.row.columns[17].into_u32().copied().unwrap(),
            self.row.columns[18].into_u32().copied().unwrap(),
            self.row.columns[19].into_u32().copied().unwrap(),
            self.row.columns[20].into_u32().copied().unwrap(),
            self.row.columns[21].into_u32().copied().unwrap(),
            self.row.columns[22].into_u32().copied().unwrap(),
            self.row.columns[23].into_u32().copied().unwrap(),
            self.row.columns[24].into_u32().copied().unwrap(),
            self.row.columns[25].into_u32().copied().unwrap(),
            self.row.columns[26].into_u32().copied().unwrap(),
            self.row.columns[27].into_u32().copied().unwrap(),
            self.row.columns[28].into_u32().copied().unwrap(),
            self.row.columns[29].into_u32().copied().unwrap(),
            self.row.columns[30].into_u32().copied().unwrap(),
            self.row.columns[31].into_u32().copied().unwrap(),
            self.row.columns[32].into_u32().copied().unwrap(),
            self.row.columns[33].into_u32().copied().unwrap(),
        ]
    }
    pub fn ModelMainHand(&'a self) -> u64 {
        self.row.columns[65].into_u64().copied().unwrap()
    }
    pub fn ModelOffHand(&'a self) -> u64 {
        self.row.columns[68].into_u64().copied().unwrap()
    }
    pub fn Scale(&'a self) -> f32 {
        self.row.columns[34].into_f32().copied().unwrap()
    }
    pub fn ModelHead(&'a self) -> u32 {
        self.row.columns[71].into_u32().copied().unwrap()
    }
    pub fn ModelBody(&'a self) -> u32 {
        self.row.columns[76].into_u32().copied().unwrap()
    }
    pub fn ModelHands(&'a self) -> u32 {
        self.row.columns[79].into_u32().copied().unwrap()
    }
    pub fn ModelLegs(&'a self) -> u32 {
        self.row.columns[82].into_u32().copied().unwrap()
    }
    pub fn ModelFeet(&'a self) -> u32 {
        self.row.columns[85].into_u32().copied().unwrap()
    }
    pub fn ModelEars(&'a self) -> u32 {
        self.row.columns[88].into_u32().copied().unwrap()
    }
    pub fn ModelNeck(&'a self) -> u32 {
        self.row.columns[91].into_u32().copied().unwrap()
    }
    pub fn ModelWrists(&'a self) -> u32 {
        self.row.columns[94].into_u32().copied().unwrap()
    }
    pub fn ModelLeftRing(&'a self) -> u32 {
        self.row.columns[97].into_u32().copied().unwrap()
    }
    pub fn ModelRightRing(&'a self) -> u32 {
        self.row.columns[100].into_u32().copied().unwrap()
    }
    pub fn EventHandler(&'a self) -> u16 {
        self.row.columns[0].into_u16().copied().unwrap()
    }
    pub fn ModelChara(&'a self) -> u16 {
        self.row.columns[35].into_u16().copied().unwrap()
    }
    pub fn NpcEquip(&'a self) -> u16 {
        self.row.columns[63].into_u16().copied().unwrap()
    }
    pub fn Behavior(&'a self) -> u16 {
        self.row.columns[64].into_u16().copied().unwrap()
    }
    pub fn Unknown_70_1(&'a self) -> u16 {
        self.row.columns[103].into_u16().copied().unwrap()
    }
    pub fn Unknown_70_2(&'a self) -> u16 {
        self.row.columns[104].into_u16().copied().unwrap()
    }
    pub fn Balloon(&'a self) -> u16 {
        self.row.columns[106].into_u16().copied().unwrap()
    }
    pub fn Race(&'a self) -> u8 {
        self.row.columns[36].into_u8().copied().unwrap()
    }
    pub fn Gender(&'a self) -> u8 {
        self.row.columns[37].into_u8().copied().unwrap()
    }
    pub fn BodyType(&'a self) -> u8 {
        self.row.columns[38].into_u8().copied().unwrap()
    }
    pub fn Height(&'a self) -> u8 {
        self.row.columns[39].into_u8().copied().unwrap()
    }
    pub fn Tribe(&'a self) -> u8 {
        self.row.columns[40].into_u8().copied().unwrap()
    }
    pub fn Face(&'a self) -> u8 {
        self.row.columns[41].into_u8().copied().unwrap()
    }
    pub fn HairStyle(&'a self) -> u8 {
        self.row.columns[42].into_u8().copied().unwrap()
    }
    pub fn HairHighlight(&'a self) -> u8 {
        self.row.columns[43].into_u8().copied().unwrap()
    }
    pub fn SkinColor(&'a self) -> u8 {
        self.row.columns[44].into_u8().copied().unwrap()
    }
    pub fn EyeHeterochromia(&'a self) -> u8 {
        self.row.columns[45].into_u8().copied().unwrap()
    }
    pub fn HairColor(&'a self) -> u8 {
        self.row.columns[46].into_u8().copied().unwrap()
    }
    pub fn HairHighlightColor(&'a self) -> u8 {
        self.row.columns[47].into_u8().copied().unwrap()
    }
    pub fn FacialFeature(&'a self) -> u8 {
        self.row.columns[48].into_u8().copied().unwrap()
    }
    pub fn FacialFeatureColor(&'a self) -> u8 {
        self.row.columns[49].into_u8().copied().unwrap()
    }
    pub fn Eyebrows(&'a self) -> u8 {
        self.row.columns[50].into_u8().copied().unwrap()
    }
    pub fn EyeColor(&'a self) -> u8 {
        self.row.columns[51].into_u8().copied().unwrap()
    }
    pub fn EyeShape(&'a self) -> u8 {
        self.row.columns[52].into_u8().copied().unwrap()
    }
    pub fn Nose(&'a self) -> u8 {
        self.row.columns[53].into_u8().copied().unwrap()
    }
    pub fn Jaw(&'a self) -> u8 {
        self.row.columns[54].into_u8().copied().unwrap()
    }
    pub fn Mouth(&'a self) -> u8 {
        self.row.columns[55].into_u8().copied().unwrap()
    }
    pub fn LipColor(&'a self) -> u8 {
        self.row.columns[56].into_u8().copied().unwrap()
    }
    pub fn BustOrTone1(&'a self) -> u8 {
        self.row.columns[57].into_u8().copied().unwrap()
    }
    pub fn ExtraFeature1(&'a self) -> u8 {
        self.row.columns[58].into_u8().copied().unwrap()
    }
    pub fn ExtraFeature2OrBust(&'a self) -> u8 {
        self.row.columns[59].into_u8().copied().unwrap()
    }
    pub fn FacePaint(&'a self) -> u8 {
        self.row.columns[60].into_u8().copied().unwrap()
    }
    pub fn FacePaintColor(&'a self) -> u8 {
        self.row.columns[61].into_u8().copied().unwrap()
    }
    pub fn Unknown0(&'a self) -> u8 {
        self.row.columns[62].into_u8().copied().unwrap()
    }
    pub fn DyeMainHand(&'a self) -> u8 {
        self.row.columns[66].into_u8().copied().unwrap()
    }
    pub fn Dye2MainHand(&'a self) -> u8 {
        self.row.columns[67].into_u8().copied().unwrap()
    }
    pub fn DyeOffHand(&'a self) -> u8 {
        self.row.columns[69].into_u8().copied().unwrap()
    }
    pub fn Dye2OffHand(&'a self) -> u8 {
        self.row.columns[70].into_u8().copied().unwrap()
    }
    pub fn DyeHead(&'a self) -> u8 {
        self.row.columns[72].into_u8().copied().unwrap()
    }
    pub fn DyeBody(&'a self) -> u8 {
        self.row.columns[77].into_u8().copied().unwrap()
    }
    pub fn DyeHands(&'a self) -> u8 {
        self.row.columns[80].into_u8().copied().unwrap()
    }
    pub fn DyeLegs(&'a self) -> u8 {
        self.row.columns[83].into_u8().copied().unwrap()
    }
    pub fn DyeFeet(&'a self) -> u8 {
        self.row.columns[86].into_u8().copied().unwrap()
    }
    pub fn DyeEars(&'a self) -> u8 {
        self.row.columns[89].into_u8().copied().unwrap()
    }
    pub fn DyeNeck(&'a self) -> u8 {
        self.row.columns[92].into_u8().copied().unwrap()
    }
    pub fn DyeWrists(&'a self) -> u8 {
        self.row.columns[95].into_u8().copied().unwrap()
    }
    pub fn DyeLeftRing(&'a self) -> u8 {
        self.row.columns[98].into_u8().copied().unwrap()
    }
    pub fn DyeRightRing(&'a self) -> u8 {
        self.row.columns[101].into_u8().copied().unwrap()
    }
    pub fn Dye2Head(&'a self) -> u8 {
        self.row.columns[73].into_u8().copied().unwrap()
    }
    pub fn Dye2Body(&'a self) -> u8 {
        self.row.columns[78].into_u8().copied().unwrap()
    }
    pub fn Dye2Hands(&'a self) -> u8 {
        self.row.columns[81].into_u8().copied().unwrap()
    }
    pub fn Dye2Legs(&'a self) -> u8 {
        self.row.columns[84].into_u8().copied().unwrap()
    }
    pub fn Dye2Feet(&'a self) -> u8 {
        self.row.columns[87].into_u8().copied().unwrap()
    }
    pub fn Dye2Ears(&'a self) -> u8 {
        self.row.columns[90].into_u8().copied().unwrap()
    }
    pub fn Dye2Neck(&'a self) -> u8 {
        self.row.columns[93].into_u8().copied().unwrap()
    }
    pub fn Dye2Wrists(&'a self) -> u8 {
        self.row.columns[96].into_u8().copied().unwrap()
    }
    pub fn Dye2LeftRing(&'a self) -> u8 {
        self.row.columns[99].into_u8().copied().unwrap()
    }
    pub fn Dye2RightRing(&'a self) -> u8 {
        self.row.columns[102].into_u8().copied().unwrap()
    }
    pub fn Invisibility(&'a self) -> u8 {
        self.row.columns[105].into_u8().copied().unwrap()
    }
    pub fn DefaultBalloon(&'a self) -> u8 {
        self.row.columns[108].into_u8().copied().unwrap()
    }
    pub fn Unknown1(&'a self) -> u8 {
        self.row.columns[109].into_u8().copied().unwrap()
    }
    pub fn Important(&'a self) -> bool {
        self.row.columns[1].into_bool().copied().unwrap()
    }
    pub fn Visor(&'a self) -> bool {
        self.row.columns[74].into_bool().copied().unwrap()
    }
    pub fn NotRewriteHeight(&'a self) -> bool {
        self.row.columns[75].into_bool().copied().unwrap()
    }
    pub fn Unknown2(&'a self) -> bool {
        self.row.columns[107].into_bool().copied().unwrap()
    }
}
