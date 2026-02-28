//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
pub struct IconsElement<'a> {
    pub AnnounceQuest: &'a Field,
    pub AnnounceQuestLocked: &'a Field,
    pub MapAnnounceQuest1: &'a Field,
    pub MapAnnounceQuestLocked: &'a Field,
    pub MapAnnounceQuest2: &'a Field,
}
#[derive(Debug, Clone)]
pub struct EventCustomIconTypeSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl EventCustomIconTypeSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("EventCustomIconType")?;
        let sheet = resolver.read_excel_sheet(&exh, "EventCustomIconType", language)?;
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
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
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
    index_mapping: Vec<usize>,
}
impl<'a> EventCustomIconTypeRow<'a> {
    pub fn Icons(&'a self) -> [IconsElement<'a>; 10] {
        [
            IconsElement {
                AnnounceQuest: &self.row.columns[self.index_mapping[0]],
                AnnounceQuestLocked: &self.row.columns[self.index_mapping[1]],
                MapAnnounceQuest1: &self.row.columns[self.index_mapping[2]],
                MapAnnounceQuestLocked: &self.row.columns[self.index_mapping[3]],
                MapAnnounceQuest2: &self.row.columns[self.index_mapping[4]],
            },
            IconsElement {
                AnnounceQuest: &self.row.columns[self.index_mapping[5]],
                AnnounceQuestLocked: &self.row.columns[self.index_mapping[6]],
                MapAnnounceQuest1: &self.row.columns[self.index_mapping[7]],
                MapAnnounceQuestLocked: &self.row.columns[self.index_mapping[8]],
                MapAnnounceQuest2: &self.row.columns[self.index_mapping[9]],
            },
            IconsElement {
                AnnounceQuest: &self.row.columns[self.index_mapping[10]],
                AnnounceQuestLocked: &self.row.columns[self.index_mapping[11]],
                MapAnnounceQuest1: &self.row.columns[self.index_mapping[12]],
                MapAnnounceQuestLocked: &self.row.columns[self.index_mapping[13]],
                MapAnnounceQuest2: &self.row.columns[self.index_mapping[14]],
            },
            IconsElement {
                AnnounceQuest: &self.row.columns[self.index_mapping[15]],
                AnnounceQuestLocked: &self.row.columns[self.index_mapping[16]],
                MapAnnounceQuest1: &self.row.columns[self.index_mapping[17]],
                MapAnnounceQuestLocked: &self.row.columns[self.index_mapping[18]],
                MapAnnounceQuest2: &self.row.columns[self.index_mapping[19]],
            },
            IconsElement {
                AnnounceQuest: &self.row.columns[self.index_mapping[20]],
                AnnounceQuestLocked: &self.row.columns[self.index_mapping[21]],
                MapAnnounceQuest1: &self.row.columns[self.index_mapping[22]],
                MapAnnounceQuestLocked: &self.row.columns[self.index_mapping[23]],
                MapAnnounceQuest2: &self.row.columns[self.index_mapping[24]],
            },
            IconsElement {
                AnnounceQuest: &self.row.columns[self.index_mapping[25]],
                AnnounceQuestLocked: &self.row.columns[self.index_mapping[26]],
                MapAnnounceQuest1: &self.row.columns[self.index_mapping[27]],
                MapAnnounceQuestLocked: &self.row.columns[self.index_mapping[28]],
                MapAnnounceQuest2: &self.row.columns[self.index_mapping[29]],
            },
            IconsElement {
                AnnounceQuest: &self.row.columns[self.index_mapping[30]],
                AnnounceQuestLocked: &self.row.columns[self.index_mapping[31]],
                MapAnnounceQuest1: &self.row.columns[self.index_mapping[32]],
                MapAnnounceQuestLocked: &self.row.columns[self.index_mapping[33]],
                MapAnnounceQuest2: &self.row.columns[self.index_mapping[34]],
            },
            IconsElement {
                AnnounceQuest: &self.row.columns[self.index_mapping[35]],
                AnnounceQuestLocked: &self.row.columns[self.index_mapping[36]],
                MapAnnounceQuest1: &self.row.columns[self.index_mapping[37]],
                MapAnnounceQuestLocked: &self.row.columns[self.index_mapping[38]],
                MapAnnounceQuest2: &self.row.columns[self.index_mapping[39]],
            },
            IconsElement {
                AnnounceQuest: &self.row.columns[self.index_mapping[40]],
                AnnounceQuestLocked: &self.row.columns[self.index_mapping[41]],
                MapAnnounceQuest1: &self.row.columns[self.index_mapping[42]],
                MapAnnounceQuestLocked: &self.row.columns[self.index_mapping[43]],
                MapAnnounceQuest2: &self.row.columns[self.index_mapping[44]],
            },
            IconsElement {
                AnnounceQuest: &self.row.columns[self.index_mapping[45]],
                AnnounceQuestLocked: &self.row.columns[self.index_mapping[46]],
                MapAnnounceQuest1: &self.row.columns[self.index_mapping[47]],
                MapAnnounceQuestLocked: &self.row.columns[self.index_mapping[48]],
                MapAnnounceQuest2: &self.row.columns[self.index_mapping[49]],
            },
        ]
    }
    pub fn Unknown0(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[50]]
    }
}
