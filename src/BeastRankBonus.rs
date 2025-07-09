#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct BeastRankBonusSheet {
exd: EXD,
exh: EXH,
}
impl BeastRankBonusSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "BeastRankBonus")?;let exd = read_excel_sheet(resource, "BeastRankBonus", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<BeastRankBonusRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(BeastRankBonusRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<BeastRankBonusRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<BeastRankBonusRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct BeastRankBonusRow {
columns: Vec<ColumnData>,
}
impl BeastRankBonusRow {
pub fn Item(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Neutral(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Recognized(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Friendly(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Trusted(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Respected(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Honored(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Sworn(&self) -> &ColumnData {
&self.columns[7]
}
pub fn AlliedBloodsworn(&self) -> &ColumnData {
&self.columns[8]
}
pub fn ItemQuantity(&self) -> [&ColumnData; 8] {
[&self.columns[9],&self.columns[10],&self.columns[11],&self.columns[12],&self.columns[13],&self.columns[14],&self.columns[15],&self.columns[16],]
}
}
