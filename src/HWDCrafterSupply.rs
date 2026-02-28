//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
pub struct HWDCrafterSupplyParamsElement<'a> {
    pub ItemTradeIn: &'a Field,
    pub BaseCollectableRating: &'a Field,
    pub MidCollectableRating: &'a Field,
    pub HighCollectableRating: &'a Field,
    pub BaseCollectableReward: &'a Field,
    pub MidCollectableReward: &'a Field,
    pub HighCollectableReward: &'a Field,
    pub BaseCollectableRewardPostPhase: &'a Field,
    pub MidCollectableRewardPostPhase: &'a Field,
    pub HighCollectableRewardPostPhase: &'a Field,
    pub Level: &'a Field,
    pub LevelMax: &'a Field,
    pub Unknown0: &'a Field,
    pub TermName: &'a Field,
}
#[derive(Debug, Clone)]
pub struct HWDCrafterSupplySheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl HWDCrafterSupplySheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("HWDCrafterSupply")?;
        let sheet = resolver.read_excel_sheet(&exh, "HWDCrafterSupply", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<HWDCrafterSupplyRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<HWDCrafterSupplyRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for HWDCrafterSupplySheet {
    type Row = HWDCrafterSupplyRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a HWDCrafterSupplySheet {
    type Item = (u32, Vec<(u16, HWDCrafterSupplyRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, HWDCrafterSupplySheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, HWDCrafterSupplySheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct HWDCrafterSupplyRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> HWDCrafterSupplyRow<'a> {
    pub fn HWDCrafterSupplyParams(&'a self) -> [HWDCrafterSupplyParamsElement<'a>; 23] {
        [
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.row.columns[self.index_mapping[0]],
                BaseCollectableRating: &self.row.columns[self.index_mapping[1]],
                MidCollectableRating: &self.row.columns[self.index_mapping[2]],
                HighCollectableRating: &self.row.columns[self.index_mapping[3]],
                BaseCollectableReward: &self.row.columns[self.index_mapping[4]],
                MidCollectableReward: &self.row.columns[self.index_mapping[5]],
                HighCollectableReward: &self.row.columns[self.index_mapping[6]],
                BaseCollectableRewardPostPhase: &self.row.columns[self.index_mapping[7]],
                MidCollectableRewardPostPhase: &self.row.columns[self.index_mapping[8]],
                HighCollectableRewardPostPhase: &self.row.columns[self.index_mapping[9]],
                Level: &self.row.columns[self.index_mapping[10]],
                LevelMax: &self.row.columns[self.index_mapping[11]],
                Unknown0: &self.row.columns[self.index_mapping[12]],
                TermName: &self.row.columns[self.index_mapping[13]],
            },
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.row.columns[self.index_mapping[14]],
                BaseCollectableRating: &self.row.columns[self.index_mapping[15]],
                MidCollectableRating: &self.row.columns[self.index_mapping[16]],
                HighCollectableRating: &self.row.columns[self.index_mapping[17]],
                BaseCollectableReward: &self.row.columns[self.index_mapping[18]],
                MidCollectableReward: &self.row.columns[self.index_mapping[19]],
                HighCollectableReward: &self.row.columns[self.index_mapping[20]],
                BaseCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[21]],
                MidCollectableRewardPostPhase: &self.row.columns[self.index_mapping[22]],
                HighCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[23]],
                Level: &self.row.columns[self.index_mapping[24]],
                LevelMax: &self.row.columns[self.index_mapping[25]],
                Unknown0: &self.row.columns[self.index_mapping[26]],
                TermName: &self.row.columns[self.index_mapping[27]],
            },
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.row.columns[self.index_mapping[28]],
                BaseCollectableRating: &self.row.columns[self.index_mapping[29]],
                MidCollectableRating: &self.row.columns[self.index_mapping[30]],
                HighCollectableRating: &self.row.columns[self.index_mapping[31]],
                BaseCollectableReward: &self.row.columns[self.index_mapping[32]],
                MidCollectableReward: &self.row.columns[self.index_mapping[33]],
                HighCollectableReward: &self.row.columns[self.index_mapping[34]],
                BaseCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[35]],
                MidCollectableRewardPostPhase: &self.row.columns[self.index_mapping[36]],
                HighCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[37]],
                Level: &self.row.columns[self.index_mapping[38]],
                LevelMax: &self.row.columns[self.index_mapping[39]],
                Unknown0: &self.row.columns[self.index_mapping[40]],
                TermName: &self.row.columns[self.index_mapping[41]],
            },
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.row.columns[self.index_mapping[42]],
                BaseCollectableRating: &self.row.columns[self.index_mapping[43]],
                MidCollectableRating: &self.row.columns[self.index_mapping[44]],
                HighCollectableRating: &self.row.columns[self.index_mapping[45]],
                BaseCollectableReward: &self.row.columns[self.index_mapping[46]],
                MidCollectableReward: &self.row.columns[self.index_mapping[47]],
                HighCollectableReward: &self.row.columns[self.index_mapping[48]],
                BaseCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[49]],
                MidCollectableRewardPostPhase: &self.row.columns[self.index_mapping[50]],
                HighCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[51]],
                Level: &self.row.columns[self.index_mapping[52]],
                LevelMax: &self.row.columns[self.index_mapping[53]],
                Unknown0: &self.row.columns[self.index_mapping[54]],
                TermName: &self.row.columns[self.index_mapping[55]],
            },
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.row.columns[self.index_mapping[56]],
                BaseCollectableRating: &self.row.columns[self.index_mapping[57]],
                MidCollectableRating: &self.row.columns[self.index_mapping[58]],
                HighCollectableRating: &self.row.columns[self.index_mapping[59]],
                BaseCollectableReward: &self.row.columns[self.index_mapping[60]],
                MidCollectableReward: &self.row.columns[self.index_mapping[61]],
                HighCollectableReward: &self.row.columns[self.index_mapping[62]],
                BaseCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[63]],
                MidCollectableRewardPostPhase: &self.row.columns[self.index_mapping[64]],
                HighCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[65]],
                Level: &self.row.columns[self.index_mapping[66]],
                LevelMax: &self.row.columns[self.index_mapping[67]],
                Unknown0: &self.row.columns[self.index_mapping[68]],
                TermName: &self.row.columns[self.index_mapping[69]],
            },
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.row.columns[self.index_mapping[70]],
                BaseCollectableRating: &self.row.columns[self.index_mapping[71]],
                MidCollectableRating: &self.row.columns[self.index_mapping[72]],
                HighCollectableRating: &self.row.columns[self.index_mapping[73]],
                BaseCollectableReward: &self.row.columns[self.index_mapping[74]],
                MidCollectableReward: &self.row.columns[self.index_mapping[75]],
                HighCollectableReward: &self.row.columns[self.index_mapping[76]],
                BaseCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[77]],
                MidCollectableRewardPostPhase: &self.row.columns[self.index_mapping[78]],
                HighCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[79]],
                Level: &self.row.columns[self.index_mapping[80]],
                LevelMax: &self.row.columns[self.index_mapping[81]],
                Unknown0: &self.row.columns[self.index_mapping[82]],
                TermName: &self.row.columns[self.index_mapping[83]],
            },
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.row.columns[self.index_mapping[84]],
                BaseCollectableRating: &self.row.columns[self.index_mapping[85]],
                MidCollectableRating: &self.row.columns[self.index_mapping[86]],
                HighCollectableRating: &self.row.columns[self.index_mapping[87]],
                BaseCollectableReward: &self.row.columns[self.index_mapping[88]],
                MidCollectableReward: &self.row.columns[self.index_mapping[89]],
                HighCollectableReward: &self.row.columns[self.index_mapping[90]],
                BaseCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[91]],
                MidCollectableRewardPostPhase: &self.row.columns[self.index_mapping[92]],
                HighCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[93]],
                Level: &self.row.columns[self.index_mapping[94]],
                LevelMax: &self.row.columns[self.index_mapping[95]],
                Unknown0: &self.row.columns[self.index_mapping[96]],
                TermName: &self.row.columns[self.index_mapping[97]],
            },
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.row.columns[self.index_mapping[98]],
                BaseCollectableRating: &self.row.columns[self.index_mapping[99]],
                MidCollectableRating: &self.row.columns[self.index_mapping[100]],
                HighCollectableRating: &self.row.columns[self.index_mapping[101]],
                BaseCollectableReward: &self.row.columns[self.index_mapping[102]],
                MidCollectableReward: &self.row.columns[self.index_mapping[103]],
                HighCollectableReward: &self.row.columns[self.index_mapping[104]],
                BaseCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[105]],
                MidCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[106]],
                HighCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[107]],
                Level: &self.row.columns[self.index_mapping[108]],
                LevelMax: &self.row.columns[self.index_mapping[109]],
                Unknown0: &self.row.columns[self.index_mapping[110]],
                TermName: &self.row.columns[self.index_mapping[111]],
            },
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.row.columns[self.index_mapping[112]],
                BaseCollectableRating: &self.row.columns[self.index_mapping[113]],
                MidCollectableRating: &self.row.columns[self.index_mapping[114]],
                HighCollectableRating: &self.row.columns[self.index_mapping[115]],
                BaseCollectableReward: &self.row.columns[self.index_mapping[116]],
                MidCollectableReward: &self.row.columns[self.index_mapping[117]],
                HighCollectableReward: &self.row.columns[self.index_mapping[118]],
                BaseCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[119]],
                MidCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[120]],
                HighCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[121]],
                Level: &self.row.columns[self.index_mapping[122]],
                LevelMax: &self.row.columns[self.index_mapping[123]],
                Unknown0: &self.row.columns[self.index_mapping[124]],
                TermName: &self.row.columns[self.index_mapping[125]],
            },
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.row.columns[self.index_mapping[126]],
                BaseCollectableRating: &self.row.columns[self.index_mapping[127]],
                MidCollectableRating: &self.row.columns[self.index_mapping[128]],
                HighCollectableRating: &self.row.columns[self.index_mapping[129]],
                BaseCollectableReward: &self.row.columns[self.index_mapping[130]],
                MidCollectableReward: &self.row.columns[self.index_mapping[131]],
                HighCollectableReward: &self.row.columns[self.index_mapping[132]],
                BaseCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[133]],
                MidCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[134]],
                HighCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[135]],
                Level: &self.row.columns[self.index_mapping[136]],
                LevelMax: &self.row.columns[self.index_mapping[137]],
                Unknown0: &self.row.columns[self.index_mapping[138]],
                TermName: &self.row.columns[self.index_mapping[139]],
            },
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.row.columns[self.index_mapping[140]],
                BaseCollectableRating: &self.row.columns[self.index_mapping[141]],
                MidCollectableRating: &self.row.columns[self.index_mapping[142]],
                HighCollectableRating: &self.row.columns[self.index_mapping[143]],
                BaseCollectableReward: &self.row.columns[self.index_mapping[144]],
                MidCollectableReward: &self.row.columns[self.index_mapping[145]],
                HighCollectableReward: &self.row.columns[self.index_mapping[146]],
                BaseCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[147]],
                MidCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[148]],
                HighCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[149]],
                Level: &self.row.columns[self.index_mapping[150]],
                LevelMax: &self.row.columns[self.index_mapping[151]],
                Unknown0: &self.row.columns[self.index_mapping[152]],
                TermName: &self.row.columns[self.index_mapping[153]],
            },
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.row.columns[self.index_mapping[154]],
                BaseCollectableRating: &self.row.columns[self.index_mapping[155]],
                MidCollectableRating: &self.row.columns[self.index_mapping[156]],
                HighCollectableRating: &self.row.columns[self.index_mapping[157]],
                BaseCollectableReward: &self.row.columns[self.index_mapping[158]],
                MidCollectableReward: &self.row.columns[self.index_mapping[159]],
                HighCollectableReward: &self.row.columns[self.index_mapping[160]],
                BaseCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[161]],
                MidCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[162]],
                HighCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[163]],
                Level: &self.row.columns[self.index_mapping[164]],
                LevelMax: &self.row.columns[self.index_mapping[165]],
                Unknown0: &self.row.columns[self.index_mapping[166]],
                TermName: &self.row.columns[self.index_mapping[167]],
            },
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.row.columns[self.index_mapping[168]],
                BaseCollectableRating: &self.row.columns[self.index_mapping[169]],
                MidCollectableRating: &self.row.columns[self.index_mapping[170]],
                HighCollectableRating: &self.row.columns[self.index_mapping[171]],
                BaseCollectableReward: &self.row.columns[self.index_mapping[172]],
                MidCollectableReward: &self.row.columns[self.index_mapping[173]],
                HighCollectableReward: &self.row.columns[self.index_mapping[174]],
                BaseCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[175]],
                MidCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[176]],
                HighCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[177]],
                Level: &self.row.columns[self.index_mapping[178]],
                LevelMax: &self.row.columns[self.index_mapping[179]],
                Unknown0: &self.row.columns[self.index_mapping[180]],
                TermName: &self.row.columns[self.index_mapping[181]],
            },
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.row.columns[self.index_mapping[182]],
                BaseCollectableRating: &self.row.columns[self.index_mapping[183]],
                MidCollectableRating: &self.row.columns[self.index_mapping[184]],
                HighCollectableRating: &self.row.columns[self.index_mapping[185]],
                BaseCollectableReward: &self.row.columns[self.index_mapping[186]],
                MidCollectableReward: &self.row.columns[self.index_mapping[187]],
                HighCollectableReward: &self.row.columns[self.index_mapping[188]],
                BaseCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[189]],
                MidCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[190]],
                HighCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[191]],
                Level: &self.row.columns[self.index_mapping[192]],
                LevelMax: &self.row.columns[self.index_mapping[193]],
                Unknown0: &self.row.columns[self.index_mapping[194]],
                TermName: &self.row.columns[self.index_mapping[195]],
            },
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.row.columns[self.index_mapping[196]],
                BaseCollectableRating: &self.row.columns[self.index_mapping[197]],
                MidCollectableRating: &self.row.columns[self.index_mapping[198]],
                HighCollectableRating: &self.row.columns[self.index_mapping[199]],
                BaseCollectableReward: &self.row.columns[self.index_mapping[200]],
                MidCollectableReward: &self.row.columns[self.index_mapping[201]],
                HighCollectableReward: &self.row.columns[self.index_mapping[202]],
                BaseCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[203]],
                MidCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[204]],
                HighCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[205]],
                Level: &self.row.columns[self.index_mapping[206]],
                LevelMax: &self.row.columns[self.index_mapping[207]],
                Unknown0: &self.row.columns[self.index_mapping[208]],
                TermName: &self.row.columns[self.index_mapping[209]],
            },
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.row.columns[self.index_mapping[210]],
                BaseCollectableRating: &self.row.columns[self.index_mapping[211]],
                MidCollectableRating: &self.row.columns[self.index_mapping[212]],
                HighCollectableRating: &self.row.columns[self.index_mapping[213]],
                BaseCollectableReward: &self.row.columns[self.index_mapping[214]],
                MidCollectableReward: &self.row.columns[self.index_mapping[215]],
                HighCollectableReward: &self.row.columns[self.index_mapping[216]],
                BaseCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[217]],
                MidCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[218]],
                HighCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[219]],
                Level: &self.row.columns[self.index_mapping[220]],
                LevelMax: &self.row.columns[self.index_mapping[221]],
                Unknown0: &self.row.columns[self.index_mapping[222]],
                TermName: &self.row.columns[self.index_mapping[223]],
            },
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.row.columns[self.index_mapping[224]],
                BaseCollectableRating: &self.row.columns[self.index_mapping[225]],
                MidCollectableRating: &self.row.columns[self.index_mapping[226]],
                HighCollectableRating: &self.row.columns[self.index_mapping[227]],
                BaseCollectableReward: &self.row.columns[self.index_mapping[228]],
                MidCollectableReward: &self.row.columns[self.index_mapping[229]],
                HighCollectableReward: &self.row.columns[self.index_mapping[230]],
                BaseCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[231]],
                MidCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[232]],
                HighCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[233]],
                Level: &self.row.columns[self.index_mapping[234]],
                LevelMax: &self.row.columns[self.index_mapping[235]],
                Unknown0: &self.row.columns[self.index_mapping[236]],
                TermName: &self.row.columns[self.index_mapping[237]],
            },
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.row.columns[self.index_mapping[238]],
                BaseCollectableRating: &self.row.columns[self.index_mapping[239]],
                MidCollectableRating: &self.row.columns[self.index_mapping[240]],
                HighCollectableRating: &self.row.columns[self.index_mapping[241]],
                BaseCollectableReward: &self.row.columns[self.index_mapping[242]],
                MidCollectableReward: &self.row.columns[self.index_mapping[243]],
                HighCollectableReward: &self.row.columns[self.index_mapping[244]],
                BaseCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[245]],
                MidCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[246]],
                HighCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[247]],
                Level: &self.row.columns[self.index_mapping[248]],
                LevelMax: &self.row.columns[self.index_mapping[249]],
                Unknown0: &self.row.columns[self.index_mapping[250]],
                TermName: &self.row.columns[self.index_mapping[251]],
            },
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.row.columns[self.index_mapping[252]],
                BaseCollectableRating: &self.row.columns[self.index_mapping[253]],
                MidCollectableRating: &self.row.columns[self.index_mapping[254]],
                HighCollectableRating: &self.row.columns[self.index_mapping[255]],
                BaseCollectableReward: &self.row.columns[self.index_mapping[256]],
                MidCollectableReward: &self.row.columns[self.index_mapping[257]],
                HighCollectableReward: &self.row.columns[self.index_mapping[258]],
                BaseCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[259]],
                MidCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[260]],
                HighCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[261]],
                Level: &self.row.columns[self.index_mapping[262]],
                LevelMax: &self.row.columns[self.index_mapping[263]],
                Unknown0: &self.row.columns[self.index_mapping[264]],
                TermName: &self.row.columns[self.index_mapping[265]],
            },
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.row.columns[self.index_mapping[266]],
                BaseCollectableRating: &self.row.columns[self.index_mapping[267]],
                MidCollectableRating: &self.row.columns[self.index_mapping[268]],
                HighCollectableRating: &self.row.columns[self.index_mapping[269]],
                BaseCollectableReward: &self.row.columns[self.index_mapping[270]],
                MidCollectableReward: &self.row.columns[self.index_mapping[271]],
                HighCollectableReward: &self.row.columns[self.index_mapping[272]],
                BaseCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[273]],
                MidCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[274]],
                HighCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[275]],
                Level: &self.row.columns[self.index_mapping[276]],
                LevelMax: &self.row.columns[self.index_mapping[277]],
                Unknown0: &self.row.columns[self.index_mapping[278]],
                TermName: &self.row.columns[self.index_mapping[279]],
            },
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.row.columns[self.index_mapping[280]],
                BaseCollectableRating: &self.row.columns[self.index_mapping[281]],
                MidCollectableRating: &self.row.columns[self.index_mapping[282]],
                HighCollectableRating: &self.row.columns[self.index_mapping[283]],
                BaseCollectableReward: &self.row.columns[self.index_mapping[284]],
                MidCollectableReward: &self.row.columns[self.index_mapping[285]],
                HighCollectableReward: &self.row.columns[self.index_mapping[286]],
                BaseCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[287]],
                MidCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[288]],
                HighCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[289]],
                Level: &self.row.columns[self.index_mapping[290]],
                LevelMax: &self.row.columns[self.index_mapping[291]],
                Unknown0: &self.row.columns[self.index_mapping[292]],
                TermName: &self.row.columns[self.index_mapping[293]],
            },
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.row.columns[self.index_mapping[294]],
                BaseCollectableRating: &self.row.columns[self.index_mapping[295]],
                MidCollectableRating: &self.row.columns[self.index_mapping[296]],
                HighCollectableRating: &self.row.columns[self.index_mapping[297]],
                BaseCollectableReward: &self.row.columns[self.index_mapping[298]],
                MidCollectableReward: &self.row.columns[self.index_mapping[299]],
                HighCollectableReward: &self.row.columns[self.index_mapping[300]],
                BaseCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[301]],
                MidCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[302]],
                HighCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[303]],
                Level: &self.row.columns[self.index_mapping[304]],
                LevelMax: &self.row.columns[self.index_mapping[305]],
                Unknown0: &self.row.columns[self.index_mapping[306]],
                TermName: &self.row.columns[self.index_mapping[307]],
            },
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.row.columns[self.index_mapping[308]],
                BaseCollectableRating: &self.row.columns[self.index_mapping[309]],
                MidCollectableRating: &self.row.columns[self.index_mapping[310]],
                HighCollectableRating: &self.row.columns[self.index_mapping[311]],
                BaseCollectableReward: &self.row.columns[self.index_mapping[312]],
                MidCollectableReward: &self.row.columns[self.index_mapping[313]],
                HighCollectableReward: &self.row.columns[self.index_mapping[314]],
                BaseCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[315]],
                MidCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[316]],
                HighCollectableRewardPostPhase: &self
                    .row
                    .columns[self.index_mapping[317]],
                Level: &self.row.columns[self.index_mapping[318]],
                LevelMax: &self.row.columns[self.index_mapping[319]],
                Unknown0: &self.row.columns[self.index_mapping[320]],
                TermName: &self.row.columns[self.index_mapping[321]],
            },
        ]
    }
}
