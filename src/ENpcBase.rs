//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    common::Language,
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
        let column_defs = &self.sheet.exh.column_definitions;
        let mut zipped: Vec<_> = row
            .columns
            .clone()
            .into_iter()
            .zip(column_defs)
            .collect();
        zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
        let (columns, _): (Vec<Field>, Vec<ExcelColumnDefinition>) = zipped
            .into_iter()
            .unzip();
        Some(Self::Row { columns })
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
#[derive(Debug, Clone)]
pub struct ENpcBaseRow {
    columns: Vec<Field>,
}
impl ENpcBaseRow {
    pub fn ENpcData<'a>(&'a self) -> [&'a Field; 32] {
        [
            &self.columns[0],
            &self.columns[1],
            &self.columns[2],
            &self.columns[3],
            &self.columns[4],
            &self.columns[5],
            &self.columns[6],
            &self.columns[7],
            &self.columns[8],
            &self.columns[9],
            &self.columns[10],
            &self.columns[11],
            &self.columns[12],
            &self.columns[13],
            &self.columns[14],
            &self.columns[15],
            &self.columns[16],
            &self.columns[17],
            &self.columns[18],
            &self.columns[19],
            &self.columns[20],
            &self.columns[21],
            &self.columns[22],
            &self.columns[23],
            &self.columns[24],
            &self.columns[25],
            &self.columns[26],
            &self.columns[27],
            &self.columns[28],
            &self.columns[29],
            &self.columns[30],
            &self.columns[31],
        ]
    }
    pub fn ModelMainHand<'a>(&'a self) -> &'a Field {
        &self.columns[32]
    }
    pub fn ModelOffHand<'a>(&'a self) -> &'a Field {
        &self.columns[33]
    }
    pub fn Scale<'a>(&'a self) -> &'a Field {
        &self.columns[34]
    }
    pub fn ModelHead<'a>(&'a self) -> &'a Field {
        &self.columns[35]
    }
    pub fn ModelBody<'a>(&'a self) -> &'a Field {
        &self.columns[36]
    }
    pub fn ModelHands<'a>(&'a self) -> &'a Field {
        &self.columns[37]
    }
    pub fn ModelLegs<'a>(&'a self) -> &'a Field {
        &self.columns[38]
    }
    pub fn ModelFeet<'a>(&'a self) -> &'a Field {
        &self.columns[39]
    }
    pub fn ModelEars<'a>(&'a self) -> &'a Field {
        &self.columns[40]
    }
    pub fn ModelNeck<'a>(&'a self) -> &'a Field {
        &self.columns[41]
    }
    pub fn ModelWrists<'a>(&'a self) -> &'a Field {
        &self.columns[42]
    }
    pub fn ModelLeftRing<'a>(&'a self) -> &'a Field {
        &self.columns[43]
    }
    pub fn ModelRightRing<'a>(&'a self) -> &'a Field {
        &self.columns[44]
    }
    pub fn EventHandler<'a>(&'a self) -> &'a Field {
        &self.columns[45]
    }
    pub fn ModelChara<'a>(&'a self) -> &'a Field {
        &self.columns[46]
    }
    pub fn NpcEquip<'a>(&'a self) -> &'a Field {
        &self.columns[47]
    }
    pub fn Behavior<'a>(&'a self) -> &'a Field {
        &self.columns[48]
    }
    pub fn Unknown_70_1<'a>(&'a self) -> &'a Field {
        &self.columns[49]
    }
    pub fn Unknown_70_2<'a>(&'a self) -> &'a Field {
        &self.columns[50]
    }
    pub fn Balloon<'a>(&'a self) -> &'a Field {
        &self.columns[51]
    }
    pub fn Race<'a>(&'a self) -> &'a Field {
        &self.columns[52]
    }
    pub fn Gender<'a>(&'a self) -> &'a Field {
        &self.columns[53]
    }
    pub fn BodyType<'a>(&'a self) -> &'a Field {
        &self.columns[54]
    }
    pub fn Height<'a>(&'a self) -> &'a Field {
        &self.columns[55]
    }
    pub fn Tribe<'a>(&'a self) -> &'a Field {
        &self.columns[56]
    }
    pub fn Face<'a>(&'a self) -> &'a Field {
        &self.columns[57]
    }
    pub fn HairStyle<'a>(&'a self) -> &'a Field {
        &self.columns[58]
    }
    pub fn HairHighlight<'a>(&'a self) -> &'a Field {
        &self.columns[59]
    }
    pub fn SkinColor<'a>(&'a self) -> &'a Field {
        &self.columns[60]
    }
    pub fn EyeHeterochromia<'a>(&'a self) -> &'a Field {
        &self.columns[61]
    }
    pub fn HairColor<'a>(&'a self) -> &'a Field {
        &self.columns[62]
    }
    pub fn HairHighlightColor<'a>(&'a self) -> &'a Field {
        &self.columns[63]
    }
    pub fn FacialFeature<'a>(&'a self) -> &'a Field {
        &self.columns[64]
    }
    pub fn FacialFeatureColor<'a>(&'a self) -> &'a Field {
        &self.columns[65]
    }
    pub fn Eyebrows<'a>(&'a self) -> &'a Field {
        &self.columns[66]
    }
    pub fn EyeColor<'a>(&'a self) -> &'a Field {
        &self.columns[67]
    }
    pub fn EyeShape<'a>(&'a self) -> &'a Field {
        &self.columns[68]
    }
    pub fn Nose<'a>(&'a self) -> &'a Field {
        &self.columns[69]
    }
    pub fn Jaw<'a>(&'a self) -> &'a Field {
        &self.columns[70]
    }
    pub fn Mouth<'a>(&'a self) -> &'a Field {
        &self.columns[71]
    }
    pub fn LipColor<'a>(&'a self) -> &'a Field {
        &self.columns[72]
    }
    pub fn BustOrTone1<'a>(&'a self) -> &'a Field {
        &self.columns[73]
    }
    pub fn ExtraFeature1<'a>(&'a self) -> &'a Field {
        &self.columns[74]
    }
    pub fn ExtraFeature2OrBust<'a>(&'a self) -> &'a Field {
        &self.columns[75]
    }
    pub fn FacePaint<'a>(&'a self) -> &'a Field {
        &self.columns[76]
    }
    pub fn FacePaintColor<'a>(&'a self) -> &'a Field {
        &self.columns[77]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a Field {
        &self.columns[78]
    }
    pub fn DyeMainHand<'a>(&'a self) -> &'a Field {
        &self.columns[79]
    }
    pub fn Dye2MainHand<'a>(&'a self) -> &'a Field {
        &self.columns[80]
    }
    pub fn DyeOffHand<'a>(&'a self) -> &'a Field {
        &self.columns[81]
    }
    pub fn Dye2OffHand<'a>(&'a self) -> &'a Field {
        &self.columns[82]
    }
    pub fn DyeHead<'a>(&'a self) -> &'a Field {
        &self.columns[83]
    }
    pub fn DyeBody<'a>(&'a self) -> &'a Field {
        &self.columns[84]
    }
    pub fn DyeHands<'a>(&'a self) -> &'a Field {
        &self.columns[85]
    }
    pub fn DyeLegs<'a>(&'a self) -> &'a Field {
        &self.columns[86]
    }
    pub fn DyeFeet<'a>(&'a self) -> &'a Field {
        &self.columns[87]
    }
    pub fn DyeEars<'a>(&'a self) -> &'a Field {
        &self.columns[88]
    }
    pub fn DyeNeck<'a>(&'a self) -> &'a Field {
        &self.columns[89]
    }
    pub fn DyeWrists<'a>(&'a self) -> &'a Field {
        &self.columns[90]
    }
    pub fn DyeLeftRing<'a>(&'a self) -> &'a Field {
        &self.columns[91]
    }
    pub fn DyeRightRing<'a>(&'a self) -> &'a Field {
        &self.columns[92]
    }
    pub fn Dye2Head<'a>(&'a self) -> &'a Field {
        &self.columns[93]
    }
    pub fn Dye2Body<'a>(&'a self) -> &'a Field {
        &self.columns[94]
    }
    pub fn Dye2Hands<'a>(&'a self) -> &'a Field {
        &self.columns[95]
    }
    pub fn Dye2Legs<'a>(&'a self) -> &'a Field {
        &self.columns[96]
    }
    pub fn Dye2Feet<'a>(&'a self) -> &'a Field {
        &self.columns[97]
    }
    pub fn Dye2Ears<'a>(&'a self) -> &'a Field {
        &self.columns[98]
    }
    pub fn Dye2Neck<'a>(&'a self) -> &'a Field {
        &self.columns[99]
    }
    pub fn Dye2Wrists<'a>(&'a self) -> &'a Field {
        &self.columns[100]
    }
    pub fn Dye2LeftRing<'a>(&'a self) -> &'a Field {
        &self.columns[101]
    }
    pub fn Dye2RightRing<'a>(&'a self) -> &'a Field {
        &self.columns[102]
    }
    pub fn Invisibility<'a>(&'a self) -> &'a Field {
        &self.columns[103]
    }
    pub fn DefaultBalloon<'a>(&'a self) -> &'a Field {
        &self.columns[104]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a Field {
        &self.columns[105]
    }
    pub fn Important<'a>(&'a self) -> &'a Field {
        &self.columns[106]
    }
    pub fn Visor<'a>(&'a self) -> &'a Field {
        &self.columns[107]
    }
    pub fn NotRewriteHeight<'a>(&'a self) -> &'a Field {
        &self.columns[108]
    }
    pub fn Unknown2<'a>(&'a self) -> &'a Field {
        &self.columns[109]
    }
}
