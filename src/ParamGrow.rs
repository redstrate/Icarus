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
pub struct ParamGrowSheet {
    sheet: Sheet,
}
impl ParamGrowSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("ParamGrow")?;
        let sheet = resolver.read_excel_sheet(&exh, "ParamGrow", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<ParamGrowRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<ParamGrowRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for ParamGrowSheet {
    type Row = ParamGrowRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            ExpToNext: row
                .columns[0]
                .into_i32()
                .copied()
                .expect("Expected column 0 to be a int32!"),
            MpModifier: row
                .columns[4]
                .into_i32()
                .copied()
                .expect("Expected column 4 to be a int32!"),
            BaseSpeed: row
                .columns[5]
                .into_i32()
                .copied()
                .expect("Expected column 5 to be a int32!"),
            LevelModifier: row
                .columns[6]
                .into_i32()
                .copied()
                .expect("Expected column 6 to be a int32!"),
            HuntingLogExpReward: row
                .columns[9]
                .into_i32()
                .copied()
                .expect("Expected column 9 to be a int32!"),
            MonsterNoteSeals: row
                .columns[10]
                .into_i32()
                .copied()
                .expect("Expected column 10 to be a int32!"),
            ScaledQuestXP: row
                .columns[3]
                .into_u16()
                .copied()
                .expect("Expected column 3 to be a uint16!"),
            HpModifier: row
                .columns[8]
                .into_u16()
                .copied()
                .expect("Expected column 8 to be a uint16!"),
            ItemLevelSync: row
                .columns[11]
                .into_u16()
                .copied()
                .expect("Expected column 11 to be a uint16!"),
            ProperDungeon: row
                .columns[12]
                .into_u16()
                .copied()
                .expect("Expected column 12 to be a uint16!"),
            ProperGuildOrder: row
                .columns[13]
                .into_u16()
                .copied()
                .expect("Expected column 13 to be a uint16!"),
            CraftingLevel: row
                .columns[14]
                .into_u16()
                .copied()
                .expect("Expected column 14 to be a uint16!"),
            AdditionalActions: row
                .columns[1]
                .into_u8()
                .copied()
                .expect("Expected column 1 to be a uint8!"),
            ApplyAction: row
                .columns[2]
                .into_u8()
                .copied()
                .expect("Expected column 2 to be a uint8!"),
            QuestExpModifier: row
                .columns[7]
                .into_u8()
                .copied()
                .expect("Expected column 7 to be a uint8!"),
        })
    }
}
impl<'a> IntoIterator for &'a ParamGrowSheet {
    type Item = (u32, Vec<(u16, ParamGrowRow)>);
    type IntoIter = StructuredSheetIterator<'a, ParamGrowSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, ParamGrowSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ParamGrowRow {
    ///""
    pub ExpToNext: i32,
    ///""
    pub MpModifier: i32,
    ///""
    pub BaseSpeed: i32,
    ///""
    pub LevelModifier: i32,
    ///""
    pub HuntingLogExpReward: i32,
    ///""
    pub MonsterNoteSeals: i32,
    ///""
    pub ScaledQuestXP: u16,
    ///""
    pub HpModifier: u16,
    ///""
    pub ItemLevelSync: u16,
    ///""
    pub ProperDungeon: u16,
    ///""
    pub ProperGuildOrder: u16,
    ///""
    pub CraftingLevel: u16,
    ///""
    pub AdditionalActions: u8,
    ///""
    pub ApplyAction: u8,
    ///""
    pub QuestExpModifier: u8,
}
