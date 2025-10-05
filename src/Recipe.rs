#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct RecipeSheet {
pages: Vec<EXD>,
exh: EXH,
row_count: u32,
}
impl RecipeSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "Recipe")?;
let mut pages = Vec::new();
for (i, _) in exh.pages.iter().enumerate() {
pages.push(read_excel_sheet(resource, "Recipe", &exh, language, i)?);
}let row_count = exh.header.row_count;
Some(Self {
exh,
pages,
row_count,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<RecipeRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(RecipeRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<RecipeRow> {
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
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<RecipeRow> {
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
pub struct RecipeRow {
columns: Vec<ColumnData>,
}
impl RecipeRow {
pub fn RequiredQuality(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Quest(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Number(&self) -> &ColumnData {
&self.columns[2]
}
pub fn CraftType(&self) -> &ColumnData {
&self.columns[3]
}
pub fn ItemResult(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Ingredient(&self) -> [&ColumnData; 8] {
[&self.columns[5],&self.columns[6],&self.columns[7],&self.columns[8],&self.columns[9],&self.columns[10],&self.columns[11],&self.columns[12],]
}
pub fn StatusRequired(&self) -> &ColumnData {
&self.columns[13]
}
pub fn ItemRequired(&self) -> &ColumnData {
&self.columns[14]
}
pub fn RecipeLevelTable(&self) -> &ColumnData {
&self.columns[15]
}
pub fn MaxAdjustableJobLevel(&self) -> &ColumnData {
&self.columns[16]
}
pub fn RecipeNotebookList(&self) -> &ColumnData {
&self.columns[17]
}
pub fn DisplayPriority(&self) -> &ColumnData {
&self.columns[18]
}
pub fn DifficultyFactor(&self) -> &ColumnData {
&self.columns[19]
}
pub fn QualityFactor(&self) -> &ColumnData {
&self.columns[20]
}
pub fn DurabilityFactor(&self) -> &ColumnData {
&self.columns[21]
}
pub fn RequiredCraftsmanship(&self) -> &ColumnData {
&self.columns[22]
}
pub fn RequiredControl(&self) -> &ColumnData {
&self.columns[23]
}
pub fn QuickSynthCraftsmanship(&self) -> &ColumnData {
&self.columns[24]
}
pub fn QuickSynthControl(&self) -> &ColumnData {
&self.columns[25]
}
pub fn SecretRecipeBook(&self) -> &ColumnData {
&self.columns[26]
}
pub fn CollectableMetadata(&self) -> &ColumnData {
&self.columns[27]
}
pub fn PatchNumber(&self) -> &ColumnData {
&self.columns[28]
}
pub fn AmountResult(&self) -> &ColumnData {
&self.columns[29]
}
pub fn AmountIngredient(&self) -> [&ColumnData; 8] {
[&self.columns[30],&self.columns[31],&self.columns[32],&self.columns[33],&self.columns[34],&self.columns[35],&self.columns[36],&self.columns[37],]
}
pub fn MaterialQualityFactor(&self) -> &ColumnData {
&self.columns[38]
}
pub fn CollectableMetadataKey(&self) -> &ColumnData {
&self.columns[39]
}
pub fn IsSecondary(&self) -> &ColumnData {
&self.columns[40]
}
pub fn CanQuickSynth(&self) -> &ColumnData {
&self.columns[41]
}
pub fn CanHq(&self) -> &ColumnData {
&self.columns[42]
}
pub fn ExpRewarded(&self) -> &ColumnData {
&self.columns[43]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[44]
}
pub fn IsSpecializationRequired(&self) -> &ColumnData {
&self.columns[45]
}
pub fn IsExpert(&self) -> &ColumnData {
&self.columns[46]
}
}
