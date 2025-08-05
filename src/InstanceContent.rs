#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct InstanceContentSheet {
exd: EXD,
exh: EXH,
}
impl InstanceContentSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "InstanceContent")?;let exd = read_excel_sheet(resource, "InstanceContent", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<InstanceContentRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(InstanceContentRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<InstanceContentRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<InstanceContentRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct InstanceContentRow {
columns: Vec<ColumnData>,
}
impl InstanceContentRow {
pub fn NewPlayerBonusGil(&self) -> &ColumnData {
&self.columns[0]
}
pub fn NewPlayerBonusExp(&self) -> &ColumnData {
&self.columns[1]
}
pub fn FinalBossExp(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[3]
}
pub fn BossExp(&self) -> [&ColumnData; 5] {
[&self.columns[4],&self.columns[5],&self.columns[6],&self.columns[7],&self.columns[8],]
}
pub fn InstanceClearExp(&self) -> &ColumnData {
&self.columns[9]
}
pub fn InstanceClearGil(&self) -> &ColumnData {
&self.columns[10]
}
pub fn InstanceContentRewardItem(&self) -> &ColumnData {
&self.columns[11]
}
pub fn NewPlayerBonusA(&self) -> &ColumnData {
&self.columns[12]
}
pub fn NewPlayerBonusB(&self) -> &ColumnData {
&self.columns[13]
}
pub fn FinalBossCurrencyA(&self) -> &ColumnData {
&self.columns[14]
}
pub fn FinalBossCurrencyB(&self) -> &ColumnData {
&self.columns[15]
}
pub fn FinalBossCurrencyC(&self) -> &ColumnData {
&self.columns[16]
}
pub fn BossCurrencyA(&self) -> [&ColumnData; 5] {
[&self.columns[17],&self.columns[18],&self.columns[19],&self.columns[20],&self.columns[21],]
}
pub fn BossCurrencyB(&self) -> [&ColumnData; 5] {
[&self.columns[22],&self.columns[23],&self.columns[24],&self.columns[25],&self.columns[26],]
}
pub fn BossCurrencyC(&self) -> [&ColumnData; 5] {
[&self.columns[27],&self.columns[28],&self.columns[29],&self.columns[30],&self.columns[31],]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[32]
}
pub fn LimitedTimeBonus(&self) -> &ColumnData {
&self.columns[33]
}
pub fn Cutscene(&self) -> &ColumnData {
&self.columns[34]
}
pub fn LGBEventRange(&self) -> &ColumnData {
&self.columns[35]
}
pub fn InstanceContentTextDataBossStart(&self) -> &ColumnData {
&self.columns[36]
}
pub fn InstanceContentTextDataBossEnd(&self) -> &ColumnData {
&self.columns[37]
}
pub fn BNpcBaseBoss(&self) -> &ColumnData {
&self.columns[38]
}
pub fn InstanceContentTextDataObjectiveStart(&self) -> &ColumnData {
&self.columns[39]
}
pub fn InstanceContentTextDataObjectiveEnd(&self) -> &ColumnData {
&self.columns[40]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[41]
}
pub fn ReqInstance(&self) -> &ColumnData {
&self.columns[42]
}
pub fn InstanceContentBuff(&self) -> &ColumnData {
&self.columns[43]
}
pub fn TimeLimitmin(&self) -> &ColumnData {
&self.columns[44]
}
pub fn BGM(&self) -> &ColumnData {
&self.columns[45]
}
pub fn WinBGM(&self) -> &ColumnData {
&self.columns[46]
}
pub fn ContentFinderCondition(&self) -> &ColumnData {
&self.columns[47]
}
pub fn SortKey(&self) -> &ColumnData {
&self.columns[48]
}
pub fn ContentRoute(&self) -> &ColumnData {
&self.columns[49]
}
pub fn ContentDirectorManagedSG(&self) -> &ColumnData {
&self.columns[50]
}
pub fn ContentTodo(&self) -> &ColumnData {
&self.columns[51]
}
pub fn Unknown6(&self) -> &ColumnData {
&self.columns[52]
}
pub fn Unknown7(&self) -> &ColumnData {
&self.columns[53]
}
pub fn ContentEventItem(&self) -> &ColumnData {
&self.columns[54]
}
pub fn ContentDirectorBattleTalk(&self) -> &ColumnData {
&self.columns[55]
}
pub fn PartyCondition(&self) -> &ColumnData {
&self.columns[56]
}
pub fn InstanceContentType(&self) -> &ColumnData {
&self.columns[57]
}
pub fn WeekRestriction(&self) -> &ColumnData {
&self.columns[58]
}
pub fn Colosseum(&self) -> &ColumnData {
&self.columns[59]
}
pub fn Unknown9(&self) -> &ColumnData {
&self.columns[60]
}
pub fn QTE1(&self) -> &ColumnData {
&self.columns[61]
}
pub fn QTE2(&self) -> &ColumnData {
&self.columns[62]
}
pub fn Unknown12(&self) -> &ColumnData {
&self.columns[63]
}
pub fn ContentAttributeRect(&self) -> &ColumnData {
&self.columns[64]
}
pub fn Unknown13(&self) -> &ColumnData {
&self.columns[65]
}
pub fn Unknown14(&self) -> &ColumnData {
&self.columns[66]
}
pub fn Unknown15(&self) -> &ColumnData {
&self.columns[67]
}
pub fn Unknown16(&self) -> &ColumnData {
&self.columns[68]
}
pub fn Unknown17(&self) -> &ColumnData {
&self.columns[69]
}
pub fn Unknown18(&self) -> &ColumnData {
&self.columns[70]
}
pub fn Unknown19(&self) -> &ColumnData {
&self.columns[71]
}
}
