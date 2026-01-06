//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    common::Language,
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
impl StructuredSheet for EventCustomIconTypeSheet {
    type Row = EventCustomIconTypeRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        let column_defs = &self.sheet.exh.column_definitions;
        let mut zipped: Vec<_> = row
            .columns
            .clone()
            .into_iter()
            .zip(column_defs)
            .collect();
        zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
        let (columns, _): (Vec<Field>, Vec<ExcelColumnDefinition>) = zipped
            .into_iter()
            .unzip();
        Some(Self::Row { columns })
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
#[derive(Debug, Clone)]
pub struct EventCustomIconTypeRow {
    columns: Vec<Field>,
}
impl EventCustomIconTypeRow {
    pub fn Icons<'a>(&'a self) -> [IconsElement<'a>; 10] {
        [
            IconsElement {
                AnnounceQuest: &self.columns[0],
                AnnounceQuestLocked: &self.columns[1],
                MapAnnounceQuest1: &self.columns[2],
                MapAnnounceQuestLocked: &self.columns[3],
                MapAnnounceQuest2: &self.columns[4],
            },
            IconsElement {
                AnnounceQuest: &self.columns[5],
                AnnounceQuestLocked: &self.columns[6],
                MapAnnounceQuest1: &self.columns[7],
                MapAnnounceQuestLocked: &self.columns[8],
                MapAnnounceQuest2: &self.columns[9],
            },
            IconsElement {
                AnnounceQuest: &self.columns[10],
                AnnounceQuestLocked: &self.columns[11],
                MapAnnounceQuest1: &self.columns[12],
                MapAnnounceQuestLocked: &self.columns[13],
                MapAnnounceQuest2: &self.columns[14],
            },
            IconsElement {
                AnnounceQuest: &self.columns[15],
                AnnounceQuestLocked: &self.columns[16],
                MapAnnounceQuest1: &self.columns[17],
                MapAnnounceQuestLocked: &self.columns[18],
                MapAnnounceQuest2: &self.columns[19],
            },
            IconsElement {
                AnnounceQuest: &self.columns[20],
                AnnounceQuestLocked: &self.columns[21],
                MapAnnounceQuest1: &self.columns[22],
                MapAnnounceQuestLocked: &self.columns[23],
                MapAnnounceQuest2: &self.columns[24],
            },
            IconsElement {
                AnnounceQuest: &self.columns[25],
                AnnounceQuestLocked: &self.columns[26],
                MapAnnounceQuest1: &self.columns[27],
                MapAnnounceQuestLocked: &self.columns[28],
                MapAnnounceQuest2: &self.columns[29],
            },
            IconsElement {
                AnnounceQuest: &self.columns[30],
                AnnounceQuestLocked: &self.columns[31],
                MapAnnounceQuest1: &self.columns[32],
                MapAnnounceQuestLocked: &self.columns[33],
                MapAnnounceQuest2: &self.columns[34],
            },
            IconsElement {
                AnnounceQuest: &self.columns[35],
                AnnounceQuestLocked: &self.columns[36],
                MapAnnounceQuest1: &self.columns[37],
                MapAnnounceQuestLocked: &self.columns[38],
                MapAnnounceQuest2: &self.columns[39],
            },
            IconsElement {
                AnnounceQuest: &self.columns[40],
                AnnounceQuestLocked: &self.columns[41],
                MapAnnounceQuest1: &self.columns[42],
                MapAnnounceQuestLocked: &self.columns[43],
                MapAnnounceQuest2: &self.columns[44],
            },
            IconsElement {
                AnnounceQuest: &self.columns[45],
                AnnounceQuestLocked: &self.columns[46],
                MapAnnounceQuest1: &self.columns[47],
                MapAnnounceQuestLocked: &self.columns[48],
                MapAnnounceQuest2: &self.columns[49],
            },
        ]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a Field {
        &self.columns[50]
    }
}
