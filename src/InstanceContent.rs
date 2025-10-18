//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    resource::{Resource, read_excel_sheet_header, read_excel_sheet},
    exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow},
    exh::{EXH, ExcelColumnDefinition},
    common::Language,
};
pub struct InstanceContentSheet {
    pages: Vec<EXD>,
    exh: EXH,
    row_count: u32,
}
impl InstanceContentSheet {
    /// Read the sheet from a `Resource`.
    pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
        let exh = read_excel_sheet_header(resource, "InstanceContent")?;
        let mut pages = Vec::new();
        for (i, _) in exh.pages.iter().enumerate() {
            pages
                .push(read_excel_sheet(resource, "InstanceContent", &exh, language, i)?);
        }
        let row_count = exh.header.row_count;
        Some(Self { exh, pages, row_count })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<InstanceContentRow> {
        let column_defs = &self.exh.column_definitions;
        let mut zipped: Vec<_> = row
            .columns
            .clone()
            .into_iter()
            .zip(column_defs)
            .collect();
        zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
        let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition>) = zipped
            .into_iter()
            .unzip();
        Some(InstanceContentRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<InstanceContentRow> {
        for page in &self.pages {
            let Some(row) = &page.get_row(row_id) else {
                continue;
            };
            let row = match row {
                ExcelRowKind::SingleRow(row) => row,
                ExcelRowKind::SubRows(rows) => &rows.first()?.1,
            };
            return self.read_row(row);
        }
        None
    }
    /// Fetches the specified subrow from the sheet.
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<InstanceContentRow> {
        for page in &self.pages {
            let Some(row) = &page.get_row(row_id) else {
                continue;
            };
            let row = match row {
                ExcelRowKind::SingleRow(row) => return None,
                ExcelRowKind::SubRows(subrows) => {
                    &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1
                }
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
pub struct InstanceContentRow {
    columns: Vec<ColumnData>,
}
impl InstanceContentRow {
    pub fn NewPlayerBonusGil<'a>(&'a self) -> &'a ColumnData {
        &self.columns[0]
    }
    pub fn NewPlayerBonusExp<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1]
    }
    pub fn FinalBossExp<'a>(&'a self) -> &'a ColumnData {
        &self.columns[2]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a ColumnData {
        &self.columns[3]
    }
    pub fn BossExp<'a>(&'a self) -> [&'a ColumnData; 5] {
        [
            &self.columns[4],
            &self.columns[5],
            &self.columns[6],
            &self.columns[7],
            &self.columns[8],
        ]
    }
    pub fn InstanceClearExp<'a>(&'a self) -> &'a ColumnData {
        &self.columns[9]
    }
    pub fn InstanceClearGil<'a>(&'a self) -> &'a ColumnData {
        &self.columns[10]
    }
    pub fn InstanceContentRewardItem<'a>(&'a self) -> &'a ColumnData {
        &self.columns[11]
    }
    pub fn NewPlayerBonusA<'a>(&'a self) -> &'a ColumnData {
        &self.columns[12]
    }
    pub fn NewPlayerBonusB<'a>(&'a self) -> &'a ColumnData {
        &self.columns[13]
    }
    pub fn FinalBossCurrencyA<'a>(&'a self) -> &'a ColumnData {
        &self.columns[14]
    }
    pub fn FinalBossCurrencyB<'a>(&'a self) -> &'a ColumnData {
        &self.columns[15]
    }
    pub fn FinalBossCurrencyC<'a>(&'a self) -> &'a ColumnData {
        &self.columns[16]
    }
    pub fn BossCurrencyA<'a>(&'a self) -> [&'a ColumnData; 5] {
        [
            &self.columns[17],
            &self.columns[18],
            &self.columns[19],
            &self.columns[20],
            &self.columns[21],
        ]
    }
    pub fn BossCurrencyB<'a>(&'a self) -> [&'a ColumnData; 5] {
        [
            &self.columns[22],
            &self.columns[23],
            &self.columns[24],
            &self.columns[25],
            &self.columns[26],
        ]
    }
    pub fn BossCurrencyC<'a>(&'a self) -> [&'a ColumnData; 5] {
        [
            &self.columns[27],
            &self.columns[28],
            &self.columns[29],
            &self.columns[30],
            &self.columns[31],
        ]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[32]
    }
    pub fn LimitedTimeBonus<'a>(&'a self) -> &'a ColumnData {
        &self.columns[33]
    }
    pub fn Cutscene<'a>(&'a self) -> &'a ColumnData {
        &self.columns[34]
    }
    pub fn LGBEventRange<'a>(&'a self) -> &'a ColumnData {
        &self.columns[35]
    }
    pub fn InstanceContentTextDataBossStart<'a>(&'a self) -> &'a ColumnData {
        &self.columns[36]
    }
    pub fn InstanceContentTextDataBossEnd<'a>(&'a self) -> &'a ColumnData {
        &self.columns[37]
    }
    pub fn BNpcBaseBoss<'a>(&'a self) -> &'a ColumnData {
        &self.columns[38]
    }
    pub fn InstanceContentTextDataObjectiveStart<'a>(&'a self) -> &'a ColumnData {
        &self.columns[39]
    }
    pub fn InstanceContentTextDataObjectiveEnd<'a>(&'a self) -> &'a ColumnData {
        &self.columns[40]
    }
    pub fn Unknown2<'a>(&'a self) -> &'a ColumnData {
        &self.columns[41]
    }
    pub fn ReqInstance<'a>(&'a self) -> &'a ColumnData {
        &self.columns[42]
    }
    pub fn InstanceContentBuff<'a>(&'a self) -> &'a ColumnData {
        &self.columns[43]
    }
    pub fn TimeLimitmin<'a>(&'a self) -> &'a ColumnData {
        &self.columns[44]
    }
    pub fn BGM<'a>(&'a self) -> &'a ColumnData {
        &self.columns[45]
    }
    pub fn WinBGM<'a>(&'a self) -> &'a ColumnData {
        &self.columns[46]
    }
    pub fn ContentFinderCondition<'a>(&'a self) -> &'a ColumnData {
        &self.columns[47]
    }
    pub fn SortKey<'a>(&'a self) -> &'a ColumnData {
        &self.columns[48]
    }
    pub fn ContentRoute<'a>(&'a self) -> &'a ColumnData {
        &self.columns[49]
    }
    pub fn ContentDirectorManagedSG<'a>(&'a self) -> &'a ColumnData {
        &self.columns[50]
    }
    pub fn ContentTodo<'a>(&'a self) -> &'a ColumnData {
        &self.columns[51]
    }
    pub fn Unknown6<'a>(&'a self) -> &'a ColumnData {
        &self.columns[52]
    }
    pub fn Unknown7<'a>(&'a self) -> &'a ColumnData {
        &self.columns[53]
    }
    pub fn ContentEventItem<'a>(&'a self) -> &'a ColumnData {
        &self.columns[54]
    }
    pub fn ContentDirectorBattleTalk<'a>(&'a self) -> &'a ColumnData {
        &self.columns[55]
    }
    pub fn PartyCondition<'a>(&'a self) -> &'a ColumnData {
        &self.columns[56]
    }
    pub fn InstanceContentType<'a>(&'a self) -> &'a ColumnData {
        &self.columns[57]
    }
    pub fn WeekRestriction<'a>(&'a self) -> &'a ColumnData {
        &self.columns[58]
    }
    pub fn Colosseum<'a>(&'a self) -> &'a ColumnData {
        &self.columns[59]
    }
    pub fn Unknown9<'a>(&'a self) -> &'a ColumnData {
        &self.columns[60]
    }
    pub fn QTE1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[61]
    }
    pub fn QTE2<'a>(&'a self) -> &'a ColumnData {
        &self.columns[62]
    }
    pub fn Unknown12<'a>(&'a self) -> &'a ColumnData {
        &self.columns[63]
    }
    pub fn ContentAttributeRect<'a>(&'a self) -> &'a ColumnData {
        &self.columns[64]
    }
    pub fn Unknown13<'a>(&'a self) -> &'a ColumnData {
        &self.columns[65]
    }
    pub fn Unknown14<'a>(&'a self) -> &'a ColumnData {
        &self.columns[66]
    }
    pub fn Unknown15<'a>(&'a self) -> &'a ColumnData {
        &self.columns[67]
    }
    pub fn Unknown16<'a>(&'a self) -> &'a ColumnData {
        &self.columns[68]
    }
    pub fn Unknown17<'a>(&'a self) -> &'a ColumnData {
        &self.columns[69]
    }
    pub fn Unknown18<'a>(&'a self) -> &'a ColumnData {
        &self.columns[70]
    }
    pub fn Unknown19<'a>(&'a self) -> &'a ColumnData {
        &self.columns[71]
    }
}
