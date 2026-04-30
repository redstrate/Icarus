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
impl StructuredSheet for ENpcBaseSheet {
    type Row = ENpcBaseRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            ENpcData: [
                row
                    .columns[2]
                    .into_u32()
                    .copied()
                    .expect("Expected column 2 to be a uint32!"),
                row
                    .columns[3]
                    .into_u32()
                    .copied()
                    .expect("Expected column 3 to be a uint32!"),
                row
                    .columns[4]
                    .into_u32()
                    .copied()
                    .expect("Expected column 4 to be a uint32!"),
                row
                    .columns[5]
                    .into_u32()
                    .copied()
                    .expect("Expected column 5 to be a uint32!"),
                row
                    .columns[6]
                    .into_u32()
                    .copied()
                    .expect("Expected column 6 to be a uint32!"),
                row
                    .columns[7]
                    .into_u32()
                    .copied()
                    .expect("Expected column 7 to be a uint32!"),
                row
                    .columns[8]
                    .into_u32()
                    .copied()
                    .expect("Expected column 8 to be a uint32!"),
                row
                    .columns[9]
                    .into_u32()
                    .copied()
                    .expect("Expected column 9 to be a uint32!"),
                row
                    .columns[10]
                    .into_u32()
                    .copied()
                    .expect("Expected column 10 to be a uint32!"),
                row
                    .columns[11]
                    .into_u32()
                    .copied()
                    .expect("Expected column 11 to be a uint32!"),
                row
                    .columns[12]
                    .into_u32()
                    .copied()
                    .expect("Expected column 12 to be a uint32!"),
                row
                    .columns[13]
                    .into_u32()
                    .copied()
                    .expect("Expected column 13 to be a uint32!"),
                row
                    .columns[14]
                    .into_u32()
                    .copied()
                    .expect("Expected column 14 to be a uint32!"),
                row
                    .columns[15]
                    .into_u32()
                    .copied()
                    .expect("Expected column 15 to be a uint32!"),
                row
                    .columns[16]
                    .into_u32()
                    .copied()
                    .expect("Expected column 16 to be a uint32!"),
                row
                    .columns[17]
                    .into_u32()
                    .copied()
                    .expect("Expected column 17 to be a uint32!"),
                row
                    .columns[18]
                    .into_u32()
                    .copied()
                    .expect("Expected column 18 to be a uint32!"),
                row
                    .columns[19]
                    .into_u32()
                    .copied()
                    .expect("Expected column 19 to be a uint32!"),
                row
                    .columns[20]
                    .into_u32()
                    .copied()
                    .expect("Expected column 20 to be a uint32!"),
                row
                    .columns[21]
                    .into_u32()
                    .copied()
                    .expect("Expected column 21 to be a uint32!"),
                row
                    .columns[22]
                    .into_u32()
                    .copied()
                    .expect("Expected column 22 to be a uint32!"),
                row
                    .columns[23]
                    .into_u32()
                    .copied()
                    .expect("Expected column 23 to be a uint32!"),
                row
                    .columns[24]
                    .into_u32()
                    .copied()
                    .expect("Expected column 24 to be a uint32!"),
                row
                    .columns[25]
                    .into_u32()
                    .copied()
                    .expect("Expected column 25 to be a uint32!"),
                row
                    .columns[26]
                    .into_u32()
                    .copied()
                    .expect("Expected column 26 to be a uint32!"),
                row
                    .columns[27]
                    .into_u32()
                    .copied()
                    .expect("Expected column 27 to be a uint32!"),
                row
                    .columns[28]
                    .into_u32()
                    .copied()
                    .expect("Expected column 28 to be a uint32!"),
                row
                    .columns[29]
                    .into_u32()
                    .copied()
                    .expect("Expected column 29 to be a uint32!"),
                row
                    .columns[30]
                    .into_u32()
                    .copied()
                    .expect("Expected column 30 to be a uint32!"),
                row
                    .columns[31]
                    .into_u32()
                    .copied()
                    .expect("Expected column 31 to be a uint32!"),
                row
                    .columns[32]
                    .into_u32()
                    .copied()
                    .expect("Expected column 32 to be a uint32!"),
                row
                    .columns[33]
                    .into_u32()
                    .copied()
                    .expect("Expected column 33 to be a uint32!"),
            ],
            ModelMainHand: row
                .columns[65]
                .into_u64()
                .copied()
                .expect("Expected column 65 to be a uint64!"),
            ModelOffHand: row
                .columns[68]
                .into_u64()
                .copied()
                .expect("Expected column 68 to be a uint64!"),
            Scale: row
                .columns[34]
                .into_f32()
                .copied()
                .expect("Expected column 34 to be a float32!"),
            ModelHead: row
                .columns[71]
                .into_u32()
                .copied()
                .expect("Expected column 71 to be a uint32!"),
            ModelBody: row
                .columns[76]
                .into_u32()
                .copied()
                .expect("Expected column 76 to be a uint32!"),
            ModelHands: row
                .columns[79]
                .into_u32()
                .copied()
                .expect("Expected column 79 to be a uint32!"),
            ModelLegs: row
                .columns[82]
                .into_u32()
                .copied()
                .expect("Expected column 82 to be a uint32!"),
            ModelFeet: row
                .columns[85]
                .into_u32()
                .copied()
                .expect("Expected column 85 to be a uint32!"),
            ModelEars: row
                .columns[88]
                .into_u32()
                .copied()
                .expect("Expected column 88 to be a uint32!"),
            ModelNeck: row
                .columns[91]
                .into_u32()
                .copied()
                .expect("Expected column 91 to be a uint32!"),
            ModelWrists: row
                .columns[94]
                .into_u32()
                .copied()
                .expect("Expected column 94 to be a uint32!"),
            ModelLeftRing: row
                .columns[97]
                .into_u32()
                .copied()
                .expect("Expected column 97 to be a uint32!"),
            ModelRightRing: row
                .columns[100]
                .into_u32()
                .copied()
                .expect("Expected column 100 to be a uint32!"),
            EventHandler: row
                .columns[0]
                .into_u16()
                .copied()
                .expect("Expected column 0 to be a uint16!"),
            ModelChara: row
                .columns[35]
                .into_u16()
                .copied()
                .expect("Expected column 35 to be a uint16!"),
            NpcEquip: row
                .columns[63]
                .into_u16()
                .copied()
                .expect("Expected column 63 to be a uint16!"),
            Behavior: row
                .columns[64]
                .into_u16()
                .copied()
                .expect("Expected column 64 to be a uint16!"),
            Unknown_70_1: row
                .columns[103]
                .into_u16()
                .copied()
                .expect("Expected column 103 to be a uint16!"),
            Unknown_70_2: row
                .columns[104]
                .into_u16()
                .copied()
                .expect("Expected column 104 to be a uint16!"),
            Balloon: row
                .columns[106]
                .into_u16()
                .copied()
                .expect("Expected column 106 to be a uint16!"),
            Race: row
                .columns[36]
                .into_u8()
                .copied()
                .expect("Expected column 36 to be a uint8!"),
            Gender: row
                .columns[37]
                .into_u8()
                .copied()
                .expect("Expected column 37 to be a uint8!"),
            BodyType: row
                .columns[38]
                .into_u8()
                .copied()
                .expect("Expected column 38 to be a uint8!"),
            Height: row
                .columns[39]
                .into_u8()
                .copied()
                .expect("Expected column 39 to be a uint8!"),
            Tribe: row
                .columns[40]
                .into_u8()
                .copied()
                .expect("Expected column 40 to be a uint8!"),
            Face: row
                .columns[41]
                .into_u8()
                .copied()
                .expect("Expected column 41 to be a uint8!"),
            HairStyle: row
                .columns[42]
                .into_u8()
                .copied()
                .expect("Expected column 42 to be a uint8!"),
            HairHighlight: row
                .columns[43]
                .into_u8()
                .copied()
                .expect("Expected column 43 to be a uint8!"),
            SkinColor: row
                .columns[44]
                .into_u8()
                .copied()
                .expect("Expected column 44 to be a uint8!"),
            EyeHeterochromia: row
                .columns[45]
                .into_u8()
                .copied()
                .expect("Expected column 45 to be a uint8!"),
            HairColor: row
                .columns[46]
                .into_u8()
                .copied()
                .expect("Expected column 46 to be a uint8!"),
            HairHighlightColor: row
                .columns[47]
                .into_u8()
                .copied()
                .expect("Expected column 47 to be a uint8!"),
            FacialFeature: row
                .columns[48]
                .into_u8()
                .copied()
                .expect("Expected column 48 to be a uint8!"),
            FacialFeatureColor: row
                .columns[49]
                .into_u8()
                .copied()
                .expect("Expected column 49 to be a uint8!"),
            Eyebrows: row
                .columns[50]
                .into_u8()
                .copied()
                .expect("Expected column 50 to be a uint8!"),
            EyeColor: row
                .columns[51]
                .into_u8()
                .copied()
                .expect("Expected column 51 to be a uint8!"),
            EyeShape: row
                .columns[52]
                .into_u8()
                .copied()
                .expect("Expected column 52 to be a uint8!"),
            Nose: row
                .columns[53]
                .into_u8()
                .copied()
                .expect("Expected column 53 to be a uint8!"),
            Jaw: row
                .columns[54]
                .into_u8()
                .copied()
                .expect("Expected column 54 to be a uint8!"),
            Mouth: row
                .columns[55]
                .into_u8()
                .copied()
                .expect("Expected column 55 to be a uint8!"),
            LipColor: row
                .columns[56]
                .into_u8()
                .copied()
                .expect("Expected column 56 to be a uint8!"),
            BustOrTone1: row
                .columns[57]
                .into_u8()
                .copied()
                .expect("Expected column 57 to be a uint8!"),
            ExtraFeature1: row
                .columns[58]
                .into_u8()
                .copied()
                .expect("Expected column 58 to be a uint8!"),
            ExtraFeature2OrBust: row
                .columns[59]
                .into_u8()
                .copied()
                .expect("Expected column 59 to be a uint8!"),
            FacePaint: row
                .columns[60]
                .into_u8()
                .copied()
                .expect("Expected column 60 to be a uint8!"),
            FacePaintColor: row
                .columns[61]
                .into_u8()
                .copied()
                .expect("Expected column 61 to be a uint8!"),
            Unknown0: row
                .columns[62]
                .into_u8()
                .copied()
                .expect("Expected column 62 to be a uint8!"),
            DyeMainHand: row
                .columns[66]
                .into_u8()
                .copied()
                .expect("Expected column 66 to be a uint8!"),
            Dye2MainHand: row
                .columns[67]
                .into_u8()
                .copied()
                .expect("Expected column 67 to be a uint8!"),
            DyeOffHand: row
                .columns[69]
                .into_u8()
                .copied()
                .expect("Expected column 69 to be a uint8!"),
            Dye2OffHand: row
                .columns[70]
                .into_u8()
                .copied()
                .expect("Expected column 70 to be a uint8!"),
            DyeHead: row
                .columns[72]
                .into_u8()
                .copied()
                .expect("Expected column 72 to be a uint8!"),
            DyeBody: row
                .columns[77]
                .into_u8()
                .copied()
                .expect("Expected column 77 to be a uint8!"),
            DyeHands: row
                .columns[80]
                .into_u8()
                .copied()
                .expect("Expected column 80 to be a uint8!"),
            DyeLegs: row
                .columns[83]
                .into_u8()
                .copied()
                .expect("Expected column 83 to be a uint8!"),
            DyeFeet: row
                .columns[86]
                .into_u8()
                .copied()
                .expect("Expected column 86 to be a uint8!"),
            DyeEars: row
                .columns[89]
                .into_u8()
                .copied()
                .expect("Expected column 89 to be a uint8!"),
            DyeNeck: row
                .columns[92]
                .into_u8()
                .copied()
                .expect("Expected column 92 to be a uint8!"),
            DyeWrists: row
                .columns[95]
                .into_u8()
                .copied()
                .expect("Expected column 95 to be a uint8!"),
            DyeLeftRing: row
                .columns[98]
                .into_u8()
                .copied()
                .expect("Expected column 98 to be a uint8!"),
            DyeRightRing: row
                .columns[101]
                .into_u8()
                .copied()
                .expect("Expected column 101 to be a uint8!"),
            Dye2Head: row
                .columns[73]
                .into_u8()
                .copied()
                .expect("Expected column 73 to be a uint8!"),
            Dye2Body: row
                .columns[78]
                .into_u8()
                .copied()
                .expect("Expected column 78 to be a uint8!"),
            Dye2Hands: row
                .columns[81]
                .into_u8()
                .copied()
                .expect("Expected column 81 to be a uint8!"),
            Dye2Legs: row
                .columns[84]
                .into_u8()
                .copied()
                .expect("Expected column 84 to be a uint8!"),
            Dye2Feet: row
                .columns[87]
                .into_u8()
                .copied()
                .expect("Expected column 87 to be a uint8!"),
            Dye2Ears: row
                .columns[90]
                .into_u8()
                .copied()
                .expect("Expected column 90 to be a uint8!"),
            Dye2Neck: row
                .columns[93]
                .into_u8()
                .copied()
                .expect("Expected column 93 to be a uint8!"),
            Dye2Wrists: row
                .columns[96]
                .into_u8()
                .copied()
                .expect("Expected column 96 to be a uint8!"),
            Dye2LeftRing: row
                .columns[99]
                .into_u8()
                .copied()
                .expect("Expected column 99 to be a uint8!"),
            Dye2RightRing: row
                .columns[102]
                .into_u8()
                .copied()
                .expect("Expected column 102 to be a uint8!"),
            Invisibility: row
                .columns[105]
                .into_u8()
                .copied()
                .expect("Expected column 105 to be a uint8!"),
            DefaultBalloon: row
                .columns[108]
                .into_u8()
                .copied()
                .expect("Expected column 108 to be a uint8!"),
            Unknown1: row
                .columns[109]
                .into_u8()
                .copied()
                .expect("Expected column 109 to be a uint8!"),
            Important: row
                .columns[1]
                .into_bool()
                .copied()
                .expect("Expected column 1 to be a bool!"),
            Visor: row
                .columns[74]
                .into_bool()
                .copied()
                .expect("Expected column 74 to be a bool!"),
            NotRewriteHeight: row
                .columns[75]
                .into_bool()
                .copied()
                .expect("Expected column 75 to be a bool!"),
            Unknown2: row
                .columns[107]
                .into_bool()
                .copied()
                .expect("Expected column 107 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a ENpcBaseSheet {
    type Item = (u32, Vec<(u16, ENpcBaseRow)>);
    type IntoIter = StructuredSheetIterator<'a, ENpcBaseSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, ENpcBaseSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ENpcBaseRow {
    ///""
    pub ENpcData: [u32; 32],
    ///""
    pub ModelMainHand: u64,
    ///""
    pub ModelOffHand: u64,
    ///""
    pub Scale: f32,
    ///""
    pub ModelHead: u32,
    ///""
    pub ModelBody: u32,
    ///""
    pub ModelHands: u32,
    ///""
    pub ModelLegs: u32,
    ///""
    pub ModelFeet: u32,
    ///""
    pub ModelEars: u32,
    ///""
    pub ModelNeck: u32,
    ///""
    pub ModelWrists: u32,
    ///""
    pub ModelLeftRing: u32,
    ///""
    pub ModelRightRing: u32,
    ///""
    pub EventHandler: u16,
    ///""
    pub ModelChara: u16,
    ///""
    pub NpcEquip: u16,
    ///""
    pub Behavior: u16,
    ///""
    pub Unknown_70_1: u16,
    ///""
    pub Unknown_70_2: u16,
    ///""
    pub Balloon: u16,
    ///""
    pub Race: u8,
    ///""
    pub Gender: u8,
    ///""
    pub BodyType: u8,
    ///""
    pub Height: u8,
    ///""
    pub Tribe: u8,
    ///""
    pub Face: u8,
    ///""
    pub HairStyle: u8,
    ///""
    pub HairHighlight: u8,
    ///""
    pub SkinColor: u8,
    ///""
    pub EyeHeterochromia: u8,
    ///""
    pub HairColor: u8,
    ///""
    pub HairHighlightColor: u8,
    ///""
    pub FacialFeature: u8,
    ///""
    pub FacialFeatureColor: u8,
    ///""
    pub Eyebrows: u8,
    ///""
    pub EyeColor: u8,
    ///""
    pub EyeShape: u8,
    ///""
    pub Nose: u8,
    ///""
    pub Jaw: u8,
    ///""
    pub Mouth: u8,
    ///""
    pub LipColor: u8,
    ///""
    pub BustOrTone1: u8,
    ///""
    pub ExtraFeature1: u8,
    ///""
    pub ExtraFeature2OrBust: u8,
    ///""
    pub FacePaint: u8,
    ///""
    pub FacePaintColor: u8,
    ///""
    pub Unknown0: u8,
    ///""
    pub DyeMainHand: u8,
    ///""
    pub Dye2MainHand: u8,
    ///""
    pub DyeOffHand: u8,
    ///""
    pub Dye2OffHand: u8,
    ///""
    pub DyeHead: u8,
    ///""
    pub DyeBody: u8,
    ///""
    pub DyeHands: u8,
    ///""
    pub DyeLegs: u8,
    ///""
    pub DyeFeet: u8,
    ///""
    pub DyeEars: u8,
    ///""
    pub DyeNeck: u8,
    ///""
    pub DyeWrists: u8,
    ///""
    pub DyeLeftRing: u8,
    ///""
    pub DyeRightRing: u8,
    ///""
    pub Dye2Head: u8,
    ///""
    pub Dye2Body: u8,
    ///""
    pub Dye2Hands: u8,
    ///""
    pub Dye2Legs: u8,
    ///""
    pub Dye2Feet: u8,
    ///""
    pub Dye2Ears: u8,
    ///""
    pub Dye2Neck: u8,
    ///""
    pub Dye2Wrists: u8,
    ///""
    pub Dye2LeftRing: u8,
    ///""
    pub Dye2RightRing: u8,
    ///""
    pub Invisibility: u8,
    ///""
    pub DefaultBalloon: u8,
    ///""
    pub Unknown1: u8,
    ///""
    pub Important: bool,
    ///""
    pub Visor: bool,
    ///""
    pub NotRewriteHeight: bool,
    ///""
    pub Unknown2: bool,
}
