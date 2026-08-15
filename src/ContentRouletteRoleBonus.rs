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
pub struct ContentRouletteRoleBonusSheet {
    sheet: Sheet,
}
impl ContentRouletteRoleBonusSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("ContentRouletteRoleBonus")?;
        let sheet = resolver
            .read_excel_sheet(&exh, "ContentRouletteRoleBonus", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<ContentRouletteRoleBonusRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<ContentRouletteRoleBonusRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for ContentRouletteRoleBonusSheet {
    type Row = ContentRouletteRoleBonusRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Item: [
                row
                    .columns[6]
                    .into_u32()
                    .copied()
                    .expect("Expected column 6 to be a uint32!"),
                row
                    .columns[9]
                    .into_u32()
                    .copied()
                    .expect("Expected column 9 to be a uint32!"),
            ],
            BaseExperience: row
                .columns[0]
                .into_u16()
                .copied()
                .expect("Expected column 0 to be a uint16!"),
            GilMultiplier: row
                .columns[1]
                .into_u16()
                .copied()
                .expect("Expected column 1 to be a uint16!"),
            GCSealsMultiplier: row
                .columns[2]
                .into_u16()
                .copied()
                .expect("Expected column 2 to be a uint16!"),
            CurrencyA: row
                .columns[3]
                .into_u16()
                .copied()
                .expect("Expected column 3 to be a uint16!"),
            CurrencyB: row
                .columns[4]
                .into_u16()
                .copied()
                .expect("Expected column 4 to be a uint16!"),
            CurrencyC: row
                .columns[5]
                .into_u16()
                .copied()
                .expect("Expected column 5 to be a uint16!"),
            ItemQuantity: [
                row
                    .columns[7]
                    .into_u8()
                    .copied()
                    .expect("Expected column 7 to be a uint8!"),
                row
                    .columns[10]
                    .into_u8()
                    .copied()
                    .expect("Expected column 10 to be a uint8!"),
            ],
            ItemMaxLevelCondition: [
                row
                    .columns[8]
                    .into_u8()
                    .copied()
                    .expect("Expected column 8 to be a uint8!"),
                row
                    .columns[11]
                    .into_u8()
                    .copied()
                    .expect("Expected column 11 to be a uint8!"),
            ],
        })
    }
}
impl<'a> IntoIterator for &'a ContentRouletteRoleBonusSheet {
    type Item = (u32, Vec<(u16, ContentRouletteRoleBonusRow)>);
    type IntoIter = StructuredSheetIterator<'a, ContentRouletteRoleBonusSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, ContentRouletteRoleBonusSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ContentRouletteRoleBonusRow {
    ///""
    pub Item: [u32; 2],
    ///"Formula: BaseExperience * ParamGrow.ScaledQuestXP * ParamGrow.QuestExpModifier / 1000"
    pub BaseExperience: u16,
    ///"Formula: InstanceContent.InstanceClearGil * GilMultiplier / 1000"
    pub GilMultiplier: u16,
    ///"Formula: CurrentLevel * GCSealsMultiplier / 1000"
    pub GCSealsMultiplier: u16,
    ///""
    pub CurrencyA: u16,
    ///""
    pub CurrencyB: u16,
    ///""
    pub CurrencyC: u16,
    ///""
    pub ItemQuantity: [u8; 2],
    ///"0 = Always rewarded\n /// 1 = Character has reached current expansions max level\n /// 2 = Character has reached at least 50\n /// 3 = Character has reached at least 60\n /// 4 = Character has reached at least 70\n /// 5 = Character has reached at least 80\n /// 6 = Character has reached at least 90\n /// "
    pub ItemMaxLevelCondition: [u8; 2],
}
