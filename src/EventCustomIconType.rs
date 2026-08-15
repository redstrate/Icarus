//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
#[derive(Clone, Debug, PartialEq)]
pub struct IconsElement {
    pub AnnounceQuest: u32,
    pub AnnounceQuestLocked: u32,
    pub MapAnnounceQuest1: u32,
    pub MapAnnounceQuestLocked: u32,
    pub MapAnnounceQuest2: u32,
}
#[derive(Debug, Clone)]
pub struct EventCustomIconTypeSheet {
    sheet: Sheet,
}
impl EventCustomIconTypeSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 32u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("EventCustomIconType")?;
        let sheet = resolver.read_excel_sheet(&exh, "EventCustomIconType", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<EventCustomIconTypeRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<EventCustomIconTypeRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for EventCustomIconTypeSheet {
    type Row = EventCustomIconTypeRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Icons: [
                IconsElement {
                    AnnounceQuest: row
                        .columns[0]
                        .into_u32()
                        .copied()
                        .expect("Expected column 0 to be a uint32!"),
                    AnnounceQuestLocked: row
                        .columns[10]
                        .into_u32()
                        .copied()
                        .expect("Expected column 10 to be a uint32!"),
                    MapAnnounceQuest1: row
                        .columns[20]
                        .into_u32()
                        .copied()
                        .expect("Expected column 20 to be a uint32!"),
                    MapAnnounceQuestLocked: row
                        .columns[30]
                        .into_u32()
                        .copied()
                        .expect("Expected column 30 to be a uint32!"),
                    MapAnnounceQuest2: row
                        .columns[40]
                        .into_u32()
                        .copied()
                        .expect("Expected column 40 to be a uint32!"),
                },
                IconsElement {
                    AnnounceQuest: row
                        .columns[1]
                        .into_u32()
                        .copied()
                        .expect("Expected column 1 to be a uint32!"),
                    AnnounceQuestLocked: row
                        .columns[11]
                        .into_u32()
                        .copied()
                        .expect("Expected column 11 to be a uint32!"),
                    MapAnnounceQuest1: row
                        .columns[21]
                        .into_u32()
                        .copied()
                        .expect("Expected column 21 to be a uint32!"),
                    MapAnnounceQuestLocked: row
                        .columns[31]
                        .into_u32()
                        .copied()
                        .expect("Expected column 31 to be a uint32!"),
                    MapAnnounceQuest2: row
                        .columns[41]
                        .into_u32()
                        .copied()
                        .expect("Expected column 41 to be a uint32!"),
                },
                IconsElement {
                    AnnounceQuest: row
                        .columns[2]
                        .into_u32()
                        .copied()
                        .expect("Expected column 2 to be a uint32!"),
                    AnnounceQuestLocked: row
                        .columns[12]
                        .into_u32()
                        .copied()
                        .expect("Expected column 12 to be a uint32!"),
                    MapAnnounceQuest1: row
                        .columns[22]
                        .into_u32()
                        .copied()
                        .expect("Expected column 22 to be a uint32!"),
                    MapAnnounceQuestLocked: row
                        .columns[32]
                        .into_u32()
                        .copied()
                        .expect("Expected column 32 to be a uint32!"),
                    MapAnnounceQuest2: row
                        .columns[42]
                        .into_u32()
                        .copied()
                        .expect("Expected column 42 to be a uint32!"),
                },
                IconsElement {
                    AnnounceQuest: row
                        .columns[3]
                        .into_u32()
                        .copied()
                        .expect("Expected column 3 to be a uint32!"),
                    AnnounceQuestLocked: row
                        .columns[13]
                        .into_u32()
                        .copied()
                        .expect("Expected column 13 to be a uint32!"),
                    MapAnnounceQuest1: row
                        .columns[23]
                        .into_u32()
                        .copied()
                        .expect("Expected column 23 to be a uint32!"),
                    MapAnnounceQuestLocked: row
                        .columns[33]
                        .into_u32()
                        .copied()
                        .expect("Expected column 33 to be a uint32!"),
                    MapAnnounceQuest2: row
                        .columns[43]
                        .into_u32()
                        .copied()
                        .expect("Expected column 43 to be a uint32!"),
                },
                IconsElement {
                    AnnounceQuest: row
                        .columns[4]
                        .into_u32()
                        .copied()
                        .expect("Expected column 4 to be a uint32!"),
                    AnnounceQuestLocked: row
                        .columns[14]
                        .into_u32()
                        .copied()
                        .expect("Expected column 14 to be a uint32!"),
                    MapAnnounceQuest1: row
                        .columns[24]
                        .into_u32()
                        .copied()
                        .expect("Expected column 24 to be a uint32!"),
                    MapAnnounceQuestLocked: row
                        .columns[34]
                        .into_u32()
                        .copied()
                        .expect("Expected column 34 to be a uint32!"),
                    MapAnnounceQuest2: row
                        .columns[44]
                        .into_u32()
                        .copied()
                        .expect("Expected column 44 to be a uint32!"),
                },
                IconsElement {
                    AnnounceQuest: row
                        .columns[5]
                        .into_u32()
                        .copied()
                        .expect("Expected column 5 to be a uint32!"),
                    AnnounceQuestLocked: row
                        .columns[15]
                        .into_u32()
                        .copied()
                        .expect("Expected column 15 to be a uint32!"),
                    MapAnnounceQuest1: row
                        .columns[25]
                        .into_u32()
                        .copied()
                        .expect("Expected column 25 to be a uint32!"),
                    MapAnnounceQuestLocked: row
                        .columns[35]
                        .into_u32()
                        .copied()
                        .expect("Expected column 35 to be a uint32!"),
                    MapAnnounceQuest2: row
                        .columns[45]
                        .into_u32()
                        .copied()
                        .expect("Expected column 45 to be a uint32!"),
                },
                IconsElement {
                    AnnounceQuest: row
                        .columns[6]
                        .into_u32()
                        .copied()
                        .expect("Expected column 6 to be a uint32!"),
                    AnnounceQuestLocked: row
                        .columns[16]
                        .into_u32()
                        .copied()
                        .expect("Expected column 16 to be a uint32!"),
                    MapAnnounceQuest1: row
                        .columns[26]
                        .into_u32()
                        .copied()
                        .expect("Expected column 26 to be a uint32!"),
                    MapAnnounceQuestLocked: row
                        .columns[36]
                        .into_u32()
                        .copied()
                        .expect("Expected column 36 to be a uint32!"),
                    MapAnnounceQuest2: row
                        .columns[46]
                        .into_u32()
                        .copied()
                        .expect("Expected column 46 to be a uint32!"),
                },
                IconsElement {
                    AnnounceQuest: row
                        .columns[7]
                        .into_u32()
                        .copied()
                        .expect("Expected column 7 to be a uint32!"),
                    AnnounceQuestLocked: row
                        .columns[17]
                        .into_u32()
                        .copied()
                        .expect("Expected column 17 to be a uint32!"),
                    MapAnnounceQuest1: row
                        .columns[27]
                        .into_u32()
                        .copied()
                        .expect("Expected column 27 to be a uint32!"),
                    MapAnnounceQuestLocked: row
                        .columns[37]
                        .into_u32()
                        .copied()
                        .expect("Expected column 37 to be a uint32!"),
                    MapAnnounceQuest2: row
                        .columns[47]
                        .into_u32()
                        .copied()
                        .expect("Expected column 47 to be a uint32!"),
                },
                IconsElement {
                    AnnounceQuest: row
                        .columns[8]
                        .into_u32()
                        .copied()
                        .expect("Expected column 8 to be a uint32!"),
                    AnnounceQuestLocked: row
                        .columns[18]
                        .into_u32()
                        .copied()
                        .expect("Expected column 18 to be a uint32!"),
                    MapAnnounceQuest1: row
                        .columns[28]
                        .into_u32()
                        .copied()
                        .expect("Expected column 28 to be a uint32!"),
                    MapAnnounceQuestLocked: row
                        .columns[38]
                        .into_u32()
                        .copied()
                        .expect("Expected column 38 to be a uint32!"),
                    MapAnnounceQuest2: row
                        .columns[48]
                        .into_u32()
                        .copied()
                        .expect("Expected column 48 to be a uint32!"),
                },
                IconsElement {
                    AnnounceQuest: row
                        .columns[9]
                        .into_u32()
                        .copied()
                        .expect("Expected column 9 to be a uint32!"),
                    AnnounceQuestLocked: row
                        .columns[19]
                        .into_u32()
                        .copied()
                        .expect("Expected column 19 to be a uint32!"),
                    MapAnnounceQuest1: row
                        .columns[29]
                        .into_u32()
                        .copied()
                        .expect("Expected column 29 to be a uint32!"),
                    MapAnnounceQuestLocked: row
                        .columns[39]
                        .into_u32()
                        .copied()
                        .expect("Expected column 39 to be a uint32!"),
                    MapAnnounceQuest2: row
                        .columns[49]
                        .into_u32()
                        .copied()
                        .expect("Expected column 49 to be a uint32!"),
                },
            ],
            Unknown0: row
                .columns[50]
                .into_u8()
                .copied()
                .expect("Expected column 50 to be a uint8!"),
        })
    }
}
impl<'a> IntoIterator for &'a EventCustomIconTypeSheet {
    type Item = (u32, Vec<(u16, EventCustomIconTypeRow)>);
    type IntoIter = StructuredSheetIterator<'a, EventCustomIconTypeSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, EventCustomIconTypeSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EventCustomIconTypeRow {
    ///""
    pub Icons: [IconsElement; 10],
    ///""
    pub Unknown0: u8,
}
