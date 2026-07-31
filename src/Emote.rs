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
pub struct EmoteSheet {
    sheet: Sheet,
}
impl EmoteSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Emote")?;
        let sheet = resolver.read_excel_sheet(&exh, "Emote", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<EmoteRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<EmoteRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for EmoteSheet {
    type Row = EmoteRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Name: row
                .columns[0]
                .into_string()
                .cloned()
                .expect("Expected column 0 to be a string!"),
            Icon: row
                .columns[20]
                .into_u32()
                .copied()
                .expect("Expected column 20 to be a uint32!"),
            UnlockLink: row
                .columns[23]
                .into_u32()
                .copied()
                .expect("Expected column 23 to be a uint32!"),
            TextCommand: row
                .columns[19]
                .into_i32()
                .copied()
                .expect("Expected column 19 to be a int32!"),
            ActionTimeline: [
                row
                    .columns[1]
                    .into_u16()
                    .copied()
                    .expect("Expected column 1 to be a uint16!"),
                row
                    .columns[2]
                    .into_u16()
                    .copied()
                    .expect("Expected column 2 to be a uint16!"),
                row
                    .columns[3]
                    .into_u16()
                    .copied()
                    .expect("Expected column 3 to be a uint16!"),
                row
                    .columns[4]
                    .into_u16()
                    .copied()
                    .expect("Expected column 4 to be a uint16!"),
                row
                    .columns[5]
                    .into_u16()
                    .copied()
                    .expect("Expected column 5 to be a uint16!"),
                row
                    .columns[6]
                    .into_u16()
                    .copied()
                    .expect("Expected column 6 to be a uint16!"),
                row
                    .columns[7]
                    .into_u16()
                    .copied()
                    .expect("Expected column 7 to be a uint16!"),
            ],
            Order: row
                .columns[18]
                .into_u16()
                .copied()
                .expect("Expected column 18 to be a uint16!"),
            LogMessageTargeted: row
                .columns[21]
                .into_u16()
                .copied()
                .expect("Expected column 21 to be a uint16!"),
            LogMessageUntargeted: row
                .columns[22]
                .into_u16()
                .copied()
                .expect("Expected column 22 to be a uint16!"),
            Patch: row
                .columns[24]
                .into_u16()
                .copied()
                .expect("Expected column 24 to be a uint16!"),
            EmoteCategory: row
                .columns[11]
                .into_u8()
                .copied()
                .expect("Expected column 11 to be a uint8!"),
            EmoteMode: row
                .columns[12]
                .into_u8()
                .copied()
                .expect("Expected column 12 to be a uint8!"),
            HasMountedAnimation: row
                .columns[8]
                .into_bool()
                .copied()
                .expect("Expected column 8 to be a bool!"),
            HasSwimmingAnimation: row
                .columns[9]
                .into_bool()
                .copied()
                .expect("Expected column 9 to be a bool!"),
            HasDivingAnimation: row
                .columns[10]
                .into_bool()
                .copied()
                .expect("Expected column 10 to be a bool!"),
            DontSetEmoteMode: row
                .columns[13]
                .into_bool()
                .copied()
                .expect("Expected column 13 to be a bool!"),
            Unknown5: row
                .columns[14]
                .into_bool()
                .copied()
                .expect("Expected column 14 to be a bool!"),
            UsableWhenFishing: row
                .columns[15]
                .into_bool()
                .copied()
                .expect("Expected column 15 to be a bool!"),
            DrawsWeapon: row
                .columns[16]
                .into_bool()
                .copied()
                .expect("Expected column 16 to be a bool!"),
            SheathesWeapon: row
                .columns[17]
                .into_bool()
                .copied()
                .expect("Expected column 17 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a EmoteSheet {
    type Item = (u32, Vec<(u16, EmoteRow)>);
    type IntoIter = StructuredSheetIterator<'a, EmoteSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, EmoteSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EmoteRow {
    ///""
    pub Name: String,
    ///""
    pub Icon: u32,
    ///""
    pub UnlockLink: u32,
    ///""
    pub TextCommand: i32,
    ///"0 = Standing\n /// 1 = Start animation\n /// 2 = Ground sit\n /// 3 = Chair sit\n /// 4 = Mounted (when HasMountedAnimation) / Swimming (when HasSwimmingAnimation) / Diving (when HasDivingAnimation)\n /// 5 = Dozing\n /// 6 = Unused\n /// "
    pub ActionTimeline: [u16; 7],
    ///""
    pub Order: u16,
    ///""
    pub LogMessageTargeted: u16,
    ///""
    pub LogMessageUntargeted: u16,
    ///""
    pub Patch: u16,
    ///""
    pub EmoteCategory: u8,
    ///""
    pub EmoteMode: u8,
    ///""
    pub HasMountedAnimation: bool,
    ///""
    pub HasSwimmingAnimation: bool,
    ///""
    pub HasDivingAnimation: bool,
    ///""
    pub DontSetEmoteMode: bool,
    ///"Maybe controls whether or not the emote can be synced in GPose?!"
    pub Unknown5: bool,
    ///""
    pub UsableWhenFishing: bool,
    ///""
    pub DrawsWeapon: bool,
    ///""
    pub SheathesWeapon: bool,
}
