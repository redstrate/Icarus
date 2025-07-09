#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct GathererCrafterLvAdjustTableSheet {
exd: EXD,
exh: EXH,
}
impl GathererCrafterLvAdjustTableSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "GathererCrafterLvAdjustTable")?;let exd = read_excel_sheet(resource, "GathererCrafterLvAdjustTable", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<GathererCrafterLvAdjustTableRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(GathererCrafterLvAdjustTableRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<GathererCrafterLvAdjustTableRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<GathererCrafterLvAdjustTableRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct GathererCrafterLvAdjustTableRow {
columns: Vec<ColumnData>,
}
impl GathererCrafterLvAdjustTableRow {
pub fn RecipeLevel(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[1]
}
pub fn CrafterLevel(&self) -> &ColumnData {
&self.columns[2]
}
pub fn GathererLevel(&self) -> &ColumnData {
&self.columns[3]
}
pub fn FisherLevel(&self) -> &ColumnData {
&self.columns[4]
}
}
