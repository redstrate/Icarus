#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct ClassJobCategorySheet {
exd: EXD,
exh: EXH,
}
impl ClassJobCategorySheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "ClassJobCategory")?;let exd = read_excel_sheet(resource, "ClassJobCategory", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<ClassJobCategoryRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(ClassJobCategoryRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<ClassJobCategoryRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<ClassJobCategoryRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct ClassJobCategoryRow {
columns: Vec<ColumnData>,
}
impl ClassJobCategoryRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn ADV(&self) -> &ColumnData {
&self.columns[1]
}
pub fn GLA(&self) -> &ColumnData {
&self.columns[2]
}
pub fn PGL(&self) -> &ColumnData {
&self.columns[3]
}
pub fn MRD(&self) -> &ColumnData {
&self.columns[4]
}
pub fn LNC(&self) -> &ColumnData {
&self.columns[5]
}
pub fn ARC(&self) -> &ColumnData {
&self.columns[6]
}
pub fn CNJ(&self) -> &ColumnData {
&self.columns[7]
}
pub fn THM(&self) -> &ColumnData {
&self.columns[8]
}
pub fn CRP(&self) -> &ColumnData {
&self.columns[9]
}
pub fn BSM(&self) -> &ColumnData {
&self.columns[10]
}
pub fn ARM(&self) -> &ColumnData {
&self.columns[11]
}
pub fn GSM(&self) -> &ColumnData {
&self.columns[12]
}
pub fn LTW(&self) -> &ColumnData {
&self.columns[13]
}
pub fn WVR(&self) -> &ColumnData {
&self.columns[14]
}
pub fn ALC(&self) -> &ColumnData {
&self.columns[15]
}
pub fn CUL(&self) -> &ColumnData {
&self.columns[16]
}
pub fn MIN(&self) -> &ColumnData {
&self.columns[17]
}
pub fn BTN(&self) -> &ColumnData {
&self.columns[18]
}
pub fn FSH(&self) -> &ColumnData {
&self.columns[19]
}
pub fn PLD(&self) -> &ColumnData {
&self.columns[20]
}
pub fn MNK(&self) -> &ColumnData {
&self.columns[21]
}
pub fn WAR(&self) -> &ColumnData {
&self.columns[22]
}
pub fn DRG(&self) -> &ColumnData {
&self.columns[23]
}
pub fn BRD(&self) -> &ColumnData {
&self.columns[24]
}
pub fn WHM(&self) -> &ColumnData {
&self.columns[25]
}
pub fn BLM(&self) -> &ColumnData {
&self.columns[26]
}
pub fn ACN(&self) -> &ColumnData {
&self.columns[27]
}
pub fn SMN(&self) -> &ColumnData {
&self.columns[28]
}
pub fn SCH(&self) -> &ColumnData {
&self.columns[29]
}
pub fn ROG(&self) -> &ColumnData {
&self.columns[30]
}
pub fn NIN(&self) -> &ColumnData {
&self.columns[31]
}
pub fn MCH(&self) -> &ColumnData {
&self.columns[32]
}
pub fn DRK(&self) -> &ColumnData {
&self.columns[33]
}
pub fn AST(&self) -> &ColumnData {
&self.columns[34]
}
pub fn SAM(&self) -> &ColumnData {
&self.columns[35]
}
pub fn RDM(&self) -> &ColumnData {
&self.columns[36]
}
pub fn BLU(&self) -> &ColumnData {
&self.columns[37]
}
pub fn GNB(&self) -> &ColumnData {
&self.columns[38]
}
pub fn DNC(&self) -> &ColumnData {
&self.columns[39]
}
pub fn RPR(&self) -> &ColumnData {
&self.columns[40]
}
pub fn SGE(&self) -> &ColumnData {
&self.columns[41]
}
pub fn VPR(&self) -> &ColumnData {
&self.columns[42]
}
pub fn PCT(&self) -> &ColumnData {
&self.columns[43]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[44]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[45]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[46]
}
}
