#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct BNpcPartsSheet {
pages: Vec<EXD>,
exh: EXH,
row_count: u32,
}
impl BNpcPartsSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "BNpcParts")?;
let mut pages = Vec::new();
for (i, _) in exh.pages.iter().enumerate() {
pages.push(read_excel_sheet(resource, "BNpcParts", &exh, language, i)?);
}let row_count = exh.header.row_count;
Some(Self {
exh,
pages,
row_count,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<BNpcPartsRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(BNpcPartsRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<BNpcPartsRow> {
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
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<BNpcPartsRow> {
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
pub struct BNpcPartsRow {
columns: Vec<ColumnData>,
}
impl BNpcPartsRow {
pub fn X1(&self) -> &ColumnData {
&self.columns[0]
}
pub fn X2(&self) -> &ColumnData {
&self.columns[1]
}
pub fn X3(&self) -> &ColumnData {
&self.columns[2]
}
pub fn X4(&self) -> &ColumnData {
&self.columns[3]
}
pub fn X5(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Y1(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Y2(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Y3(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Y4(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Y5(&self) -> &ColumnData {
&self.columns[10]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[11]
}
pub fn Z1(&self) -> &ColumnData {
&self.columns[12]
}
pub fn Z2(&self) -> &ColumnData {
&self.columns[13]
}
pub fn Z3(&self) -> &ColumnData {
&self.columns[14]
}
pub fn Z4(&self) -> &ColumnData {
&self.columns[15]
}
pub fn Z5(&self) -> &ColumnData {
&self.columns[16]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[17]
}
pub fn Scale1(&self) -> &ColumnData {
&self.columns[18]
}
pub fn Scale2(&self) -> &ColumnData {
&self.columns[19]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[20]
}
pub fn Scale4(&self) -> &ColumnData {
&self.columns[21]
}
pub fn Scale5(&self) -> &ColumnData {
&self.columns[22]
}
pub fn Unknown4(&self) -> &ColumnData {
&self.columns[23]
}
pub fn BNpcBase1(&self) -> &ColumnData {
&self.columns[24]
}
pub fn BNpcBase2(&self) -> &ColumnData {
&self.columns[25]
}
pub fn BNpcBase3(&self) -> &ColumnData {
&self.columns[26]
}
pub fn BNpcBase4(&self) -> &ColumnData {
&self.columns[27]
}
pub fn BNpcBase5(&self) -> &ColumnData {
&self.columns[28]
}
pub fn Unknown5(&self) -> &ColumnData {
&self.columns[29]
}
pub fn Unknown6(&self) -> &ColumnData {
&self.columns[30]
}
pub fn Unknown7(&self) -> &ColumnData {
&self.columns[31]
}
pub fn Scale3(&self) -> &ColumnData {
&self.columns[32]
}
pub fn Unknown8(&self) -> &ColumnData {
&self.columns[33]
}
pub fn Unknown9(&self) -> &ColumnData {
&self.columns[34]
}
pub fn Unknown10(&self) -> &ColumnData {
&self.columns[35]
}
pub fn PartSlot1(&self) -> &ColumnData {
&self.columns[36]
}
pub fn PartSlot2(&self) -> &ColumnData {
&self.columns[37]
}
pub fn PartSlot3(&self) -> &ColumnData {
&self.columns[38]
}
pub fn PartSlot4(&self) -> &ColumnData {
&self.columns[39]
}
pub fn PartSlot5(&self) -> &ColumnData {
&self.columns[40]
}
pub fn Unknown11(&self) -> &ColumnData {
&self.columns[41]
}
pub fn Unknown12(&self) -> &ColumnData {
&self.columns[42]
}
pub fn Unknown13(&self) -> &ColumnData {
&self.columns[43]
}
pub fn Unknown14(&self) -> &ColumnData {
&self.columns[44]
}
pub fn Unknown15(&self) -> &ColumnData {
&self.columns[45]
}
pub fn Unknown16(&self) -> &ColumnData {
&self.columns[46]
}
pub fn Unknown17(&self) -> &ColumnData {
&self.columns[47]
}
pub fn Unknown18(&self) -> &ColumnData {
&self.columns[48]
}
pub fn Unknown19(&self) -> &ColumnData {
&self.columns[49]
}
pub fn Unknown20(&self) -> &ColumnData {
&self.columns[50]
}
pub fn Unknown21(&self) -> &ColumnData {
&self.columns[51]
}
pub fn Unknown22(&self) -> &ColumnData {
&self.columns[52]
}
pub fn Unknown23(&self) -> &ColumnData {
&self.columns[53]
}
pub fn Unknown24(&self) -> &ColumnData {
&self.columns[54]
}
pub fn Unknown25(&self) -> &ColumnData {
&self.columns[55]
}
pub fn Unknown26(&self) -> &ColumnData {
&self.columns[56]
}
pub fn Unknown27(&self) -> &ColumnData {
&self.columns[57]
}
pub fn Unknown28(&self) -> &ColumnData {
&self.columns[58]
}
pub fn Unknown29(&self) -> &ColumnData {
&self.columns[59]
}
pub fn Unknown30(&self) -> &ColumnData {
&self.columns[60]
}
pub fn Unknown31(&self) -> &ColumnData {
&self.columns[61]
}
pub fn Unknown32(&self) -> &ColumnData {
&self.columns[62]
}
pub fn Unknown33(&self) -> &ColumnData {
&self.columns[63]
}
pub fn Unknown34(&self) -> &ColumnData {
&self.columns[64]
}
pub fn Unknown35(&self) -> &ColumnData {
&self.columns[65]
}
pub fn Unknown36(&self) -> &ColumnData {
&self.columns[66]
}
}
