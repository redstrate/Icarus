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
    index_mapping: Vec<usize>,
}
impl ENpcBaseSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("ENpcBase")?;
        let sheet = resolver.read_excel_sheet(&exh, "ENpcBase", language)?;
        let mut index_mapping: Vec<(usize, &ExcelColumnDefinition)> = sheet
            .exh
            .column_definitions
            .iter()
            .enumerate()
            .collect();
        index_mapping.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
        let index_mapping: Vec<usize> = index_mapping
            .iter()
            .map(|(index, _)| *index)
            .collect();
        Ok(Self { sheet, index_mapping })
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
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
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
    index_mapping: Vec<usize>,
}
impl<'a> ENpcBaseRow<'a> {
    pub fn ENpcData(&'a self) -> [&'a Field; 32] {
        [
            &self.row.columns[self.index_mapping[0]],
            &self.row.columns[self.index_mapping[1]],
            &self.row.columns[self.index_mapping[2]],
            &self.row.columns[self.index_mapping[3]],
            &self.row.columns[self.index_mapping[4]],
            &self.row.columns[self.index_mapping[5]],
            &self.row.columns[self.index_mapping[6]],
            &self.row.columns[self.index_mapping[7]],
            &self.row.columns[self.index_mapping[8]],
            &self.row.columns[self.index_mapping[9]],
            &self.row.columns[self.index_mapping[10]],
            &self.row.columns[self.index_mapping[11]],
            &self.row.columns[self.index_mapping[12]],
            &self.row.columns[self.index_mapping[13]],
            &self.row.columns[self.index_mapping[14]],
            &self.row.columns[self.index_mapping[15]],
            &self.row.columns[self.index_mapping[16]],
            &self.row.columns[self.index_mapping[17]],
            &self.row.columns[self.index_mapping[18]],
            &self.row.columns[self.index_mapping[19]],
            &self.row.columns[self.index_mapping[20]],
            &self.row.columns[self.index_mapping[21]],
            &self.row.columns[self.index_mapping[22]],
            &self.row.columns[self.index_mapping[23]],
            &self.row.columns[self.index_mapping[24]],
            &self.row.columns[self.index_mapping[25]],
            &self.row.columns[self.index_mapping[26]],
            &self.row.columns[self.index_mapping[27]],
            &self.row.columns[self.index_mapping[28]],
            &self.row.columns[self.index_mapping[29]],
            &self.row.columns[self.index_mapping[30]],
            &self.row.columns[self.index_mapping[31]],
        ]
    }
    pub fn ModelMainHand(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[32]]
    }
    pub fn ModelOffHand(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[33]]
    }
    pub fn Scale(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[34]]
    }
    pub fn ModelHead(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[35]]
    }
    pub fn ModelBody(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[36]]
    }
    pub fn ModelHands(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[37]]
    }
    pub fn ModelLegs(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[38]]
    }
    pub fn ModelFeet(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[39]]
    }
    pub fn ModelEars(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[40]]
    }
    pub fn ModelNeck(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[41]]
    }
    pub fn ModelWrists(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[42]]
    }
    pub fn ModelLeftRing(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[43]]
    }
    pub fn ModelRightRing(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[44]]
    }
    pub fn EventHandler(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[45]]
    }
    pub fn ModelChara(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[46]]
    }
    pub fn NpcEquip(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[47]]
    }
    pub fn Behavior(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[48]]
    }
    pub fn Unknown_70_1(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[49]]
    }
    pub fn Unknown_70_2(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[50]]
    }
    pub fn Balloon(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[51]]
    }
    pub fn Race(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[52]]
    }
    pub fn Gender(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[53]]
    }
    pub fn BodyType(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[54]]
    }
    pub fn Height(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[55]]
    }
    pub fn Tribe(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[56]]
    }
    pub fn Face(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[57]]
    }
    pub fn HairStyle(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[58]]
    }
    pub fn HairHighlight(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[59]]
    }
    pub fn SkinColor(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[60]]
    }
    pub fn EyeHeterochromia(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[61]]
    }
    pub fn HairColor(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[62]]
    }
    pub fn HairHighlightColor(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[63]]
    }
    pub fn FacialFeature(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[64]]
    }
    pub fn FacialFeatureColor(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[65]]
    }
    pub fn Eyebrows(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[66]]
    }
    pub fn EyeColor(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[67]]
    }
    pub fn EyeShape(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[68]]
    }
    pub fn Nose(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[69]]
    }
    pub fn Jaw(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[70]]
    }
    pub fn Mouth(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[71]]
    }
    pub fn LipColor(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[72]]
    }
    pub fn BustOrTone1(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[73]]
    }
    pub fn ExtraFeature1(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[74]]
    }
    pub fn ExtraFeature2OrBust(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[75]]
    }
    pub fn FacePaint(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[76]]
    }
    pub fn FacePaintColor(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[77]]
    }
    pub fn Unknown0(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[78]]
    }
    pub fn DyeMainHand(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[79]]
    }
    pub fn Dye2MainHand(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[80]]
    }
    pub fn DyeOffHand(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[81]]
    }
    pub fn Dye2OffHand(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[82]]
    }
    pub fn DyeHead(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[83]]
    }
    pub fn DyeBody(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[84]]
    }
    pub fn DyeHands(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[85]]
    }
    pub fn DyeLegs(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[86]]
    }
    pub fn DyeFeet(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[87]]
    }
    pub fn DyeEars(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[88]]
    }
    pub fn DyeNeck(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[89]]
    }
    pub fn DyeWrists(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[90]]
    }
    pub fn DyeLeftRing(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[91]]
    }
    pub fn DyeRightRing(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[92]]
    }
    pub fn Dye2Head(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[93]]
    }
    pub fn Dye2Body(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[94]]
    }
    pub fn Dye2Hands(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[95]]
    }
    pub fn Dye2Legs(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[96]]
    }
    pub fn Dye2Feet(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[97]]
    }
    pub fn Dye2Ears(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[98]]
    }
    pub fn Dye2Neck(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[99]]
    }
    pub fn Dye2Wrists(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[100]]
    }
    pub fn Dye2LeftRing(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[101]]
    }
    pub fn Dye2RightRing(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[102]]
    }
    pub fn Invisibility(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[103]]
    }
    pub fn DefaultBalloon(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[104]]
    }
    pub fn Unknown1(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[105]]
    }
    pub fn Important(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[106]]
    }
    pub fn Visor(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[107]]
    }
    pub fn NotRewriteHeight(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[108]]
    }
    pub fn Unknown2(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[109]]
    }
}
