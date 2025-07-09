#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct NpcEquipSheet {
exd: EXD,
exh: EXH,
}
impl NpcEquipSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "NpcEquip")?;let exd = read_excel_sheet(resource, "NpcEquip", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<NpcEquipRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(NpcEquipRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<NpcEquipRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<NpcEquipRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct NpcEquipRow {
columns: Vec<ColumnData>,
}
impl NpcEquipRow {
pub fn ModelMainHand(&self) -> &ColumnData {
&self.columns[0]
}
pub fn ModelOffHand(&self) -> &ColumnData {
&self.columns[1]
}
pub fn ModelHead(&self) -> &ColumnData {
&self.columns[2]
}
pub fn ModelBody(&self) -> &ColumnData {
&self.columns[3]
}
pub fn ModelHands(&self) -> &ColumnData {
&self.columns[4]
}
pub fn ModelLegs(&self) -> &ColumnData {
&self.columns[5]
}
pub fn ModelFeet(&self) -> &ColumnData {
&self.columns[6]
}
pub fn ModelEars(&self) -> &ColumnData {
&self.columns[7]
}
pub fn ModelNeck(&self) -> &ColumnData {
&self.columns[8]
}
pub fn ModelWrists(&self) -> &ColumnData {
&self.columns[9]
}
pub fn ModelLeftRing(&self) -> &ColumnData {
&self.columns[10]
}
pub fn ModelRightRing(&self) -> &ColumnData {
&self.columns[11]
}
pub fn Unknown_70_1(&self) -> &ColumnData {
&self.columns[12]
}
pub fn Unknown_70_2(&self) -> &ColumnData {
&self.columns[13]
}
pub fn DyeMainHand(&self) -> &ColumnData {
&self.columns[14]
}
pub fn Dye2MainHand(&self) -> &ColumnData {
&self.columns[15]
}
pub fn DyeOffHand(&self) -> &ColumnData {
&self.columns[16]
}
pub fn Dye2OffHand(&self) -> &ColumnData {
&self.columns[17]
}
pub fn DyeHead(&self) -> &ColumnData {
&self.columns[18]
}
pub fn DyeBody(&self) -> &ColumnData {
&self.columns[19]
}
pub fn DyeHands(&self) -> &ColumnData {
&self.columns[20]
}
pub fn DyeLegs(&self) -> &ColumnData {
&self.columns[21]
}
pub fn DyeFeet(&self) -> &ColumnData {
&self.columns[22]
}
pub fn DyeEars(&self) -> &ColumnData {
&self.columns[23]
}
pub fn DyeNeck(&self) -> &ColumnData {
&self.columns[24]
}
pub fn DyeWrists(&self) -> &ColumnData {
&self.columns[25]
}
pub fn DyeLeftRing(&self) -> &ColumnData {
&self.columns[26]
}
pub fn DyeRightRing(&self) -> &ColumnData {
&self.columns[27]
}
pub fn Dye2Head(&self) -> &ColumnData {
&self.columns[28]
}
pub fn Dye2Body(&self) -> &ColumnData {
&self.columns[29]
}
pub fn Dye2Hands(&self) -> &ColumnData {
&self.columns[30]
}
pub fn Dye2Legs(&self) -> &ColumnData {
&self.columns[31]
}
pub fn Dye2Feet(&self) -> &ColumnData {
&self.columns[32]
}
pub fn Dye2Ears(&self) -> &ColumnData {
&self.columns[33]
}
pub fn Dye2Neck(&self) -> &ColumnData {
&self.columns[34]
}
pub fn Dye2Wrists(&self) -> &ColumnData {
&self.columns[35]
}
pub fn Dye2LeftRing(&self) -> &ColumnData {
&self.columns[36]
}
pub fn Dye2RightRing(&self) -> &ColumnData {
&self.columns[37]
}
pub fn Visor(&self) -> &ColumnData {
&self.columns[38]
}
}
