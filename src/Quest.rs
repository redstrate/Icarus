#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct QuestSheet {
exd: EXD,
exh: EXH,
}
impl QuestSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "Quest")?;let exd = read_excel_sheet(resource, "Quest", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<QuestRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(QuestRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<QuestRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<QuestRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct QuestRow {
columns: Vec<ColumnData>,
}
impl QuestRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn QuestParams(&self) -> [&ColumnData; 50] {
[&self.columns[1],&self.columns[2],&self.columns[3],&self.columns[4],&self.columns[5],&self.columns[6],&self.columns[7],&self.columns[8],&self.columns[9],&self.columns[10],&self.columns[11],&self.columns[12],&self.columns[13],&self.columns[14],&self.columns[15],&self.columns[16],&self.columns[17],&self.columns[18],&self.columns[19],&self.columns[20],&self.columns[21],&self.columns[22],&self.columns[23],&self.columns[24],&self.columns[25],&self.columns[26],&self.columns[27],&self.columns[28],&self.columns[29],&self.columns[30],&self.columns[31],&self.columns[32],&self.columns[33],&self.columns[34],&self.columns[35],&self.columns[36],&self.columns[37],&self.columns[38],&self.columns[39],&self.columns[40],&self.columns[41],&self.columns[42],&self.columns[43],&self.columns[44],&self.columns[45],&self.columns[46],&self.columns[47],&self.columns[48],&self.columns[49],&self.columns[50],]
}
pub fn QuestListenerParams(&self) -> [&ColumnData; 64] {
[&self.columns[51],&self.columns[52],&self.columns[53],&self.columns[54],&self.columns[55],&self.columns[56],&self.columns[57],&self.columns[58],&self.columns[59],&self.columns[60],&self.columns[61],&self.columns[62],&self.columns[63],&self.columns[64],&self.columns[65],&self.columns[66],&self.columns[67],&self.columns[68],&self.columns[69],&self.columns[70],&self.columns[71],&self.columns[72],&self.columns[73],&self.columns[74],&self.columns[75],&self.columns[76],&self.columns[77],&self.columns[78],&self.columns[79],&self.columns[80],&self.columns[81],&self.columns[82],&self.columns[83],&self.columns[84],&self.columns[85],&self.columns[86],&self.columns[87],&self.columns[88],&self.columns[89],&self.columns[90],&self.columns[91],&self.columns[92],&self.columns[93],&self.columns[94],&self.columns[95],&self.columns[96],&self.columns[97],&self.columns[98],&self.columns[99],&self.columns[100],&self.columns[101],&self.columns[102],&self.columns[103],&self.columns[104],&self.columns[105],&self.columns[106],&self.columns[107],&self.columns[108],&self.columns[109],&self.columns[110],&self.columns[111],&self.columns[112],&self.columns[113],&self.columns[114],]
}
pub fn TodoParams(&self) -> [&ColumnData; 24] {
[&self.columns[115],&self.columns[116],&self.columns[117],&self.columns[118],&self.columns[119],&self.columns[120],&self.columns[121],&self.columns[122],&self.columns[123],&self.columns[124],&self.columns[125],&self.columns[126],&self.columns[127],&self.columns[128],&self.columns[129],&self.columns[130],&self.columns[131],&self.columns[132],&self.columns[133],&self.columns[134],&self.columns[135],&self.columns[136],&self.columns[137],&self.columns[138],]
}
pub fn GilReward(&self) -> &ColumnData {
&self.columns[139]
}
pub fn CurrencyReward(&self) -> &ColumnData {
&self.columns[140]
}
pub fn CurrencyRewardCount(&self) -> &ColumnData {
&self.columns[141]
}
pub fn Reward(&self) -> [&ColumnData; 7] {
[&self.columns[142],&self.columns[143],&self.columns[144],&self.columns[145],&self.columns[146],&self.columns[147],&self.columns[148],]
}
pub fn OptionalItemReward(&self) -> [&ColumnData; 5] {
[&self.columns[149],&self.columns[150],&self.columns[151],&self.columns[152],&self.columns[153],]
}
pub fn InstanceContentUnlock(&self) -> &ColumnData {
&self.columns[154]
}
pub fn ExpFactor(&self) -> &ColumnData {
&self.columns[155]
}
pub fn EmoteReward(&self) -> &ColumnData {
&self.columns[156]
}
pub fn ActionReward(&self) -> &ColumnData {
&self.columns[157]
}
pub fn SystemReward(&self) -> [&ColumnData; 2] {
[&self.columns[158],&self.columns[159],]
}
pub fn GCTypeReward(&self) -> &ColumnData {
&self.columns[160]
}
pub fn ItemCatalyst(&self) -> [&ColumnData; 3] {
[&self.columns[161],&self.columns[162],&self.columns[163],]
}
pub fn ItemCountCatalyst(&self) -> [&ColumnData; 3] {
[&self.columns[164],&self.columns[165],&self.columns[166],]
}
pub fn ItemRewardType(&self) -> &ColumnData {
&self.columns[167]
}
pub fn ItemCountReward(&self) -> [&ColumnData; 7] {
[&self.columns[168],&self.columns[169],&self.columns[170],&self.columns[171],&self.columns[172],&self.columns[173],&self.columns[174],]
}
pub fn RewardStain(&self) -> [&ColumnData; 7] {
[&self.columns[175],&self.columns[176],&self.columns[177],&self.columns[178],&self.columns[179],&self.columns[180],&self.columns[181],]
}
pub fn OptionalItemCountReward(&self) -> [&ColumnData; 5] {
[&self.columns[182],&self.columns[183],&self.columns[184],&self.columns[185],&self.columns[186],]
}
pub fn OptionalItemStainReward(&self) -> [&ColumnData; 5] {
[&self.columns[187],&self.columns[188],&self.columns[189],&self.columns[190],&self.columns[191],]
}
pub fn GeneralActionReward(&self) -> [&ColumnData; 2] {
[&self.columns[192],&self.columns[193],]
}
pub fn OtherReward(&self) -> &ColumnData {
&self.columns[194]
}
pub fn Tomestone(&self) -> &ColumnData {
&self.columns[195]
}
pub fn TomestoneReward(&self) -> &ColumnData {
&self.columns[196]
}
pub fn TomestoneCountReward(&self) -> &ColumnData {
&self.columns[197]
}
pub fn ReputationReward(&self) -> &ColumnData {
&self.columns[198]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[199]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[200]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[201]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[202]
}
pub fn Unknown4(&self) -> &ColumnData {
&self.columns[203]
}
pub fn Unknown5(&self) -> &ColumnData {
&self.columns[204]
}
pub fn Unknown6(&self) -> &ColumnData {
&self.columns[205]
}
pub fn OptionalItemIsHQReward(&self) -> [&ColumnData; 5] {
[&self.columns[206],&self.columns[207],&self.columns[208],&self.columns[209],&self.columns[210],]
}
pub fn Id(&self) -> &ColumnData {
&self.columns[211]
}
pub fn PreviousQuest(&self) -> [&ColumnData; 3] {
[&self.columns[212],&self.columns[213],&self.columns[214],]
}
pub fn QuestLock(&self) -> [&ColumnData; 2] {
[&self.columns[215],&self.columns[216],]
}
pub fn InstanceContent(&self) -> [&ColumnData; 3] {
[&self.columns[217],&self.columns[218],&self.columns[219],]
}
pub fn IssuerStart(&self) -> &ColumnData {
&self.columns[220]
}
pub fn IssuerLocation(&self) -> &ColumnData {
&self.columns[221]
}
pub fn TargetEnd(&self) -> &ColumnData {
&self.columns[222]
}
pub fn JournalGenre(&self) -> &ColumnData {
&self.columns[223]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[224]
}
pub fn IconSpecial(&self) -> &ColumnData {
&self.columns[225]
}
pub fn MountRequired(&self) -> &ColumnData {
&self.columns[226]
}
pub fn ClassJobLevel(&self) -> [&ColumnData; 2] {
[&self.columns[227],&self.columns[228],]
}
pub fn Header(&self) -> &ColumnData {
&self.columns[229]
}
pub fn BellStart(&self) -> &ColumnData {
&self.columns[230]
}
pub fn BellEnd(&self) -> &ColumnData {
&self.columns[231]
}
pub fn BeastReputationValue(&self) -> &ColumnData {
&self.columns[232]
}
pub fn ClientBehavior(&self) -> &ColumnData {
&self.columns[233]
}
pub fn QuestClassJobSupply(&self) -> &ColumnData {
&self.columns[234]
}
pub fn PlaceName(&self) -> &ColumnData {
&self.columns[235]
}
pub fn SortKey(&self) -> &ColumnData {
&self.columns[236]
}
pub fn Expansion(&self) -> &ColumnData {
&self.columns[237]
}
pub fn ClassJobCategory0(&self) -> &ColumnData {
&self.columns[238]
}
pub fn QuestLevelOffset(&self) -> &ColumnData {
&self.columns[239]
}
pub fn ClassJobCategory1(&self) -> &ColumnData {
&self.columns[240]
}
pub fn PreviousQuestJoin(&self) -> &ColumnData {
&self.columns[241]
}
pub fn Unknown7(&self) -> &ColumnData {
&self.columns[242]
}
pub fn QuestLockJoin(&self) -> &ColumnData {
&self.columns[243]
}
pub fn Unknown8(&self) -> &ColumnData {
&self.columns[244]
}
pub fn Unknown9(&self) -> &ColumnData {
&self.columns[245]
}
pub fn ClassJobUnlock(&self) -> &ColumnData {
&self.columns[246]
}
pub fn GrandCompany(&self) -> &ColumnData {
&self.columns[247]
}
pub fn GrandCompanyRank(&self) -> &ColumnData {
&self.columns[248]
}
pub fn InstanceContentJoin(&self) -> &ColumnData {
&self.columns[249]
}
pub fn Festival(&self) -> &ColumnData {
&self.columns[250]
}
pub fn FestivalBegin(&self) -> &ColumnData {
&self.columns[251]
}
pub fn FestivalEnd(&self) -> &ColumnData {
&self.columns[252]
}
pub fn BeastTribe(&self) -> &ColumnData {
&self.columns[253]
}
pub fn BeastReputationRank(&self) -> &ColumnData {
&self.columns[254]
}
pub fn SatisfactionNpc(&self) -> &ColumnData {
&self.columns[255]
}
pub fn SatisfactionLevel(&self) -> &ColumnData {
&self.columns[256]
}
pub fn DeliveryQuest(&self) -> &ColumnData {
&self.columns[257]
}
pub fn RepeatIntervalType(&self) -> &ColumnData {
&self.columns[258]
}
pub fn QuestRepeatFlag(&self) -> &ColumnData {
&self.columns[259]
}
pub fn Type(&self) -> &ColumnData {
&self.columns[260]
}
pub fn Unknown_70(&self) -> &ColumnData {
&self.columns[261]
}
pub fn LevelMax(&self) -> &ColumnData {
&self.columns[262]
}
pub fn ClassJobRequired(&self) -> &ColumnData {
&self.columns[263]
}
pub fn QuestRewardOtherDisplay(&self) -> &ColumnData {
&self.columns[264]
}
pub fn Unknown10(&self) -> &ColumnData {
&self.columns[265]
}
pub fn EventIconType(&self) -> &ColumnData {
&self.columns[266]
}
/// 1/2 - normal daily beast tribe quests, 3 - 'exclusive' (if player's rank is not greater than max rank requirement of quests offered by npc, exactly one of the available quests will be from this pool)
pub fn DailyQuestPool(&self) -> &ColumnData {
&self.columns[267]
}
pub fn IsHouseRequired(&self) -> &ColumnData {
&self.columns[268]
}
pub fn IsRepeatable(&self) -> &ColumnData {
&self.columns[269]
}
pub fn CanCancel(&self) -> &ColumnData {
&self.columns[270]
}
pub fn Introduction(&self) -> &ColumnData {
&self.columns[271]
}
pub fn HideOfferIcon(&self) -> &ColumnData {
&self.columns[272]
}
pub fn Unknown12(&self) -> &ColumnData {
&self.columns[273]
}
pub fn Unknown13(&self) -> &ColumnData {
&self.columns[274]
}
}
