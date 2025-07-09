#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct RecipeNotebookListSheet {
exd: EXD,
exh: EXH,
}
impl RecipeNotebookListSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "RecipeNotebookList")?;let exd = read_excel_sheet(resource, "RecipeNotebookList", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<RecipeNotebookListRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(RecipeNotebookListRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<RecipeNotebookListRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<RecipeNotebookListRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct RecipeNotebookListRow {
columns: Vec<ColumnData>,
}
impl RecipeNotebookListRow {
pub fn Recipe(&self) -> [&ColumnData; 160] {
[&self.columns[0],&self.columns[1],&self.columns[2],&self.columns[3],&self.columns[4],&self.columns[5],&self.columns[6],&self.columns[7],&self.columns[8],&self.columns[9],&self.columns[10],&self.columns[11],&self.columns[12],&self.columns[13],&self.columns[14],&self.columns[15],&self.columns[16],&self.columns[17],&self.columns[18],&self.columns[19],&self.columns[20],&self.columns[21],&self.columns[22],&self.columns[23],&self.columns[24],&self.columns[25],&self.columns[26],&self.columns[27],&self.columns[28],&self.columns[29],&self.columns[30],&self.columns[31],&self.columns[32],&self.columns[33],&self.columns[34],&self.columns[35],&self.columns[36],&self.columns[37],&self.columns[38],&self.columns[39],&self.columns[40],&self.columns[41],&self.columns[42],&self.columns[43],&self.columns[44],&self.columns[45],&self.columns[46],&self.columns[47],&self.columns[48],&self.columns[49],&self.columns[50],&self.columns[51],&self.columns[52],&self.columns[53],&self.columns[54],&self.columns[55],&self.columns[56],&self.columns[57],&self.columns[58],&self.columns[59],&self.columns[60],&self.columns[61],&self.columns[62],&self.columns[63],&self.columns[64],&self.columns[65],&self.columns[66],&self.columns[67],&self.columns[68],&self.columns[69],&self.columns[70],&self.columns[71],&self.columns[72],&self.columns[73],&self.columns[74],&self.columns[75],&self.columns[76],&self.columns[77],&self.columns[78],&self.columns[79],&self.columns[80],&self.columns[81],&self.columns[82],&self.columns[83],&self.columns[84],&self.columns[85],&self.columns[86],&self.columns[87],&self.columns[88],&self.columns[89],&self.columns[90],&self.columns[91],&self.columns[92],&self.columns[93],&self.columns[94],&self.columns[95],&self.columns[96],&self.columns[97],&self.columns[98],&self.columns[99],&self.columns[100],&self.columns[101],&self.columns[102],&self.columns[103],&self.columns[104],&self.columns[105],&self.columns[106],&self.columns[107],&self.columns[108],&self.columns[109],&self.columns[110],&self.columns[111],&self.columns[112],&self.columns[113],&self.columns[114],&self.columns[115],&self.columns[116],&self.columns[117],&self.columns[118],&self.columns[119],&self.columns[120],&self.columns[121],&self.columns[122],&self.columns[123],&self.columns[124],&self.columns[125],&self.columns[126],&self.columns[127],&self.columns[128],&self.columns[129],&self.columns[130],&self.columns[131],&self.columns[132],&self.columns[133],&self.columns[134],&self.columns[135],&self.columns[136],&self.columns[137],&self.columns[138],&self.columns[139],&self.columns[140],&self.columns[141],&self.columns[142],&self.columns[143],&self.columns[144],&self.columns[145],&self.columns[146],&self.columns[147],&self.columns[148],&self.columns[149],&self.columns[150],&self.columns[151],&self.columns[152],&self.columns[153],&self.columns[154],&self.columns[155],&self.columns[156],&self.columns[157],&self.columns[158],&self.columns[159],]
}
pub fn Count(&self) -> &ColumnData {
&self.columns[160]
}
}
