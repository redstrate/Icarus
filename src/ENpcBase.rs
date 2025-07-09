#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct ENpcBaseSheet {
exd: EXD,
exh: EXH,
}
impl ENpcBaseSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "ENpcBase")?;let exd = read_excel_sheet(resource, "ENpcBase", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<ENpcBaseRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(ENpcBaseRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<ENpcBaseRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<ENpcBaseRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct ENpcBaseRow {
columns: Vec<ColumnData>,
}
impl ENpcBaseRow {
pub fn ENpcData(&self) -> [&ColumnData; 32] {
[&self.columns[0],&self.columns[1],&self.columns[2],&self.columns[3],&self.columns[4],&self.columns[5],&self.columns[6],&self.columns[7],&self.columns[8],&self.columns[9],&self.columns[10],&self.columns[11],&self.columns[12],&self.columns[13],&self.columns[14],&self.columns[15],&self.columns[16],&self.columns[17],&self.columns[18],&self.columns[19],&self.columns[20],&self.columns[21],&self.columns[22],&self.columns[23],&self.columns[24],&self.columns[25],&self.columns[26],&self.columns[27],&self.columns[28],&self.columns[29],&self.columns[30],&self.columns[31],]
}
pub fn ModelMainHand(&self) -> &ColumnData {
&self.columns[32]
}
pub fn ModelOffHand(&self) -> &ColumnData {
&self.columns[33]
}
pub fn Scale(&self) -> &ColumnData {
&self.columns[34]
}
pub fn ModelHead(&self) -> &ColumnData {
&self.columns[35]
}
pub fn ModelBody(&self) -> &ColumnData {
&self.columns[36]
}
pub fn ModelHands(&self) -> &ColumnData {
&self.columns[37]
}
pub fn ModelLegs(&self) -> &ColumnData {
&self.columns[38]
}
pub fn ModelFeet(&self) -> &ColumnData {
&self.columns[39]
}
pub fn ModelEars(&self) -> &ColumnData {
&self.columns[40]
}
pub fn ModelNeck(&self) -> &ColumnData {
&self.columns[41]
}
pub fn ModelWrists(&self) -> &ColumnData {
&self.columns[42]
}
pub fn ModelLeftRing(&self) -> &ColumnData {
&self.columns[43]
}
pub fn ModelRightRing(&self) -> &ColumnData {
&self.columns[44]
}
pub fn EventHandler(&self) -> &ColumnData {
&self.columns[45]
}
pub fn ModelChara(&self) -> &ColumnData {
&self.columns[46]
}
pub fn NpcEquip(&self) -> &ColumnData {
&self.columns[47]
}
pub fn Behavior(&self) -> &ColumnData {
&self.columns[48]
}
pub fn Unknown_70_1(&self) -> &ColumnData {
&self.columns[49]
}
pub fn Unknown_70_2(&self) -> &ColumnData {
&self.columns[50]
}
pub fn Balloon(&self) -> &ColumnData {
&self.columns[51]
}
pub fn Race(&self) -> &ColumnData {
&self.columns[52]
}
pub fn Gender(&self) -> &ColumnData {
&self.columns[53]
}
pub fn BodyType(&self) -> &ColumnData {
&self.columns[54]
}
pub fn Height(&self) -> &ColumnData {
&self.columns[55]
}
pub fn Tribe(&self) -> &ColumnData {
&self.columns[56]
}
pub fn Face(&self) -> &ColumnData {
&self.columns[57]
}
pub fn HairStyle(&self) -> &ColumnData {
&self.columns[58]
}
pub fn HairHighlight(&self) -> &ColumnData {
&self.columns[59]
}
pub fn SkinColor(&self) -> &ColumnData {
&self.columns[60]
}
pub fn EyeHeterochromia(&self) -> &ColumnData {
&self.columns[61]
}
pub fn HairColor(&self) -> &ColumnData {
&self.columns[62]
}
pub fn HairHighlightColor(&self) -> &ColumnData {
&self.columns[63]
}
pub fn FacialFeature(&self) -> &ColumnData {
&self.columns[64]
}
pub fn FacialFeatureColor(&self) -> &ColumnData {
&self.columns[65]
}
pub fn Eyebrows(&self) -> &ColumnData {
&self.columns[66]
}
pub fn EyeColor(&self) -> &ColumnData {
&self.columns[67]
}
pub fn EyeShape(&self) -> &ColumnData {
&self.columns[68]
}
pub fn Nose(&self) -> &ColumnData {
&self.columns[69]
}
pub fn Jaw(&self) -> &ColumnData {
&self.columns[70]
}
pub fn Mouth(&self) -> &ColumnData {
&self.columns[71]
}
pub fn LipColor(&self) -> &ColumnData {
&self.columns[72]
}
pub fn BustOrTone1(&self) -> &ColumnData {
&self.columns[73]
}
pub fn ExtraFeature1(&self) -> &ColumnData {
&self.columns[74]
}
pub fn ExtraFeature2OrBust(&self) -> &ColumnData {
&self.columns[75]
}
pub fn FacePaint(&self) -> &ColumnData {
&self.columns[76]
}
pub fn FacePaintColor(&self) -> &ColumnData {
&self.columns[77]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[78]
}
pub fn DyeMainHand(&self) -> &ColumnData {
&self.columns[79]
}
pub fn Dye2MainHand(&self) -> &ColumnData {
&self.columns[80]
}
pub fn DyeOffHand(&self) -> &ColumnData {
&self.columns[81]
}
pub fn Dye2OffHand(&self) -> &ColumnData {
&self.columns[82]
}
pub fn DyeHead(&self) -> &ColumnData {
&self.columns[83]
}
pub fn DyeBody(&self) -> &ColumnData {
&self.columns[84]
}
pub fn DyeHands(&self) -> &ColumnData {
&self.columns[85]
}
pub fn DyeLegs(&self) -> &ColumnData {
&self.columns[86]
}
pub fn DyeFeet(&self) -> &ColumnData {
&self.columns[87]
}
pub fn DyeEars(&self) -> &ColumnData {
&self.columns[88]
}
pub fn DyeNeck(&self) -> &ColumnData {
&self.columns[89]
}
pub fn DyeWrists(&self) -> &ColumnData {
&self.columns[90]
}
pub fn DyeLeftRing(&self) -> &ColumnData {
&self.columns[91]
}
pub fn DyeRightRing(&self) -> &ColumnData {
&self.columns[92]
}
pub fn Dye2Head(&self) -> &ColumnData {
&self.columns[93]
}
pub fn Dye2Body(&self) -> &ColumnData {
&self.columns[94]
}
pub fn Dye2Hands(&self) -> &ColumnData {
&self.columns[95]
}
pub fn Dye2Legs(&self) -> &ColumnData {
&self.columns[96]
}
pub fn Dye2Feet(&self) -> &ColumnData {
&self.columns[97]
}
pub fn Dye2Ears(&self) -> &ColumnData {
&self.columns[98]
}
pub fn Dye2Neck(&self) -> &ColumnData {
&self.columns[99]
}
pub fn Dye2Wrists(&self) -> &ColumnData {
&self.columns[100]
}
pub fn Dye2LeftRing(&self) -> &ColumnData {
&self.columns[101]
}
pub fn Dye2RightRing(&self) -> &ColumnData {
&self.columns[102]
}
pub fn Invisibility(&self) -> &ColumnData {
&self.columns[103]
}
pub fn DefaultBalloon(&self) -> &ColumnData {
&self.columns[104]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[105]
}
pub fn Important(&self) -> &ColumnData {
&self.columns[106]
}
pub fn Visor(&self) -> &ColumnData {
&self.columns[107]
}
pub fn NotRewriteHeight(&self) -> &ColumnData {
&self.columns[108]
}
}
