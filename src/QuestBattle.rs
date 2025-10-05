#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct QuestBattleSheet {
pages: Vec<EXD>,
exh: EXH,
row_count: u32,
}
impl QuestBattleSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "QuestBattle")?;
let mut pages = Vec::new();
for (i, _) in exh.pages.iter().enumerate() {
pages.push(read_excel_sheet(resource, "QuestBattle", &exh, language, i)?);
}let row_count = exh.header.row_count;
Some(Self {
exh,
pages,
row_count,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<QuestBattleRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(QuestBattleRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<QuestBattleRow> {
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
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<QuestBattleRow> {
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
pub struct QuestBattleRow {
columns: Vec<ColumnData>,
}
impl QuestBattleRow {
pub fn QuestBattleParams(&self) -> [&ColumnData; 220] {
[&self.columns[0],&self.columns[1],&self.columns[2],&self.columns[3],&self.columns[4],&self.columns[5],&self.columns[6],&self.columns[7],&self.columns[8],&self.columns[9],&self.columns[10],&self.columns[11],&self.columns[12],&self.columns[13],&self.columns[14],&self.columns[15],&self.columns[16],&self.columns[17],&self.columns[18],&self.columns[19],&self.columns[20],&self.columns[21],&self.columns[22],&self.columns[23],&self.columns[24],&self.columns[25],&self.columns[26],&self.columns[27],&self.columns[28],&self.columns[29],&self.columns[30],&self.columns[31],&self.columns[32],&self.columns[33],&self.columns[34],&self.columns[35],&self.columns[36],&self.columns[37],&self.columns[38],&self.columns[39],&self.columns[40],&self.columns[41],&self.columns[42],&self.columns[43],&self.columns[44],&self.columns[45],&self.columns[46],&self.columns[47],&self.columns[48],&self.columns[49],&self.columns[50],&self.columns[51],&self.columns[52],&self.columns[53],&self.columns[54],&self.columns[55],&self.columns[56],&self.columns[57],&self.columns[58],&self.columns[59],&self.columns[60],&self.columns[61],&self.columns[62],&self.columns[63],&self.columns[64],&self.columns[65],&self.columns[66],&self.columns[67],&self.columns[68],&self.columns[69],&self.columns[70],&self.columns[71],&self.columns[72],&self.columns[73],&self.columns[74],&self.columns[75],&self.columns[76],&self.columns[77],&self.columns[78],&self.columns[79],&self.columns[80],&self.columns[81],&self.columns[82],&self.columns[83],&self.columns[84],&self.columns[85],&self.columns[86],&self.columns[87],&self.columns[88],&self.columns[89],&self.columns[90],&self.columns[91],&self.columns[92],&self.columns[93],&self.columns[94],&self.columns[95],&self.columns[96],&self.columns[97],&self.columns[98],&self.columns[99],&self.columns[100],&self.columns[101],&self.columns[102],&self.columns[103],&self.columns[104],&self.columns[105],&self.columns[106],&self.columns[107],&self.columns[108],&self.columns[109],&self.columns[110],&self.columns[111],&self.columns[112],&self.columns[113],&self.columns[114],&self.columns[115],&self.columns[116],&self.columns[117],&self.columns[118],&self.columns[119],&self.columns[120],&self.columns[121],&self.columns[122],&self.columns[123],&self.columns[124],&self.columns[125],&self.columns[126],&self.columns[127],&self.columns[128],&self.columns[129],&self.columns[130],&self.columns[131],&self.columns[132],&self.columns[133],&self.columns[134],&self.columns[135],&self.columns[136],&self.columns[137],&self.columns[138],&self.columns[139],&self.columns[140],&self.columns[141],&self.columns[142],&self.columns[143],&self.columns[144],&self.columns[145],&self.columns[146],&self.columns[147],&self.columns[148],&self.columns[149],&self.columns[150],&self.columns[151],&self.columns[152],&self.columns[153],&self.columns[154],&self.columns[155],&self.columns[156],&self.columns[157],&self.columns[158],&self.columns[159],&self.columns[160],&self.columns[161],&self.columns[162],&self.columns[163],&self.columns[164],&self.columns[165],&self.columns[166],&self.columns[167],&self.columns[168],&self.columns[169],&self.columns[170],&self.columns[171],&self.columns[172],&self.columns[173],&self.columns[174],&self.columns[175],&self.columns[176],&self.columns[177],&self.columns[178],&self.columns[179],&self.columns[180],&self.columns[181],&self.columns[182],&self.columns[183],&self.columns[184],&self.columns[185],&self.columns[186],&self.columns[187],&self.columns[188],&self.columns[189],&self.columns[190],&self.columns[191],&self.columns[192],&self.columns[193],&self.columns[194],&self.columns[195],&self.columns[196],&self.columns[197],&self.columns[198],&self.columns[199],&self.columns[200],&self.columns[201],&self.columns[202],&self.columns[203],&self.columns[204],&self.columns[205],&self.columns[206],&self.columns[207],&self.columns[208],&self.columns[209],&self.columns[210],&self.columns[211],&self.columns[212],&self.columns[213],&self.columns[214],&self.columns[215],&self.columns[216],&self.columns[217],&self.columns[218],&self.columns[219],]
}
pub fn Quest(&self) -> &ColumnData {
&self.columns[220]
}
pub fn TimeLimit(&self) -> &ColumnData {
&self.columns[221]
}
pub fn LevelSync(&self) -> &ColumnData {
&self.columns[222]
}
pub fn QuestBattleScene(&self) -> &ColumnData {
&self.columns[223]
}
}
