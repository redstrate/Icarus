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
pub struct ActionTimelineSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl ActionTimelineSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("ActionTimeline")?;
        let sheet = resolver.read_excel_sheet(&exh, "ActionTimeline", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<ActionTimelineRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<ActionTimelineRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for ActionTimelineSheet {
    type Row = ActionTimelineRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a ActionTimelineSheet {
    type Item = (u32, Vec<(u16, ActionTimelineRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, ActionTimelineSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, ActionTimelineSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ActionTimelineRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> ActionTimelineRow<'a> {
    pub fn Key(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[0]]
    }
    pub fn WeaponTimeline(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1]]
    }
    pub fn KillUpper(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[2]]
    }
    pub fn Unknown_70(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[3]]
    }
    pub fn Type(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[4]]
    }
    pub fn Priority(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[5]]
    }
    pub fn Stance(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[6]]
    }
    pub fn Slot(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[7]]
    }
    pub fn LookAtMode(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[8]]
    }
    pub fn ActionTimelineIDMode(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[9]]
    }
    pub fn LoadType(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[10]]
    }
    pub fn StartAttach(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[11]]
    }
    pub fn ResidentPap(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[12]]
    }
    pub fn Unknown6(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[13]]
    }
    pub fn Unknown1(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[14]]
    }
    pub fn Pause(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[15]]
    }
    pub fn Resident(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[16]]
    }
    pub fn IsMotionCanceledByMoving(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[17]]
    }
    pub fn Unknown2(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[18]]
    }
    pub fn Unknown3(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[19]]
    }
    pub fn IsLoop(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[20]]
    }
    pub fn Unknown4(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[21]]
    }
}
