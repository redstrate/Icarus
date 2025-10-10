#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct EventParametersElement<'a> {
Gesture: &'a ColumnData,
LipSync: &'a ColumnData,
Facial: &'a ColumnData,
Shape: &'a ColumnData,
Turn: &'a ColumnData,
WidgetType: &'a ColumnData,
IsAutoShake: &'a ColumnData,
}
pub struct FateEventSheet {
pages: Vec<EXD>,
exh: EXH,
row_count: u32,
}
impl FateEventSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "FateEvent")?;
let mut pages = Vec::new();
for (i, _) in exh.pages.iter().enumerate() {
pages.push(read_excel_sheet(resource, "FateEvent", &exh, language, i)?);
}let row_count = exh.header.row_count;
Some(Self {
exh,
pages,
row_count,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<FateEventRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(FateEventRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<FateEventRow> {
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
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<FateEventRow> {
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
pub struct FateEventRow {
columns: Vec<ColumnData>,
}
impl FateEventRow {
pub fn EventParameters<'a>(&'a self) -> [EventParametersElement<'a>; 8] {
[EventParametersElement {Gesture: &self.columns[0],
LipSync: &self.columns[1],
Facial: &self.columns[2],
Shape: &self.columns[3],
Turn: &self.columns[4],
WidgetType: &self.columns[5],
IsAutoShake: &self.columns[6],
},
EventParametersElement {Gesture: &self.columns[7],
LipSync: &self.columns[8],
Facial: &self.columns[9],
Shape: &self.columns[10],
Turn: &self.columns[11],
WidgetType: &self.columns[12],
IsAutoShake: &self.columns[13],
},
EventParametersElement {Gesture: &self.columns[14],
LipSync: &self.columns[15],
Facial: &self.columns[16],
Shape: &self.columns[17],
Turn: &self.columns[18],
WidgetType: &self.columns[19],
IsAutoShake: &self.columns[20],
},
EventParametersElement {Gesture: &self.columns[21],
LipSync: &self.columns[22],
Facial: &self.columns[23],
Shape: &self.columns[24],
Turn: &self.columns[25],
WidgetType: &self.columns[26],
IsAutoShake: &self.columns[27],
},
EventParametersElement {Gesture: &self.columns[28],
LipSync: &self.columns[29],
Facial: &self.columns[30],
Shape: &self.columns[31],
Turn: &self.columns[32],
WidgetType: &self.columns[33],
IsAutoShake: &self.columns[34],
},
EventParametersElement {Gesture: &self.columns[35],
LipSync: &self.columns[36],
Facial: &self.columns[37],
Shape: &self.columns[38],
Turn: &self.columns[39],
WidgetType: &self.columns[40],
IsAutoShake: &self.columns[41],
},
EventParametersElement {Gesture: &self.columns[42],
LipSync: &self.columns[43],
Facial: &self.columns[44],
Shape: &self.columns[45],
Turn: &self.columns[46],
WidgetType: &self.columns[47],
IsAutoShake: &self.columns[48],
},
EventParametersElement {Gesture: &self.columns[49],
LipSync: &self.columns[50],
Facial: &self.columns[51],
Shape: &self.columns[52],
Turn: &self.columns[53],
WidgetType: &self.columns[54],
IsAutoShake: &self.columns[55],
},
]
}
pub fn Text(&self) -> [&ColumnData; 8] {
[&self.columns[56],&self.columns[57],&self.columns[58],&self.columns[59],&self.columns[60],&self.columns[61],&self.columns[62],&self.columns[63],]
}
}
