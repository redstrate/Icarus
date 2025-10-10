#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct LeveDataElement<'a> {
BNpcName: &'a ColumnData,
ToDoNumberInvolved: &'a ColumnData,
ToDoParam: &'a ColumnData,
BaseID: &'a ColumnData,
ItemsInvolved: &'a ColumnData,
EnemyLevel: &'a ColumnData,
ItemsInvolvedQty: &'a ColumnData,
ItemDropRate: &'a ColumnData,
NumOfAppearance: &'a ColumnData,
}
pub struct BattleLeveSheet {
pages: Vec<EXD>,
exh: EXH,
row_count: u32,
}
impl BattleLeveSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "BattleLeve")?;
let mut pages = Vec::new();
for (i, _) in exh.pages.iter().enumerate() {
pages.push(read_excel_sheet(resource, "BattleLeve", &exh, language, i)?);
}let row_count = exh.header.row_count;
Some(Self {
exh,
pages,
row_count,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<BattleLeveRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(BattleLeveRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<BattleLeveRow> {
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
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<BattleLeveRow> {
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
pub struct BattleLeveRow {
columns: Vec<ColumnData>,
}
impl BattleLeveRow {
pub fn Time(&self) -> [&ColumnData; 8] {
[&self.columns[0],&self.columns[1],&self.columns[2],&self.columns[3],&self.columns[4],&self.columns[5],&self.columns[6],&self.columns[7],]
}
pub fn LeveData<'a>(&'a self) -> [LeveDataElement<'a>; 8] {
[LeveDataElement {BNpcName: &self.columns[8],
ToDoNumberInvolved: &self.columns[9],
ToDoParam: &self.columns[10],
BaseID: &self.columns[11],
ItemsInvolved: &self.columns[12],
EnemyLevel: &self.columns[13],
ItemsInvolvedQty: &self.columns[14],
ItemDropRate: &self.columns[15],
NumOfAppearance: &self.columns[16],
},
LeveDataElement {BNpcName: &self.columns[17],
ToDoNumberInvolved: &self.columns[18],
ToDoParam: &self.columns[19],
BaseID: &self.columns[20],
ItemsInvolved: &self.columns[21],
EnemyLevel: &self.columns[22],
ItemsInvolvedQty: &self.columns[23],
ItemDropRate: &self.columns[24],
NumOfAppearance: &self.columns[25],
},
LeveDataElement {BNpcName: &self.columns[26],
ToDoNumberInvolved: &self.columns[27],
ToDoParam: &self.columns[28],
BaseID: &self.columns[29],
ItemsInvolved: &self.columns[30],
EnemyLevel: &self.columns[31],
ItemsInvolvedQty: &self.columns[32],
ItemDropRate: &self.columns[33],
NumOfAppearance: &self.columns[34],
},
LeveDataElement {BNpcName: &self.columns[35],
ToDoNumberInvolved: &self.columns[36],
ToDoParam: &self.columns[37],
BaseID: &self.columns[38],
ItemsInvolved: &self.columns[39],
EnemyLevel: &self.columns[40],
ItemsInvolvedQty: &self.columns[41],
ItemDropRate: &self.columns[42],
NumOfAppearance: &self.columns[43],
},
LeveDataElement {BNpcName: &self.columns[44],
ToDoNumberInvolved: &self.columns[45],
ToDoParam: &self.columns[46],
BaseID: &self.columns[47],
ItemsInvolved: &self.columns[48],
EnemyLevel: &self.columns[49],
ItemsInvolvedQty: &self.columns[50],
ItemDropRate: &self.columns[51],
NumOfAppearance: &self.columns[52],
},
LeveDataElement {BNpcName: &self.columns[53],
ToDoNumberInvolved: &self.columns[54],
ToDoParam: &self.columns[55],
BaseID: &self.columns[56],
ItemsInvolved: &self.columns[57],
EnemyLevel: &self.columns[58],
ItemsInvolvedQty: &self.columns[59],
ItemDropRate: &self.columns[60],
NumOfAppearance: &self.columns[61],
},
LeveDataElement {BNpcName: &self.columns[62],
ToDoNumberInvolved: &self.columns[63],
ToDoParam: &self.columns[64],
BaseID: &self.columns[65],
ItemsInvolved: &self.columns[66],
EnemyLevel: &self.columns[67],
ItemsInvolvedQty: &self.columns[68],
ItemDropRate: &self.columns[69],
NumOfAppearance: &self.columns[70],
},
LeveDataElement {BNpcName: &self.columns[71],
ToDoNumberInvolved: &self.columns[72],
ToDoParam: &self.columns[73],
BaseID: &self.columns[74],
ItemsInvolved: &self.columns[75],
EnemyLevel: &self.columns[76],
ItemsInvolvedQty: &self.columns[77],
ItemDropRate: &self.columns[78],
NumOfAppearance: &self.columns[79],
},
]
}
pub fn ToDoSequence(&self) -> [&ColumnData; 8] {
[&self.columns[80],&self.columns[81],&self.columns[82],&self.columns[83],&self.columns[84],&self.columns[85],&self.columns[86],&self.columns[87],]
}
pub fn Rule(&self) -> &ColumnData {
&self.columns[88]
}
pub fn Objectives(&self) -> [&ColumnData; 3] {
[&self.columns[89],&self.columns[90],&self.columns[91],]
}
pub fn Help(&self) -> [&ColumnData; 2] {
[&self.columns[92],&self.columns[93],]
}
pub fn Variant(&self) -> &ColumnData {
&self.columns[94]
}
}
