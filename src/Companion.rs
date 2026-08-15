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
pub struct CompanionSheet {
    sheet: Sheet,
}
impl CompanionSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Companion")?;
        let sheet = resolver.read_excel_sheet(&exh, "Companion", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<CompanionRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<CompanionRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for CompanionSheet {
    type Row = CompanionRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Singular: row
                .columns[0]
                .into_string()
                .cloned()
                .expect("Expected column 0 to be a string!"),
            Plural: row
                .columns[2]
                .into_string()
                .cloned()
                .expect("Expected column 2 to be a string!"),
            Adjective: row
                .columns[1]
                .into_i8()
                .copied()
                .expect("Expected column 1 to be a int8!"),
            PossessivePronoun: row
                .columns[3]
                .into_i8()
                .copied()
                .expect("Expected column 3 to be a int8!"),
            StartsWithVowel: row
                .columns[4]
                .into_i8()
                .copied()
                .expect("Expected column 4 to be a int8!"),
            Unknown0: row
                .columns[5]
                .into_i8()
                .copied()
                .expect("Expected column 5 to be a int8!"),
            Pronoun: row
                .columns[6]
                .into_i8()
                .copied()
                .expect("Expected column 6 to be a int8!"),
            Article: row
                .columns[7]
                .into_i8()
                .copied()
                .expect("Expected column 7 to be a int8!"),
            Model: row
                .columns[8]
                .into_u16()
                .copied()
                .expect("Expected column 8 to be a uint16!"),
            Priority: row
                .columns[19]
                .into_u16()
                .copied()
                .expect("Expected column 19 to be a uint16!"),
            Enemy: row
                .columns[25]
                .into_u16()
                .copied()
                .expect("Expected column 25 to be a uint16!"),
            Icon: row
                .columns[28]
                .into_u16()
                .copied()
                .expect("Expected column 28 to be a uint16!"),
            Order: row
                .columns[29]
                .into_u16()
                .copied()
                .expect("Expected column 29 to be a uint16!"),
            HP: row
                .columns[33]
                .into_u16()
                .copied()
                .expect("Expected column 33 to be a uint16!"),
            SkillAngle: row
                .columns[35]
                .into_u16()
                .copied()
                .expect("Expected column 35 to be a uint16!"),
            Unknown1: row
                .columns[38]
                .into_u16()
                .copied()
                .expect("Expected column 38 to be a uint16!"),
            Scale: row
                .columns[9]
                .into_u8()
                .copied()
                .expect("Expected column 9 to be a uint8!"),
            InactiveIdle0: row
                .columns[10]
                .into_u8()
                .copied()
                .expect("Expected column 10 to be a uint8!"),
            InactiveIdle1: row
                .columns[11]
                .into_u8()
                .copied()
                .expect("Expected column 11 to be a uint8!"),
            InactiveBattle: row
                .columns[12]
                .into_u8()
                .copied()
                .expect("Expected column 12 to be a uint8!"),
            InactiveWandering: row
                .columns[13]
                .into_u8()
                .copied()
                .expect("Expected column 13 to be a uint8!"),
            Behavior: row
                .columns[14]
                .into_u8()
                .copied()
                .expect("Expected column 14 to be a uint8!"),
            Special: row
                .columns[15]
                .into_u8()
                .copied()
                .expect("Expected column 15 to be a uint8!"),
            Unknown10: row
                .columns[16]
                .into_u8()
                .copied()
                .expect("Expected column 16 to be a uint8!"),
            Unknown11: row
                .columns[17]
                .into_u8()
                .copied()
                .expect("Expected column 17 to be a uint8!"),
            WanderingWait: row
                .columns[18]
                .into_u8()
                .copied()
                .expect("Expected column 18 to be a uint8!"),
            Unknown2: row
                .columns[31]
                .into_u8()
                .copied()
                .expect("Expected column 31 to be a uint8!"),
            Cost: row
                .columns[32]
                .into_u8()
                .copied()
                .expect("Expected column 32 to be a uint8!"),
            Unknown3: row
                .columns[34]
                .into_u8()
                .copied()
                .expect("Expected column 34 to be a uint8!"),
            SkillCost: row
                .columns[36]
                .into_u8()
                .copied()
                .expect("Expected column 36 to be a uint8!"),
            Unknown4: row
                .columns[37]
                .into_u8()
                .copied()
                .expect("Expected column 37 to be a uint8!"),
            MinionRace: row
                .columns[39]
                .into_u8()
                .copied()
                .expect("Expected column 39 to be a uint8!"),
            Unknown5: row
                .columns[20]
                .into_bool()
                .copied()
                .expect("Expected column 20 to be a bool!"),
            Unknown6: row
                .columns[21]
                .into_bool()
                .copied()
                .expect("Expected column 21 to be a bool!"),
            Unknown7: row
                .columns[22]
                .into_bool()
                .copied()
                .expect("Expected column 22 to be a bool!"),
            Unknown8: row
                .columns[23]
                .into_bool()
                .copied()
                .expect("Expected column 23 to be a bool!"),
            Unknown9: row
                .columns[24]
                .into_bool()
                .copied()
                .expect("Expected column 24 to be a bool!"),
            Battle: row
                .columns[26]
                .into_bool()
                .copied()
                .expect("Expected column 26 to be a bool!"),
            Roulette: row
                .columns[27]
                .into_bool()
                .copied()
                .expect("Expected column 27 to be a bool!"),
            IdleAnimation: row
                .columns[30]
                .into_bool()
                .copied()
                .expect("Expected column 30 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a CompanionSheet {
    type Item = (u32, Vec<(u16, CompanionRow)>);
    type IntoIter = StructuredSheetIterator<'a, CompanionSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, CompanionSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CompanionRow {
    ///""
    pub Singular: String,
    ///""
    pub Plural: String,
    ///""
    pub Adjective: i8,
    ///""
    pub PossessivePronoun: i8,
    ///""
    pub StartsWithVowel: i8,
    ///""
    pub Unknown0: i8,
    ///""
    pub Pronoun: i8,
    ///""
    pub Article: i8,
    ///""
    pub Model: u16,
    ///""
    pub Priority: u16,
    ///""
    pub Enemy: u16,
    ///""
    pub Icon: u16,
    ///""
    pub Order: u16,
    ///""
    pub HP: u16,
    ///""
    pub SkillAngle: u16,
    ///""
    pub Unknown1: u16,
    ///""
    pub Scale: u8,
    ///""
    pub InactiveIdle0: u8,
    ///""
    pub InactiveIdle1: u8,
    ///""
    pub InactiveBattle: u8,
    ///""
    pub InactiveWandering: u8,
    ///""
    pub Behavior: u8,
    ///""
    pub Special: u8,
    ///""
    pub Unknown10: u8,
    ///""
    pub Unknown11: u8,
    ///""
    pub WanderingWait: u8,
    ///""
    pub Unknown2: u8,
    ///""
    pub Cost: u8,
    ///""
    pub Unknown3: u8,
    ///""
    pub SkillCost: u8,
    ///""
    pub Unknown4: u8,
    ///""
    pub MinionRace: u8,
    ///""
    pub Unknown5: bool,
    ///""
    pub Unknown6: bool,
    ///""
    pub Unknown7: bool,
    ///""
    pub Unknown8: bool,
    ///""
    pub Unknown9: bool,
    ///""
    pub Battle: bool,
    ///""
    pub Roulette: bool,
    ///""
    pub IdleAnimation: bool,
}
