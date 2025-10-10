#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct CompanyLeveStructElement<'a> {
BNpcName: &'a ColumnData,
ToDoParam: &'a ColumnData,
BaseID: &'a ColumnData,
ItemsInvolved: &'a ColumnData,
EnemyLevel: &'a ColumnData,
ItemsInvolvedQty: &'a ColumnData,
ItemDropRate: &'a ColumnData,
NumOfAppearance: &'a ColumnData,
}
pub struct CompanyLeveSheet {
pages: Vec<EXD>,
exh: EXH,
row_count: u32,
}
impl CompanyLeveSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "CompanyLeve")?;
let mut pages = Vec::new();
for (i, _) in exh.pages.iter().enumerate() {
pages.push(read_excel_sheet(resource, "CompanyLeve", &exh, language, i)?);
}let row_count = exh.header.row_count;
Some(Self {
exh,
pages,
row_count,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<CompanyLeveRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(CompanyLeveRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<CompanyLeveRow> {
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
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<CompanyLeveRow> {
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
pub struct CompanyLeveRow {
columns: Vec<ColumnData>,
}
impl CompanyLeveRow {
pub fn RoutePointTime(&self) -> [&ColumnData; 8] {
[&self.columns[0],&self.columns[1],&self.columns[2],&self.columns[3],&self.columns[4],&self.columns[5],&self.columns[6],&self.columns[7],]
}
pub fn CompanyLeveStruct<'a>(&'a self) -> [CompanyLeveStructElement<'a>; 8] {
[CompanyLeveStructElement {BNpcName: &self.columns[8],
ToDoParam: &self.columns[9],
BaseID: &self.columns[10],
ItemsInvolved: &self.columns[11],
EnemyLevel: &self.columns[12],
ItemsInvolvedQty: &self.columns[13],
ItemDropRate: &self.columns[14],
NumOfAppearance: &self.columns[15],
},
CompanyLeveStructElement {BNpcName: &self.columns[16],
ToDoParam: &self.columns[17],
BaseID: &self.columns[18],
ItemsInvolved: &self.columns[19],
EnemyLevel: &self.columns[20],
ItemsInvolvedQty: &self.columns[21],
ItemDropRate: &self.columns[22],
NumOfAppearance: &self.columns[23],
},
CompanyLeveStructElement {BNpcName: &self.columns[24],
ToDoParam: &self.columns[25],
BaseID: &self.columns[26],
ItemsInvolved: &self.columns[27],
EnemyLevel: &self.columns[28],
ItemsInvolvedQty: &self.columns[29],
ItemDropRate: &self.columns[30],
NumOfAppearance: &self.columns[31],
},
CompanyLeveStructElement {BNpcName: &self.columns[32],
ToDoParam: &self.columns[33],
BaseID: &self.columns[34],
ItemsInvolved: &self.columns[35],
EnemyLevel: &self.columns[36],
ItemsInvolvedQty: &self.columns[37],
ItemDropRate: &self.columns[38],
NumOfAppearance: &self.columns[39],
},
CompanyLeveStructElement {BNpcName: &self.columns[40],
ToDoParam: &self.columns[41],
BaseID: &self.columns[42],
ItemsInvolved: &self.columns[43],
EnemyLevel: &self.columns[44],
ItemsInvolvedQty: &self.columns[45],
ItemDropRate: &self.columns[46],
NumOfAppearance: &self.columns[47],
},
CompanyLeveStructElement {BNpcName: &self.columns[48],
ToDoParam: &self.columns[49],
BaseID: &self.columns[50],
ItemsInvolved: &self.columns[51],
EnemyLevel: &self.columns[52],
ItemsInvolvedQty: &self.columns[53],
ItemDropRate: &self.columns[54],
NumOfAppearance: &self.columns[55],
},
CompanyLeveStructElement {BNpcName: &self.columns[56],
ToDoParam: &self.columns[57],
BaseID: &self.columns[58],
ItemsInvolved: &self.columns[59],
EnemyLevel: &self.columns[60],
ItemsInvolvedQty: &self.columns[61],
ItemDropRate: &self.columns[62],
NumOfAppearance: &self.columns[63],
},
CompanyLeveStructElement {BNpcName: &self.columns[64],
ToDoParam: &self.columns[65],
BaseID: &self.columns[66],
ItemsInvolved: &self.columns[67],
EnemyLevel: &self.columns[68],
ItemsInvolvedQty: &self.columns[69],
ItemDropRate: &self.columns[70],
NumOfAppearance: &self.columns[71],
},
]
}
pub fn ToDoSequence(&self) -> [&ColumnData; 8] {
[&self.columns[72],&self.columns[73],&self.columns[74],&self.columns[75],&self.columns[76],&self.columns[77],&self.columns[78],&self.columns[79],]
}
pub fn Rule(&self) -> &ColumnData {
&self.columns[80]
}
pub fn RuleParam(&self) -> &ColumnData {
&self.columns[81]
}
}
