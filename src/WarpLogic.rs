#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct WarpParamsElement<'a> {
Function: &'a ColumnData,
Argument: &'a ColumnData,
}
pub struct WarpLogicSheet {
pages: Vec<EXD>,
exh: EXH,
row_count: u32,
}
impl WarpLogicSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "WarpLogic")?;
let mut pages = Vec::new();
for (i, _) in exh.pages.iter().enumerate() {
pages.push(read_excel_sheet(resource, "WarpLogic", &exh, language, i)?);
}let row_count = exh.header.row_count;
Some(Self {
exh,
pages,
row_count,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<WarpLogicRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(WarpLogicRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<WarpLogicRow> {
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
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<WarpLogicRow> {
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
pub struct WarpLogicRow {
columns: Vec<ColumnData>,
}
impl WarpLogicRow {
pub fn WarpParams<'a>(&'a self) -> [WarpParamsElement<'a>; 10] {
[WarpParamsElement {Function: &self.columns[0],
Argument: &self.columns[1],
},
WarpParamsElement {Function: &self.columns[2],
Argument: &self.columns[3],
},
WarpParamsElement {Function: &self.columns[4],
Argument: &self.columns[5],
},
WarpParamsElement {Function: &self.columns[6],
Argument: &self.columns[7],
},
WarpParamsElement {Function: &self.columns[8],
Argument: &self.columns[9],
},
WarpParamsElement {Function: &self.columns[10],
Argument: &self.columns[11],
},
WarpParamsElement {Function: &self.columns[12],
Argument: &self.columns[13],
},
WarpParamsElement {Function: &self.columns[14],
Argument: &self.columns[15],
},
WarpParamsElement {Function: &self.columns[16],
Argument: &self.columns[17],
},
WarpParamsElement {Function: &self.columns[18],
Argument: &self.columns[19],
},
]
}
pub fn Question(&self) -> &ColumnData {
&self.columns[20]
}
pub fn ResponseYes(&self) -> &ColumnData {
&self.columns[21]
}
pub fn ResponseNo(&self) -> &ColumnData {
&self.columns[22]
}
pub fn WarpName(&self) -> &ColumnData {
&self.columns[23]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[24]
}
pub fn CanSkipCutscene(&self) -> &ColumnData {
&self.columns[25]
}
}
