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
pub struct ClassJobSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl ClassJobSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("ClassJob")?;
        let sheet = resolver.read_excel_sheet(&exh, "ClassJob", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<ClassJobRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<ClassJobRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for ClassJobSheet {
    type Row = ClassJobRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a ClassJobSheet {
    type Item = (u32, Vec<(u16, ClassJobRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, ClassJobSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, ClassJobSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ClassJobRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> ClassJobRow<'a> {
    pub fn Name(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[0]]
    }
    pub fn Abbreviation(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1]]
    }
    pub fn NameFemale(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[2]]
    }
    pub fn CanQueueForDuty(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[3]]
    }
    pub fn NameEnglish(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[4]]
    }
    pub fn ItemSoulCrystal(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[5]]
    }
    pub fn UnlockQuest(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[6]]
    }
    pub fn RelicQuest(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[7]]
    }
    pub fn Prerequisite(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[8]]
    }
    pub fn Unknown_70_1(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[9]]
    }
    pub fn Unknown_70_2(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[10]]
    }
    pub fn Unknown9(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[11]]
    }
    pub fn ItemStartingWeaponMainHand(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[12]]
    }
    pub fn ItemStartingWeaponOffHand(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[13]]
    }
    pub fn ModifierHitPoints(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[14]]
    }
    pub fn ModifierManaPoints(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[15]]
    }
    pub fn ModifierStrength(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[16]]
    }
    pub fn ModifierVitality(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[17]]
    }
    pub fn ModifierDexterity(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[18]]
    }
    pub fn ModifierIntelligence(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[19]]
    }
    pub fn ModifierMind(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[20]]
    }
    pub fn ModifierPiety(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[21]]
    }
    pub fn Unknown2(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[22]]
    }
    pub fn Unknown3(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[23]]
    }
    pub fn Unknown4(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[24]]
    }
    pub fn Unknown5(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[25]]
    }
    pub fn Unknown6(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[26]]
    }
    pub fn Unknown7(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[27]]
    }
    pub fn LimitBreak1(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[28]]
    }
    pub fn LimitBreak2(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[29]]
    }
    pub fn LimitBreak3(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[30]]
    }
    pub fn ClassJobCategory(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[31]]
    }
    pub fn Unknown8(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[32]]
    }
    pub fn JobIndex(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[33]]
    }
    pub fn PvPBaseParamValue(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[34]]
    }
    pub fn PvPActionSortRow(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[35]]
    }
    pub fn PvPInitialSelectActionTrait(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[36]]
    }
    pub fn ClassJobParent(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[37]]
    }
    pub fn Role(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[38]]
    }
    pub fn StartingTown(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[39]]
    }
    pub fn PrimaryStat(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[40]]
    }
    pub fn UIPriority(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[41]]
    }
    pub fn StartingLevel(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[42]]
    }
    pub fn PartyBonus(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[43]]
    }
    /// 1 = Tank
    /// 2 = Pure Healer
    /// 3 = Melee
    /// 4 = Physical Ranged
    /// 5 = Magical Ranged
    /// 6 = Barrier Healer
    ///
    pub fn JobType(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[44]]
    }
    pub fn ExpArrayIndex(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[45]]
    }
    pub fn BattleClassIndex(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[46]]
    }
    pub fn DohDolJobIndex(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[47]]
    }
    pub fn MonsterNote(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[48]]
    }
    pub fn IsLimitedJob(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[49]]
    }
    pub fn Unknown10(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[50]]
    }
}
