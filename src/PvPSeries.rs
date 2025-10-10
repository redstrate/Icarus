#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct LevelRewardsElement<'a> {
LevelRewardItem: &'a ColumnData,
Unknown0: &'a ColumnData,
LevelRewardCount: &'a ColumnData,
}
pub struct PvPSeriesSheet {
pages: Vec<EXD>,
exh: EXH,
row_count: u32,
}
impl PvPSeriesSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "PvPSeries")?;
let mut pages = Vec::new();
for (i, _) in exh.pages.iter().enumerate() {
pages.push(read_excel_sheet(resource, "PvPSeries", &exh, language, i)?);
}let row_count = exh.header.row_count;
Some(Self {
exh,
pages,
row_count,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<PvPSeriesRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(PvPSeriesRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<PvPSeriesRow> {
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
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<PvPSeriesRow> {
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
pub struct PvPSeriesRow {
columns: Vec<ColumnData>,
}
impl PvPSeriesRow {
pub fn LevelRewards<'a>(&'a self) -> [LevelRewardsElement<'a>; 32] {
[LevelRewardsElement {LevelRewardItem: &self.columns[0],
Unknown0: &self.columns[1],
LevelRewardCount: &self.columns[2],
},
LevelRewardsElement {LevelRewardItem: &self.columns[3],
Unknown0: &self.columns[4],
LevelRewardCount: &self.columns[5],
},
LevelRewardsElement {LevelRewardItem: &self.columns[6],
Unknown0: &self.columns[7],
LevelRewardCount: &self.columns[8],
},
LevelRewardsElement {LevelRewardItem: &self.columns[9],
Unknown0: &self.columns[10],
LevelRewardCount: &self.columns[11],
},
LevelRewardsElement {LevelRewardItem: &self.columns[12],
Unknown0: &self.columns[13],
LevelRewardCount: &self.columns[14],
},
LevelRewardsElement {LevelRewardItem: &self.columns[15],
Unknown0: &self.columns[16],
LevelRewardCount: &self.columns[17],
},
LevelRewardsElement {LevelRewardItem: &self.columns[18],
Unknown0: &self.columns[19],
LevelRewardCount: &self.columns[20],
},
LevelRewardsElement {LevelRewardItem: &self.columns[21],
Unknown0: &self.columns[22],
LevelRewardCount: &self.columns[23],
},
LevelRewardsElement {LevelRewardItem: &self.columns[24],
Unknown0: &self.columns[25],
LevelRewardCount: &self.columns[26],
},
LevelRewardsElement {LevelRewardItem: &self.columns[27],
Unknown0: &self.columns[28],
LevelRewardCount: &self.columns[29],
},
LevelRewardsElement {LevelRewardItem: &self.columns[30],
Unknown0: &self.columns[31],
LevelRewardCount: &self.columns[32],
},
LevelRewardsElement {LevelRewardItem: &self.columns[33],
Unknown0: &self.columns[34],
LevelRewardCount: &self.columns[35],
},
LevelRewardsElement {LevelRewardItem: &self.columns[36],
Unknown0: &self.columns[37],
LevelRewardCount: &self.columns[38],
},
LevelRewardsElement {LevelRewardItem: &self.columns[39],
Unknown0: &self.columns[40],
LevelRewardCount: &self.columns[41],
},
LevelRewardsElement {LevelRewardItem: &self.columns[42],
Unknown0: &self.columns[43],
LevelRewardCount: &self.columns[44],
},
LevelRewardsElement {LevelRewardItem: &self.columns[45],
Unknown0: &self.columns[46],
LevelRewardCount: &self.columns[47],
},
LevelRewardsElement {LevelRewardItem: &self.columns[48],
Unknown0: &self.columns[49],
LevelRewardCount: &self.columns[50],
},
LevelRewardsElement {LevelRewardItem: &self.columns[51],
Unknown0: &self.columns[52],
LevelRewardCount: &self.columns[53],
},
LevelRewardsElement {LevelRewardItem: &self.columns[54],
Unknown0: &self.columns[55],
LevelRewardCount: &self.columns[56],
},
LevelRewardsElement {LevelRewardItem: &self.columns[57],
Unknown0: &self.columns[58],
LevelRewardCount: &self.columns[59],
},
LevelRewardsElement {LevelRewardItem: &self.columns[60],
Unknown0: &self.columns[61],
LevelRewardCount: &self.columns[62],
},
LevelRewardsElement {LevelRewardItem: &self.columns[63],
Unknown0: &self.columns[64],
LevelRewardCount: &self.columns[65],
},
LevelRewardsElement {LevelRewardItem: &self.columns[66],
Unknown0: &self.columns[67],
LevelRewardCount: &self.columns[68],
},
LevelRewardsElement {LevelRewardItem: &self.columns[69],
Unknown0: &self.columns[70],
LevelRewardCount: &self.columns[71],
},
LevelRewardsElement {LevelRewardItem: &self.columns[72],
Unknown0: &self.columns[73],
LevelRewardCount: &self.columns[74],
},
LevelRewardsElement {LevelRewardItem: &self.columns[75],
Unknown0: &self.columns[76],
LevelRewardCount: &self.columns[77],
},
LevelRewardsElement {LevelRewardItem: &self.columns[78],
Unknown0: &self.columns[79],
LevelRewardCount: &self.columns[80],
},
LevelRewardsElement {LevelRewardItem: &self.columns[81],
Unknown0: &self.columns[82],
LevelRewardCount: &self.columns[83],
},
LevelRewardsElement {LevelRewardItem: &self.columns[84],
Unknown0: &self.columns[85],
LevelRewardCount: &self.columns[86],
},
LevelRewardsElement {LevelRewardItem: &self.columns[87],
Unknown0: &self.columns[88],
LevelRewardCount: &self.columns[89],
},
LevelRewardsElement {LevelRewardItem: &self.columns[90],
Unknown0: &self.columns[91],
LevelRewardCount: &self.columns[92],
},
LevelRewardsElement {LevelRewardItem: &self.columns[93],
Unknown0: &self.columns[94],
LevelRewardCount: &self.columns[95],
},
]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[96]
}
}
