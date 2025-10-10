#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct OvooDataElement<'a> {
EmptyIcon: &'a ColumnData,
MaelstromIcon: &'a ColumnData,
TwinAdderIcon: &'a ColumnData,
ImmortalFlamesIcon: &'a ColumnData,
Unknown0: &'a ColumnData,
Unknown1: &'a ColumnData,
Unknown2: &'a ColumnData,
}
pub struct Frontline03Sheet {
pages: Vec<EXD>,
exh: EXH,
row_count: u32,
}
impl Frontline03Sheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "Frontline03")?;
let mut pages = Vec::new();
for (i, _) in exh.pages.iter().enumerate() {
pages.push(read_excel_sheet(resource, "Frontline03", &exh, language, i)?);
}let row_count = exh.header.row_count;
Some(Self {
exh,
pages,
row_count,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<Frontline03Row> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(Frontline03Row { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<Frontline03Row> {
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
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<Frontline03Row> {
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
pub struct Frontline03Row {
columns: Vec<ColumnData>,
}
impl Frontline03Row {
pub fn OvooData<'a>(&'a self) -> [OvooDataElement<'a>; 3] {
[OvooDataElement {EmptyIcon: &self.columns[0],
MaelstromIcon: &self.columns[1],
TwinAdderIcon: &self.columns[2],
ImmortalFlamesIcon: &self.columns[3],
Unknown0: &self.columns[4],
Unknown1: &self.columns[5],
Unknown2: &self.columns[6],
},
OvooDataElement {EmptyIcon: &self.columns[7],
MaelstromIcon: &self.columns[8],
TwinAdderIcon: &self.columns[9],
ImmortalFlamesIcon: &self.columns[10],
Unknown0: &self.columns[11],
Unknown1: &self.columns[12],
Unknown2: &self.columns[13],
},
OvooDataElement {EmptyIcon: &self.columns[14],
MaelstromIcon: &self.columns[15],
TwinAdderIcon: &self.columns[16],
ImmortalFlamesIcon: &self.columns[17],
Unknown0: &self.columns[18],
Unknown1: &self.columns[19],
Unknown2: &self.columns[20],
},
]
}
}
