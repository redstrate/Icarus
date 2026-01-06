//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    common::Language,
};
pub struct LevelRewardsElement<'a> {
    pub LevelRewardItem: [&'a Field; 2],
    pub Unknown0: &'a Field,
    pub LevelRewardCount: [&'a Field; 2],
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
impl StructuredSheet for PvPSeriesSheet {
    type Row = PvPSeriesRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        let column_defs = &self.sheet.exh.column_definitions;
        let mut zipped: Vec<_> = row
            .columns
            .clone()
            .into_iter()
            .zip(column_defs)
            .collect();
        zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
        let (columns, _): (Vec<Field>, Vec<ExcelColumnDefinition>) = zipped
            .into_iter()
            .unzip();
        Some(Self::Row { columns })
    }
}
impl<'a> IntoIterator for &'a PvPSeriesSheet {
    type Item = (u32, Vec<(u16, PvPSeriesRow)>);
    type IntoIter = StructuredSheetIterator<'a, PvPSeriesSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, PvPSeriesSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct PvPSeriesRow {
    columns: Vec<Field>,
}
impl PvPSeriesRow {
    pub fn LevelRewards<'a>(&'a self) -> [LevelRewardsElement<'a>; 32] {
        [
            LevelRewardsElement {
                LevelRewardItem: [&self.columns[0], &self.columns[1]],
                Unknown0: &self.columns[2],
                LevelRewardCount: [&self.columns[3], &self.columns[4]],
            },
            LevelRewardsElement {
                LevelRewardItem: [&self.columns[5], &self.columns[6]],
                Unknown0: &self.columns[7],
                LevelRewardCount: [&self.columns[8], &self.columns[9]],
            },
            LevelRewardsElement {
                LevelRewardItem: [&self.columns[10], &self.columns[11]],
                Unknown0: &self.columns[12],
                LevelRewardCount: [&self.columns[13], &self.columns[14]],
            },
            LevelRewardsElement {
                LevelRewardItem: [&self.columns[15], &self.columns[16]],
                Unknown0: &self.columns[17],
                LevelRewardCount: [&self.columns[18], &self.columns[19]],
            },
            LevelRewardsElement {
                LevelRewardItem: [&self.columns[20], &self.columns[21]],
                Unknown0: &self.columns[22],
                LevelRewardCount: [&self.columns[23], &self.columns[24]],
            },
            LevelRewardsElement {
                LevelRewardItem: [&self.columns[25], &self.columns[26]],
                Unknown0: &self.columns[27],
                LevelRewardCount: [&self.columns[28], &self.columns[29]],
            },
            LevelRewardsElement {
                LevelRewardItem: [&self.columns[30], &self.columns[31]],
                Unknown0: &self.columns[32],
                LevelRewardCount: [&self.columns[33], &self.columns[34]],
            },
            LevelRewardsElement {
                LevelRewardItem: [&self.columns[35], &self.columns[36]],
                Unknown0: &self.columns[37],
                LevelRewardCount: [&self.columns[38], &self.columns[39]],
            },
            LevelRewardsElement {
                LevelRewardItem: [&self.columns[40], &self.columns[41]],
                Unknown0: &self.columns[42],
                LevelRewardCount: [&self.columns[43], &self.columns[44]],
            },
            LevelRewardsElement {
                LevelRewardItem: [&self.columns[45], &self.columns[46]],
                Unknown0: &self.columns[47],
                LevelRewardCount: [&self.columns[48], &self.columns[49]],
            },
            LevelRewardsElement {
                LevelRewardItem: [&self.columns[50], &self.columns[51]],
                Unknown0: &self.columns[52],
                LevelRewardCount: [&self.columns[53], &self.columns[54]],
            },
            LevelRewardsElement {
                LevelRewardItem: [&self.columns[55], &self.columns[56]],
                Unknown0: &self.columns[57],
                LevelRewardCount: [&self.columns[58], &self.columns[59]],
            },
            LevelRewardsElement {
                LevelRewardItem: [&self.columns[60], &self.columns[61]],
                Unknown0: &self.columns[62],
                LevelRewardCount: [&self.columns[63], &self.columns[64]],
            },
            LevelRewardsElement {
                LevelRewardItem: [&self.columns[65], &self.columns[66]],
                Unknown0: &self.columns[67],
                LevelRewardCount: [&self.columns[68], &self.columns[69]],
            },
            LevelRewardsElement {
                LevelRewardItem: [&self.columns[70], &self.columns[71]],
                Unknown0: &self.columns[72],
                LevelRewardCount: [&self.columns[73], &self.columns[74]],
            },
            LevelRewardsElement {
                LevelRewardItem: [&self.columns[75], &self.columns[76]],
                Unknown0: &self.columns[77],
                LevelRewardCount: [&self.columns[78], &self.columns[79]],
            },
            LevelRewardsElement {
                LevelRewardItem: [&self.columns[80], &self.columns[81]],
                Unknown0: &self.columns[82],
                LevelRewardCount: [&self.columns[83], &self.columns[84]],
            },
            LevelRewardsElement {
                LevelRewardItem: [&self.columns[85], &self.columns[86]],
                Unknown0: &self.columns[87],
                LevelRewardCount: [&self.columns[88], &self.columns[89]],
            },
            LevelRewardsElement {
                LevelRewardItem: [&self.columns[90], &self.columns[91]],
                Unknown0: &self.columns[92],
                LevelRewardCount: [&self.columns[93], &self.columns[94]],
            },
            LevelRewardsElement {
                LevelRewardItem: [&self.columns[95], &self.columns[96]],
                Unknown0: &self.columns[97],
                LevelRewardCount: [&self.columns[98], &self.columns[99]],
            },
            LevelRewardsElement {
                LevelRewardItem: [&self.columns[100], &self.columns[101]],
                Unknown0: &self.columns[102],
                LevelRewardCount: [&self.columns[103], &self.columns[104]],
            },
            LevelRewardsElement {
                LevelRewardItem: [&self.columns[105], &self.columns[106]],
                Unknown0: &self.columns[107],
                LevelRewardCount: [&self.columns[108], &self.columns[109]],
            },
            LevelRewardsElement {
                LevelRewardItem: [&self.columns[110], &self.columns[111]],
                Unknown0: &self.columns[112],
                LevelRewardCount: [&self.columns[113], &self.columns[114]],
            },
            LevelRewardsElement {
                LevelRewardItem: [&self.columns[115], &self.columns[116]],
                Unknown0: &self.columns[117],
                LevelRewardCount: [&self.columns[118], &self.columns[119]],
            },
            LevelRewardsElement {
                LevelRewardItem: [&self.columns[120], &self.columns[121]],
                Unknown0: &self.columns[122],
                LevelRewardCount: [&self.columns[123], &self.columns[124]],
            },
            LevelRewardsElement {
                LevelRewardItem: [&self.columns[125], &self.columns[126]],
                Unknown0: &self.columns[127],
                LevelRewardCount: [&self.columns[128], &self.columns[129]],
            },
            LevelRewardsElement {
                LevelRewardItem: [&self.columns[130], &self.columns[131]],
                Unknown0: &self.columns[132],
                LevelRewardCount: [&self.columns[133], &self.columns[134]],
            },
            LevelRewardsElement {
                LevelRewardItem: [&self.columns[135], &self.columns[136]],
                Unknown0: &self.columns[137],
                LevelRewardCount: [&self.columns[138], &self.columns[139]],
            },
            LevelRewardsElement {
                LevelRewardItem: [&self.columns[140], &self.columns[141]],
                Unknown0: &self.columns[142],
                LevelRewardCount: [&self.columns[143], &self.columns[144]],
            },
            LevelRewardsElement {
                LevelRewardItem: [&self.columns[145], &self.columns[146]],
                Unknown0: &self.columns[147],
                LevelRewardCount: [&self.columns[148], &self.columns[149]],
            },
            LevelRewardsElement {
                LevelRewardItem: [&self.columns[150], &self.columns[151]],
                Unknown0: &self.columns[152],
                LevelRewardCount: [&self.columns[153], &self.columns[154]],
            },
            LevelRewardsElement {
                LevelRewardItem: [&self.columns[155], &self.columns[156]],
                Unknown0: &self.columns[157],
                LevelRewardCount: [&self.columns[158], &self.columns[159]],
            },
        ]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a Field {
        &self.columns[160]
    }
}
