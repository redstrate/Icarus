#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct ExpeditionParamsElement<'a> {
RewardItem: &'a ColumnData,
RequiredPhysical: &'a ColumnData,
RequiredMental: &'a ColumnData,
RequiredTactical: &'a ColumnData,
RewardQuantity: &'a ColumnData,
PercentPhysicalMet: &'a ColumnData,
PercentMentalMet: &'a ColumnData,
PercentTacticalMet: &'a ColumnData,
PercentAllMet: &'a ColumnData,
}
pub struct GcArmyExpeditionSheet {
pages: Vec<EXD>,
exh: EXH,
row_count: u32,
}
impl GcArmyExpeditionSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "GcArmyExpedition")?;
let mut pages = Vec::new();
for (i, _) in exh.pages.iter().enumerate() {
pages.push(read_excel_sheet(resource, "GcArmyExpedition", &exh, language, i)?);
}let row_count = exh.header.row_count;
Some(Self {
exh,
pages,
row_count,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<GcArmyExpeditionRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(GcArmyExpeditionRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<GcArmyExpeditionRow> {
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
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<GcArmyExpeditionRow> {
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
pub struct GcArmyExpeditionRow {
columns: Vec<ColumnData>,
}
impl GcArmyExpeditionRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Description(&self) -> &ColumnData {
&self.columns[1]
}
pub fn ExpeditionParams<'a>(&'a self) -> [ExpeditionParamsElement<'a>; 6] {
[ExpeditionParamsElement {RewardItem: &self.columns[2],
RequiredPhysical: &self.columns[3],
RequiredMental: &self.columns[4],
RequiredTactical: &self.columns[5],
RewardQuantity: &self.columns[6],
PercentPhysicalMet: &self.columns[7],
PercentMentalMet: &self.columns[8],
PercentTacticalMet: &self.columns[9],
PercentAllMet: &self.columns[10],
},
ExpeditionParamsElement {RewardItem: &self.columns[11],
RequiredPhysical: &self.columns[12],
RequiredMental: &self.columns[13],
RequiredTactical: &self.columns[14],
RewardQuantity: &self.columns[15],
PercentPhysicalMet: &self.columns[16],
PercentMentalMet: &self.columns[17],
PercentTacticalMet: &self.columns[18],
PercentAllMet: &self.columns[19],
},
ExpeditionParamsElement {RewardItem: &self.columns[20],
RequiredPhysical: &self.columns[21],
RequiredMental: &self.columns[22],
RequiredTactical: &self.columns[23],
RewardQuantity: &self.columns[24],
PercentPhysicalMet: &self.columns[25],
PercentMentalMet: &self.columns[26],
PercentTacticalMet: &self.columns[27],
PercentAllMet: &self.columns[28],
},
ExpeditionParamsElement {RewardItem: &self.columns[29],
RequiredPhysical: &self.columns[30],
RequiredMental: &self.columns[31],
RequiredTactical: &self.columns[32],
RewardQuantity: &self.columns[33],
PercentPhysicalMet: &self.columns[34],
PercentMentalMet: &self.columns[35],
PercentTacticalMet: &self.columns[36],
PercentAllMet: &self.columns[37],
},
ExpeditionParamsElement {RewardItem: &self.columns[38],
RequiredPhysical: &self.columns[39],
RequiredMental: &self.columns[40],
RequiredTactical: &self.columns[41],
RewardQuantity: &self.columns[42],
PercentPhysicalMet: &self.columns[43],
PercentMentalMet: &self.columns[44],
PercentTacticalMet: &self.columns[45],
PercentAllMet: &self.columns[46],
},
ExpeditionParamsElement {RewardItem: &self.columns[47],
RequiredPhysical: &self.columns[48],
RequiredMental: &self.columns[49],
RequiredTactical: &self.columns[50],
RewardQuantity: &self.columns[51],
PercentPhysicalMet: &self.columns[52],
PercentMentalMet: &self.columns[53],
PercentTacticalMet: &self.columns[54],
PercentAllMet: &self.columns[55],
},
]
}
pub fn RewardExperience(&self) -> &ColumnData {
&self.columns[56]
}
pub fn RequiredSeals(&self) -> &ColumnData {
&self.columns[57]
}
pub fn RequiredFlag(&self) -> &ColumnData {
&self.columns[58]
}
pub fn UnlockFlag(&self) -> &ColumnData {
&self.columns[59]
}
pub fn RequiredLevel(&self) -> &ColumnData {
&self.columns[60]
}
pub fn PercentBase(&self) -> &ColumnData {
&self.columns[61]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[62]
}
pub fn GcArmyExpeditionType(&self) -> &ColumnData {
&self.columns[63]
}
}
