#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct MountSheet {
pages: Vec<EXD>,
exh: EXH,
row_count: u32,
}
impl MountSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "Mount")?;
let mut pages = Vec::new();
for (i, _) in exh.pages.iter().enumerate() {
pages.push(read_excel_sheet(resource, "Mount", &exh, language, i)?);
}let row_count = exh.header.row_count;
Some(Self {
exh,
pages,
row_count,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<MountRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(MountRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<MountRow> {
for page in &self.pages {
let Some(row) = &page.get_row(row_id) else { continue; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
return self.read_row(row);
}
None
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<MountRow> {
for page in &self.pages {
let Some(row) = &page.get_row(row_id) else { continue; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
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
pub struct MountRow {
columns: Vec<ColumnData>,
}
impl MountRow {
pub fn Singular(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Plural(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Adjective(&self) -> &ColumnData {
&self.columns[2]
}
pub fn PossessivePronoun(&self) -> &ColumnData {
&self.columns[3]
}
pub fn StartsWithVowel(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Pronoun(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Article(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[10]
}
pub fn ModelChara(&self) -> &ColumnData {
&self.columns[11]
}
pub fn EquipHead(&self) -> &ColumnData {
&self.columns[12]
}
pub fn EquipBody(&self) -> &ColumnData {
&self.columns[13]
}
pub fn EquipLeg(&self) -> &ColumnData {
&self.columns[14]
}
pub fn EquipFoot(&self) -> &ColumnData {
&self.columns[15]
}
pub fn MoveControl(&self) -> &ColumnData {
&self.columns[16]
}
pub fn RideBGM(&self) -> &ColumnData {
&self.columns[17]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[18]
}
pub fn UIPriority(&self) -> &ColumnData {
&self.columns[19]
}
pub fn MountAction(&self) -> &ColumnData {
&self.columns[20]
}
pub fn Unknown_70_1(&self) -> &ColumnData {
&self.columns[21]
}
pub fn Unknown_70_2(&self) -> &ColumnData {
&self.columns[22]
}
pub fn Unknown16(&self) -> &ColumnData {
&self.columns[23]
}
pub fn Unknown17(&self) -> &ColumnData {
&self.columns[24]
}
pub fn Order(&self) -> &ColumnData {
&self.columns[25]
}
pub fn FlyingCondition(&self) -> &ColumnData {
&self.columns[26]
}
pub fn Unknown5(&self) -> &ColumnData {
&self.columns[27]
}
pub fn Unknown6(&self) -> &ColumnData {
&self.columns[28]
}
pub fn Unknown7(&self) -> &ColumnData {
&self.columns[29]
}
pub fn IsFlying(&self) -> &ColumnData {
&self.columns[30]
}
pub fn Unknown8(&self) -> &ColumnData {
&self.columns[31]
}
pub fn MountCustomize(&self) -> &ColumnData {
&self.columns[32]
}
pub fn ExitMoveDist(&self) -> &ColumnData {
&self.columns[33]
}
pub fn ExitMoveSpeed(&self) -> &ColumnData {
&self.columns[34]
}
pub fn RadiusRate(&self) -> &ColumnData {
&self.columns[35]
}
pub fn BaseMotionSpeed_Run(&self) -> &ColumnData {
&self.columns[36]
}
pub fn BaseMotionSpeed_Walk(&self) -> &ColumnData {
&self.columns[37]
}
pub fn Unknown9(&self) -> &ColumnData {
&self.columns[38]
}
pub fn ExtraSeats(&self) -> &ColumnData {
&self.columns[39]
}
pub fn Unknown10(&self) -> &ColumnData {
&self.columns[40]
}
pub fn Unknown11(&self) -> &ColumnData {
&self.columns[41]
}
pub fn Unknown12(&self) -> &ColumnData {
&self.columns[42]
}
pub fn IsEmote(&self) -> &ColumnData {
&self.columns[43]
}
pub fn Unknown20(&self) -> &ColumnData {
&self.columns[44]
}
pub fn IsAirborne(&self) -> &ColumnData {
&self.columns[45]
}
pub fn ExHotbarEnableConfig(&self) -> &ColumnData {
&self.columns[46]
}
pub fn UseEP(&self) -> &ColumnData {
&self.columns[47]
}
pub fn Unknown13(&self) -> &ColumnData {
&self.columns[48]
}
pub fn IsImmobile(&self) -> &ColumnData {
&self.columns[49]
}
pub fn Unknown14(&self) -> &ColumnData {
&self.columns[50]
}
pub fn Unknown15(&self) -> &ColumnData {
&self.columns[51]
}
pub fn Unknown18(&self) -> &ColumnData {
&self.columns[52]
}
pub fn Unknown19(&self) -> &ColumnData {
&self.columns[53]
}
}
