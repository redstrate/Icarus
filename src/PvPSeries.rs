//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
pub struct LevelRewardsElement<'a> {
    pub LevelRewardItem: [&'a Field; 2],
    pub Unknown0: &'a Field,
    pub LevelRewardCount: [&'a Field; 2],
}
#[derive(Debug, Clone)]
pub struct PvPSeriesSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl PvPSeriesSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("PvPSeries")?;
        let sheet = resolver.read_excel_sheet(&exh, "PvPSeries", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<PvPSeriesRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<PvPSeriesRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for PvPSeriesSheet {
    type Row = PvPSeriesRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a PvPSeriesSheet {
    type Item = (u32, Vec<(u16, PvPSeriesRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, PvPSeriesSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, PvPSeriesSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct PvPSeriesRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> PvPSeriesRow<'a> {
    pub fn LevelRewards(&'a self) -> [LevelRewardsElement<'a>; 32] {
        [
            LevelRewardsElement {
                LevelRewardItem: [
                    &self.row.columns[self.index_mapping[0]],
                    &self.row.columns[self.index_mapping[1]],
                ],
                Unknown0: &self.row.columns[self.index_mapping[2]],
                LevelRewardCount: [
                    &self.row.columns[self.index_mapping[3]],
                    &self.row.columns[self.index_mapping[4]],
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    &self.row.columns[self.index_mapping[5]],
                    &self.row.columns[self.index_mapping[6]],
                ],
                Unknown0: &self.row.columns[self.index_mapping[7]],
                LevelRewardCount: [
                    &self.row.columns[self.index_mapping[8]],
                    &self.row.columns[self.index_mapping[9]],
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    &self.row.columns[self.index_mapping[10]],
                    &self.row.columns[self.index_mapping[11]],
                ],
                Unknown0: &self.row.columns[self.index_mapping[12]],
                LevelRewardCount: [
                    &self.row.columns[self.index_mapping[13]],
                    &self.row.columns[self.index_mapping[14]],
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    &self.row.columns[self.index_mapping[15]],
                    &self.row.columns[self.index_mapping[16]],
                ],
                Unknown0: &self.row.columns[self.index_mapping[17]],
                LevelRewardCount: [
                    &self.row.columns[self.index_mapping[18]],
                    &self.row.columns[self.index_mapping[19]],
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    &self.row.columns[self.index_mapping[20]],
                    &self.row.columns[self.index_mapping[21]],
                ],
                Unknown0: &self.row.columns[self.index_mapping[22]],
                LevelRewardCount: [
                    &self.row.columns[self.index_mapping[23]],
                    &self.row.columns[self.index_mapping[24]],
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    &self.row.columns[self.index_mapping[25]],
                    &self.row.columns[self.index_mapping[26]],
                ],
                Unknown0: &self.row.columns[self.index_mapping[27]],
                LevelRewardCount: [
                    &self.row.columns[self.index_mapping[28]],
                    &self.row.columns[self.index_mapping[29]],
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    &self.row.columns[self.index_mapping[30]],
                    &self.row.columns[self.index_mapping[31]],
                ],
                Unknown0: &self.row.columns[self.index_mapping[32]],
                LevelRewardCount: [
                    &self.row.columns[self.index_mapping[33]],
                    &self.row.columns[self.index_mapping[34]],
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    &self.row.columns[self.index_mapping[35]],
                    &self.row.columns[self.index_mapping[36]],
                ],
                Unknown0: &self.row.columns[self.index_mapping[37]],
                LevelRewardCount: [
                    &self.row.columns[self.index_mapping[38]],
                    &self.row.columns[self.index_mapping[39]],
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    &self.row.columns[self.index_mapping[40]],
                    &self.row.columns[self.index_mapping[41]],
                ],
                Unknown0: &self.row.columns[self.index_mapping[42]],
                LevelRewardCount: [
                    &self.row.columns[self.index_mapping[43]],
                    &self.row.columns[self.index_mapping[44]],
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    &self.row.columns[self.index_mapping[45]],
                    &self.row.columns[self.index_mapping[46]],
                ],
                Unknown0: &self.row.columns[self.index_mapping[47]],
                LevelRewardCount: [
                    &self.row.columns[self.index_mapping[48]],
                    &self.row.columns[self.index_mapping[49]],
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    &self.row.columns[self.index_mapping[50]],
                    &self.row.columns[self.index_mapping[51]],
                ],
                Unknown0: &self.row.columns[self.index_mapping[52]],
                LevelRewardCount: [
                    &self.row.columns[self.index_mapping[53]],
                    &self.row.columns[self.index_mapping[54]],
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    &self.row.columns[self.index_mapping[55]],
                    &self.row.columns[self.index_mapping[56]],
                ],
                Unknown0: &self.row.columns[self.index_mapping[57]],
                LevelRewardCount: [
                    &self.row.columns[self.index_mapping[58]],
                    &self.row.columns[self.index_mapping[59]],
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    &self.row.columns[self.index_mapping[60]],
                    &self.row.columns[self.index_mapping[61]],
                ],
                Unknown0: &self.row.columns[self.index_mapping[62]],
                LevelRewardCount: [
                    &self.row.columns[self.index_mapping[63]],
                    &self.row.columns[self.index_mapping[64]],
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    &self.row.columns[self.index_mapping[65]],
                    &self.row.columns[self.index_mapping[66]],
                ],
                Unknown0: &self.row.columns[self.index_mapping[67]],
                LevelRewardCount: [
                    &self.row.columns[self.index_mapping[68]],
                    &self.row.columns[self.index_mapping[69]],
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    &self.row.columns[self.index_mapping[70]],
                    &self.row.columns[self.index_mapping[71]],
                ],
                Unknown0: &self.row.columns[self.index_mapping[72]],
                LevelRewardCount: [
                    &self.row.columns[self.index_mapping[73]],
                    &self.row.columns[self.index_mapping[74]],
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    &self.row.columns[self.index_mapping[75]],
                    &self.row.columns[self.index_mapping[76]],
                ],
                Unknown0: &self.row.columns[self.index_mapping[77]],
                LevelRewardCount: [
                    &self.row.columns[self.index_mapping[78]],
                    &self.row.columns[self.index_mapping[79]],
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    &self.row.columns[self.index_mapping[80]],
                    &self.row.columns[self.index_mapping[81]],
                ],
                Unknown0: &self.row.columns[self.index_mapping[82]],
                LevelRewardCount: [
                    &self.row.columns[self.index_mapping[83]],
                    &self.row.columns[self.index_mapping[84]],
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    &self.row.columns[self.index_mapping[85]],
                    &self.row.columns[self.index_mapping[86]],
                ],
                Unknown0: &self.row.columns[self.index_mapping[87]],
                LevelRewardCount: [
                    &self.row.columns[self.index_mapping[88]],
                    &self.row.columns[self.index_mapping[89]],
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    &self.row.columns[self.index_mapping[90]],
                    &self.row.columns[self.index_mapping[91]],
                ],
                Unknown0: &self.row.columns[self.index_mapping[92]],
                LevelRewardCount: [
                    &self.row.columns[self.index_mapping[93]],
                    &self.row.columns[self.index_mapping[94]],
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    &self.row.columns[self.index_mapping[95]],
                    &self.row.columns[self.index_mapping[96]],
                ],
                Unknown0: &self.row.columns[self.index_mapping[97]],
                LevelRewardCount: [
                    &self.row.columns[self.index_mapping[98]],
                    &self.row.columns[self.index_mapping[99]],
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    &self.row.columns[self.index_mapping[100]],
                    &self.row.columns[self.index_mapping[101]],
                ],
                Unknown0: &self.row.columns[self.index_mapping[102]],
                LevelRewardCount: [
                    &self.row.columns[self.index_mapping[103]],
                    &self.row.columns[self.index_mapping[104]],
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    &self.row.columns[self.index_mapping[105]],
                    &self.row.columns[self.index_mapping[106]],
                ],
                Unknown0: &self.row.columns[self.index_mapping[107]],
                LevelRewardCount: [
                    &self.row.columns[self.index_mapping[108]],
                    &self.row.columns[self.index_mapping[109]],
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    &self.row.columns[self.index_mapping[110]],
                    &self.row.columns[self.index_mapping[111]],
                ],
                Unknown0: &self.row.columns[self.index_mapping[112]],
                LevelRewardCount: [
                    &self.row.columns[self.index_mapping[113]],
                    &self.row.columns[self.index_mapping[114]],
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    &self.row.columns[self.index_mapping[115]],
                    &self.row.columns[self.index_mapping[116]],
                ],
                Unknown0: &self.row.columns[self.index_mapping[117]],
                LevelRewardCount: [
                    &self.row.columns[self.index_mapping[118]],
                    &self.row.columns[self.index_mapping[119]],
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    &self.row.columns[self.index_mapping[120]],
                    &self.row.columns[self.index_mapping[121]],
                ],
                Unknown0: &self.row.columns[self.index_mapping[122]],
                LevelRewardCount: [
                    &self.row.columns[self.index_mapping[123]],
                    &self.row.columns[self.index_mapping[124]],
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    &self.row.columns[self.index_mapping[125]],
                    &self.row.columns[self.index_mapping[126]],
                ],
                Unknown0: &self.row.columns[self.index_mapping[127]],
                LevelRewardCount: [
                    &self.row.columns[self.index_mapping[128]],
                    &self.row.columns[self.index_mapping[129]],
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    &self.row.columns[self.index_mapping[130]],
                    &self.row.columns[self.index_mapping[131]],
                ],
                Unknown0: &self.row.columns[self.index_mapping[132]],
                LevelRewardCount: [
                    &self.row.columns[self.index_mapping[133]],
                    &self.row.columns[self.index_mapping[134]],
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    &self.row.columns[self.index_mapping[135]],
                    &self.row.columns[self.index_mapping[136]],
                ],
                Unknown0: &self.row.columns[self.index_mapping[137]],
                LevelRewardCount: [
                    &self.row.columns[self.index_mapping[138]],
                    &self.row.columns[self.index_mapping[139]],
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    &self.row.columns[self.index_mapping[140]],
                    &self.row.columns[self.index_mapping[141]],
                ],
                Unknown0: &self.row.columns[self.index_mapping[142]],
                LevelRewardCount: [
                    &self.row.columns[self.index_mapping[143]],
                    &self.row.columns[self.index_mapping[144]],
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    &self.row.columns[self.index_mapping[145]],
                    &self.row.columns[self.index_mapping[146]],
                ],
                Unknown0: &self.row.columns[self.index_mapping[147]],
                LevelRewardCount: [
                    &self.row.columns[self.index_mapping[148]],
                    &self.row.columns[self.index_mapping[149]],
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    &self.row.columns[self.index_mapping[150]],
                    &self.row.columns[self.index_mapping[151]],
                ],
                Unknown0: &self.row.columns[self.index_mapping[152]],
                LevelRewardCount: [
                    &self.row.columns[self.index_mapping[153]],
                    &self.row.columns[self.index_mapping[154]],
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    &self.row.columns[self.index_mapping[155]],
                    &self.row.columns[self.index_mapping[156]],
                ],
                Unknown0: &self.row.columns[self.index_mapping[157]],
                LevelRewardCount: [
                    &self.row.columns[self.index_mapping[158]],
                    &self.row.columns[self.index_mapping[159]],
                ],
            },
        ]
    }
    pub fn Unknown0(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[160]]
    }
}
