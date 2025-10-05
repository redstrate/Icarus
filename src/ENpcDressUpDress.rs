#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct ENpcDressUpDressSheet {
pages: Vec<EXD>,
exh: EXH,
row_count: u32,
}
impl ENpcDressUpDressSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "ENpcDressUpDress")?;
let mut pages = Vec::new();
for (i, _) in exh.pages.iter().enumerate() {
pages.push(read_excel_sheet(resource, "ENpcDressUpDress", &exh, language, i)?);
}let row_count = exh.header.row_count;
Some(Self {
exh,
pages,
row_count,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<ENpcDressUpDressRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(ENpcDressUpDressRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<ENpcDressUpDressRow> {
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
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<ENpcDressUpDressRow> {
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
pub struct ENpcDressUpDressRow {
columns: Vec<ColumnData>,
}
impl ENpcDressUpDressRow {
pub fn ModelMainHand(&self) -> &ColumnData {
&self.columns[0]
}
pub fn ModelOffHand(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[2]
}
pub fn ENpc(&self) -> &ColumnData {
&self.columns[3]
}
pub fn ModelHead(&self) -> &ColumnData {
&self.columns[4]
}
pub fn ModelBody(&self) -> &ColumnData {
&self.columns[5]
}
pub fn ModelHands(&self) -> &ColumnData {
&self.columns[6]
}
pub fn ModelLegs(&self) -> &ColumnData {
&self.columns[7]
}
pub fn ModelFeet(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[10]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[11]
}
pub fn Unknown4(&self) -> &ColumnData {
&self.columns[12]
}
pub fn Unknown5(&self) -> &ColumnData {
&self.columns[13]
}
pub fn Unknown6(&self) -> &ColumnData {
&self.columns[14]
}
pub fn Behavior(&self) -> &ColumnData {
&self.columns[15]
}
pub fn Unknown7(&self) -> &ColumnData {
&self.columns[16]
}
pub fn Unknown8(&self) -> &ColumnData {
&self.columns[17]
}
pub fn Unknown9(&self) -> &ColumnData {
&self.columns[18]
}
pub fn Unknown10(&self) -> &ColumnData {
&self.columns[19]
}
pub fn Unknown11(&self) -> &ColumnData {
&self.columns[20]
}
pub fn Unknown12(&self) -> &ColumnData {
&self.columns[21]
}
pub fn Unknown13(&self) -> &ColumnData {
&self.columns[22]
}
pub fn Unknown14(&self) -> &ColumnData {
&self.columns[23]
}
pub fn Unknown15(&self) -> &ColumnData {
&self.columns[24]
}
pub fn Unknown16(&self) -> &ColumnData {
&self.columns[25]
}
pub fn Unknown17(&self) -> &ColumnData {
&self.columns[26]
}
pub fn Unknown18(&self) -> &ColumnData {
&self.columns[27]
}
pub fn Unknown19(&self) -> &ColumnData {
&self.columns[28]
}
pub fn Unknown20(&self) -> &ColumnData {
&self.columns[29]
}
pub fn Unknown21(&self) -> &ColumnData {
&self.columns[30]
}
pub fn Unknown22(&self) -> &ColumnData {
&self.columns[31]
}
pub fn Unknown23(&self) -> &ColumnData {
&self.columns[32]
}
pub fn Unknown24(&self) -> &ColumnData {
&self.columns[33]
}
pub fn Unknown25(&self) -> &ColumnData {
&self.columns[34]
}
pub fn Unknown26(&self) -> &ColumnData {
&self.columns[35]
}
pub fn Unknown27(&self) -> &ColumnData {
&self.columns[36]
}
pub fn Unknown28(&self) -> &ColumnData {
&self.columns[37]
}
pub fn Unknown29(&self) -> &ColumnData {
&self.columns[38]
}
pub fn Unknown30(&self) -> &ColumnData {
&self.columns[39]
}
pub fn Unknown31(&self) -> &ColumnData {
&self.columns[40]
}
pub fn Unknown32(&self) -> &ColumnData {
&self.columns[41]
}
pub fn Unknown33(&self) -> &ColumnData {
&self.columns[42]
}
pub fn Unknown34(&self) -> &ColumnData {
&self.columns[43]
}
pub fn DyeMainHand(&self) -> &ColumnData {
&self.columns[44]
}
pub fn Dye2MainHand(&self) -> &ColumnData {
&self.columns[45]
}
pub fn DyeOffHand(&self) -> &ColumnData {
&self.columns[46]
}
pub fn Dye2OffHand(&self) -> &ColumnData {
&self.columns[47]
}
pub fn DyeHead(&self) -> &ColumnData {
&self.columns[48]
}
pub fn DyeBody(&self) -> &ColumnData {
&self.columns[49]
}
pub fn DyeHands(&self) -> &ColumnData {
&self.columns[50]
}
pub fn DyeLegs(&self) -> &ColumnData {
&self.columns[51]
}
pub fn DyeFeet(&self) -> &ColumnData {
&self.columns[52]
}
pub fn DyeEars(&self) -> &ColumnData {
&self.columns[53]
}
pub fn DyeNeck(&self) -> &ColumnData {
&self.columns[54]
}
pub fn DyeWrists(&self) -> &ColumnData {
&self.columns[55]
}
pub fn DyeLeftRing(&self) -> &ColumnData {
&self.columns[56]
}
pub fn DyeRightRing(&self) -> &ColumnData {
&self.columns[57]
}
pub fn Dye2Head(&self) -> &ColumnData {
&self.columns[58]
}
pub fn Dye2Body(&self) -> &ColumnData {
&self.columns[59]
}
pub fn Dye2Hands(&self) -> &ColumnData {
&self.columns[60]
}
pub fn Dye2Legs(&self) -> &ColumnData {
&self.columns[61]
}
pub fn Dye2Feet(&self) -> &ColumnData {
&self.columns[62]
}
pub fn Dye2Ears(&self) -> &ColumnData {
&self.columns[63]
}
pub fn Dye2Neck(&self) -> &ColumnData {
&self.columns[64]
}
pub fn Dye2Wrists(&self) -> &ColumnData {
&self.columns[65]
}
pub fn Dye2LeftRing(&self) -> &ColumnData {
&self.columns[66]
}
pub fn Dye2RightRing(&self) -> &ColumnData {
&self.columns[67]
}
pub fn Unknown40(&self) -> &ColumnData {
&self.columns[68]
}
pub fn Unknown41(&self) -> &ColumnData {
&self.columns[69]
}
pub fn Unknown42(&self) -> &ColumnData {
&self.columns[70]
}
pub fn Unknown43(&self) -> &ColumnData {
&self.columns[71]
}
pub fn Unknown44(&self) -> &ColumnData {
&self.columns[72]
}
}
