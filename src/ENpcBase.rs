//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    resource::{Resource, read_excel_sheet_header, read_excel_sheet},
    exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow},
    exh::{EXH, ExcelColumnDefinition},
    common::Language,
};
pub struct ENpcBaseSheet {
    pages: Vec<EXD>,
    exh: EXH,
    row_count: u32,
}
impl ENpcBaseSheet {
    /// Read the sheet from a `Resource`.
    pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
        let exh = read_excel_sheet_header(resource, "ENpcBase")?;
        let mut pages = Vec::new();
        for (i, _) in exh.pages.iter().enumerate() {
            pages.push(read_excel_sheet(resource, "ENpcBase", &exh, language, i)?);
        }
        let row_count = exh.header.row_count;
        Some(Self { exh, pages, row_count })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<ENpcBaseRow> {
        let column_defs = &self.exh.column_definitions;
        let mut zipped: Vec<_> = row
            .columns
            .clone()
            .into_iter()
            .zip(column_defs)
            .collect();
        zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
        let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition>) = zipped
            .into_iter()
            .unzip();
        Some(ENpcBaseRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<ENpcBaseRow> {
        for page in &self.pages {
            let Some(row) = &page.get_row(row_id) else {
                continue;
            };
            let row = match row {
                ExcelRowKind::SingleRow(row) => row,
                ExcelRowKind::SubRows(rows) => &rows.first()?.1,
            };
            return self.read_row(row);
        }
        None
    }
    /// Fetches the specified subrow from the sheet.
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<ENpcBaseRow> {
        for page in &self.pages {
            let Some(row) = &page.get_row(row_id) else {
                continue;
            };
            let row = match row {
                ExcelRowKind::SingleRow(row) => return None,
                ExcelRowKind::SubRows(subrows) => {
                    &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1
                }
            };
            return self.read_row(row);
        }
        None
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.row_count
    }
}
pub struct ENpcBaseRow {
    columns: Vec<ColumnData>,
}
impl ENpcBaseRow {
    pub fn ENpcData<'a>(&'a self) -> [&'a ColumnData; 32] {
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
    pub fn ModelMainHand<'a>(&'a self) -> &'a ColumnData {
        &self.columns[32]
    }
    pub fn ModelOffHand<'a>(&'a self) -> &'a ColumnData {
        &self.columns[33]
    }
    pub fn Scale<'a>(&'a self) -> &'a ColumnData {
        &self.columns[34]
    }
    pub fn ModelHead<'a>(&'a self) -> &'a ColumnData {
        &self.columns[35]
    }
    pub fn ModelBody<'a>(&'a self) -> &'a ColumnData {
        &self.columns[36]
    }
    pub fn ModelHands<'a>(&'a self) -> &'a ColumnData {
        &self.columns[37]
    }
    pub fn ModelLegs<'a>(&'a self) -> &'a ColumnData {
        &self.columns[38]
    }
    pub fn ModelFeet<'a>(&'a self) -> &'a ColumnData {
        &self.columns[39]
    }
    pub fn ModelEars<'a>(&'a self) -> &'a ColumnData {
        &self.columns[40]
    }
    pub fn ModelNeck<'a>(&'a self) -> &'a ColumnData {
        &self.columns[41]
    }
    pub fn ModelWrists<'a>(&'a self) -> &'a ColumnData {
        &self.columns[42]
    }
    pub fn ModelLeftRing<'a>(&'a self) -> &'a ColumnData {
        &self.columns[43]
    }
    pub fn ModelRightRing<'a>(&'a self) -> &'a ColumnData {
        &self.columns[44]
    }
    pub fn EventHandler<'a>(&'a self) -> &'a ColumnData {
        &self.columns[45]
    }
    pub fn ModelChara<'a>(&'a self) -> &'a ColumnData {
        &self.columns[46]
    }
    pub fn NpcEquip<'a>(&'a self) -> &'a ColumnData {
        &self.columns[47]
    }
    pub fn Behavior<'a>(&'a self) -> &'a ColumnData {
        &self.columns[48]
    }
    pub fn Unknown_70_1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[49]
    }
    pub fn Unknown_70_2<'a>(&'a self) -> &'a ColumnData {
        &self.columns[50]
    }
    pub fn Balloon<'a>(&'a self) -> &'a ColumnData {
        &self.columns[51]
    }
    pub fn Race<'a>(&'a self) -> &'a ColumnData {
        &self.columns[52]
    }
    pub fn Gender<'a>(&'a self) -> &'a ColumnData {
        &self.columns[53]
    }
    pub fn BodyType<'a>(&'a self) -> &'a ColumnData {
        &self.columns[54]
    }
    pub fn Height<'a>(&'a self) -> &'a ColumnData {
        &self.columns[55]
    }
    pub fn Tribe<'a>(&'a self) -> &'a ColumnData {
        &self.columns[56]
    }
    pub fn Face<'a>(&'a self) -> &'a ColumnData {
        &self.columns[57]
    }
    pub fn HairStyle<'a>(&'a self) -> &'a ColumnData {
        &self.columns[58]
    }
    pub fn HairHighlight<'a>(&'a self) -> &'a ColumnData {
        &self.columns[59]
    }
    pub fn SkinColor<'a>(&'a self) -> &'a ColumnData {
        &self.columns[60]
    }
    pub fn EyeHeterochromia<'a>(&'a self) -> &'a ColumnData {
        &self.columns[61]
    }
    pub fn HairColor<'a>(&'a self) -> &'a ColumnData {
        &self.columns[62]
    }
    pub fn HairHighlightColor<'a>(&'a self) -> &'a ColumnData {
        &self.columns[63]
    }
    pub fn FacialFeature<'a>(&'a self) -> &'a ColumnData {
        &self.columns[64]
    }
    pub fn FacialFeatureColor<'a>(&'a self) -> &'a ColumnData {
        &self.columns[65]
    }
    pub fn Eyebrows<'a>(&'a self) -> &'a ColumnData {
        &self.columns[66]
    }
    pub fn EyeColor<'a>(&'a self) -> &'a ColumnData {
        &self.columns[67]
    }
    pub fn EyeShape<'a>(&'a self) -> &'a ColumnData {
        &self.columns[68]
    }
    pub fn Nose<'a>(&'a self) -> &'a ColumnData {
        &self.columns[69]
    }
    pub fn Jaw<'a>(&'a self) -> &'a ColumnData {
        &self.columns[70]
    }
    pub fn Mouth<'a>(&'a self) -> &'a ColumnData {
        &self.columns[71]
    }
    pub fn LipColor<'a>(&'a self) -> &'a ColumnData {
        &self.columns[72]
    }
    pub fn BustOrTone1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[73]
    }
    pub fn ExtraFeature1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[74]
    }
    pub fn ExtraFeature2OrBust<'a>(&'a self) -> &'a ColumnData {
        &self.columns[75]
    }
    pub fn FacePaint<'a>(&'a self) -> &'a ColumnData {
        &self.columns[76]
    }
    pub fn FacePaintColor<'a>(&'a self) -> &'a ColumnData {
        &self.columns[77]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a ColumnData {
        &self.columns[78]
    }
    pub fn DyeMainHand<'a>(&'a self) -> &'a ColumnData {
        &self.columns[79]
    }
    pub fn Dye2MainHand<'a>(&'a self) -> &'a ColumnData {
        &self.columns[80]
    }
    pub fn DyeOffHand<'a>(&'a self) -> &'a ColumnData {
        &self.columns[81]
    }
    pub fn Dye2OffHand<'a>(&'a self) -> &'a ColumnData {
        &self.columns[82]
    }
    pub fn DyeHead<'a>(&'a self) -> &'a ColumnData {
        &self.columns[83]
    }
    pub fn DyeBody<'a>(&'a self) -> &'a ColumnData {
        &self.columns[84]
    }
    pub fn DyeHands<'a>(&'a self) -> &'a ColumnData {
        &self.columns[85]
    }
    pub fn DyeLegs<'a>(&'a self) -> &'a ColumnData {
        &self.columns[86]
    }
    pub fn DyeFeet<'a>(&'a self) -> &'a ColumnData {
        &self.columns[87]
    }
    pub fn DyeEars<'a>(&'a self) -> &'a ColumnData {
        &self.columns[88]
    }
    pub fn DyeNeck<'a>(&'a self) -> &'a ColumnData {
        &self.columns[89]
    }
    pub fn DyeWrists<'a>(&'a self) -> &'a ColumnData {
        &self.columns[90]
    }
    pub fn DyeLeftRing<'a>(&'a self) -> &'a ColumnData {
        &self.columns[91]
    }
    pub fn DyeRightRing<'a>(&'a self) -> &'a ColumnData {
        &self.columns[92]
    }
    pub fn Dye2Head<'a>(&'a self) -> &'a ColumnData {
        &self.columns[93]
    }
    pub fn Dye2Body<'a>(&'a self) -> &'a ColumnData {
        &self.columns[94]
    }
    pub fn Dye2Hands<'a>(&'a self) -> &'a ColumnData {
        &self.columns[95]
    }
    pub fn Dye2Legs<'a>(&'a self) -> &'a ColumnData {
        &self.columns[96]
    }
    pub fn Dye2Feet<'a>(&'a self) -> &'a ColumnData {
        &self.columns[97]
    }
    pub fn Dye2Ears<'a>(&'a self) -> &'a ColumnData {
        &self.columns[98]
    }
    pub fn Dye2Neck<'a>(&'a self) -> &'a ColumnData {
        &self.columns[99]
    }
    pub fn Dye2Wrists<'a>(&'a self) -> &'a ColumnData {
        &self.columns[100]
    }
    pub fn Dye2LeftRing<'a>(&'a self) -> &'a ColumnData {
        &self.columns[101]
    }
    pub fn Dye2RightRing<'a>(&'a self) -> &'a ColumnData {
        &self.columns[102]
    }
    pub fn Invisibility<'a>(&'a self) -> &'a ColumnData {
        &self.columns[103]
    }
    pub fn DefaultBalloon<'a>(&'a self) -> &'a ColumnData {
        &self.columns[104]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[105]
    }
    pub fn Important<'a>(&'a self) -> &'a ColumnData {
        &self.columns[106]
    }
    pub fn Visor<'a>(&'a self) -> &'a ColumnData {
        &self.columns[107]
    }
    pub fn NotRewriteHeight<'a>(&'a self) -> &'a ColumnData {
        &self.columns[108]
    }
    pub fn Unknown2<'a>(&'a self) -> &'a ColumnData {
        &self.columns[109]
    }
}
