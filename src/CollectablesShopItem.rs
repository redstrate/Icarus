#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct CollectablesShopItemSheet {
exd: EXD,
exh: EXH,
}
impl CollectablesShopItemSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "CollectablesShopItem")?;let exd = read_excel_sheet(resource, "CollectablesShopItem", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<CollectablesShopItemRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(CollectablesShopItemRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<CollectablesShopItemRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<CollectablesShopItemRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct CollectablesShopItemRow {
columns: Vec<ColumnData>,
}
impl CollectablesShopItemRow {
pub fn Item(&self) -> &ColumnData {
&self.columns[0]
}
pub fn RequiredQuest(&self) -> &ColumnData {
&self.columns[1]
}
pub fn LevelMin(&self) -> &ColumnData {
&self.columns[2]
}
pub fn LevelMax(&self) -> &ColumnData {
&self.columns[3]
}
pub fn CollectablesShopRefine(&self) -> &ColumnData {
&self.columns[4]
}
pub fn CollectablesShopRewardScrip(&self) -> &ColumnData {
&self.columns[5]
}
pub fn CollectablesShopItemGroup(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Stars(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Key(&self) -> &ColumnData {
&self.columns[8]
}
}
