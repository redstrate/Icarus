//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
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
impl<'a> StructuredSheet<'a> for EventCustomIconTypeSheet {
    type Row = EventCustomIconTypeRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a EventCustomIconTypeSheet {
    type Item = (u32, Vec<(u16, EventCustomIconTypeRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, EventCustomIconTypeSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, EventCustomIconTypeSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct EventCustomIconTypeRow<'a> {
    row: &'a Row,
}
impl<'a> EventCustomIconTypeRow<'a> {
    pub fn Icons(&'a self) -> [IconsElement; 10] {
        [
            IconsElement {
                AnnounceQuest: self.row.columns[0].into_u32().copied().unwrap(),
                AnnounceQuestLocked: self.row.columns[10].into_u32().copied().unwrap(),
                MapAnnounceQuest1: self.row.columns[20].into_u32().copied().unwrap(),
                MapAnnounceQuestLocked: self
                    .row
                    .columns[30]
                    .into_u32()
                    .copied()
                    .unwrap(),
                MapAnnounceQuest2: self.row.columns[40].into_u32().copied().unwrap(),
            },
            IconsElement {
                AnnounceQuest: self.row.columns[1].into_u32().copied().unwrap(),
                AnnounceQuestLocked: self.row.columns[11].into_u32().copied().unwrap(),
                MapAnnounceQuest1: self.row.columns[21].into_u32().copied().unwrap(),
                MapAnnounceQuestLocked: self
                    .row
                    .columns[31]
                    .into_u32()
                    .copied()
                    .unwrap(),
                MapAnnounceQuest2: self.row.columns[41].into_u32().copied().unwrap(),
            },
            IconsElement {
                AnnounceQuest: self.row.columns[2].into_u32().copied().unwrap(),
                AnnounceQuestLocked: self.row.columns[12].into_u32().copied().unwrap(),
                MapAnnounceQuest1: self.row.columns[22].into_u32().copied().unwrap(),
                MapAnnounceQuestLocked: self
                    .row
                    .columns[32]
                    .into_u32()
                    .copied()
                    .unwrap(),
                MapAnnounceQuest2: self.row.columns[42].into_u32().copied().unwrap(),
            },
            IconsElement {
                AnnounceQuest: self.row.columns[3].into_u32().copied().unwrap(),
                AnnounceQuestLocked: self.row.columns[13].into_u32().copied().unwrap(),
                MapAnnounceQuest1: self.row.columns[23].into_u32().copied().unwrap(),
                MapAnnounceQuestLocked: self
                    .row
                    .columns[33]
                    .into_u32()
                    .copied()
                    .unwrap(),
                MapAnnounceQuest2: self.row.columns[43].into_u32().copied().unwrap(),
            },
            IconsElement {
                AnnounceQuest: self.row.columns[4].into_u32().copied().unwrap(),
                AnnounceQuestLocked: self.row.columns[14].into_u32().copied().unwrap(),
                MapAnnounceQuest1: self.row.columns[24].into_u32().copied().unwrap(),
                MapAnnounceQuestLocked: self
                    .row
                    .columns[34]
                    .into_u32()
                    .copied()
                    .unwrap(),
                MapAnnounceQuest2: self.row.columns[44].into_u32().copied().unwrap(),
            },
            IconsElement {
                AnnounceQuest: self.row.columns[5].into_u32().copied().unwrap(),
                AnnounceQuestLocked: self.row.columns[15].into_u32().copied().unwrap(),
                MapAnnounceQuest1: self.row.columns[25].into_u32().copied().unwrap(),
                MapAnnounceQuestLocked: self
                    .row
                    .columns[35]
                    .into_u32()
                    .copied()
                    .unwrap(),
                MapAnnounceQuest2: self.row.columns[45].into_u32().copied().unwrap(),
            },
            IconsElement {
                AnnounceQuest: self.row.columns[6].into_u32().copied().unwrap(),
                AnnounceQuestLocked: self.row.columns[16].into_u32().copied().unwrap(),
                MapAnnounceQuest1: self.row.columns[26].into_u32().copied().unwrap(),
                MapAnnounceQuestLocked: self
                    .row
                    .columns[36]
                    .into_u32()
                    .copied()
                    .unwrap(),
                MapAnnounceQuest2: self.row.columns[46].into_u32().copied().unwrap(),
            },
            IconsElement {
                AnnounceQuest: self.row.columns[7].into_u32().copied().unwrap(),
                AnnounceQuestLocked: self.row.columns[17].into_u32().copied().unwrap(),
                MapAnnounceQuest1: self.row.columns[27].into_u32().copied().unwrap(),
                MapAnnounceQuestLocked: self
                    .row
                    .columns[37]
                    .into_u32()
                    .copied()
                    .unwrap(),
                MapAnnounceQuest2: self.row.columns[47].into_u32().copied().unwrap(),
            },
            IconsElement {
                AnnounceQuest: self.row.columns[8].into_u32().copied().unwrap(),
                AnnounceQuestLocked: self.row.columns[18].into_u32().copied().unwrap(),
                MapAnnounceQuest1: self.row.columns[28].into_u32().copied().unwrap(),
                MapAnnounceQuestLocked: self
                    .row
                    .columns[38]
                    .into_u32()
                    .copied()
                    .unwrap(),
                MapAnnounceQuest2: self.row.columns[48].into_u32().copied().unwrap(),
            },
            IconsElement {
                AnnounceQuest: self.row.columns[9].into_u32().copied().unwrap(),
                AnnounceQuestLocked: self.row.columns[19].into_u32().copied().unwrap(),
                MapAnnounceQuest1: self.row.columns[29].into_u32().copied().unwrap(),
                MapAnnounceQuestLocked: self
                    .row
                    .columns[39]
                    .into_u32()
                    .copied()
                    .unwrap(),
                MapAnnounceQuest2: self.row.columns[49].into_u32().copied().unwrap(),
            },
        ]
    }
    pub fn Unknown0(&'a self) -> u8 {
        self.row.columns[50].into_u8().copied().unwrap()
    }
}
