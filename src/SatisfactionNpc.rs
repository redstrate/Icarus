#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct SatisfactionNpcParamsElement<'a> {
SupplyIndex: &'a ColumnData,
Item: &'a ColumnData,
SatisfactionRequired: &'a ColumnData,
ItemCount: &'a ColumnData,
IsHQ: &'a ColumnData,
}
pub struct RankParamsElement<'a> {
ImageId: &'a ColumnData,
Unknown1: &'a ColumnData,
Quest: &'a ColumnData,
}
pub struct SatisfactionNpcSheet {
pages: Vec<EXD>,
exh: EXH,
row_count: u32,
}
impl SatisfactionNpcSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "SatisfactionNpc")?;
let mut pages = Vec::new();
for (i, _) in exh.pages.iter().enumerate() {
pages.push(read_excel_sheet(resource, "SatisfactionNpc", &exh, language, i)?);
}let row_count = exh.header.row_count;
Some(Self {
exh,
pages,
row_count,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<SatisfactionNpcRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(SatisfactionNpcRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<SatisfactionNpcRow> {
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
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<SatisfactionNpcRow> {
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
pub struct SatisfactionNpcRow {
columns: Vec<ColumnData>,
}
impl SatisfactionNpcRow {
pub fn SatisfactionNpcParams<'a>(&'a self) -> [SatisfactionNpcParamsElement<'a>; 6] {
[SatisfactionNpcParamsElement {SupplyIndex: &self.columns[0],
Item: &self.columns[1],
SatisfactionRequired: &self.columns[2],
ItemCount: &self.columns[3],
IsHQ: &self.columns[4],
},
SatisfactionNpcParamsElement {SupplyIndex: &self.columns[5],
Item: &self.columns[6],
SatisfactionRequired: &self.columns[7],
ItemCount: &self.columns[8],
IsHQ: &self.columns[9],
},
SatisfactionNpcParamsElement {SupplyIndex: &self.columns[10],
Item: &self.columns[11],
SatisfactionRequired: &self.columns[12],
ItemCount: &self.columns[13],
IsHQ: &self.columns[14],
},
SatisfactionNpcParamsElement {SupplyIndex: &self.columns[15],
Item: &self.columns[16],
SatisfactionRequired: &self.columns[17],
ItemCount: &self.columns[18],
IsHQ: &self.columns[19],
},
SatisfactionNpcParamsElement {SupplyIndex: &self.columns[20],
Item: &self.columns[21],
SatisfactionRequired: &self.columns[22],
ItemCount: &self.columns[23],
IsHQ: &self.columns[24],
},
SatisfactionNpcParamsElement {SupplyIndex: &self.columns[25],
Item: &self.columns[26],
SatisfactionRequired: &self.columns[27],
ItemCount: &self.columns[28],
IsHQ: &self.columns[29],
},
]
}
pub fn RankParams<'a>(&'a self) -> [RankParamsElement<'a>; 6] {
[RankParamsElement {ImageId: &self.columns[30],
Unknown1: &self.columns[31],
Quest: &self.columns[32],
},
RankParamsElement {ImageId: &self.columns[33],
Unknown1: &self.columns[34],
Quest: &self.columns[35],
},
RankParamsElement {ImageId: &self.columns[36],
Unknown1: &self.columns[37],
Quest: &self.columns[38],
},
RankParamsElement {ImageId: &self.columns[39],
Unknown1: &self.columns[40],
Quest: &self.columns[41],
},
RankParamsElement {ImageId: &self.columns[42],
Unknown1: &self.columns[43],
Quest: &self.columns[44],
},
RankParamsElement {ImageId: &self.columns[45],
Unknown1: &self.columns[46],
Quest: &self.columns[47],
},
]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[48]
}
pub fn Npc(&self) -> &ColumnData {
&self.columns[49]
}
pub fn QuestRequired(&self) -> &ColumnData {
&self.columns[50]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[51]
}
pub fn LevelUnlock(&self) -> &ColumnData {
&self.columns[52]
}
pub fn DeliveriesPerWeek(&self) -> &ColumnData {
&self.columns[53]
}
pub fn GlamourIndex(&self) -> &ColumnData {
&self.columns[54]
}
pub fn Unknown19(&self) -> &ColumnData {
&self.columns[55]
}
pub fn Unknown20(&self) -> &ColumnData {
&self.columns[56]
}
}
