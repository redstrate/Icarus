#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct IndividualWeatherDataElement<'a> {
Quest: &'a ColumnData,
Unknown0: &'a ColumnData,
Weather: &'a ColumnData,
Unknown1: &'a ColumnData,
}
pub struct IndividualWeatherSheet {
pages: Vec<EXD>,
exh: EXH,
row_count: u32,
}
impl IndividualWeatherSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "IndividualWeather")?;
let mut pages = Vec::new();
for (i, _) in exh.pages.iter().enumerate() {
pages.push(read_excel_sheet(resource, "IndividualWeather", &exh, language, i)?);
}let row_count = exh.header.row_count;
Some(Self {
exh,
pages,
row_count,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<IndividualWeatherRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(IndividualWeatherRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<IndividualWeatherRow> {
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
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<IndividualWeatherRow> {
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
pub struct IndividualWeatherRow {
columns: Vec<ColumnData>,
}
impl IndividualWeatherRow {
pub fn IndividualWeatherData<'a>(&'a self) -> [IndividualWeatherDataElement<'a>; 7] {
[IndividualWeatherDataElement {Quest: &self.columns[0],
Unknown0: &self.columns[1],
Weather: &self.columns[2],
Unknown1: &self.columns[3],
},
IndividualWeatherDataElement {Quest: &self.columns[4],
Unknown0: &self.columns[5],
Weather: &self.columns[6],
Unknown1: &self.columns[7],
},
IndividualWeatherDataElement {Quest: &self.columns[8],
Unknown0: &self.columns[9],
Weather: &self.columns[10],
Unknown1: &self.columns[11],
},
IndividualWeatherDataElement {Quest: &self.columns[12],
Unknown0: &self.columns[13],
Weather: &self.columns[14],
Unknown1: &self.columns[15],
},
IndividualWeatherDataElement {Quest: &self.columns[16],
Unknown0: &self.columns[17],
Weather: &self.columns[18],
Unknown1: &self.columns[19],
},
IndividualWeatherDataElement {Quest: &self.columns[20],
Unknown0: &self.columns[21],
Weather: &self.columns[22],
Unknown1: &self.columns[23],
},
IndividualWeatherDataElement {Quest: &self.columns[24],
Unknown0: &self.columns[25],
Weather: &self.columns[26],
Unknown1: &self.columns[27],
},
]
}
}
