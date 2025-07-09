#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct AozActionTransientSheet {
exd: EXD,
exh: EXH,
}
impl AozActionTransientSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "AozActionTransient")?;let exd = read_excel_sheet(resource, "AozActionTransient", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<AozActionTransientRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(AozActionTransientRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<AozActionTransientRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<AozActionTransientRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct AozActionTransientRow {
columns: Vec<ColumnData>,
}
impl AozActionTransientRow {
pub fn Stats(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Description(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[2]
}
pub fn RequiredForQuest(&self) -> &ColumnData {
&self.columns[3]
}
pub fn PreviousQuest(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Location(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Number(&self) -> &ColumnData {
&self.columns[6]
}
pub fn LocationKey(&self) -> &ColumnData {
&self.columns[7]
}
pub fn TargetsEnemy(&self) -> &ColumnData {
&self.columns[8]
}
pub fn TargetsSelfOrAlly(&self) -> &ColumnData {
&self.columns[9]
}
pub fn CauseSlow(&self) -> &ColumnData {
&self.columns[10]
}
pub fn CausePetrify(&self) -> &ColumnData {
&self.columns[11]
}
pub fn CauseParalysis(&self) -> &ColumnData {
&self.columns[12]
}
pub fn CauseInterrupt(&self) -> &ColumnData {
&self.columns[13]
}
pub fn CauseBlind(&self) -> &ColumnData {
&self.columns[14]
}
pub fn CauseStun(&self) -> &ColumnData {
&self.columns[15]
}
pub fn CauseSleep(&self) -> &ColumnData {
&self.columns[16]
}
pub fn CauseBind(&self) -> &ColumnData {
&self.columns[17]
}
pub fn CauseHeavy(&self) -> &ColumnData {
&self.columns[18]
}
pub fn CauseDeath(&self) -> &ColumnData {
&self.columns[19]
}
}
