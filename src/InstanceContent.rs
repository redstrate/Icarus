//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
#[derive(Debug, Clone)]
pub struct InstanceContentSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl InstanceContentSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("InstanceContent")?;
        let sheet = resolver.read_excel_sheet(&exh, "InstanceContent", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<InstanceContentRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<InstanceContentRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for InstanceContentSheet {
    type Row = InstanceContentRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a InstanceContentSheet {
    type Item = (u32, Vec<(u16, InstanceContentRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, InstanceContentSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, InstanceContentSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct InstanceContentRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> InstanceContentRow<'a> {
    pub fn NewPlayerBonusGil(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[0]]
    }
    pub fn NewPlayerBonusExp(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1]]
    }
    pub fn FinalBossExp(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[2]]
    }
    pub fn Unknown0(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[3]]
    }
    pub fn BossExp(&'a self) -> [&'a Field; 5] {
        [
            &self.row.columns[self.index_mapping[4]],
            &self.row.columns[self.index_mapping[5]],
            &self.row.columns[self.index_mapping[6]],
            &self.row.columns[self.index_mapping[7]],
            &self.row.columns[self.index_mapping[8]],
        ]
    }
    pub fn InstanceClearExp(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[9]]
    }
    pub fn InstanceClearGil(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[10]]
    }
    pub fn InstanceContentRewardItem(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[11]]
    }
    pub fn NewPlayerBonusA(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[12]]
    }
    pub fn NewPlayerBonusB(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[13]]
    }
    pub fn FinalBossCurrencyA(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[14]]
    }
    pub fn FinalBossCurrencyB(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[15]]
    }
    pub fn FinalBossCurrencyC(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[16]]
    }
    pub fn BossCurrencyA(&'a self) -> [&'a Field; 5] {
        [
            &self.row.columns[self.index_mapping[17]],
            &self.row.columns[self.index_mapping[18]],
            &self.row.columns[self.index_mapping[19]],
            &self.row.columns[self.index_mapping[20]],
            &self.row.columns[self.index_mapping[21]],
        ]
    }
    pub fn BossCurrencyB(&'a self) -> [&'a Field; 5] {
        [
            &self.row.columns[self.index_mapping[22]],
            &self.row.columns[self.index_mapping[23]],
            &self.row.columns[self.index_mapping[24]],
            &self.row.columns[self.index_mapping[25]],
            &self.row.columns[self.index_mapping[26]],
        ]
    }
    pub fn BossCurrencyC(&'a self) -> [&'a Field; 5] {
        [
            &self.row.columns[self.index_mapping[27]],
            &self.row.columns[self.index_mapping[28]],
            &self.row.columns[self.index_mapping[29]],
            &self.row.columns[self.index_mapping[30]],
            &self.row.columns[self.index_mapping[31]],
        ]
    }
    pub fn Unknown1(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[32]]
    }
    pub fn Unknown8(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[33]]
    }
    pub fn LimitedTimeBonus(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[34]]
    }
    pub fn Cutscene(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[35]]
    }
    pub fn LGBEventRange(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[36]]
    }
    pub fn InstanceContentTextDataBossStart(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[37]]
    }
    pub fn InstanceContentTextDataBossEnd(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[38]]
    }
    pub fn BNpcBaseBoss(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[39]]
    }
    pub fn InstanceContentTextDataObjectiveStart(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[40]]
    }
    pub fn InstanceContentTextDataObjectiveEnd(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[41]]
    }
    pub fn Unknown2(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[42]]
    }
    pub fn ReqInstance(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[43]]
    }
    pub fn InstanceContentBuff(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[44]]
    }
    pub fn TimeLimitmin(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[45]]
    }
    pub fn BGM(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[46]]
    }
    pub fn WinBGM(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[47]]
    }
    pub fn ContentFinderCondition(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[48]]
    }
    pub fn SortKey(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[49]]
    }
    pub fn ContentRoute(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[50]]
    }
    pub fn ContentDirectorManagedSG(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[51]]
    }
    pub fn ContentTodo(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[52]]
    }
    pub fn Unknown6(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[53]]
    }
    pub fn Unknown7(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[54]]
    }
    pub fn ContentEventItem(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[55]]
    }
    pub fn ContentDirectorBattleTalk(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[56]]
    }
    pub fn PartyCondition(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[57]]
    }
    pub fn InstanceContentType(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[58]]
    }
    pub fn WeekRestriction(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[59]]
    }
    pub fn Colosseum(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[60]]
    }
    pub fn Unknown9(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[61]]
    }
    pub fn QTE1(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[62]]
    }
    pub fn QTE2(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[63]]
    }
    pub fn Unknown12(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[64]]
    }
    pub fn ContentAttributeRect(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[65]]
    }
    pub fn Unknown13(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[66]]
    }
    pub fn Unknown14(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[67]]
    }
    pub fn Unknown15(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[68]]
    }
    pub fn Unknown16(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[69]]
    }
    pub fn Unknown17(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[70]]
    }
    pub fn Unknown18(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[71]]
    }
    pub fn AllowPhoenixDown(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[72]]
    }
}
