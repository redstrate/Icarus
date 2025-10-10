#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct StagesElement<'a> {
Unknown0: &'a ColumnData,
Item: &'a ColumnData,
Name: &'a ColumnData,
}
pub struct TypesElement<'a> {
Icon: &'a ColumnData,
Name: &'a ColumnData,
CosmicName: &'a ColumnData,
}
pub struct WKSCosmoToolClassSheet {
pages: Vec<EXD>,
exh: EXH,
row_count: u32,
}
impl WKSCosmoToolClassSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "WKSCosmoToolClass")?;
let mut pages = Vec::new();
for (i, _) in exh.pages.iter().enumerate() {
pages.push(read_excel_sheet(resource, "WKSCosmoToolClass", &exh, language, i)?);
}let row_count = exh.header.row_count;
Some(Self {
exh,
pages,
row_count,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<WKSCosmoToolClassRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(WKSCosmoToolClassRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<WKSCosmoToolClassRow> {
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
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<WKSCosmoToolClassRow> {
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
pub struct WKSCosmoToolClassRow {
columns: Vec<ColumnData>,
}
impl WKSCosmoToolClassRow {
pub fn Stages<'a>(&'a self) -> [StagesElement<'a>; 14] {
[StagesElement {Unknown0: &self.columns[0],
Item: &self.columns[1],
Name: &self.columns[2],
},
StagesElement {Unknown0: &self.columns[3],
Item: &self.columns[4],
Name: &self.columns[5],
},
StagesElement {Unknown0: &self.columns[6],
Item: &self.columns[7],
Name: &self.columns[8],
},
StagesElement {Unknown0: &self.columns[9],
Item: &self.columns[10],
Name: &self.columns[11],
},
StagesElement {Unknown0: &self.columns[12],
Item: &self.columns[13],
Name: &self.columns[14],
},
StagesElement {Unknown0: &self.columns[15],
Item: &self.columns[16],
Name: &self.columns[17],
},
StagesElement {Unknown0: &self.columns[18],
Item: &self.columns[19],
Name: &self.columns[20],
},
StagesElement {Unknown0: &self.columns[21],
Item: &self.columns[22],
Name: &self.columns[23],
},
StagesElement {Unknown0: &self.columns[24],
Item: &self.columns[25],
Name: &self.columns[26],
},
StagesElement {Unknown0: &self.columns[27],
Item: &self.columns[28],
Name: &self.columns[29],
},
StagesElement {Unknown0: &self.columns[30],
Item: &self.columns[31],
Name: &self.columns[32],
},
StagesElement {Unknown0: &self.columns[33],
Item: &self.columns[34],
Name: &self.columns[35],
},
StagesElement {Unknown0: &self.columns[36],
Item: &self.columns[37],
Name: &self.columns[38],
},
StagesElement {Unknown0: &self.columns[39],
Item: &self.columns[40],
Name: &self.columns[41],
},
]
}
pub fn Types<'a>(&'a self) -> [TypesElement<'a>; 5] {
[TypesElement {Icon: &self.columns[42],
Name: &self.columns[43],
CosmicName: &self.columns[44],
},
TypesElement {Icon: &self.columns[45],
Name: &self.columns[46],
CosmicName: &self.columns[47],
},
TypesElement {Icon: &self.columns[48],
Name: &self.columns[49],
CosmicName: &self.columns[50],
},
TypesElement {Icon: &self.columns[51],
Name: &self.columns[52],
CosmicName: &self.columns[53],
},
TypesElement {Icon: &self.columns[54],
Name: &self.columns[55],
CosmicName: &self.columns[56],
},
]
}
pub fn Name(&self) -> &ColumnData {
&self.columns[57]
}
pub fn DataAmount(&self) -> &ColumnData {
&self.columns[58]
}
}
