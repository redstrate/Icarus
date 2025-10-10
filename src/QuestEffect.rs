#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct UnknownStructElement<'a> {
Unknown1: &'a ColumnData,
Unknown_70: &'a ColumnData,
Unknown2: &'a ColumnData,
}
pub struct QuestEffectSheet {
pages: Vec<EXD>,
exh: EXH,
row_count: u32,
}
impl QuestEffectSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "QuestEffect")?;
let mut pages = Vec::new();
for (i, _) in exh.pages.iter().enumerate() {
pages.push(read_excel_sheet(resource, "QuestEffect", &exh, language, i)?);
}let row_count = exh.header.row_count;
Some(Self {
exh,
pages,
row_count,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<QuestEffectRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(QuestEffectRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<QuestEffectRow> {
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
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<QuestEffectRow> {
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
pub struct QuestEffectRow {
columns: Vec<ColumnData>,
}
impl QuestEffectRow {
pub fn UnknownStruct<'a>(&'a self) -> [UnknownStructElement<'a>; 4] {
[UnknownStructElement {Unknown1: &self.columns[0],
Unknown_70: &self.columns[1],
Unknown2: &self.columns[2],
},
UnknownStructElement {Unknown1: &self.columns[3],
Unknown_70: &self.columns[4],
Unknown2: &self.columns[5],
},
UnknownStructElement {Unknown1: &self.columns[6],
Unknown_70: &self.columns[7],
Unknown2: &self.columns[8],
},
UnknownStructElement {Unknown1: &self.columns[9],
Unknown_70: &self.columns[10],
Unknown2: &self.columns[11],
},
]
}
pub fn Unknown8(&self) -> &ColumnData {
&self.columns[12]
}
pub fn Unknown9(&self) -> &ColumnData {
&self.columns[13]
}
pub fn Unknown_70(&self) -> &ColumnData {
&self.columns[14]
}
}
