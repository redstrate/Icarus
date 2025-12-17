//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    resource::{Resource, read_excel_sheet_header, read_excel_sheet},
    exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow},
    exh::{EXH, ExcelColumnDefinition},
    common::Language,
};
pub struct ClassJobSheet {
    pages: Vec<EXD>,
    exh: EXH,
    row_count: u32,
}
impl ClassJobSheet {
    /// Read the sheet from a `Resource`.
    pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
        let exh = read_excel_sheet_header(resource, "ClassJob")?;
        let mut pages = Vec::new();
        for (i, _) in exh.pages.iter().enumerate() {
            pages.push(read_excel_sheet(resource, "ClassJob", &exh, language, i)?);
        }
        let row_count = exh.header.row_count;
        Some(Self { exh, pages, row_count })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<ClassJobRow> {
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
        Some(ClassJobRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<ClassJobRow> {
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
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<ClassJobRow> {
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
pub struct ClassJobRow {
    columns: Vec<ColumnData>,
}
impl ClassJobRow {
    pub fn Name<'a>(&'a self) -> &'a ColumnData {
        &self.columns[0]
    }
    pub fn Abbreviation<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1]
    }
    pub fn NameFemale<'a>(&'a self) -> &'a ColumnData {
        &self.columns[2]
    }
    pub fn CanQueueForDuty<'a>(&'a self) -> &'a ColumnData {
        &self.columns[3]
    }
    pub fn NameEnglish<'a>(&'a self) -> &'a ColumnData {
        &self.columns[4]
    }
    pub fn ItemSoulCrystal<'a>(&'a self) -> &'a ColumnData {
        &self.columns[5]
    }
    pub fn UnlockQuest<'a>(&'a self) -> &'a ColumnData {
        &self.columns[6]
    }
    pub fn RelicQuest<'a>(&'a self) -> &'a ColumnData {
        &self.columns[7]
    }
    pub fn Prerequisite<'a>(&'a self) -> &'a ColumnData {
        &self.columns[8]
    }
    pub fn Unknown_70_1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[9]
    }
    pub fn Unknown_70_2<'a>(&'a self) -> &'a ColumnData {
        &self.columns[10]
    }
    pub fn Unknown9<'a>(&'a self) -> &'a ColumnData {
        &self.columns[11]
    }
    pub fn ItemStartingWeaponMainHand<'a>(&'a self) -> &'a ColumnData {
        &self.columns[12]
    }
    pub fn ItemStartingWeaponOffHand<'a>(&'a self) -> &'a ColumnData {
        &self.columns[13]
    }
    pub fn ModifierHitPoints<'a>(&'a self) -> &'a ColumnData {
        &self.columns[14]
    }
    pub fn ModifierManaPoints<'a>(&'a self) -> &'a ColumnData {
        &self.columns[15]
    }
    pub fn ModifierStrength<'a>(&'a self) -> &'a ColumnData {
        &self.columns[16]
    }
    pub fn ModifierVitality<'a>(&'a self) -> &'a ColumnData {
        &self.columns[17]
    }
    pub fn ModifierDexterity<'a>(&'a self) -> &'a ColumnData {
        &self.columns[18]
    }
    pub fn ModifierIntelligence<'a>(&'a self) -> &'a ColumnData {
        &self.columns[19]
    }
    pub fn ModifierMind<'a>(&'a self) -> &'a ColumnData {
        &self.columns[20]
    }
    pub fn ModifierPiety<'a>(&'a self) -> &'a ColumnData {
        &self.columns[21]
    }
    pub fn Unknown2<'a>(&'a self) -> &'a ColumnData {
        &self.columns[22]
    }
    pub fn Unknown3<'a>(&'a self) -> &'a ColumnData {
        &self.columns[23]
    }
    pub fn Unknown4<'a>(&'a self) -> &'a ColumnData {
        &self.columns[24]
    }
    pub fn Unknown5<'a>(&'a self) -> &'a ColumnData {
        &self.columns[25]
    }
    pub fn Unknown6<'a>(&'a self) -> &'a ColumnData {
        &self.columns[26]
    }
    pub fn Unknown7<'a>(&'a self) -> &'a ColumnData {
        &self.columns[27]
    }
    pub fn LimitBreak1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[28]
    }
    pub fn LimitBreak2<'a>(&'a self) -> &'a ColumnData {
        &self.columns[29]
    }
    pub fn LimitBreak3<'a>(&'a self) -> &'a ColumnData {
        &self.columns[30]
    }
    pub fn ClassJobCategory<'a>(&'a self) -> &'a ColumnData {
        &self.columns[31]
    }
    pub fn Unknown8<'a>(&'a self) -> &'a ColumnData {
        &self.columns[32]
    }
    pub fn JobIndex<'a>(&'a self) -> &'a ColumnData {
        &self.columns[33]
    }
    pub fn PvPBaseParamValue<'a>(&'a self) -> &'a ColumnData {
        &self.columns[34]
    }
    pub fn PvPActionSortRow<'a>(&'a self) -> &'a ColumnData {
        &self.columns[35]
    }
    pub fn PvPInitialSelectActionTrait<'a>(&'a self) -> &'a ColumnData {
        &self.columns[36]
    }
    pub fn ClassJobParent<'a>(&'a self) -> &'a ColumnData {
        &self.columns[37]
    }
    pub fn Role<'a>(&'a self) -> &'a ColumnData {
        &self.columns[38]
    }
    pub fn StartingTown<'a>(&'a self) -> &'a ColumnData {
        &self.columns[39]
    }
    pub fn PrimaryStat<'a>(&'a self) -> &'a ColumnData {
        &self.columns[40]
    }
    pub fn UIPriority<'a>(&'a self) -> &'a ColumnData {
        &self.columns[41]
    }
    pub fn StartingLevel<'a>(&'a self) -> &'a ColumnData {
        &self.columns[42]
    }
    pub fn PartyBonus<'a>(&'a self) -> &'a ColumnData {
        &self.columns[43]
    }
    /// 1 = Tank
    /// 2 = Pure Healer
    /// 3 = Melee
    /// 4 = Physical Ranged
    /// 5 = Magical Ranged
    /// 6 = Barrier Healer
    ///
    pub fn JobType<'a>(&'a self) -> &'a ColumnData {
        &self.columns[44]
    }
    pub fn ExpArrayIndex<'a>(&'a self) -> &'a ColumnData {
        &self.columns[45]
    }
    pub fn BattleClassIndex<'a>(&'a self) -> &'a ColumnData {
        &self.columns[46]
    }
    pub fn DohDolJobIndex<'a>(&'a self) -> &'a ColumnData {
        &self.columns[47]
    }
    pub fn MonsterNote<'a>(&'a self) -> &'a ColumnData {
        &self.columns[48]
    }
    pub fn IsLimitedJob<'a>(&'a self) -> &'a ColumnData {
        &self.columns[49]
    }
    pub fn Unknown10<'a>(&'a self) -> &'a ColumnData {
        &self.columns[50]
    }
}
