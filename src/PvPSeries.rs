//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LevelRewardsElement {
    pub LevelRewardItem: [i32; 2],
    pub Unknown0: i32,
    pub LevelRewardCount: [u16; 2],
}
#[derive(Debug, Clone)]
pub struct PvPSeriesSheet {
    sheet: Sheet,
}
impl PvPSeriesSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("PvPSeries")?;
        let sheet = resolver.read_excel_sheet(&exh, "PvPSeries", language)?;
        Ok(Self { sheet })
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
        Some(Self::Row { row })
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
}
impl<'a> PvPSeriesRow<'a> {
    pub fn LevelRewards(&'a self) -> [LevelRewardsElement; 32] {
        [
            LevelRewardsElement {
                LevelRewardItem: [
                    self.row.columns[1].into_i32().copied().unwrap(),
                    self.row.columns[33].into_i32().copied().unwrap(),
                ],
                Unknown0: self.row.columns[129].into_i32().copied().unwrap(),
                LevelRewardCount: [
                    self.row.columns[65].into_u16().copied().unwrap(),
                    self.row.columns[97].into_u16().copied().unwrap(),
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    self.row.columns[2].into_i32().copied().unwrap(),
                    self.row.columns[34].into_i32().copied().unwrap(),
                ],
                Unknown0: self.row.columns[130].into_i32().copied().unwrap(),
                LevelRewardCount: [
                    self.row.columns[66].into_u16().copied().unwrap(),
                    self.row.columns[98].into_u16().copied().unwrap(),
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    self.row.columns[3].into_i32().copied().unwrap(),
                    self.row.columns[35].into_i32().copied().unwrap(),
                ],
                Unknown0: self.row.columns[131].into_i32().copied().unwrap(),
                LevelRewardCount: [
                    self.row.columns[67].into_u16().copied().unwrap(),
                    self.row.columns[99].into_u16().copied().unwrap(),
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    self.row.columns[4].into_i32().copied().unwrap(),
                    self.row.columns[36].into_i32().copied().unwrap(),
                ],
                Unknown0: self.row.columns[132].into_i32().copied().unwrap(),
                LevelRewardCount: [
                    self.row.columns[68].into_u16().copied().unwrap(),
                    self.row.columns[100].into_u16().copied().unwrap(),
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    self.row.columns[5].into_i32().copied().unwrap(),
                    self.row.columns[37].into_i32().copied().unwrap(),
                ],
                Unknown0: self.row.columns[133].into_i32().copied().unwrap(),
                LevelRewardCount: [
                    self.row.columns[69].into_u16().copied().unwrap(),
                    self.row.columns[101].into_u16().copied().unwrap(),
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    self.row.columns[6].into_i32().copied().unwrap(),
                    self.row.columns[38].into_i32().copied().unwrap(),
                ],
                Unknown0: self.row.columns[134].into_i32().copied().unwrap(),
                LevelRewardCount: [
                    self.row.columns[70].into_u16().copied().unwrap(),
                    self.row.columns[102].into_u16().copied().unwrap(),
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    self.row.columns[7].into_i32().copied().unwrap(),
                    self.row.columns[39].into_i32().copied().unwrap(),
                ],
                Unknown0: self.row.columns[135].into_i32().copied().unwrap(),
                LevelRewardCount: [
                    self.row.columns[71].into_u16().copied().unwrap(),
                    self.row.columns[103].into_u16().copied().unwrap(),
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    self.row.columns[8].into_i32().copied().unwrap(),
                    self.row.columns[40].into_i32().copied().unwrap(),
                ],
                Unknown0: self.row.columns[136].into_i32().copied().unwrap(),
                LevelRewardCount: [
                    self.row.columns[72].into_u16().copied().unwrap(),
                    self.row.columns[104].into_u16().copied().unwrap(),
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    self.row.columns[9].into_i32().copied().unwrap(),
                    self.row.columns[41].into_i32().copied().unwrap(),
                ],
                Unknown0: self.row.columns[137].into_i32().copied().unwrap(),
                LevelRewardCount: [
                    self.row.columns[73].into_u16().copied().unwrap(),
                    self.row.columns[105].into_u16().copied().unwrap(),
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    self.row.columns[10].into_i32().copied().unwrap(),
                    self.row.columns[42].into_i32().copied().unwrap(),
                ],
                Unknown0: self.row.columns[138].into_i32().copied().unwrap(),
                LevelRewardCount: [
                    self.row.columns[74].into_u16().copied().unwrap(),
                    self.row.columns[106].into_u16().copied().unwrap(),
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    self.row.columns[11].into_i32().copied().unwrap(),
                    self.row.columns[43].into_i32().copied().unwrap(),
                ],
                Unknown0: self.row.columns[139].into_i32().copied().unwrap(),
                LevelRewardCount: [
                    self.row.columns[75].into_u16().copied().unwrap(),
                    self.row.columns[107].into_u16().copied().unwrap(),
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    self.row.columns[12].into_i32().copied().unwrap(),
                    self.row.columns[44].into_i32().copied().unwrap(),
                ],
                Unknown0: self.row.columns[140].into_i32().copied().unwrap(),
                LevelRewardCount: [
                    self.row.columns[76].into_u16().copied().unwrap(),
                    self.row.columns[108].into_u16().copied().unwrap(),
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    self.row.columns[13].into_i32().copied().unwrap(),
                    self.row.columns[45].into_i32().copied().unwrap(),
                ],
                Unknown0: self.row.columns[141].into_i32().copied().unwrap(),
                LevelRewardCount: [
                    self.row.columns[77].into_u16().copied().unwrap(),
                    self.row.columns[109].into_u16().copied().unwrap(),
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    self.row.columns[14].into_i32().copied().unwrap(),
                    self.row.columns[46].into_i32().copied().unwrap(),
                ],
                Unknown0: self.row.columns[142].into_i32().copied().unwrap(),
                LevelRewardCount: [
                    self.row.columns[78].into_u16().copied().unwrap(),
                    self.row.columns[110].into_u16().copied().unwrap(),
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    self.row.columns[15].into_i32().copied().unwrap(),
                    self.row.columns[47].into_i32().copied().unwrap(),
                ],
                Unknown0: self.row.columns[143].into_i32().copied().unwrap(),
                LevelRewardCount: [
                    self.row.columns[79].into_u16().copied().unwrap(),
                    self.row.columns[111].into_u16().copied().unwrap(),
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    self.row.columns[16].into_i32().copied().unwrap(),
                    self.row.columns[48].into_i32().copied().unwrap(),
                ],
                Unknown0: self.row.columns[144].into_i32().copied().unwrap(),
                LevelRewardCount: [
                    self.row.columns[80].into_u16().copied().unwrap(),
                    self.row.columns[112].into_u16().copied().unwrap(),
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    self.row.columns[17].into_i32().copied().unwrap(),
                    self.row.columns[49].into_i32().copied().unwrap(),
                ],
                Unknown0: self.row.columns[145].into_i32().copied().unwrap(),
                LevelRewardCount: [
                    self.row.columns[81].into_u16().copied().unwrap(),
                    self.row.columns[113].into_u16().copied().unwrap(),
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    self.row.columns[18].into_i32().copied().unwrap(),
                    self.row.columns[50].into_i32().copied().unwrap(),
                ],
                Unknown0: self.row.columns[146].into_i32().copied().unwrap(),
                LevelRewardCount: [
                    self.row.columns[82].into_u16().copied().unwrap(),
                    self.row.columns[114].into_u16().copied().unwrap(),
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    self.row.columns[19].into_i32().copied().unwrap(),
                    self.row.columns[51].into_i32().copied().unwrap(),
                ],
                Unknown0: self.row.columns[147].into_i32().copied().unwrap(),
                LevelRewardCount: [
                    self.row.columns[83].into_u16().copied().unwrap(),
                    self.row.columns[115].into_u16().copied().unwrap(),
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    self.row.columns[20].into_i32().copied().unwrap(),
                    self.row.columns[52].into_i32().copied().unwrap(),
                ],
                Unknown0: self.row.columns[148].into_i32().copied().unwrap(),
                LevelRewardCount: [
                    self.row.columns[84].into_u16().copied().unwrap(),
                    self.row.columns[116].into_u16().copied().unwrap(),
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    self.row.columns[21].into_i32().copied().unwrap(),
                    self.row.columns[53].into_i32().copied().unwrap(),
                ],
                Unknown0: self.row.columns[149].into_i32().copied().unwrap(),
                LevelRewardCount: [
                    self.row.columns[85].into_u16().copied().unwrap(),
                    self.row.columns[117].into_u16().copied().unwrap(),
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    self.row.columns[22].into_i32().copied().unwrap(),
                    self.row.columns[54].into_i32().copied().unwrap(),
                ],
                Unknown0: self.row.columns[150].into_i32().copied().unwrap(),
                LevelRewardCount: [
                    self.row.columns[86].into_u16().copied().unwrap(),
                    self.row.columns[118].into_u16().copied().unwrap(),
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    self.row.columns[23].into_i32().copied().unwrap(),
                    self.row.columns[55].into_i32().copied().unwrap(),
                ],
                Unknown0: self.row.columns[151].into_i32().copied().unwrap(),
                LevelRewardCount: [
                    self.row.columns[87].into_u16().copied().unwrap(),
                    self.row.columns[119].into_u16().copied().unwrap(),
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    self.row.columns[24].into_i32().copied().unwrap(),
                    self.row.columns[56].into_i32().copied().unwrap(),
                ],
                Unknown0: self.row.columns[152].into_i32().copied().unwrap(),
                LevelRewardCount: [
                    self.row.columns[88].into_u16().copied().unwrap(),
                    self.row.columns[120].into_u16().copied().unwrap(),
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    self.row.columns[25].into_i32().copied().unwrap(),
                    self.row.columns[57].into_i32().copied().unwrap(),
                ],
                Unknown0: self.row.columns[153].into_i32().copied().unwrap(),
                LevelRewardCount: [
                    self.row.columns[89].into_u16().copied().unwrap(),
                    self.row.columns[121].into_u16().copied().unwrap(),
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    self.row.columns[26].into_i32().copied().unwrap(),
                    self.row.columns[58].into_i32().copied().unwrap(),
                ],
                Unknown0: self.row.columns[154].into_i32().copied().unwrap(),
                LevelRewardCount: [
                    self.row.columns[90].into_u16().copied().unwrap(),
                    self.row.columns[122].into_u16().copied().unwrap(),
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    self.row.columns[27].into_i32().copied().unwrap(),
                    self.row.columns[59].into_i32().copied().unwrap(),
                ],
                Unknown0: self.row.columns[155].into_i32().copied().unwrap(),
                LevelRewardCount: [
                    self.row.columns[91].into_u16().copied().unwrap(),
                    self.row.columns[123].into_u16().copied().unwrap(),
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    self.row.columns[28].into_i32().copied().unwrap(),
                    self.row.columns[60].into_i32().copied().unwrap(),
                ],
                Unknown0: self.row.columns[156].into_i32().copied().unwrap(),
                LevelRewardCount: [
                    self.row.columns[92].into_u16().copied().unwrap(),
                    self.row.columns[124].into_u16().copied().unwrap(),
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    self.row.columns[29].into_i32().copied().unwrap(),
                    self.row.columns[61].into_i32().copied().unwrap(),
                ],
                Unknown0: self.row.columns[157].into_i32().copied().unwrap(),
                LevelRewardCount: [
                    self.row.columns[93].into_u16().copied().unwrap(),
                    self.row.columns[125].into_u16().copied().unwrap(),
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    self.row.columns[30].into_i32().copied().unwrap(),
                    self.row.columns[62].into_i32().copied().unwrap(),
                ],
                Unknown0: self.row.columns[158].into_i32().copied().unwrap(),
                LevelRewardCount: [
                    self.row.columns[94].into_u16().copied().unwrap(),
                    self.row.columns[126].into_u16().copied().unwrap(),
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    self.row.columns[31].into_i32().copied().unwrap(),
                    self.row.columns[63].into_i32().copied().unwrap(),
                ],
                Unknown0: self.row.columns[159].into_i32().copied().unwrap(),
                LevelRewardCount: [
                    self.row.columns[95].into_u16().copied().unwrap(),
                    self.row.columns[127].into_u16().copied().unwrap(),
                ],
            },
            LevelRewardsElement {
                LevelRewardItem: [
                    self.row.columns[32].into_i32().copied().unwrap(),
                    self.row.columns[64].into_i32().copied().unwrap(),
                ],
                Unknown0: self.row.columns[160].into_i32().copied().unwrap(),
                LevelRewardCount: [
                    self.row.columns[96].into_u16().copied().unwrap(),
                    self.row.columns[128].into_u16().copied().unwrap(),
                ],
            },
        ]
    }
    pub fn Unknown0(&'a self) -> u8 {
        self.row.columns[0].into_u8().copied().unwrap()
    }
}
