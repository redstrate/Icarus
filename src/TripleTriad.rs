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
pub struct TripleTriadSheet {
    sheet: Sheet,
}
impl TripleTriadSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 2293760u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("TripleTriad")?;
        let sheet = resolver.read_excel_sheet(&exh, "TripleTriad", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<TripleTriadRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<TripleTriadRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for TripleTriadSheet {
    type Row = TripleTriadRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            ItemPossibleReward: [
                row
                    .columns[26]
                    .into_u32()
                    .copied()
                    .expect("Expected column 26 to be a uint32!"),
                row
                    .columns[27]
                    .into_u32()
                    .copied()
                    .expect("Expected column 27 to be a uint32!"),
                row
                    .columns[28]
                    .into_u32()
                    .copied()
                    .expect("Expected column 28 to be a uint32!"),
                row
                    .columns[29]
                    .into_u32()
                    .copied()
                    .expect("Expected column 29 to be a uint32!"),
            ],
            PreviousQuest: [
                row
                    .columns[15]
                    .into_u32()
                    .copied()
                    .expect("Expected column 15 to be a uint32!"),
                row
                    .columns[16]
                    .into_u32()
                    .copied()
                    .expect("Expected column 16 to be a uint32!"),
                row
                    .columns[17]
                    .into_u32()
                    .copied()
                    .expect("Expected column 17 to be a uint32!"),
            ],
            DefaultTalkChallenge: row
                .columns[20]
                .into_u32()
                .copied()
                .expect("Expected column 20 to be a uint32!"),
            DefaultTalkUnavailable: row
                .columns[21]
                .into_u32()
                .copied()
                .expect("Expected column 21 to be a uint32!"),
            DefaultTalkNPCWin: row
                .columns[22]
                .into_u32()
                .copied()
                .expect("Expected column 22 to be a uint32!"),
            DefaultTalkDraw: row
                .columns[23]
                .into_u32()
                .copied()
                .expect("Expected column 23 to be a uint32!"),
            DefaultTalkPCWin: row
                .columns[24]
                .into_u32()
                .copied()
                .expect("Expected column 24 to be a uint32!"),
            TripleTriadCardFixed: [
                row
                    .columns[0]
                    .into_u16()
                    .copied()
                    .expect("Expected column 0 to be a uint16!"),
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
            ],
            TripleTriadCardVariable: [
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
                row
                    .columns[8]
                    .into_u16()
                    .copied()
                    .expect("Expected column 8 to be a uint16!"),
                row
                    .columns[9]
                    .into_u16()
                    .copied()
                    .expect("Expected column 9 to be a uint16!"),
            ],
            Fee: row
                .columns[13]
                .into_u16()
                .copied()
                .expect("Expected column 13 to be a uint16!"),
            StartTime: row
                .columns[18]
                .into_u16()
                .copied()
                .expect("Expected column 18 to be a uint16!"),
            EndTime: row
                .columns[19]
                .into_u16()
                .copied()
                .expect("Expected column 19 to be a uint16!"),
            TripleTriadRule: [
                row
                    .columns[10]
                    .into_u8()
                    .copied()
                    .expect("Expected column 10 to be a uint8!"),
                row
                    .columns[11]
                    .into_u8()
                    .copied()
                    .expect("Expected column 11 to be a uint8!"),
            ],
            PreviousQuestJoin: row
                .columns[14]
                .into_u8()
                .copied()
                .expect("Expected column 14 to be a uint8!"),
            UsesRegionalRules: row
                .columns[12]
                .into_bool()
                .copied()
                .expect("Expected column 12 to be a bool!"),
            Unknown0: row
                .columns[25]
                .into_bool()
                .copied()
                .expect("Expected column 25 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a TripleTriadSheet {
    type Item = (u32, Vec<(u16, TripleTriadRow)>);
    type IntoIter = StructuredSheetIterator<'a, TripleTriadSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, TripleTriadSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TripleTriadRow {
    ///""
    pub ItemPossibleReward: [u32; 4],
    ///""
    pub PreviousQuest: [u32; 3],
    ///""
    pub DefaultTalkChallenge: u32,
    ///""
    pub DefaultTalkUnavailable: u32,
    ///""
    pub DefaultTalkNPCWin: u32,
    ///""
    pub DefaultTalkDraw: u32,
    ///""
    pub DefaultTalkPCWin: u32,
    ///""
    pub TripleTriadCardFixed: [u16; 5],
    ///""
    pub TripleTriadCardVariable: [u16; 5],
    ///""
    pub Fee: u16,
    ///""
    pub StartTime: u16,
    ///""
    pub EndTime: u16,
    ///""
    pub TripleTriadRule: [u8; 2],
    ///""
    pub PreviousQuestJoin: u8,
    ///""
    pub UsesRegionalRules: bool,
    ///""
    pub Unknown0: bool,
}
