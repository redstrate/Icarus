#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct RelaysElement<'a> {
EnterTerritory: &'a ColumnData,
ExitTerritory: &'a ColumnData,
Cost: &'a ColumnData,
}
pub struct TelepoRelaySheet {
pages: Vec<EXD>,
exh: EXH,
row_count: u32,
}
impl TelepoRelaySheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "TelepoRelay")?;
let mut pages = Vec::new();
for (i, _) in exh.pages.iter().enumerate() {
pages.push(read_excel_sheet(resource, "TelepoRelay", &exh, language, i)?);
}let row_count = exh.header.row_count;
Some(Self {
exh,
pages,
row_count,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<TelepoRelayRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(TelepoRelayRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<TelepoRelayRow> {
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
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<TelepoRelayRow> {
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
pub struct TelepoRelayRow {
columns: Vec<ColumnData>,
}
impl TelepoRelayRow {
pub fn Relays<'a>(&'a self) -> [RelaysElement<'a>; 9] {
[RelaysElement {EnterTerritory: &self.columns[0],
ExitTerritory: &self.columns[1],
Cost: &self.columns[2],
},
RelaysElement {EnterTerritory: &self.columns[3],
ExitTerritory: &self.columns[4],
Cost: &self.columns[5],
},
RelaysElement {EnterTerritory: &self.columns[6],
ExitTerritory: &self.columns[7],
Cost: &self.columns[8],
},
RelaysElement {EnterTerritory: &self.columns[9],
ExitTerritory: &self.columns[10],
Cost: &self.columns[11],
},
RelaysElement {EnterTerritory: &self.columns[12],
ExitTerritory: &self.columns[13],
Cost: &self.columns[14],
},
RelaysElement {EnterTerritory: &self.columns[15],
ExitTerritory: &self.columns[16],
Cost: &self.columns[17],
},
RelaysElement {EnterTerritory: &self.columns[18],
ExitTerritory: &self.columns[19],
Cost: &self.columns[20],
},
RelaysElement {EnterTerritory: &self.columns[21],
ExitTerritory: &self.columns[22],
Cost: &self.columns[23],
},
RelaysElement {EnterTerritory: &self.columns[24],
ExitTerritory: &self.columns[25],
Cost: &self.columns[26],
},
]
}
pub fn Unknown_70(&self) -> &ColumnData {
&self.columns[27]
}
}
