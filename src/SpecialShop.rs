//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
pub struct ItemElement<'a> {
    pub ReceiveCount: [&'a Field; 2],
    pub CurrencyCost: [&'a Field; 3],
    pub Item: [&'a Field; 2],
    pub Category: [&'a Field; 2],
    pub ItemCost: [&'a Field; 3],
    pub Quest: &'a Field,
    pub Unknown0: [&'a Field; 4],
    pub AchievementUnlock: &'a Field,
    pub Unknown2: &'a Field,
    pub CollectabilityCost: [&'a Field; 3],
    pub PatchNumber: &'a Field,
    pub HqCost: [&'a Field; 3],
    pub Unknown1: [&'a Field; 5],
    pub Order: &'a Field,
    pub ReceiveHq: [&'a Field; 2],
}
#[derive(Debug, Clone)]
pub struct SpecialShopSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl SpecialShopSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("SpecialShop")?;
        let sheet = resolver.read_excel_sheet(&exh, "SpecialShop", language)?;
        let mut index_mapping: Vec<(usize, &ExcelColumnDefinition)> = sheet
            .exh
            .column_definitions
            .iter()
            .enumerate()
            .collect();
        index_mapping.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
        let index_mapping: Vec<usize> = index_mapping
            .iter()
            .map(|(index, _)| *index)
            .collect();
        Ok(Self { sheet, index_mapping })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<SpecialShopRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<SpecialShopRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for SpecialShopSheet {
    type Row = SpecialShopRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a SpecialShopSheet {
    type Item = (u32, Vec<(u16, SpecialShopRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, SpecialShopSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, SpecialShopSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct SpecialShopRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> SpecialShopRow<'a> {
    pub fn Name(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[0]]
    }
    pub fn Item(&'a self) -> [ItemElement<'a>; 60] {
        [
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[1]],
                    &self.row.columns[self.index_mapping[2]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[3]],
                    &self.row.columns[self.index_mapping[4]],
                    &self.row.columns[self.index_mapping[5]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[6]],
                    &self.row.columns[self.index_mapping[7]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[8]],
                    &self.row.columns[self.index_mapping[9]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[10]],
                    &self.row.columns[self.index_mapping[11]],
                    &self.row.columns[self.index_mapping[12]],
                ],
                Quest: &self.row.columns[self.index_mapping[13]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[14]],
                    &self.row.columns[self.index_mapping[15]],
                    &self.row.columns[self.index_mapping[16]],
                    &self.row.columns[self.index_mapping[17]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[18]],
                Unknown2: &self.row.columns[self.index_mapping[19]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[20]],
                    &self.row.columns[self.index_mapping[21]],
                    &self.row.columns[self.index_mapping[22]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[23]],
                HqCost: [
                    &self.row.columns[self.index_mapping[24]],
                    &self.row.columns[self.index_mapping[25]],
                    &self.row.columns[self.index_mapping[26]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[27]],
                    &self.row.columns[self.index_mapping[28]],
                    &self.row.columns[self.index_mapping[29]],
                    &self.row.columns[self.index_mapping[30]],
                    &self.row.columns[self.index_mapping[31]],
                ],
                Order: &self.row.columns[self.index_mapping[32]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[33]],
                    &self.row.columns[self.index_mapping[34]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[35]],
                    &self.row.columns[self.index_mapping[36]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[37]],
                    &self.row.columns[self.index_mapping[38]],
                    &self.row.columns[self.index_mapping[39]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[40]],
                    &self.row.columns[self.index_mapping[41]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[42]],
                    &self.row.columns[self.index_mapping[43]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[44]],
                    &self.row.columns[self.index_mapping[45]],
                    &self.row.columns[self.index_mapping[46]],
                ],
                Quest: &self.row.columns[self.index_mapping[47]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[48]],
                    &self.row.columns[self.index_mapping[49]],
                    &self.row.columns[self.index_mapping[50]],
                    &self.row.columns[self.index_mapping[51]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[52]],
                Unknown2: &self.row.columns[self.index_mapping[53]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[54]],
                    &self.row.columns[self.index_mapping[55]],
                    &self.row.columns[self.index_mapping[56]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[57]],
                HqCost: [
                    &self.row.columns[self.index_mapping[58]],
                    &self.row.columns[self.index_mapping[59]],
                    &self.row.columns[self.index_mapping[60]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[61]],
                    &self.row.columns[self.index_mapping[62]],
                    &self.row.columns[self.index_mapping[63]],
                    &self.row.columns[self.index_mapping[64]],
                    &self.row.columns[self.index_mapping[65]],
                ],
                Order: &self.row.columns[self.index_mapping[66]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[67]],
                    &self.row.columns[self.index_mapping[68]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[69]],
                    &self.row.columns[self.index_mapping[70]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[71]],
                    &self.row.columns[self.index_mapping[72]],
                    &self.row.columns[self.index_mapping[73]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[74]],
                    &self.row.columns[self.index_mapping[75]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[76]],
                    &self.row.columns[self.index_mapping[77]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[78]],
                    &self.row.columns[self.index_mapping[79]],
                    &self.row.columns[self.index_mapping[80]],
                ],
                Quest: &self.row.columns[self.index_mapping[81]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[82]],
                    &self.row.columns[self.index_mapping[83]],
                    &self.row.columns[self.index_mapping[84]],
                    &self.row.columns[self.index_mapping[85]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[86]],
                Unknown2: &self.row.columns[self.index_mapping[87]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[88]],
                    &self.row.columns[self.index_mapping[89]],
                    &self.row.columns[self.index_mapping[90]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[91]],
                HqCost: [
                    &self.row.columns[self.index_mapping[92]],
                    &self.row.columns[self.index_mapping[93]],
                    &self.row.columns[self.index_mapping[94]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[95]],
                    &self.row.columns[self.index_mapping[96]],
                    &self.row.columns[self.index_mapping[97]],
                    &self.row.columns[self.index_mapping[98]],
                    &self.row.columns[self.index_mapping[99]],
                ],
                Order: &self.row.columns[self.index_mapping[100]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[101]],
                    &self.row.columns[self.index_mapping[102]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[103]],
                    &self.row.columns[self.index_mapping[104]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[105]],
                    &self.row.columns[self.index_mapping[106]],
                    &self.row.columns[self.index_mapping[107]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[108]],
                    &self.row.columns[self.index_mapping[109]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[110]],
                    &self.row.columns[self.index_mapping[111]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[112]],
                    &self.row.columns[self.index_mapping[113]],
                    &self.row.columns[self.index_mapping[114]],
                ],
                Quest: &self.row.columns[self.index_mapping[115]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[116]],
                    &self.row.columns[self.index_mapping[117]],
                    &self.row.columns[self.index_mapping[118]],
                    &self.row.columns[self.index_mapping[119]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[120]],
                Unknown2: &self.row.columns[self.index_mapping[121]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[122]],
                    &self.row.columns[self.index_mapping[123]],
                    &self.row.columns[self.index_mapping[124]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[125]],
                HqCost: [
                    &self.row.columns[self.index_mapping[126]],
                    &self.row.columns[self.index_mapping[127]],
                    &self.row.columns[self.index_mapping[128]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[129]],
                    &self.row.columns[self.index_mapping[130]],
                    &self.row.columns[self.index_mapping[131]],
                    &self.row.columns[self.index_mapping[132]],
                    &self.row.columns[self.index_mapping[133]],
                ],
                Order: &self.row.columns[self.index_mapping[134]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[135]],
                    &self.row.columns[self.index_mapping[136]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[137]],
                    &self.row.columns[self.index_mapping[138]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[139]],
                    &self.row.columns[self.index_mapping[140]],
                    &self.row.columns[self.index_mapping[141]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[142]],
                    &self.row.columns[self.index_mapping[143]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[144]],
                    &self.row.columns[self.index_mapping[145]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[146]],
                    &self.row.columns[self.index_mapping[147]],
                    &self.row.columns[self.index_mapping[148]],
                ],
                Quest: &self.row.columns[self.index_mapping[149]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[150]],
                    &self.row.columns[self.index_mapping[151]],
                    &self.row.columns[self.index_mapping[152]],
                    &self.row.columns[self.index_mapping[153]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[154]],
                Unknown2: &self.row.columns[self.index_mapping[155]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[156]],
                    &self.row.columns[self.index_mapping[157]],
                    &self.row.columns[self.index_mapping[158]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[159]],
                HqCost: [
                    &self.row.columns[self.index_mapping[160]],
                    &self.row.columns[self.index_mapping[161]],
                    &self.row.columns[self.index_mapping[162]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[163]],
                    &self.row.columns[self.index_mapping[164]],
                    &self.row.columns[self.index_mapping[165]],
                    &self.row.columns[self.index_mapping[166]],
                    &self.row.columns[self.index_mapping[167]],
                ],
                Order: &self.row.columns[self.index_mapping[168]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[169]],
                    &self.row.columns[self.index_mapping[170]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[171]],
                    &self.row.columns[self.index_mapping[172]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[173]],
                    &self.row.columns[self.index_mapping[174]],
                    &self.row.columns[self.index_mapping[175]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[176]],
                    &self.row.columns[self.index_mapping[177]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[178]],
                    &self.row.columns[self.index_mapping[179]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[180]],
                    &self.row.columns[self.index_mapping[181]],
                    &self.row.columns[self.index_mapping[182]],
                ],
                Quest: &self.row.columns[self.index_mapping[183]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[184]],
                    &self.row.columns[self.index_mapping[185]],
                    &self.row.columns[self.index_mapping[186]],
                    &self.row.columns[self.index_mapping[187]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[188]],
                Unknown2: &self.row.columns[self.index_mapping[189]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[190]],
                    &self.row.columns[self.index_mapping[191]],
                    &self.row.columns[self.index_mapping[192]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[193]],
                HqCost: [
                    &self.row.columns[self.index_mapping[194]],
                    &self.row.columns[self.index_mapping[195]],
                    &self.row.columns[self.index_mapping[196]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[197]],
                    &self.row.columns[self.index_mapping[198]],
                    &self.row.columns[self.index_mapping[199]],
                    &self.row.columns[self.index_mapping[200]],
                    &self.row.columns[self.index_mapping[201]],
                ],
                Order: &self.row.columns[self.index_mapping[202]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[203]],
                    &self.row.columns[self.index_mapping[204]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[205]],
                    &self.row.columns[self.index_mapping[206]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[207]],
                    &self.row.columns[self.index_mapping[208]],
                    &self.row.columns[self.index_mapping[209]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[210]],
                    &self.row.columns[self.index_mapping[211]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[212]],
                    &self.row.columns[self.index_mapping[213]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[214]],
                    &self.row.columns[self.index_mapping[215]],
                    &self.row.columns[self.index_mapping[216]],
                ],
                Quest: &self.row.columns[self.index_mapping[217]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[218]],
                    &self.row.columns[self.index_mapping[219]],
                    &self.row.columns[self.index_mapping[220]],
                    &self.row.columns[self.index_mapping[221]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[222]],
                Unknown2: &self.row.columns[self.index_mapping[223]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[224]],
                    &self.row.columns[self.index_mapping[225]],
                    &self.row.columns[self.index_mapping[226]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[227]],
                HqCost: [
                    &self.row.columns[self.index_mapping[228]],
                    &self.row.columns[self.index_mapping[229]],
                    &self.row.columns[self.index_mapping[230]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[231]],
                    &self.row.columns[self.index_mapping[232]],
                    &self.row.columns[self.index_mapping[233]],
                    &self.row.columns[self.index_mapping[234]],
                    &self.row.columns[self.index_mapping[235]],
                ],
                Order: &self.row.columns[self.index_mapping[236]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[237]],
                    &self.row.columns[self.index_mapping[238]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[239]],
                    &self.row.columns[self.index_mapping[240]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[241]],
                    &self.row.columns[self.index_mapping[242]],
                    &self.row.columns[self.index_mapping[243]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[244]],
                    &self.row.columns[self.index_mapping[245]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[246]],
                    &self.row.columns[self.index_mapping[247]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[248]],
                    &self.row.columns[self.index_mapping[249]],
                    &self.row.columns[self.index_mapping[250]],
                ],
                Quest: &self.row.columns[self.index_mapping[251]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[252]],
                    &self.row.columns[self.index_mapping[253]],
                    &self.row.columns[self.index_mapping[254]],
                    &self.row.columns[self.index_mapping[255]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[256]],
                Unknown2: &self.row.columns[self.index_mapping[257]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[258]],
                    &self.row.columns[self.index_mapping[259]],
                    &self.row.columns[self.index_mapping[260]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[261]],
                HqCost: [
                    &self.row.columns[self.index_mapping[262]],
                    &self.row.columns[self.index_mapping[263]],
                    &self.row.columns[self.index_mapping[264]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[265]],
                    &self.row.columns[self.index_mapping[266]],
                    &self.row.columns[self.index_mapping[267]],
                    &self.row.columns[self.index_mapping[268]],
                    &self.row.columns[self.index_mapping[269]],
                ],
                Order: &self.row.columns[self.index_mapping[270]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[271]],
                    &self.row.columns[self.index_mapping[272]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[273]],
                    &self.row.columns[self.index_mapping[274]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[275]],
                    &self.row.columns[self.index_mapping[276]],
                    &self.row.columns[self.index_mapping[277]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[278]],
                    &self.row.columns[self.index_mapping[279]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[280]],
                    &self.row.columns[self.index_mapping[281]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[282]],
                    &self.row.columns[self.index_mapping[283]],
                    &self.row.columns[self.index_mapping[284]],
                ],
                Quest: &self.row.columns[self.index_mapping[285]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[286]],
                    &self.row.columns[self.index_mapping[287]],
                    &self.row.columns[self.index_mapping[288]],
                    &self.row.columns[self.index_mapping[289]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[290]],
                Unknown2: &self.row.columns[self.index_mapping[291]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[292]],
                    &self.row.columns[self.index_mapping[293]],
                    &self.row.columns[self.index_mapping[294]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[295]],
                HqCost: [
                    &self.row.columns[self.index_mapping[296]],
                    &self.row.columns[self.index_mapping[297]],
                    &self.row.columns[self.index_mapping[298]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[299]],
                    &self.row.columns[self.index_mapping[300]],
                    &self.row.columns[self.index_mapping[301]],
                    &self.row.columns[self.index_mapping[302]],
                    &self.row.columns[self.index_mapping[303]],
                ],
                Order: &self.row.columns[self.index_mapping[304]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[305]],
                    &self.row.columns[self.index_mapping[306]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[307]],
                    &self.row.columns[self.index_mapping[308]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[309]],
                    &self.row.columns[self.index_mapping[310]],
                    &self.row.columns[self.index_mapping[311]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[312]],
                    &self.row.columns[self.index_mapping[313]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[314]],
                    &self.row.columns[self.index_mapping[315]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[316]],
                    &self.row.columns[self.index_mapping[317]],
                    &self.row.columns[self.index_mapping[318]],
                ],
                Quest: &self.row.columns[self.index_mapping[319]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[320]],
                    &self.row.columns[self.index_mapping[321]],
                    &self.row.columns[self.index_mapping[322]],
                    &self.row.columns[self.index_mapping[323]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[324]],
                Unknown2: &self.row.columns[self.index_mapping[325]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[326]],
                    &self.row.columns[self.index_mapping[327]],
                    &self.row.columns[self.index_mapping[328]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[329]],
                HqCost: [
                    &self.row.columns[self.index_mapping[330]],
                    &self.row.columns[self.index_mapping[331]],
                    &self.row.columns[self.index_mapping[332]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[333]],
                    &self.row.columns[self.index_mapping[334]],
                    &self.row.columns[self.index_mapping[335]],
                    &self.row.columns[self.index_mapping[336]],
                    &self.row.columns[self.index_mapping[337]],
                ],
                Order: &self.row.columns[self.index_mapping[338]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[339]],
                    &self.row.columns[self.index_mapping[340]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[341]],
                    &self.row.columns[self.index_mapping[342]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[343]],
                    &self.row.columns[self.index_mapping[344]],
                    &self.row.columns[self.index_mapping[345]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[346]],
                    &self.row.columns[self.index_mapping[347]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[348]],
                    &self.row.columns[self.index_mapping[349]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[350]],
                    &self.row.columns[self.index_mapping[351]],
                    &self.row.columns[self.index_mapping[352]],
                ],
                Quest: &self.row.columns[self.index_mapping[353]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[354]],
                    &self.row.columns[self.index_mapping[355]],
                    &self.row.columns[self.index_mapping[356]],
                    &self.row.columns[self.index_mapping[357]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[358]],
                Unknown2: &self.row.columns[self.index_mapping[359]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[360]],
                    &self.row.columns[self.index_mapping[361]],
                    &self.row.columns[self.index_mapping[362]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[363]],
                HqCost: [
                    &self.row.columns[self.index_mapping[364]],
                    &self.row.columns[self.index_mapping[365]],
                    &self.row.columns[self.index_mapping[366]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[367]],
                    &self.row.columns[self.index_mapping[368]],
                    &self.row.columns[self.index_mapping[369]],
                    &self.row.columns[self.index_mapping[370]],
                    &self.row.columns[self.index_mapping[371]],
                ],
                Order: &self.row.columns[self.index_mapping[372]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[373]],
                    &self.row.columns[self.index_mapping[374]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[375]],
                    &self.row.columns[self.index_mapping[376]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[377]],
                    &self.row.columns[self.index_mapping[378]],
                    &self.row.columns[self.index_mapping[379]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[380]],
                    &self.row.columns[self.index_mapping[381]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[382]],
                    &self.row.columns[self.index_mapping[383]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[384]],
                    &self.row.columns[self.index_mapping[385]],
                    &self.row.columns[self.index_mapping[386]],
                ],
                Quest: &self.row.columns[self.index_mapping[387]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[388]],
                    &self.row.columns[self.index_mapping[389]],
                    &self.row.columns[self.index_mapping[390]],
                    &self.row.columns[self.index_mapping[391]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[392]],
                Unknown2: &self.row.columns[self.index_mapping[393]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[394]],
                    &self.row.columns[self.index_mapping[395]],
                    &self.row.columns[self.index_mapping[396]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[397]],
                HqCost: [
                    &self.row.columns[self.index_mapping[398]],
                    &self.row.columns[self.index_mapping[399]],
                    &self.row.columns[self.index_mapping[400]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[401]],
                    &self.row.columns[self.index_mapping[402]],
                    &self.row.columns[self.index_mapping[403]],
                    &self.row.columns[self.index_mapping[404]],
                    &self.row.columns[self.index_mapping[405]],
                ],
                Order: &self.row.columns[self.index_mapping[406]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[407]],
                    &self.row.columns[self.index_mapping[408]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[409]],
                    &self.row.columns[self.index_mapping[410]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[411]],
                    &self.row.columns[self.index_mapping[412]],
                    &self.row.columns[self.index_mapping[413]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[414]],
                    &self.row.columns[self.index_mapping[415]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[416]],
                    &self.row.columns[self.index_mapping[417]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[418]],
                    &self.row.columns[self.index_mapping[419]],
                    &self.row.columns[self.index_mapping[420]],
                ],
                Quest: &self.row.columns[self.index_mapping[421]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[422]],
                    &self.row.columns[self.index_mapping[423]],
                    &self.row.columns[self.index_mapping[424]],
                    &self.row.columns[self.index_mapping[425]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[426]],
                Unknown2: &self.row.columns[self.index_mapping[427]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[428]],
                    &self.row.columns[self.index_mapping[429]],
                    &self.row.columns[self.index_mapping[430]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[431]],
                HqCost: [
                    &self.row.columns[self.index_mapping[432]],
                    &self.row.columns[self.index_mapping[433]],
                    &self.row.columns[self.index_mapping[434]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[435]],
                    &self.row.columns[self.index_mapping[436]],
                    &self.row.columns[self.index_mapping[437]],
                    &self.row.columns[self.index_mapping[438]],
                    &self.row.columns[self.index_mapping[439]],
                ],
                Order: &self.row.columns[self.index_mapping[440]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[441]],
                    &self.row.columns[self.index_mapping[442]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[443]],
                    &self.row.columns[self.index_mapping[444]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[445]],
                    &self.row.columns[self.index_mapping[446]],
                    &self.row.columns[self.index_mapping[447]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[448]],
                    &self.row.columns[self.index_mapping[449]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[450]],
                    &self.row.columns[self.index_mapping[451]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[452]],
                    &self.row.columns[self.index_mapping[453]],
                    &self.row.columns[self.index_mapping[454]],
                ],
                Quest: &self.row.columns[self.index_mapping[455]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[456]],
                    &self.row.columns[self.index_mapping[457]],
                    &self.row.columns[self.index_mapping[458]],
                    &self.row.columns[self.index_mapping[459]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[460]],
                Unknown2: &self.row.columns[self.index_mapping[461]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[462]],
                    &self.row.columns[self.index_mapping[463]],
                    &self.row.columns[self.index_mapping[464]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[465]],
                HqCost: [
                    &self.row.columns[self.index_mapping[466]],
                    &self.row.columns[self.index_mapping[467]],
                    &self.row.columns[self.index_mapping[468]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[469]],
                    &self.row.columns[self.index_mapping[470]],
                    &self.row.columns[self.index_mapping[471]],
                    &self.row.columns[self.index_mapping[472]],
                    &self.row.columns[self.index_mapping[473]],
                ],
                Order: &self.row.columns[self.index_mapping[474]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[475]],
                    &self.row.columns[self.index_mapping[476]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[477]],
                    &self.row.columns[self.index_mapping[478]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[479]],
                    &self.row.columns[self.index_mapping[480]],
                    &self.row.columns[self.index_mapping[481]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[482]],
                    &self.row.columns[self.index_mapping[483]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[484]],
                    &self.row.columns[self.index_mapping[485]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[486]],
                    &self.row.columns[self.index_mapping[487]],
                    &self.row.columns[self.index_mapping[488]],
                ],
                Quest: &self.row.columns[self.index_mapping[489]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[490]],
                    &self.row.columns[self.index_mapping[491]],
                    &self.row.columns[self.index_mapping[492]],
                    &self.row.columns[self.index_mapping[493]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[494]],
                Unknown2: &self.row.columns[self.index_mapping[495]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[496]],
                    &self.row.columns[self.index_mapping[497]],
                    &self.row.columns[self.index_mapping[498]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[499]],
                HqCost: [
                    &self.row.columns[self.index_mapping[500]],
                    &self.row.columns[self.index_mapping[501]],
                    &self.row.columns[self.index_mapping[502]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[503]],
                    &self.row.columns[self.index_mapping[504]],
                    &self.row.columns[self.index_mapping[505]],
                    &self.row.columns[self.index_mapping[506]],
                    &self.row.columns[self.index_mapping[507]],
                ],
                Order: &self.row.columns[self.index_mapping[508]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[509]],
                    &self.row.columns[self.index_mapping[510]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[511]],
                    &self.row.columns[self.index_mapping[512]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[513]],
                    &self.row.columns[self.index_mapping[514]],
                    &self.row.columns[self.index_mapping[515]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[516]],
                    &self.row.columns[self.index_mapping[517]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[518]],
                    &self.row.columns[self.index_mapping[519]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[520]],
                    &self.row.columns[self.index_mapping[521]],
                    &self.row.columns[self.index_mapping[522]],
                ],
                Quest: &self.row.columns[self.index_mapping[523]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[524]],
                    &self.row.columns[self.index_mapping[525]],
                    &self.row.columns[self.index_mapping[526]],
                    &self.row.columns[self.index_mapping[527]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[528]],
                Unknown2: &self.row.columns[self.index_mapping[529]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[530]],
                    &self.row.columns[self.index_mapping[531]],
                    &self.row.columns[self.index_mapping[532]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[533]],
                HqCost: [
                    &self.row.columns[self.index_mapping[534]],
                    &self.row.columns[self.index_mapping[535]],
                    &self.row.columns[self.index_mapping[536]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[537]],
                    &self.row.columns[self.index_mapping[538]],
                    &self.row.columns[self.index_mapping[539]],
                    &self.row.columns[self.index_mapping[540]],
                    &self.row.columns[self.index_mapping[541]],
                ],
                Order: &self.row.columns[self.index_mapping[542]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[543]],
                    &self.row.columns[self.index_mapping[544]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[545]],
                    &self.row.columns[self.index_mapping[546]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[547]],
                    &self.row.columns[self.index_mapping[548]],
                    &self.row.columns[self.index_mapping[549]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[550]],
                    &self.row.columns[self.index_mapping[551]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[552]],
                    &self.row.columns[self.index_mapping[553]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[554]],
                    &self.row.columns[self.index_mapping[555]],
                    &self.row.columns[self.index_mapping[556]],
                ],
                Quest: &self.row.columns[self.index_mapping[557]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[558]],
                    &self.row.columns[self.index_mapping[559]],
                    &self.row.columns[self.index_mapping[560]],
                    &self.row.columns[self.index_mapping[561]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[562]],
                Unknown2: &self.row.columns[self.index_mapping[563]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[564]],
                    &self.row.columns[self.index_mapping[565]],
                    &self.row.columns[self.index_mapping[566]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[567]],
                HqCost: [
                    &self.row.columns[self.index_mapping[568]],
                    &self.row.columns[self.index_mapping[569]],
                    &self.row.columns[self.index_mapping[570]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[571]],
                    &self.row.columns[self.index_mapping[572]],
                    &self.row.columns[self.index_mapping[573]],
                    &self.row.columns[self.index_mapping[574]],
                    &self.row.columns[self.index_mapping[575]],
                ],
                Order: &self.row.columns[self.index_mapping[576]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[577]],
                    &self.row.columns[self.index_mapping[578]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[579]],
                    &self.row.columns[self.index_mapping[580]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[581]],
                    &self.row.columns[self.index_mapping[582]],
                    &self.row.columns[self.index_mapping[583]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[584]],
                    &self.row.columns[self.index_mapping[585]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[586]],
                    &self.row.columns[self.index_mapping[587]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[588]],
                    &self.row.columns[self.index_mapping[589]],
                    &self.row.columns[self.index_mapping[590]],
                ],
                Quest: &self.row.columns[self.index_mapping[591]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[592]],
                    &self.row.columns[self.index_mapping[593]],
                    &self.row.columns[self.index_mapping[594]],
                    &self.row.columns[self.index_mapping[595]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[596]],
                Unknown2: &self.row.columns[self.index_mapping[597]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[598]],
                    &self.row.columns[self.index_mapping[599]],
                    &self.row.columns[self.index_mapping[600]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[601]],
                HqCost: [
                    &self.row.columns[self.index_mapping[602]],
                    &self.row.columns[self.index_mapping[603]],
                    &self.row.columns[self.index_mapping[604]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[605]],
                    &self.row.columns[self.index_mapping[606]],
                    &self.row.columns[self.index_mapping[607]],
                    &self.row.columns[self.index_mapping[608]],
                    &self.row.columns[self.index_mapping[609]],
                ],
                Order: &self.row.columns[self.index_mapping[610]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[611]],
                    &self.row.columns[self.index_mapping[612]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[613]],
                    &self.row.columns[self.index_mapping[614]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[615]],
                    &self.row.columns[self.index_mapping[616]],
                    &self.row.columns[self.index_mapping[617]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[618]],
                    &self.row.columns[self.index_mapping[619]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[620]],
                    &self.row.columns[self.index_mapping[621]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[622]],
                    &self.row.columns[self.index_mapping[623]],
                    &self.row.columns[self.index_mapping[624]],
                ],
                Quest: &self.row.columns[self.index_mapping[625]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[626]],
                    &self.row.columns[self.index_mapping[627]],
                    &self.row.columns[self.index_mapping[628]],
                    &self.row.columns[self.index_mapping[629]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[630]],
                Unknown2: &self.row.columns[self.index_mapping[631]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[632]],
                    &self.row.columns[self.index_mapping[633]],
                    &self.row.columns[self.index_mapping[634]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[635]],
                HqCost: [
                    &self.row.columns[self.index_mapping[636]],
                    &self.row.columns[self.index_mapping[637]],
                    &self.row.columns[self.index_mapping[638]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[639]],
                    &self.row.columns[self.index_mapping[640]],
                    &self.row.columns[self.index_mapping[641]],
                    &self.row.columns[self.index_mapping[642]],
                    &self.row.columns[self.index_mapping[643]],
                ],
                Order: &self.row.columns[self.index_mapping[644]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[645]],
                    &self.row.columns[self.index_mapping[646]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[647]],
                    &self.row.columns[self.index_mapping[648]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[649]],
                    &self.row.columns[self.index_mapping[650]],
                    &self.row.columns[self.index_mapping[651]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[652]],
                    &self.row.columns[self.index_mapping[653]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[654]],
                    &self.row.columns[self.index_mapping[655]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[656]],
                    &self.row.columns[self.index_mapping[657]],
                    &self.row.columns[self.index_mapping[658]],
                ],
                Quest: &self.row.columns[self.index_mapping[659]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[660]],
                    &self.row.columns[self.index_mapping[661]],
                    &self.row.columns[self.index_mapping[662]],
                    &self.row.columns[self.index_mapping[663]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[664]],
                Unknown2: &self.row.columns[self.index_mapping[665]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[666]],
                    &self.row.columns[self.index_mapping[667]],
                    &self.row.columns[self.index_mapping[668]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[669]],
                HqCost: [
                    &self.row.columns[self.index_mapping[670]],
                    &self.row.columns[self.index_mapping[671]],
                    &self.row.columns[self.index_mapping[672]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[673]],
                    &self.row.columns[self.index_mapping[674]],
                    &self.row.columns[self.index_mapping[675]],
                    &self.row.columns[self.index_mapping[676]],
                    &self.row.columns[self.index_mapping[677]],
                ],
                Order: &self.row.columns[self.index_mapping[678]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[679]],
                    &self.row.columns[self.index_mapping[680]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[681]],
                    &self.row.columns[self.index_mapping[682]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[683]],
                    &self.row.columns[self.index_mapping[684]],
                    &self.row.columns[self.index_mapping[685]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[686]],
                    &self.row.columns[self.index_mapping[687]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[688]],
                    &self.row.columns[self.index_mapping[689]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[690]],
                    &self.row.columns[self.index_mapping[691]],
                    &self.row.columns[self.index_mapping[692]],
                ],
                Quest: &self.row.columns[self.index_mapping[693]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[694]],
                    &self.row.columns[self.index_mapping[695]],
                    &self.row.columns[self.index_mapping[696]],
                    &self.row.columns[self.index_mapping[697]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[698]],
                Unknown2: &self.row.columns[self.index_mapping[699]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[700]],
                    &self.row.columns[self.index_mapping[701]],
                    &self.row.columns[self.index_mapping[702]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[703]],
                HqCost: [
                    &self.row.columns[self.index_mapping[704]],
                    &self.row.columns[self.index_mapping[705]],
                    &self.row.columns[self.index_mapping[706]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[707]],
                    &self.row.columns[self.index_mapping[708]],
                    &self.row.columns[self.index_mapping[709]],
                    &self.row.columns[self.index_mapping[710]],
                    &self.row.columns[self.index_mapping[711]],
                ],
                Order: &self.row.columns[self.index_mapping[712]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[713]],
                    &self.row.columns[self.index_mapping[714]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[715]],
                    &self.row.columns[self.index_mapping[716]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[717]],
                    &self.row.columns[self.index_mapping[718]],
                    &self.row.columns[self.index_mapping[719]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[720]],
                    &self.row.columns[self.index_mapping[721]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[722]],
                    &self.row.columns[self.index_mapping[723]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[724]],
                    &self.row.columns[self.index_mapping[725]],
                    &self.row.columns[self.index_mapping[726]],
                ],
                Quest: &self.row.columns[self.index_mapping[727]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[728]],
                    &self.row.columns[self.index_mapping[729]],
                    &self.row.columns[self.index_mapping[730]],
                    &self.row.columns[self.index_mapping[731]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[732]],
                Unknown2: &self.row.columns[self.index_mapping[733]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[734]],
                    &self.row.columns[self.index_mapping[735]],
                    &self.row.columns[self.index_mapping[736]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[737]],
                HqCost: [
                    &self.row.columns[self.index_mapping[738]],
                    &self.row.columns[self.index_mapping[739]],
                    &self.row.columns[self.index_mapping[740]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[741]],
                    &self.row.columns[self.index_mapping[742]],
                    &self.row.columns[self.index_mapping[743]],
                    &self.row.columns[self.index_mapping[744]],
                    &self.row.columns[self.index_mapping[745]],
                ],
                Order: &self.row.columns[self.index_mapping[746]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[747]],
                    &self.row.columns[self.index_mapping[748]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[749]],
                    &self.row.columns[self.index_mapping[750]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[751]],
                    &self.row.columns[self.index_mapping[752]],
                    &self.row.columns[self.index_mapping[753]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[754]],
                    &self.row.columns[self.index_mapping[755]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[756]],
                    &self.row.columns[self.index_mapping[757]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[758]],
                    &self.row.columns[self.index_mapping[759]],
                    &self.row.columns[self.index_mapping[760]],
                ],
                Quest: &self.row.columns[self.index_mapping[761]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[762]],
                    &self.row.columns[self.index_mapping[763]],
                    &self.row.columns[self.index_mapping[764]],
                    &self.row.columns[self.index_mapping[765]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[766]],
                Unknown2: &self.row.columns[self.index_mapping[767]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[768]],
                    &self.row.columns[self.index_mapping[769]],
                    &self.row.columns[self.index_mapping[770]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[771]],
                HqCost: [
                    &self.row.columns[self.index_mapping[772]],
                    &self.row.columns[self.index_mapping[773]],
                    &self.row.columns[self.index_mapping[774]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[775]],
                    &self.row.columns[self.index_mapping[776]],
                    &self.row.columns[self.index_mapping[777]],
                    &self.row.columns[self.index_mapping[778]],
                    &self.row.columns[self.index_mapping[779]],
                ],
                Order: &self.row.columns[self.index_mapping[780]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[781]],
                    &self.row.columns[self.index_mapping[782]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[783]],
                    &self.row.columns[self.index_mapping[784]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[785]],
                    &self.row.columns[self.index_mapping[786]],
                    &self.row.columns[self.index_mapping[787]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[788]],
                    &self.row.columns[self.index_mapping[789]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[790]],
                    &self.row.columns[self.index_mapping[791]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[792]],
                    &self.row.columns[self.index_mapping[793]],
                    &self.row.columns[self.index_mapping[794]],
                ],
                Quest: &self.row.columns[self.index_mapping[795]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[796]],
                    &self.row.columns[self.index_mapping[797]],
                    &self.row.columns[self.index_mapping[798]],
                    &self.row.columns[self.index_mapping[799]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[800]],
                Unknown2: &self.row.columns[self.index_mapping[801]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[802]],
                    &self.row.columns[self.index_mapping[803]],
                    &self.row.columns[self.index_mapping[804]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[805]],
                HqCost: [
                    &self.row.columns[self.index_mapping[806]],
                    &self.row.columns[self.index_mapping[807]],
                    &self.row.columns[self.index_mapping[808]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[809]],
                    &self.row.columns[self.index_mapping[810]],
                    &self.row.columns[self.index_mapping[811]],
                    &self.row.columns[self.index_mapping[812]],
                    &self.row.columns[self.index_mapping[813]],
                ],
                Order: &self.row.columns[self.index_mapping[814]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[815]],
                    &self.row.columns[self.index_mapping[816]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[817]],
                    &self.row.columns[self.index_mapping[818]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[819]],
                    &self.row.columns[self.index_mapping[820]],
                    &self.row.columns[self.index_mapping[821]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[822]],
                    &self.row.columns[self.index_mapping[823]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[824]],
                    &self.row.columns[self.index_mapping[825]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[826]],
                    &self.row.columns[self.index_mapping[827]],
                    &self.row.columns[self.index_mapping[828]],
                ],
                Quest: &self.row.columns[self.index_mapping[829]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[830]],
                    &self.row.columns[self.index_mapping[831]],
                    &self.row.columns[self.index_mapping[832]],
                    &self.row.columns[self.index_mapping[833]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[834]],
                Unknown2: &self.row.columns[self.index_mapping[835]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[836]],
                    &self.row.columns[self.index_mapping[837]],
                    &self.row.columns[self.index_mapping[838]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[839]],
                HqCost: [
                    &self.row.columns[self.index_mapping[840]],
                    &self.row.columns[self.index_mapping[841]],
                    &self.row.columns[self.index_mapping[842]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[843]],
                    &self.row.columns[self.index_mapping[844]],
                    &self.row.columns[self.index_mapping[845]],
                    &self.row.columns[self.index_mapping[846]],
                    &self.row.columns[self.index_mapping[847]],
                ],
                Order: &self.row.columns[self.index_mapping[848]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[849]],
                    &self.row.columns[self.index_mapping[850]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[851]],
                    &self.row.columns[self.index_mapping[852]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[853]],
                    &self.row.columns[self.index_mapping[854]],
                    &self.row.columns[self.index_mapping[855]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[856]],
                    &self.row.columns[self.index_mapping[857]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[858]],
                    &self.row.columns[self.index_mapping[859]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[860]],
                    &self.row.columns[self.index_mapping[861]],
                    &self.row.columns[self.index_mapping[862]],
                ],
                Quest: &self.row.columns[self.index_mapping[863]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[864]],
                    &self.row.columns[self.index_mapping[865]],
                    &self.row.columns[self.index_mapping[866]],
                    &self.row.columns[self.index_mapping[867]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[868]],
                Unknown2: &self.row.columns[self.index_mapping[869]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[870]],
                    &self.row.columns[self.index_mapping[871]],
                    &self.row.columns[self.index_mapping[872]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[873]],
                HqCost: [
                    &self.row.columns[self.index_mapping[874]],
                    &self.row.columns[self.index_mapping[875]],
                    &self.row.columns[self.index_mapping[876]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[877]],
                    &self.row.columns[self.index_mapping[878]],
                    &self.row.columns[self.index_mapping[879]],
                    &self.row.columns[self.index_mapping[880]],
                    &self.row.columns[self.index_mapping[881]],
                ],
                Order: &self.row.columns[self.index_mapping[882]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[883]],
                    &self.row.columns[self.index_mapping[884]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[885]],
                    &self.row.columns[self.index_mapping[886]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[887]],
                    &self.row.columns[self.index_mapping[888]],
                    &self.row.columns[self.index_mapping[889]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[890]],
                    &self.row.columns[self.index_mapping[891]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[892]],
                    &self.row.columns[self.index_mapping[893]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[894]],
                    &self.row.columns[self.index_mapping[895]],
                    &self.row.columns[self.index_mapping[896]],
                ],
                Quest: &self.row.columns[self.index_mapping[897]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[898]],
                    &self.row.columns[self.index_mapping[899]],
                    &self.row.columns[self.index_mapping[900]],
                    &self.row.columns[self.index_mapping[901]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[902]],
                Unknown2: &self.row.columns[self.index_mapping[903]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[904]],
                    &self.row.columns[self.index_mapping[905]],
                    &self.row.columns[self.index_mapping[906]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[907]],
                HqCost: [
                    &self.row.columns[self.index_mapping[908]],
                    &self.row.columns[self.index_mapping[909]],
                    &self.row.columns[self.index_mapping[910]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[911]],
                    &self.row.columns[self.index_mapping[912]],
                    &self.row.columns[self.index_mapping[913]],
                    &self.row.columns[self.index_mapping[914]],
                    &self.row.columns[self.index_mapping[915]],
                ],
                Order: &self.row.columns[self.index_mapping[916]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[917]],
                    &self.row.columns[self.index_mapping[918]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[919]],
                    &self.row.columns[self.index_mapping[920]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[921]],
                    &self.row.columns[self.index_mapping[922]],
                    &self.row.columns[self.index_mapping[923]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[924]],
                    &self.row.columns[self.index_mapping[925]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[926]],
                    &self.row.columns[self.index_mapping[927]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[928]],
                    &self.row.columns[self.index_mapping[929]],
                    &self.row.columns[self.index_mapping[930]],
                ],
                Quest: &self.row.columns[self.index_mapping[931]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[932]],
                    &self.row.columns[self.index_mapping[933]],
                    &self.row.columns[self.index_mapping[934]],
                    &self.row.columns[self.index_mapping[935]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[936]],
                Unknown2: &self.row.columns[self.index_mapping[937]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[938]],
                    &self.row.columns[self.index_mapping[939]],
                    &self.row.columns[self.index_mapping[940]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[941]],
                HqCost: [
                    &self.row.columns[self.index_mapping[942]],
                    &self.row.columns[self.index_mapping[943]],
                    &self.row.columns[self.index_mapping[944]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[945]],
                    &self.row.columns[self.index_mapping[946]],
                    &self.row.columns[self.index_mapping[947]],
                    &self.row.columns[self.index_mapping[948]],
                    &self.row.columns[self.index_mapping[949]],
                ],
                Order: &self.row.columns[self.index_mapping[950]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[951]],
                    &self.row.columns[self.index_mapping[952]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[953]],
                    &self.row.columns[self.index_mapping[954]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[955]],
                    &self.row.columns[self.index_mapping[956]],
                    &self.row.columns[self.index_mapping[957]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[958]],
                    &self.row.columns[self.index_mapping[959]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[960]],
                    &self.row.columns[self.index_mapping[961]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[962]],
                    &self.row.columns[self.index_mapping[963]],
                    &self.row.columns[self.index_mapping[964]],
                ],
                Quest: &self.row.columns[self.index_mapping[965]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[966]],
                    &self.row.columns[self.index_mapping[967]],
                    &self.row.columns[self.index_mapping[968]],
                    &self.row.columns[self.index_mapping[969]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[970]],
                Unknown2: &self.row.columns[self.index_mapping[971]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[972]],
                    &self.row.columns[self.index_mapping[973]],
                    &self.row.columns[self.index_mapping[974]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[975]],
                HqCost: [
                    &self.row.columns[self.index_mapping[976]],
                    &self.row.columns[self.index_mapping[977]],
                    &self.row.columns[self.index_mapping[978]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[979]],
                    &self.row.columns[self.index_mapping[980]],
                    &self.row.columns[self.index_mapping[981]],
                    &self.row.columns[self.index_mapping[982]],
                    &self.row.columns[self.index_mapping[983]],
                ],
                Order: &self.row.columns[self.index_mapping[984]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[985]],
                    &self.row.columns[self.index_mapping[986]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[987]],
                    &self.row.columns[self.index_mapping[988]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[989]],
                    &self.row.columns[self.index_mapping[990]],
                    &self.row.columns[self.index_mapping[991]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[992]],
                    &self.row.columns[self.index_mapping[993]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[994]],
                    &self.row.columns[self.index_mapping[995]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[996]],
                    &self.row.columns[self.index_mapping[997]],
                    &self.row.columns[self.index_mapping[998]],
                ],
                Quest: &self.row.columns[self.index_mapping[999]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[1000]],
                    &self.row.columns[self.index_mapping[1001]],
                    &self.row.columns[self.index_mapping[1002]],
                    &self.row.columns[self.index_mapping[1003]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[1004]],
                Unknown2: &self.row.columns[self.index_mapping[1005]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[1006]],
                    &self.row.columns[self.index_mapping[1007]],
                    &self.row.columns[self.index_mapping[1008]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[1009]],
                HqCost: [
                    &self.row.columns[self.index_mapping[1010]],
                    &self.row.columns[self.index_mapping[1011]],
                    &self.row.columns[self.index_mapping[1012]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[1013]],
                    &self.row.columns[self.index_mapping[1014]],
                    &self.row.columns[self.index_mapping[1015]],
                    &self.row.columns[self.index_mapping[1016]],
                    &self.row.columns[self.index_mapping[1017]],
                ],
                Order: &self.row.columns[self.index_mapping[1018]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[1019]],
                    &self.row.columns[self.index_mapping[1020]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[1021]],
                    &self.row.columns[self.index_mapping[1022]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[1023]],
                    &self.row.columns[self.index_mapping[1024]],
                    &self.row.columns[self.index_mapping[1025]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[1026]],
                    &self.row.columns[self.index_mapping[1027]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[1028]],
                    &self.row.columns[self.index_mapping[1029]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[1030]],
                    &self.row.columns[self.index_mapping[1031]],
                    &self.row.columns[self.index_mapping[1032]],
                ],
                Quest: &self.row.columns[self.index_mapping[1033]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[1034]],
                    &self.row.columns[self.index_mapping[1035]],
                    &self.row.columns[self.index_mapping[1036]],
                    &self.row.columns[self.index_mapping[1037]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[1038]],
                Unknown2: &self.row.columns[self.index_mapping[1039]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[1040]],
                    &self.row.columns[self.index_mapping[1041]],
                    &self.row.columns[self.index_mapping[1042]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[1043]],
                HqCost: [
                    &self.row.columns[self.index_mapping[1044]],
                    &self.row.columns[self.index_mapping[1045]],
                    &self.row.columns[self.index_mapping[1046]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[1047]],
                    &self.row.columns[self.index_mapping[1048]],
                    &self.row.columns[self.index_mapping[1049]],
                    &self.row.columns[self.index_mapping[1050]],
                    &self.row.columns[self.index_mapping[1051]],
                ],
                Order: &self.row.columns[self.index_mapping[1052]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[1053]],
                    &self.row.columns[self.index_mapping[1054]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[1055]],
                    &self.row.columns[self.index_mapping[1056]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[1057]],
                    &self.row.columns[self.index_mapping[1058]],
                    &self.row.columns[self.index_mapping[1059]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[1060]],
                    &self.row.columns[self.index_mapping[1061]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[1062]],
                    &self.row.columns[self.index_mapping[1063]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[1064]],
                    &self.row.columns[self.index_mapping[1065]],
                    &self.row.columns[self.index_mapping[1066]],
                ],
                Quest: &self.row.columns[self.index_mapping[1067]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[1068]],
                    &self.row.columns[self.index_mapping[1069]],
                    &self.row.columns[self.index_mapping[1070]],
                    &self.row.columns[self.index_mapping[1071]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[1072]],
                Unknown2: &self.row.columns[self.index_mapping[1073]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[1074]],
                    &self.row.columns[self.index_mapping[1075]],
                    &self.row.columns[self.index_mapping[1076]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[1077]],
                HqCost: [
                    &self.row.columns[self.index_mapping[1078]],
                    &self.row.columns[self.index_mapping[1079]],
                    &self.row.columns[self.index_mapping[1080]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[1081]],
                    &self.row.columns[self.index_mapping[1082]],
                    &self.row.columns[self.index_mapping[1083]],
                    &self.row.columns[self.index_mapping[1084]],
                    &self.row.columns[self.index_mapping[1085]],
                ],
                Order: &self.row.columns[self.index_mapping[1086]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[1087]],
                    &self.row.columns[self.index_mapping[1088]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[1089]],
                    &self.row.columns[self.index_mapping[1090]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[1091]],
                    &self.row.columns[self.index_mapping[1092]],
                    &self.row.columns[self.index_mapping[1093]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[1094]],
                    &self.row.columns[self.index_mapping[1095]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[1096]],
                    &self.row.columns[self.index_mapping[1097]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[1098]],
                    &self.row.columns[self.index_mapping[1099]],
                    &self.row.columns[self.index_mapping[1100]],
                ],
                Quest: &self.row.columns[self.index_mapping[1101]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[1102]],
                    &self.row.columns[self.index_mapping[1103]],
                    &self.row.columns[self.index_mapping[1104]],
                    &self.row.columns[self.index_mapping[1105]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[1106]],
                Unknown2: &self.row.columns[self.index_mapping[1107]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[1108]],
                    &self.row.columns[self.index_mapping[1109]],
                    &self.row.columns[self.index_mapping[1110]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[1111]],
                HqCost: [
                    &self.row.columns[self.index_mapping[1112]],
                    &self.row.columns[self.index_mapping[1113]],
                    &self.row.columns[self.index_mapping[1114]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[1115]],
                    &self.row.columns[self.index_mapping[1116]],
                    &self.row.columns[self.index_mapping[1117]],
                    &self.row.columns[self.index_mapping[1118]],
                    &self.row.columns[self.index_mapping[1119]],
                ],
                Order: &self.row.columns[self.index_mapping[1120]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[1121]],
                    &self.row.columns[self.index_mapping[1122]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[1123]],
                    &self.row.columns[self.index_mapping[1124]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[1125]],
                    &self.row.columns[self.index_mapping[1126]],
                    &self.row.columns[self.index_mapping[1127]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[1128]],
                    &self.row.columns[self.index_mapping[1129]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[1130]],
                    &self.row.columns[self.index_mapping[1131]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[1132]],
                    &self.row.columns[self.index_mapping[1133]],
                    &self.row.columns[self.index_mapping[1134]],
                ],
                Quest: &self.row.columns[self.index_mapping[1135]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[1136]],
                    &self.row.columns[self.index_mapping[1137]],
                    &self.row.columns[self.index_mapping[1138]],
                    &self.row.columns[self.index_mapping[1139]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[1140]],
                Unknown2: &self.row.columns[self.index_mapping[1141]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[1142]],
                    &self.row.columns[self.index_mapping[1143]],
                    &self.row.columns[self.index_mapping[1144]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[1145]],
                HqCost: [
                    &self.row.columns[self.index_mapping[1146]],
                    &self.row.columns[self.index_mapping[1147]],
                    &self.row.columns[self.index_mapping[1148]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[1149]],
                    &self.row.columns[self.index_mapping[1150]],
                    &self.row.columns[self.index_mapping[1151]],
                    &self.row.columns[self.index_mapping[1152]],
                    &self.row.columns[self.index_mapping[1153]],
                ],
                Order: &self.row.columns[self.index_mapping[1154]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[1155]],
                    &self.row.columns[self.index_mapping[1156]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[1157]],
                    &self.row.columns[self.index_mapping[1158]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[1159]],
                    &self.row.columns[self.index_mapping[1160]],
                    &self.row.columns[self.index_mapping[1161]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[1162]],
                    &self.row.columns[self.index_mapping[1163]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[1164]],
                    &self.row.columns[self.index_mapping[1165]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[1166]],
                    &self.row.columns[self.index_mapping[1167]],
                    &self.row.columns[self.index_mapping[1168]],
                ],
                Quest: &self.row.columns[self.index_mapping[1169]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[1170]],
                    &self.row.columns[self.index_mapping[1171]],
                    &self.row.columns[self.index_mapping[1172]],
                    &self.row.columns[self.index_mapping[1173]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[1174]],
                Unknown2: &self.row.columns[self.index_mapping[1175]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[1176]],
                    &self.row.columns[self.index_mapping[1177]],
                    &self.row.columns[self.index_mapping[1178]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[1179]],
                HqCost: [
                    &self.row.columns[self.index_mapping[1180]],
                    &self.row.columns[self.index_mapping[1181]],
                    &self.row.columns[self.index_mapping[1182]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[1183]],
                    &self.row.columns[self.index_mapping[1184]],
                    &self.row.columns[self.index_mapping[1185]],
                    &self.row.columns[self.index_mapping[1186]],
                    &self.row.columns[self.index_mapping[1187]],
                ],
                Order: &self.row.columns[self.index_mapping[1188]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[1189]],
                    &self.row.columns[self.index_mapping[1190]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[1191]],
                    &self.row.columns[self.index_mapping[1192]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[1193]],
                    &self.row.columns[self.index_mapping[1194]],
                    &self.row.columns[self.index_mapping[1195]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[1196]],
                    &self.row.columns[self.index_mapping[1197]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[1198]],
                    &self.row.columns[self.index_mapping[1199]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[1200]],
                    &self.row.columns[self.index_mapping[1201]],
                    &self.row.columns[self.index_mapping[1202]],
                ],
                Quest: &self.row.columns[self.index_mapping[1203]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[1204]],
                    &self.row.columns[self.index_mapping[1205]],
                    &self.row.columns[self.index_mapping[1206]],
                    &self.row.columns[self.index_mapping[1207]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[1208]],
                Unknown2: &self.row.columns[self.index_mapping[1209]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[1210]],
                    &self.row.columns[self.index_mapping[1211]],
                    &self.row.columns[self.index_mapping[1212]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[1213]],
                HqCost: [
                    &self.row.columns[self.index_mapping[1214]],
                    &self.row.columns[self.index_mapping[1215]],
                    &self.row.columns[self.index_mapping[1216]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[1217]],
                    &self.row.columns[self.index_mapping[1218]],
                    &self.row.columns[self.index_mapping[1219]],
                    &self.row.columns[self.index_mapping[1220]],
                    &self.row.columns[self.index_mapping[1221]],
                ],
                Order: &self.row.columns[self.index_mapping[1222]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[1223]],
                    &self.row.columns[self.index_mapping[1224]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[1225]],
                    &self.row.columns[self.index_mapping[1226]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[1227]],
                    &self.row.columns[self.index_mapping[1228]],
                    &self.row.columns[self.index_mapping[1229]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[1230]],
                    &self.row.columns[self.index_mapping[1231]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[1232]],
                    &self.row.columns[self.index_mapping[1233]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[1234]],
                    &self.row.columns[self.index_mapping[1235]],
                    &self.row.columns[self.index_mapping[1236]],
                ],
                Quest: &self.row.columns[self.index_mapping[1237]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[1238]],
                    &self.row.columns[self.index_mapping[1239]],
                    &self.row.columns[self.index_mapping[1240]],
                    &self.row.columns[self.index_mapping[1241]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[1242]],
                Unknown2: &self.row.columns[self.index_mapping[1243]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[1244]],
                    &self.row.columns[self.index_mapping[1245]],
                    &self.row.columns[self.index_mapping[1246]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[1247]],
                HqCost: [
                    &self.row.columns[self.index_mapping[1248]],
                    &self.row.columns[self.index_mapping[1249]],
                    &self.row.columns[self.index_mapping[1250]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[1251]],
                    &self.row.columns[self.index_mapping[1252]],
                    &self.row.columns[self.index_mapping[1253]],
                    &self.row.columns[self.index_mapping[1254]],
                    &self.row.columns[self.index_mapping[1255]],
                ],
                Order: &self.row.columns[self.index_mapping[1256]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[1257]],
                    &self.row.columns[self.index_mapping[1258]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[1259]],
                    &self.row.columns[self.index_mapping[1260]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[1261]],
                    &self.row.columns[self.index_mapping[1262]],
                    &self.row.columns[self.index_mapping[1263]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[1264]],
                    &self.row.columns[self.index_mapping[1265]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[1266]],
                    &self.row.columns[self.index_mapping[1267]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[1268]],
                    &self.row.columns[self.index_mapping[1269]],
                    &self.row.columns[self.index_mapping[1270]],
                ],
                Quest: &self.row.columns[self.index_mapping[1271]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[1272]],
                    &self.row.columns[self.index_mapping[1273]],
                    &self.row.columns[self.index_mapping[1274]],
                    &self.row.columns[self.index_mapping[1275]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[1276]],
                Unknown2: &self.row.columns[self.index_mapping[1277]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[1278]],
                    &self.row.columns[self.index_mapping[1279]],
                    &self.row.columns[self.index_mapping[1280]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[1281]],
                HqCost: [
                    &self.row.columns[self.index_mapping[1282]],
                    &self.row.columns[self.index_mapping[1283]],
                    &self.row.columns[self.index_mapping[1284]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[1285]],
                    &self.row.columns[self.index_mapping[1286]],
                    &self.row.columns[self.index_mapping[1287]],
                    &self.row.columns[self.index_mapping[1288]],
                    &self.row.columns[self.index_mapping[1289]],
                ],
                Order: &self.row.columns[self.index_mapping[1290]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[1291]],
                    &self.row.columns[self.index_mapping[1292]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[1293]],
                    &self.row.columns[self.index_mapping[1294]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[1295]],
                    &self.row.columns[self.index_mapping[1296]],
                    &self.row.columns[self.index_mapping[1297]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[1298]],
                    &self.row.columns[self.index_mapping[1299]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[1300]],
                    &self.row.columns[self.index_mapping[1301]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[1302]],
                    &self.row.columns[self.index_mapping[1303]],
                    &self.row.columns[self.index_mapping[1304]],
                ],
                Quest: &self.row.columns[self.index_mapping[1305]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[1306]],
                    &self.row.columns[self.index_mapping[1307]],
                    &self.row.columns[self.index_mapping[1308]],
                    &self.row.columns[self.index_mapping[1309]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[1310]],
                Unknown2: &self.row.columns[self.index_mapping[1311]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[1312]],
                    &self.row.columns[self.index_mapping[1313]],
                    &self.row.columns[self.index_mapping[1314]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[1315]],
                HqCost: [
                    &self.row.columns[self.index_mapping[1316]],
                    &self.row.columns[self.index_mapping[1317]],
                    &self.row.columns[self.index_mapping[1318]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[1319]],
                    &self.row.columns[self.index_mapping[1320]],
                    &self.row.columns[self.index_mapping[1321]],
                    &self.row.columns[self.index_mapping[1322]],
                    &self.row.columns[self.index_mapping[1323]],
                ],
                Order: &self.row.columns[self.index_mapping[1324]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[1325]],
                    &self.row.columns[self.index_mapping[1326]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[1327]],
                    &self.row.columns[self.index_mapping[1328]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[1329]],
                    &self.row.columns[self.index_mapping[1330]],
                    &self.row.columns[self.index_mapping[1331]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[1332]],
                    &self.row.columns[self.index_mapping[1333]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[1334]],
                    &self.row.columns[self.index_mapping[1335]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[1336]],
                    &self.row.columns[self.index_mapping[1337]],
                    &self.row.columns[self.index_mapping[1338]],
                ],
                Quest: &self.row.columns[self.index_mapping[1339]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[1340]],
                    &self.row.columns[self.index_mapping[1341]],
                    &self.row.columns[self.index_mapping[1342]],
                    &self.row.columns[self.index_mapping[1343]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[1344]],
                Unknown2: &self.row.columns[self.index_mapping[1345]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[1346]],
                    &self.row.columns[self.index_mapping[1347]],
                    &self.row.columns[self.index_mapping[1348]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[1349]],
                HqCost: [
                    &self.row.columns[self.index_mapping[1350]],
                    &self.row.columns[self.index_mapping[1351]],
                    &self.row.columns[self.index_mapping[1352]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[1353]],
                    &self.row.columns[self.index_mapping[1354]],
                    &self.row.columns[self.index_mapping[1355]],
                    &self.row.columns[self.index_mapping[1356]],
                    &self.row.columns[self.index_mapping[1357]],
                ],
                Order: &self.row.columns[self.index_mapping[1358]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[1359]],
                    &self.row.columns[self.index_mapping[1360]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[1361]],
                    &self.row.columns[self.index_mapping[1362]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[1363]],
                    &self.row.columns[self.index_mapping[1364]],
                    &self.row.columns[self.index_mapping[1365]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[1366]],
                    &self.row.columns[self.index_mapping[1367]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[1368]],
                    &self.row.columns[self.index_mapping[1369]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[1370]],
                    &self.row.columns[self.index_mapping[1371]],
                    &self.row.columns[self.index_mapping[1372]],
                ],
                Quest: &self.row.columns[self.index_mapping[1373]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[1374]],
                    &self.row.columns[self.index_mapping[1375]],
                    &self.row.columns[self.index_mapping[1376]],
                    &self.row.columns[self.index_mapping[1377]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[1378]],
                Unknown2: &self.row.columns[self.index_mapping[1379]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[1380]],
                    &self.row.columns[self.index_mapping[1381]],
                    &self.row.columns[self.index_mapping[1382]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[1383]],
                HqCost: [
                    &self.row.columns[self.index_mapping[1384]],
                    &self.row.columns[self.index_mapping[1385]],
                    &self.row.columns[self.index_mapping[1386]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[1387]],
                    &self.row.columns[self.index_mapping[1388]],
                    &self.row.columns[self.index_mapping[1389]],
                    &self.row.columns[self.index_mapping[1390]],
                    &self.row.columns[self.index_mapping[1391]],
                ],
                Order: &self.row.columns[self.index_mapping[1392]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[1393]],
                    &self.row.columns[self.index_mapping[1394]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[1395]],
                    &self.row.columns[self.index_mapping[1396]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[1397]],
                    &self.row.columns[self.index_mapping[1398]],
                    &self.row.columns[self.index_mapping[1399]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[1400]],
                    &self.row.columns[self.index_mapping[1401]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[1402]],
                    &self.row.columns[self.index_mapping[1403]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[1404]],
                    &self.row.columns[self.index_mapping[1405]],
                    &self.row.columns[self.index_mapping[1406]],
                ],
                Quest: &self.row.columns[self.index_mapping[1407]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[1408]],
                    &self.row.columns[self.index_mapping[1409]],
                    &self.row.columns[self.index_mapping[1410]],
                    &self.row.columns[self.index_mapping[1411]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[1412]],
                Unknown2: &self.row.columns[self.index_mapping[1413]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[1414]],
                    &self.row.columns[self.index_mapping[1415]],
                    &self.row.columns[self.index_mapping[1416]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[1417]],
                HqCost: [
                    &self.row.columns[self.index_mapping[1418]],
                    &self.row.columns[self.index_mapping[1419]],
                    &self.row.columns[self.index_mapping[1420]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[1421]],
                    &self.row.columns[self.index_mapping[1422]],
                    &self.row.columns[self.index_mapping[1423]],
                    &self.row.columns[self.index_mapping[1424]],
                    &self.row.columns[self.index_mapping[1425]],
                ],
                Order: &self.row.columns[self.index_mapping[1426]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[1427]],
                    &self.row.columns[self.index_mapping[1428]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[1429]],
                    &self.row.columns[self.index_mapping[1430]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[1431]],
                    &self.row.columns[self.index_mapping[1432]],
                    &self.row.columns[self.index_mapping[1433]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[1434]],
                    &self.row.columns[self.index_mapping[1435]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[1436]],
                    &self.row.columns[self.index_mapping[1437]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[1438]],
                    &self.row.columns[self.index_mapping[1439]],
                    &self.row.columns[self.index_mapping[1440]],
                ],
                Quest: &self.row.columns[self.index_mapping[1441]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[1442]],
                    &self.row.columns[self.index_mapping[1443]],
                    &self.row.columns[self.index_mapping[1444]],
                    &self.row.columns[self.index_mapping[1445]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[1446]],
                Unknown2: &self.row.columns[self.index_mapping[1447]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[1448]],
                    &self.row.columns[self.index_mapping[1449]],
                    &self.row.columns[self.index_mapping[1450]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[1451]],
                HqCost: [
                    &self.row.columns[self.index_mapping[1452]],
                    &self.row.columns[self.index_mapping[1453]],
                    &self.row.columns[self.index_mapping[1454]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[1455]],
                    &self.row.columns[self.index_mapping[1456]],
                    &self.row.columns[self.index_mapping[1457]],
                    &self.row.columns[self.index_mapping[1458]],
                    &self.row.columns[self.index_mapping[1459]],
                ],
                Order: &self.row.columns[self.index_mapping[1460]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[1461]],
                    &self.row.columns[self.index_mapping[1462]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[1463]],
                    &self.row.columns[self.index_mapping[1464]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[1465]],
                    &self.row.columns[self.index_mapping[1466]],
                    &self.row.columns[self.index_mapping[1467]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[1468]],
                    &self.row.columns[self.index_mapping[1469]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[1470]],
                    &self.row.columns[self.index_mapping[1471]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[1472]],
                    &self.row.columns[self.index_mapping[1473]],
                    &self.row.columns[self.index_mapping[1474]],
                ],
                Quest: &self.row.columns[self.index_mapping[1475]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[1476]],
                    &self.row.columns[self.index_mapping[1477]],
                    &self.row.columns[self.index_mapping[1478]],
                    &self.row.columns[self.index_mapping[1479]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[1480]],
                Unknown2: &self.row.columns[self.index_mapping[1481]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[1482]],
                    &self.row.columns[self.index_mapping[1483]],
                    &self.row.columns[self.index_mapping[1484]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[1485]],
                HqCost: [
                    &self.row.columns[self.index_mapping[1486]],
                    &self.row.columns[self.index_mapping[1487]],
                    &self.row.columns[self.index_mapping[1488]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[1489]],
                    &self.row.columns[self.index_mapping[1490]],
                    &self.row.columns[self.index_mapping[1491]],
                    &self.row.columns[self.index_mapping[1492]],
                    &self.row.columns[self.index_mapping[1493]],
                ],
                Order: &self.row.columns[self.index_mapping[1494]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[1495]],
                    &self.row.columns[self.index_mapping[1496]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[1497]],
                    &self.row.columns[self.index_mapping[1498]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[1499]],
                    &self.row.columns[self.index_mapping[1500]],
                    &self.row.columns[self.index_mapping[1501]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[1502]],
                    &self.row.columns[self.index_mapping[1503]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[1504]],
                    &self.row.columns[self.index_mapping[1505]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[1506]],
                    &self.row.columns[self.index_mapping[1507]],
                    &self.row.columns[self.index_mapping[1508]],
                ],
                Quest: &self.row.columns[self.index_mapping[1509]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[1510]],
                    &self.row.columns[self.index_mapping[1511]],
                    &self.row.columns[self.index_mapping[1512]],
                    &self.row.columns[self.index_mapping[1513]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[1514]],
                Unknown2: &self.row.columns[self.index_mapping[1515]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[1516]],
                    &self.row.columns[self.index_mapping[1517]],
                    &self.row.columns[self.index_mapping[1518]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[1519]],
                HqCost: [
                    &self.row.columns[self.index_mapping[1520]],
                    &self.row.columns[self.index_mapping[1521]],
                    &self.row.columns[self.index_mapping[1522]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[1523]],
                    &self.row.columns[self.index_mapping[1524]],
                    &self.row.columns[self.index_mapping[1525]],
                    &self.row.columns[self.index_mapping[1526]],
                    &self.row.columns[self.index_mapping[1527]],
                ],
                Order: &self.row.columns[self.index_mapping[1528]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[1529]],
                    &self.row.columns[self.index_mapping[1530]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[1531]],
                    &self.row.columns[self.index_mapping[1532]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[1533]],
                    &self.row.columns[self.index_mapping[1534]],
                    &self.row.columns[self.index_mapping[1535]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[1536]],
                    &self.row.columns[self.index_mapping[1537]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[1538]],
                    &self.row.columns[self.index_mapping[1539]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[1540]],
                    &self.row.columns[self.index_mapping[1541]],
                    &self.row.columns[self.index_mapping[1542]],
                ],
                Quest: &self.row.columns[self.index_mapping[1543]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[1544]],
                    &self.row.columns[self.index_mapping[1545]],
                    &self.row.columns[self.index_mapping[1546]],
                    &self.row.columns[self.index_mapping[1547]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[1548]],
                Unknown2: &self.row.columns[self.index_mapping[1549]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[1550]],
                    &self.row.columns[self.index_mapping[1551]],
                    &self.row.columns[self.index_mapping[1552]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[1553]],
                HqCost: [
                    &self.row.columns[self.index_mapping[1554]],
                    &self.row.columns[self.index_mapping[1555]],
                    &self.row.columns[self.index_mapping[1556]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[1557]],
                    &self.row.columns[self.index_mapping[1558]],
                    &self.row.columns[self.index_mapping[1559]],
                    &self.row.columns[self.index_mapping[1560]],
                    &self.row.columns[self.index_mapping[1561]],
                ],
                Order: &self.row.columns[self.index_mapping[1562]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[1563]],
                    &self.row.columns[self.index_mapping[1564]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[1565]],
                    &self.row.columns[self.index_mapping[1566]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[1567]],
                    &self.row.columns[self.index_mapping[1568]],
                    &self.row.columns[self.index_mapping[1569]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[1570]],
                    &self.row.columns[self.index_mapping[1571]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[1572]],
                    &self.row.columns[self.index_mapping[1573]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[1574]],
                    &self.row.columns[self.index_mapping[1575]],
                    &self.row.columns[self.index_mapping[1576]],
                ],
                Quest: &self.row.columns[self.index_mapping[1577]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[1578]],
                    &self.row.columns[self.index_mapping[1579]],
                    &self.row.columns[self.index_mapping[1580]],
                    &self.row.columns[self.index_mapping[1581]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[1582]],
                Unknown2: &self.row.columns[self.index_mapping[1583]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[1584]],
                    &self.row.columns[self.index_mapping[1585]],
                    &self.row.columns[self.index_mapping[1586]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[1587]],
                HqCost: [
                    &self.row.columns[self.index_mapping[1588]],
                    &self.row.columns[self.index_mapping[1589]],
                    &self.row.columns[self.index_mapping[1590]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[1591]],
                    &self.row.columns[self.index_mapping[1592]],
                    &self.row.columns[self.index_mapping[1593]],
                    &self.row.columns[self.index_mapping[1594]],
                    &self.row.columns[self.index_mapping[1595]],
                ],
                Order: &self.row.columns[self.index_mapping[1596]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[1597]],
                    &self.row.columns[self.index_mapping[1598]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[1599]],
                    &self.row.columns[self.index_mapping[1600]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[1601]],
                    &self.row.columns[self.index_mapping[1602]],
                    &self.row.columns[self.index_mapping[1603]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[1604]],
                    &self.row.columns[self.index_mapping[1605]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[1606]],
                    &self.row.columns[self.index_mapping[1607]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[1608]],
                    &self.row.columns[self.index_mapping[1609]],
                    &self.row.columns[self.index_mapping[1610]],
                ],
                Quest: &self.row.columns[self.index_mapping[1611]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[1612]],
                    &self.row.columns[self.index_mapping[1613]],
                    &self.row.columns[self.index_mapping[1614]],
                    &self.row.columns[self.index_mapping[1615]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[1616]],
                Unknown2: &self.row.columns[self.index_mapping[1617]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[1618]],
                    &self.row.columns[self.index_mapping[1619]],
                    &self.row.columns[self.index_mapping[1620]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[1621]],
                HqCost: [
                    &self.row.columns[self.index_mapping[1622]],
                    &self.row.columns[self.index_mapping[1623]],
                    &self.row.columns[self.index_mapping[1624]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[1625]],
                    &self.row.columns[self.index_mapping[1626]],
                    &self.row.columns[self.index_mapping[1627]],
                    &self.row.columns[self.index_mapping[1628]],
                    &self.row.columns[self.index_mapping[1629]],
                ],
                Order: &self.row.columns[self.index_mapping[1630]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[1631]],
                    &self.row.columns[self.index_mapping[1632]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[1633]],
                    &self.row.columns[self.index_mapping[1634]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[1635]],
                    &self.row.columns[self.index_mapping[1636]],
                    &self.row.columns[self.index_mapping[1637]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[1638]],
                    &self.row.columns[self.index_mapping[1639]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[1640]],
                    &self.row.columns[self.index_mapping[1641]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[1642]],
                    &self.row.columns[self.index_mapping[1643]],
                    &self.row.columns[self.index_mapping[1644]],
                ],
                Quest: &self.row.columns[self.index_mapping[1645]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[1646]],
                    &self.row.columns[self.index_mapping[1647]],
                    &self.row.columns[self.index_mapping[1648]],
                    &self.row.columns[self.index_mapping[1649]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[1650]],
                Unknown2: &self.row.columns[self.index_mapping[1651]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[1652]],
                    &self.row.columns[self.index_mapping[1653]],
                    &self.row.columns[self.index_mapping[1654]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[1655]],
                HqCost: [
                    &self.row.columns[self.index_mapping[1656]],
                    &self.row.columns[self.index_mapping[1657]],
                    &self.row.columns[self.index_mapping[1658]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[1659]],
                    &self.row.columns[self.index_mapping[1660]],
                    &self.row.columns[self.index_mapping[1661]],
                    &self.row.columns[self.index_mapping[1662]],
                    &self.row.columns[self.index_mapping[1663]],
                ],
                Order: &self.row.columns[self.index_mapping[1664]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[1665]],
                    &self.row.columns[self.index_mapping[1666]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[1667]],
                    &self.row.columns[self.index_mapping[1668]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[1669]],
                    &self.row.columns[self.index_mapping[1670]],
                    &self.row.columns[self.index_mapping[1671]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[1672]],
                    &self.row.columns[self.index_mapping[1673]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[1674]],
                    &self.row.columns[self.index_mapping[1675]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[1676]],
                    &self.row.columns[self.index_mapping[1677]],
                    &self.row.columns[self.index_mapping[1678]],
                ],
                Quest: &self.row.columns[self.index_mapping[1679]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[1680]],
                    &self.row.columns[self.index_mapping[1681]],
                    &self.row.columns[self.index_mapping[1682]],
                    &self.row.columns[self.index_mapping[1683]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[1684]],
                Unknown2: &self.row.columns[self.index_mapping[1685]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[1686]],
                    &self.row.columns[self.index_mapping[1687]],
                    &self.row.columns[self.index_mapping[1688]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[1689]],
                HqCost: [
                    &self.row.columns[self.index_mapping[1690]],
                    &self.row.columns[self.index_mapping[1691]],
                    &self.row.columns[self.index_mapping[1692]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[1693]],
                    &self.row.columns[self.index_mapping[1694]],
                    &self.row.columns[self.index_mapping[1695]],
                    &self.row.columns[self.index_mapping[1696]],
                    &self.row.columns[self.index_mapping[1697]],
                ],
                Order: &self.row.columns[self.index_mapping[1698]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[1699]],
                    &self.row.columns[self.index_mapping[1700]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[1701]],
                    &self.row.columns[self.index_mapping[1702]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[1703]],
                    &self.row.columns[self.index_mapping[1704]],
                    &self.row.columns[self.index_mapping[1705]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[1706]],
                    &self.row.columns[self.index_mapping[1707]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[1708]],
                    &self.row.columns[self.index_mapping[1709]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[1710]],
                    &self.row.columns[self.index_mapping[1711]],
                    &self.row.columns[self.index_mapping[1712]],
                ],
                Quest: &self.row.columns[self.index_mapping[1713]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[1714]],
                    &self.row.columns[self.index_mapping[1715]],
                    &self.row.columns[self.index_mapping[1716]],
                    &self.row.columns[self.index_mapping[1717]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[1718]],
                Unknown2: &self.row.columns[self.index_mapping[1719]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[1720]],
                    &self.row.columns[self.index_mapping[1721]],
                    &self.row.columns[self.index_mapping[1722]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[1723]],
                HqCost: [
                    &self.row.columns[self.index_mapping[1724]],
                    &self.row.columns[self.index_mapping[1725]],
                    &self.row.columns[self.index_mapping[1726]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[1727]],
                    &self.row.columns[self.index_mapping[1728]],
                    &self.row.columns[self.index_mapping[1729]],
                    &self.row.columns[self.index_mapping[1730]],
                    &self.row.columns[self.index_mapping[1731]],
                ],
                Order: &self.row.columns[self.index_mapping[1732]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[1733]],
                    &self.row.columns[self.index_mapping[1734]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[1735]],
                    &self.row.columns[self.index_mapping[1736]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[1737]],
                    &self.row.columns[self.index_mapping[1738]],
                    &self.row.columns[self.index_mapping[1739]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[1740]],
                    &self.row.columns[self.index_mapping[1741]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[1742]],
                    &self.row.columns[self.index_mapping[1743]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[1744]],
                    &self.row.columns[self.index_mapping[1745]],
                    &self.row.columns[self.index_mapping[1746]],
                ],
                Quest: &self.row.columns[self.index_mapping[1747]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[1748]],
                    &self.row.columns[self.index_mapping[1749]],
                    &self.row.columns[self.index_mapping[1750]],
                    &self.row.columns[self.index_mapping[1751]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[1752]],
                Unknown2: &self.row.columns[self.index_mapping[1753]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[1754]],
                    &self.row.columns[self.index_mapping[1755]],
                    &self.row.columns[self.index_mapping[1756]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[1757]],
                HqCost: [
                    &self.row.columns[self.index_mapping[1758]],
                    &self.row.columns[self.index_mapping[1759]],
                    &self.row.columns[self.index_mapping[1760]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[1761]],
                    &self.row.columns[self.index_mapping[1762]],
                    &self.row.columns[self.index_mapping[1763]],
                    &self.row.columns[self.index_mapping[1764]],
                    &self.row.columns[self.index_mapping[1765]],
                ],
                Order: &self.row.columns[self.index_mapping[1766]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[1767]],
                    &self.row.columns[self.index_mapping[1768]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[1769]],
                    &self.row.columns[self.index_mapping[1770]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[1771]],
                    &self.row.columns[self.index_mapping[1772]],
                    &self.row.columns[self.index_mapping[1773]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[1774]],
                    &self.row.columns[self.index_mapping[1775]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[1776]],
                    &self.row.columns[self.index_mapping[1777]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[1778]],
                    &self.row.columns[self.index_mapping[1779]],
                    &self.row.columns[self.index_mapping[1780]],
                ],
                Quest: &self.row.columns[self.index_mapping[1781]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[1782]],
                    &self.row.columns[self.index_mapping[1783]],
                    &self.row.columns[self.index_mapping[1784]],
                    &self.row.columns[self.index_mapping[1785]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[1786]],
                Unknown2: &self.row.columns[self.index_mapping[1787]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[1788]],
                    &self.row.columns[self.index_mapping[1789]],
                    &self.row.columns[self.index_mapping[1790]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[1791]],
                HqCost: [
                    &self.row.columns[self.index_mapping[1792]],
                    &self.row.columns[self.index_mapping[1793]],
                    &self.row.columns[self.index_mapping[1794]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[1795]],
                    &self.row.columns[self.index_mapping[1796]],
                    &self.row.columns[self.index_mapping[1797]],
                    &self.row.columns[self.index_mapping[1798]],
                    &self.row.columns[self.index_mapping[1799]],
                ],
                Order: &self.row.columns[self.index_mapping[1800]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[1801]],
                    &self.row.columns[self.index_mapping[1802]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[1803]],
                    &self.row.columns[self.index_mapping[1804]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[1805]],
                    &self.row.columns[self.index_mapping[1806]],
                    &self.row.columns[self.index_mapping[1807]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[1808]],
                    &self.row.columns[self.index_mapping[1809]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[1810]],
                    &self.row.columns[self.index_mapping[1811]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[1812]],
                    &self.row.columns[self.index_mapping[1813]],
                    &self.row.columns[self.index_mapping[1814]],
                ],
                Quest: &self.row.columns[self.index_mapping[1815]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[1816]],
                    &self.row.columns[self.index_mapping[1817]],
                    &self.row.columns[self.index_mapping[1818]],
                    &self.row.columns[self.index_mapping[1819]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[1820]],
                Unknown2: &self.row.columns[self.index_mapping[1821]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[1822]],
                    &self.row.columns[self.index_mapping[1823]],
                    &self.row.columns[self.index_mapping[1824]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[1825]],
                HqCost: [
                    &self.row.columns[self.index_mapping[1826]],
                    &self.row.columns[self.index_mapping[1827]],
                    &self.row.columns[self.index_mapping[1828]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[1829]],
                    &self.row.columns[self.index_mapping[1830]],
                    &self.row.columns[self.index_mapping[1831]],
                    &self.row.columns[self.index_mapping[1832]],
                    &self.row.columns[self.index_mapping[1833]],
                ],
                Order: &self.row.columns[self.index_mapping[1834]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[1835]],
                    &self.row.columns[self.index_mapping[1836]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[1837]],
                    &self.row.columns[self.index_mapping[1838]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[1839]],
                    &self.row.columns[self.index_mapping[1840]],
                    &self.row.columns[self.index_mapping[1841]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[1842]],
                    &self.row.columns[self.index_mapping[1843]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[1844]],
                    &self.row.columns[self.index_mapping[1845]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[1846]],
                    &self.row.columns[self.index_mapping[1847]],
                    &self.row.columns[self.index_mapping[1848]],
                ],
                Quest: &self.row.columns[self.index_mapping[1849]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[1850]],
                    &self.row.columns[self.index_mapping[1851]],
                    &self.row.columns[self.index_mapping[1852]],
                    &self.row.columns[self.index_mapping[1853]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[1854]],
                Unknown2: &self.row.columns[self.index_mapping[1855]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[1856]],
                    &self.row.columns[self.index_mapping[1857]],
                    &self.row.columns[self.index_mapping[1858]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[1859]],
                HqCost: [
                    &self.row.columns[self.index_mapping[1860]],
                    &self.row.columns[self.index_mapping[1861]],
                    &self.row.columns[self.index_mapping[1862]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[1863]],
                    &self.row.columns[self.index_mapping[1864]],
                    &self.row.columns[self.index_mapping[1865]],
                    &self.row.columns[self.index_mapping[1866]],
                    &self.row.columns[self.index_mapping[1867]],
                ],
                Order: &self.row.columns[self.index_mapping[1868]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[1869]],
                    &self.row.columns[self.index_mapping[1870]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[1871]],
                    &self.row.columns[self.index_mapping[1872]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[1873]],
                    &self.row.columns[self.index_mapping[1874]],
                    &self.row.columns[self.index_mapping[1875]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[1876]],
                    &self.row.columns[self.index_mapping[1877]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[1878]],
                    &self.row.columns[self.index_mapping[1879]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[1880]],
                    &self.row.columns[self.index_mapping[1881]],
                    &self.row.columns[self.index_mapping[1882]],
                ],
                Quest: &self.row.columns[self.index_mapping[1883]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[1884]],
                    &self.row.columns[self.index_mapping[1885]],
                    &self.row.columns[self.index_mapping[1886]],
                    &self.row.columns[self.index_mapping[1887]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[1888]],
                Unknown2: &self.row.columns[self.index_mapping[1889]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[1890]],
                    &self.row.columns[self.index_mapping[1891]],
                    &self.row.columns[self.index_mapping[1892]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[1893]],
                HqCost: [
                    &self.row.columns[self.index_mapping[1894]],
                    &self.row.columns[self.index_mapping[1895]],
                    &self.row.columns[self.index_mapping[1896]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[1897]],
                    &self.row.columns[self.index_mapping[1898]],
                    &self.row.columns[self.index_mapping[1899]],
                    &self.row.columns[self.index_mapping[1900]],
                    &self.row.columns[self.index_mapping[1901]],
                ],
                Order: &self.row.columns[self.index_mapping[1902]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[1903]],
                    &self.row.columns[self.index_mapping[1904]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[1905]],
                    &self.row.columns[self.index_mapping[1906]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[1907]],
                    &self.row.columns[self.index_mapping[1908]],
                    &self.row.columns[self.index_mapping[1909]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[1910]],
                    &self.row.columns[self.index_mapping[1911]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[1912]],
                    &self.row.columns[self.index_mapping[1913]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[1914]],
                    &self.row.columns[self.index_mapping[1915]],
                    &self.row.columns[self.index_mapping[1916]],
                ],
                Quest: &self.row.columns[self.index_mapping[1917]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[1918]],
                    &self.row.columns[self.index_mapping[1919]],
                    &self.row.columns[self.index_mapping[1920]],
                    &self.row.columns[self.index_mapping[1921]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[1922]],
                Unknown2: &self.row.columns[self.index_mapping[1923]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[1924]],
                    &self.row.columns[self.index_mapping[1925]],
                    &self.row.columns[self.index_mapping[1926]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[1927]],
                HqCost: [
                    &self.row.columns[self.index_mapping[1928]],
                    &self.row.columns[self.index_mapping[1929]],
                    &self.row.columns[self.index_mapping[1930]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[1931]],
                    &self.row.columns[self.index_mapping[1932]],
                    &self.row.columns[self.index_mapping[1933]],
                    &self.row.columns[self.index_mapping[1934]],
                    &self.row.columns[self.index_mapping[1935]],
                ],
                Order: &self.row.columns[self.index_mapping[1936]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[1937]],
                    &self.row.columns[self.index_mapping[1938]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[1939]],
                    &self.row.columns[self.index_mapping[1940]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[1941]],
                    &self.row.columns[self.index_mapping[1942]],
                    &self.row.columns[self.index_mapping[1943]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[1944]],
                    &self.row.columns[self.index_mapping[1945]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[1946]],
                    &self.row.columns[self.index_mapping[1947]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[1948]],
                    &self.row.columns[self.index_mapping[1949]],
                    &self.row.columns[self.index_mapping[1950]],
                ],
                Quest: &self.row.columns[self.index_mapping[1951]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[1952]],
                    &self.row.columns[self.index_mapping[1953]],
                    &self.row.columns[self.index_mapping[1954]],
                    &self.row.columns[self.index_mapping[1955]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[1956]],
                Unknown2: &self.row.columns[self.index_mapping[1957]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[1958]],
                    &self.row.columns[self.index_mapping[1959]],
                    &self.row.columns[self.index_mapping[1960]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[1961]],
                HqCost: [
                    &self.row.columns[self.index_mapping[1962]],
                    &self.row.columns[self.index_mapping[1963]],
                    &self.row.columns[self.index_mapping[1964]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[1965]],
                    &self.row.columns[self.index_mapping[1966]],
                    &self.row.columns[self.index_mapping[1967]],
                    &self.row.columns[self.index_mapping[1968]],
                    &self.row.columns[self.index_mapping[1969]],
                ],
                Order: &self.row.columns[self.index_mapping[1970]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[1971]],
                    &self.row.columns[self.index_mapping[1972]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[1973]],
                    &self.row.columns[self.index_mapping[1974]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[1975]],
                    &self.row.columns[self.index_mapping[1976]],
                    &self.row.columns[self.index_mapping[1977]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[1978]],
                    &self.row.columns[self.index_mapping[1979]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[1980]],
                    &self.row.columns[self.index_mapping[1981]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[1982]],
                    &self.row.columns[self.index_mapping[1983]],
                    &self.row.columns[self.index_mapping[1984]],
                ],
                Quest: &self.row.columns[self.index_mapping[1985]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[1986]],
                    &self.row.columns[self.index_mapping[1987]],
                    &self.row.columns[self.index_mapping[1988]],
                    &self.row.columns[self.index_mapping[1989]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[1990]],
                Unknown2: &self.row.columns[self.index_mapping[1991]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[1992]],
                    &self.row.columns[self.index_mapping[1993]],
                    &self.row.columns[self.index_mapping[1994]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[1995]],
                HqCost: [
                    &self.row.columns[self.index_mapping[1996]],
                    &self.row.columns[self.index_mapping[1997]],
                    &self.row.columns[self.index_mapping[1998]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[1999]],
                    &self.row.columns[self.index_mapping[2000]],
                    &self.row.columns[self.index_mapping[2001]],
                    &self.row.columns[self.index_mapping[2002]],
                    &self.row.columns[self.index_mapping[2003]],
                ],
                Order: &self.row.columns[self.index_mapping[2004]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[2005]],
                    &self.row.columns[self.index_mapping[2006]],
                ],
            },
            ItemElement {
                ReceiveCount: [
                    &self.row.columns[self.index_mapping[2007]],
                    &self.row.columns[self.index_mapping[2008]],
                ],
                CurrencyCost: [
                    &self.row.columns[self.index_mapping[2009]],
                    &self.row.columns[self.index_mapping[2010]],
                    &self.row.columns[self.index_mapping[2011]],
                ],
                Item: [
                    &self.row.columns[self.index_mapping[2012]],
                    &self.row.columns[self.index_mapping[2013]],
                ],
                Category: [
                    &self.row.columns[self.index_mapping[2014]],
                    &self.row.columns[self.index_mapping[2015]],
                ],
                ItemCost: [
                    &self.row.columns[self.index_mapping[2016]],
                    &self.row.columns[self.index_mapping[2017]],
                    &self.row.columns[self.index_mapping[2018]],
                ],
                Quest: &self.row.columns[self.index_mapping[2019]],
                Unknown0: [
                    &self.row.columns[self.index_mapping[2020]],
                    &self.row.columns[self.index_mapping[2021]],
                    &self.row.columns[self.index_mapping[2022]],
                    &self.row.columns[self.index_mapping[2023]],
                ],
                AchievementUnlock: &self.row.columns[self.index_mapping[2024]],
                Unknown2: &self.row.columns[self.index_mapping[2025]],
                CollectabilityCost: [
                    &self.row.columns[self.index_mapping[2026]],
                    &self.row.columns[self.index_mapping[2027]],
                    &self.row.columns[self.index_mapping[2028]],
                ],
                PatchNumber: &self.row.columns[self.index_mapping[2029]],
                HqCost: [
                    &self.row.columns[self.index_mapping[2030]],
                    &self.row.columns[self.index_mapping[2031]],
                    &self.row.columns[self.index_mapping[2032]],
                ],
                Unknown1: [
                    &self.row.columns[self.index_mapping[2033]],
                    &self.row.columns[self.index_mapping[2034]],
                    &self.row.columns[self.index_mapping[2035]],
                    &self.row.columns[self.index_mapping[2036]],
                    &self.row.columns[self.index_mapping[2037]],
                ],
                Order: &self.row.columns[self.index_mapping[2038]],
                ReceiveHq: [
                    &self.row.columns[self.index_mapping[2039]],
                    &self.row.columns[self.index_mapping[2040]],
                ],
            },
        ]
    }
    pub fn Quest(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[2041]]
    }
    pub fn Unknown0(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[2042]]
    }
    pub fn RequiredContentFinderCondition(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[2043]]
    }
    pub fn CompleteText(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[2044]]
    }
    pub fn NotCompleteText(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[2045]]
    }
    pub fn RequiredFestival(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[2046]]
    }
    pub fn RequiredFestivalPhase(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[2047]]
    }
    pub fn UseCurrencyType(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[2048]]
    }
    pub fn Unknown3(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[2049]]
    }
    /// If this is true, then the CFC needs to be completed; If this is false, then the CFC just needs to be unlocked
    pub fn RequiredContentFinderConditionComplete(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[2050]]
    }
    pub fn Unknown4(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[2051]]
    }
}
