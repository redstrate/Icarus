#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct HugeCraftworksTurnInParamElement<'a> {
RequestedItem: &'a ColumnData,
Unknown0: &'a ColumnData,
RequestedQuantity: &'a ColumnData,
Unknown1: &'a ColumnData,
Unknown2: &'a ColumnData,
Unknown3: &'a ColumnData,
Unknown4: &'a ColumnData,
Unknown5: &'a ColumnData,
Unknown6: &'a ColumnData,
}
pub struct HugeCraftworksRewardParamElement<'a> {
RewardItem: &'a ColumnData,
RewardQuantity: &'a ColumnData,
RewardHQ: &'a ColumnData,
}
pub struct HugeCraftworksNpcSheet {
pages: Vec<EXD>,
exh: EXH,
row_count: u32,
}
impl HugeCraftworksNpcSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "HugeCraftworksNpc")?;
let mut pages = Vec::new();
for (i, _) in exh.pages.iter().enumerate() {
pages.push(read_excel_sheet(resource, "HugeCraftworksNpc", &exh, language, i)?);
}let row_count = exh.header.row_count;
Some(Self {
exh,
pages,
row_count,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<HugeCraftworksNpcRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(HugeCraftworksNpcRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<HugeCraftworksNpcRow> {
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
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<HugeCraftworksNpcRow> {
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
pub struct HugeCraftworksNpcRow {
columns: Vec<ColumnData>,
}
impl HugeCraftworksNpcRow {
pub fn HugeCraftworksTurnInParam<'a>(&'a self) -> [HugeCraftworksTurnInParamElement<'a>; 6] {
[HugeCraftworksTurnInParamElement {RequestedItem: &self.columns[0],
Unknown0: &self.columns[1],
RequestedQuantity: &self.columns[2],
Unknown1: &self.columns[3],
Unknown2: &self.columns[4],
Unknown3: &self.columns[5],
Unknown4: &self.columns[6],
Unknown5: &self.columns[7],
Unknown6: &self.columns[8],
},
HugeCraftworksTurnInParamElement {RequestedItem: &self.columns[9],
Unknown0: &self.columns[10],
RequestedQuantity: &self.columns[11],
Unknown1: &self.columns[12],
Unknown2: &self.columns[13],
Unknown3: &self.columns[14],
Unknown4: &self.columns[15],
Unknown5: &self.columns[16],
Unknown6: &self.columns[17],
},
HugeCraftworksTurnInParamElement {RequestedItem: &self.columns[18],
Unknown0: &self.columns[19],
RequestedQuantity: &self.columns[20],
Unknown1: &self.columns[21],
Unknown2: &self.columns[22],
Unknown3: &self.columns[23],
Unknown4: &self.columns[24],
Unknown5: &self.columns[25],
Unknown6: &self.columns[26],
},
HugeCraftworksTurnInParamElement {RequestedItem: &self.columns[27],
Unknown0: &self.columns[28],
RequestedQuantity: &self.columns[29],
Unknown1: &self.columns[30],
Unknown2: &self.columns[31],
Unknown3: &self.columns[32],
Unknown4: &self.columns[33],
Unknown5: &self.columns[34],
Unknown6: &self.columns[35],
},
HugeCraftworksTurnInParamElement {RequestedItem: &self.columns[36],
Unknown0: &self.columns[37],
RequestedQuantity: &self.columns[38],
Unknown1: &self.columns[39],
Unknown2: &self.columns[40],
Unknown3: &self.columns[41],
Unknown4: &self.columns[42],
Unknown5: &self.columns[43],
Unknown6: &self.columns[44],
},
HugeCraftworksTurnInParamElement {RequestedItem: &self.columns[45],
Unknown0: &self.columns[46],
RequestedQuantity: &self.columns[47],
Unknown1: &self.columns[48],
Unknown2: &self.columns[49],
Unknown3: &self.columns[50],
Unknown4: &self.columns[51],
Unknown5: &self.columns[52],
Unknown6: &self.columns[53],
},
]
}
pub fn HugeCraftworksRewardParam<'a>(&'a self) -> [HugeCraftworksRewardParamElement<'a>; 6] {
[HugeCraftworksRewardParamElement {RewardItem: &self.columns[54],
RewardQuantity: &self.columns[55],
RewardHQ: &self.columns[56],
},
HugeCraftworksRewardParamElement {RewardItem: &self.columns[57],
RewardQuantity: &self.columns[58],
RewardHQ: &self.columns[59],
},
HugeCraftworksRewardParamElement {RewardItem: &self.columns[60],
RewardQuantity: &self.columns[61],
RewardHQ: &self.columns[62],
},
HugeCraftworksRewardParamElement {RewardItem: &self.columns[63],
RewardQuantity: &self.columns[64],
RewardHQ: &self.columns[65],
},
HugeCraftworksRewardParamElement {RewardItem: &self.columns[66],
RewardQuantity: &self.columns[67],
RewardHQ: &self.columns[68],
},
HugeCraftworksRewardParamElement {RewardItem: &self.columns[69],
RewardQuantity: &self.columns[70],
RewardHQ: &self.columns[71],
},
]
}
pub fn Transient(&self) -> &ColumnData {
&self.columns[72]
}
pub fn EventNpc(&self) -> &ColumnData {
&self.columns[73]
}
pub fn ClassJobCategory(&self) -> &ColumnData {
&self.columns[74]
}
}
