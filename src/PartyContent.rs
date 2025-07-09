#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct PartyContentSheet {
exd: EXD,
exh: EXH,
}
impl PartyContentSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "PartyContent")?;let exd = read_excel_sheet(resource, "PartyContent", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<PartyContentRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(PartyContentRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<PartyContentRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<PartyContentRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct PartyContentRow {
columns: Vec<ColumnData>,
}
impl PartyContentRow {
pub fn LGBEventObject(&self) -> [&ColumnData; 9] {
[&self.columns[0],&self.columns[1],&self.columns[2],&self.columns[3],&self.columns[4],&self.columns[5],&self.columns[6],&self.columns[7],&self.columns[8],]
}
pub fn LGBEventRange(&self) -> [&ColumnData; 9] {
[&self.columns[9],&self.columns[10],&self.columns[11],&self.columns[12],&self.columns[13],&self.columns[14],&self.columns[15],&self.columns[16],&self.columns[17],]
}
pub fn LGBEventObject2(&self) -> [&ColumnData; 9] {
[&self.columns[18],&self.columns[19],&self.columns[20],&self.columns[21],&self.columns[22],&self.columns[23],&self.columns[24],&self.columns[25],&self.columns[26],]
}
pub fn TextDataStart(&self) -> &ColumnData {
&self.columns[27]
}
pub fn TextDataEnd(&self) -> &ColumnData {
&self.columns[28]
}
pub fn Image(&self) -> &ColumnData {
&self.columns[29]
}
pub fn TimeLimit(&self) -> &ColumnData {
&self.columns[30]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[31]
}
pub fn ContentFinderCondition(&self) -> &ColumnData {
&self.columns[32]
}
pub fn Key(&self) -> &ColumnData {
&self.columns[33]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[34]
}
pub fn Name(&self) -> &ColumnData {
&self.columns[35]
}
}
