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
pub struct AOZContentBriefingBNpcSheet {
    sheet: Sheet,
}
impl AOZContentBriefingBNpcSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("AOZContentBriefingBNpc")?;
        let sheet = resolver.read_excel_sheet(&exh, "AOZContentBriefingBNpc", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<AOZContentBriefingBNpcRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<AOZContentBriefingBNpcRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for AOZContentBriefingBNpcSheet {
    type Row = AOZContentBriefingBNpcRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            BNpcName: row
                .columns[0]
                .into_u32()
                .copied()
                .expect("Expected column 0 to be a uint32!"),
            TargetSmall: row
                .columns[1]
                .into_u32()
                .copied()
                .expect("Expected column 1 to be a uint32!"),
            TargetLarge: row
                .columns[2]
                .into_u32()
                .copied()
                .expect("Expected column 2 to be a uint32!"),
            Endurance: row
                .columns[4]
                .into_u8()
                .copied()
                .expect("Expected column 4 to be a uint8!"),
            Fire: row
                .columns[5]
                .into_u8()
                .copied()
                .expect("Expected column 5 to be a uint8!"),
            Ice: row
                .columns[6]
                .into_u8()
                .copied()
                .expect("Expected column 6 to be a uint8!"),
            Wind: row
                .columns[7]
                .into_u8()
                .copied()
                .expect("Expected column 7 to be a uint8!"),
            Earth: row
                .columns[8]
                .into_u8()
                .copied()
                .expect("Expected column 8 to be a uint8!"),
            Thunder: row
                .columns[9]
                .into_u8()
                .copied()
                .expect("Expected column 9 to be a uint8!"),
            Water: row
                .columns[10]
                .into_u8()
                .copied()
                .expect("Expected column 10 to be a uint8!"),
            Slashing: row
                .columns[11]
                .into_u8()
                .copied()
                .expect("Expected column 11 to be a uint8!"),
            Piercing: row
                .columns[12]
                .into_u8()
                .copied()
                .expect("Expected column 12 to be a uint8!"),
            Blunt: row
                .columns[13]
                .into_u8()
                .copied()
                .expect("Expected column 13 to be a uint8!"),
            Magic: row
                .columns[14]
                .into_u8()
                .copied()
                .expect("Expected column 14 to be a uint8!"),
            HideStats: row
                .columns[3]
                .into_bool()
                .copied()
                .expect("Expected column 3 to be a bool!"),
            SlowVuln: row
                .columns[15]
                .into_bool()
                .copied()
                .expect("Expected column 15 to be a bool!"),
            PetrificationVuln: row
                .columns[16]
                .into_bool()
                .copied()
                .expect("Expected column 16 to be a bool!"),
            ParalysisVuln: row
                .columns[17]
                .into_bool()
                .copied()
                .expect("Expected column 17 to be a bool!"),
            InterruptionVuln: row
                .columns[18]
                .into_bool()
                .copied()
                .expect("Expected column 18 to be a bool!"),
            BlindVuln: row
                .columns[19]
                .into_bool()
                .copied()
                .expect("Expected column 19 to be a bool!"),
            StunVuln: row
                .columns[20]
                .into_bool()
                .copied()
                .expect("Expected column 20 to be a bool!"),
            SleepVuln: row
                .columns[21]
                .into_bool()
                .copied()
                .expect("Expected column 21 to be a bool!"),
            BindVuln: row
                .columns[22]
                .into_bool()
                .copied()
                .expect("Expected column 22 to be a bool!"),
            HeavyVuln: row
                .columns[23]
                .into_bool()
                .copied()
                .expect("Expected column 23 to be a bool!"),
            FlatOrDeathVuln: row
                .columns[24]
                .into_bool()
                .copied()
                .expect("Expected column 24 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a AOZContentBriefingBNpcSheet {
    type Item = (u32, Vec<(u16, AOZContentBriefingBNpcRow)>);
    type IntoIter = StructuredSheetIterator<'a, AOZContentBriefingBNpcSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, AOZContentBriefingBNpcSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AOZContentBriefingBNpcRow {
    ///""
    pub BNpcName: u32,
    ///""
    pub TargetSmall: u32,
    ///""
    pub TargetLarge: u32,
    ///""
    pub Endurance: u8,
    ///""
    pub Fire: u8,
    ///""
    pub Ice: u8,
    ///""
    pub Wind: u8,
    ///""
    pub Earth: u8,
    ///""
    pub Thunder: u8,
    ///""
    pub Water: u8,
    ///""
    pub Slashing: u8,
    ///""
    pub Piercing: u8,
    ///""
    pub Blunt: u8,
    ///""
    pub Magic: u8,
    ///""
    pub HideStats: bool,
    ///""
    pub SlowVuln: bool,
    ///""
    pub PetrificationVuln: bool,
    ///""
    pub ParalysisVuln: bool,
    ///""
    pub InterruptionVuln: bool,
    ///""
    pub BlindVuln: bool,
    ///""
    pub StunVuln: bool,
    ///""
    pub SleepVuln: bool,
    ///""
    pub BindVuln: bool,
    ///""
    pub HeavyVuln: bool,
    ///""
    pub FlatOrDeathVuln: bool,
}
