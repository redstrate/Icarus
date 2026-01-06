//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    common::Language,
};
#[derive(Debug, Clone)]
pub struct InstanceContentSheet {
    sheet: Sheet,
}
impl InstanceContentSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("InstanceContent")?;
        let sheet = resolver.read_excel_sheet(&exh, "InstanceContent", language)?;
        Ok(Self { sheet })
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
impl StructuredSheet for InstanceContentSheet {
    type Row = InstanceContentRow;
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
impl<'a> IntoIterator for &'a InstanceContentSheet {
    type Item = (u32, Vec<(u16, InstanceContentRow)>);
    type IntoIter = StructuredSheetIterator<'a, InstanceContentSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, InstanceContentSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct InstanceContentRow {
    columns: Vec<Field>,
}
impl InstanceContentRow {
    pub fn NewPlayerBonusGil<'a>(&'a self) -> &'a Field {
        &self.columns[0]
    }
    pub fn NewPlayerBonusExp<'a>(&'a self) -> &'a Field {
        &self.columns[1]
    }
    pub fn FinalBossExp<'a>(&'a self) -> &'a Field {
        &self.columns[2]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a Field {
        &self.columns[3]
    }
    pub fn BossExp<'a>(&'a self) -> [&'a Field; 5] {
        [
            &self.columns[4],
            &self.columns[5],
            &self.columns[6],
            &self.columns[7],
            &self.columns[8],
        ]
    }
    pub fn InstanceClearExp<'a>(&'a self) -> &'a Field {
        &self.columns[9]
    }
    pub fn InstanceClearGil<'a>(&'a self) -> &'a Field {
        &self.columns[10]
    }
    pub fn InstanceContentRewardItem<'a>(&'a self) -> &'a Field {
        &self.columns[11]
    }
    pub fn NewPlayerBonusA<'a>(&'a self) -> &'a Field {
        &self.columns[12]
    }
    pub fn NewPlayerBonusB<'a>(&'a self) -> &'a Field {
        &self.columns[13]
    }
    pub fn FinalBossCurrencyA<'a>(&'a self) -> &'a Field {
        &self.columns[14]
    }
    pub fn FinalBossCurrencyB<'a>(&'a self) -> &'a Field {
        &self.columns[15]
    }
    pub fn FinalBossCurrencyC<'a>(&'a self) -> &'a Field {
        &self.columns[16]
    }
    pub fn BossCurrencyA<'a>(&'a self) -> [&'a Field; 5] {
        [
            &self.columns[17],
            &self.columns[18],
            &self.columns[19],
            &self.columns[20],
            &self.columns[21],
        ]
    }
    pub fn BossCurrencyB<'a>(&'a self) -> [&'a Field; 5] {
        [
            &self.columns[22],
            &self.columns[23],
            &self.columns[24],
            &self.columns[25],
            &self.columns[26],
        ]
    }
    pub fn BossCurrencyC<'a>(&'a self) -> [&'a Field; 5] {
        [
            &self.columns[27],
            &self.columns[28],
            &self.columns[29],
            &self.columns[30],
            &self.columns[31],
        ]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a Field {
        &self.columns[32]
    }
    pub fn Unknown8<'a>(&'a self) -> &'a Field {
        &self.columns[33]
    }
    pub fn LimitedTimeBonus<'a>(&'a self) -> &'a Field {
        &self.columns[34]
    }
    pub fn Cutscene<'a>(&'a self) -> &'a Field {
        &self.columns[35]
    }
    pub fn LGBEventRange<'a>(&'a self) -> &'a Field {
        &self.columns[36]
    }
    pub fn InstanceContentTextDataBossStart<'a>(&'a self) -> &'a Field {
        &self.columns[37]
    }
    pub fn InstanceContentTextDataBossEnd<'a>(&'a self) -> &'a Field {
        &self.columns[38]
    }
    pub fn BNpcBaseBoss<'a>(&'a self) -> &'a Field {
        &self.columns[39]
    }
    pub fn InstanceContentTextDataObjectiveStart<'a>(&'a self) -> &'a Field {
        &self.columns[40]
    }
    pub fn InstanceContentTextDataObjectiveEnd<'a>(&'a self) -> &'a Field {
        &self.columns[41]
    }
    pub fn Unknown2<'a>(&'a self) -> &'a Field {
        &self.columns[42]
    }
    pub fn ReqInstance<'a>(&'a self) -> &'a Field {
        &self.columns[43]
    }
    pub fn InstanceContentBuff<'a>(&'a self) -> &'a Field {
        &self.columns[44]
    }
    pub fn TimeLimitmin<'a>(&'a self) -> &'a Field {
        &self.columns[45]
    }
    pub fn BGM<'a>(&'a self) -> &'a Field {
        &self.columns[46]
    }
    pub fn WinBGM<'a>(&'a self) -> &'a Field {
        &self.columns[47]
    }
    pub fn ContentFinderCondition<'a>(&'a self) -> &'a Field {
        &self.columns[48]
    }
    pub fn SortKey<'a>(&'a self) -> &'a Field {
        &self.columns[49]
    }
    pub fn ContentRoute<'a>(&'a self) -> &'a Field {
        &self.columns[50]
    }
    pub fn ContentDirectorManagedSG<'a>(&'a self) -> &'a Field {
        &self.columns[51]
    }
    pub fn ContentTodo<'a>(&'a self) -> &'a Field {
        &self.columns[52]
    }
    pub fn Unknown6<'a>(&'a self) -> &'a Field {
        &self.columns[53]
    }
    pub fn Unknown7<'a>(&'a self) -> &'a Field {
        &self.columns[54]
    }
    pub fn ContentEventItem<'a>(&'a self) -> &'a Field {
        &self.columns[55]
    }
    pub fn ContentDirectorBattleTalk<'a>(&'a self) -> &'a Field {
        &self.columns[56]
    }
    pub fn PartyCondition<'a>(&'a self) -> &'a Field {
        &self.columns[57]
    }
    pub fn InstanceContentType<'a>(&'a self) -> &'a Field {
        &self.columns[58]
    }
    pub fn WeekRestriction<'a>(&'a self) -> &'a Field {
        &self.columns[59]
    }
    pub fn Colosseum<'a>(&'a self) -> &'a Field {
        &self.columns[60]
    }
    pub fn Unknown9<'a>(&'a self) -> &'a Field {
        &self.columns[61]
    }
    pub fn QTE1<'a>(&'a self) -> &'a Field {
        &self.columns[62]
    }
    pub fn QTE2<'a>(&'a self) -> &'a Field {
        &self.columns[63]
    }
    pub fn Unknown12<'a>(&'a self) -> &'a Field {
        &self.columns[64]
    }
    pub fn ContentAttributeRect<'a>(&'a self) -> &'a Field {
        &self.columns[65]
    }
    pub fn Unknown13<'a>(&'a self) -> &'a Field {
        &self.columns[66]
    }
    pub fn Unknown14<'a>(&'a self) -> &'a Field {
        &self.columns[67]
    }
    pub fn Unknown15<'a>(&'a self) -> &'a Field {
        &self.columns[68]
    }
    pub fn Unknown16<'a>(&'a self) -> &'a Field {
        &self.columns[69]
    }
    pub fn Unknown17<'a>(&'a self) -> &'a Field {
        &self.columns[70]
    }
    pub fn Unknown18<'a>(&'a self) -> &'a Field {
        &self.columns[71]
    }
    pub fn AllowPhoenixDown<'a>(&'a self) -> &'a Field {
        &self.columns[72]
    }
}
