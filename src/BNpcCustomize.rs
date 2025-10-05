#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct BNpcCustomizeSheet {
pages: Vec<EXD>,
exh: EXH,
row_count: u32,
}
impl BNpcCustomizeSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "BNpcCustomize")?;
let mut pages = Vec::new();
for (i, _) in exh.pages.iter().enumerate() {
pages.push(read_excel_sheet(resource, "BNpcCustomize", &exh, language, i)?);
}let row_count = exh.header.row_count;
Some(Self {
exh,
pages,
row_count,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<BNpcCustomizeRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(BNpcCustomizeRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<BNpcCustomizeRow> {
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
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<BNpcCustomizeRow> {
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
pub struct BNpcCustomizeRow {
columns: Vec<ColumnData>,
}
impl BNpcCustomizeRow {
pub fn Race(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Gender(&self) -> &ColumnData {
&self.columns[1]
}
pub fn BodyType(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Height(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Tribe(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Face(&self) -> &ColumnData {
&self.columns[5]
}
pub fn HairStyle(&self) -> &ColumnData {
&self.columns[6]
}
pub fn HairHighlight(&self) -> &ColumnData {
&self.columns[7]
}
pub fn SkinColor(&self) -> &ColumnData {
&self.columns[8]
}
pub fn EyeHeterochromia(&self) -> &ColumnData {
&self.columns[9]
}
pub fn HairColor(&self) -> &ColumnData {
&self.columns[10]
}
pub fn HairHighlightColor(&self) -> &ColumnData {
&self.columns[11]
}
pub fn FacialFeature(&self) -> &ColumnData {
&self.columns[12]
}
pub fn FacialFeatureColor(&self) -> &ColumnData {
&self.columns[13]
}
pub fn Eyebrows(&self) -> &ColumnData {
&self.columns[14]
}
pub fn EyeColor(&self) -> &ColumnData {
&self.columns[15]
}
pub fn EyeShape(&self) -> &ColumnData {
&self.columns[16]
}
pub fn Nose(&self) -> &ColumnData {
&self.columns[17]
}
pub fn Jaw(&self) -> &ColumnData {
&self.columns[18]
}
pub fn Mouth(&self) -> &ColumnData {
&self.columns[19]
}
pub fn LipColor(&self) -> &ColumnData {
&self.columns[20]
}
pub fn BustOrTone1(&self) -> &ColumnData {
&self.columns[21]
}
pub fn ExtraFeature1(&self) -> &ColumnData {
&self.columns[22]
}
pub fn ExtraFeature2OrBust(&self) -> &ColumnData {
&self.columns[23]
}
pub fn FacePaint(&self) -> &ColumnData {
&self.columns[24]
}
pub fn FacePaintColor(&self) -> &ColumnData {
&self.columns[25]
}
}
