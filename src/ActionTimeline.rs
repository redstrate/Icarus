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
}
impl ActionTimelineSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("ActionTimeline")?;
        let sheet = resolver.read_excel_sheet(&exh, "ActionTimeline", language)?;
        Ok(Self { sheet })
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
impl StructuredSheet for ActionTimelineSheet {
    type Row = ActionTimelineRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Key: row
                .columns[6]
                .into_string()
                .cloned()
                .expect("Expected column 6 to be a string!"),
            WeaponTimeline: row
                .columns[8]
                .into_u16()
                .copied()
                .expect("Expected column 8 to be a uint16!"),
            KillUpper: row
                .columns[12]
                .into_u16()
                .copied()
                .expect("Expected column 12 to be a uint16!"),
            Unknown_70: row
                .columns[0]
                .into_u8()
                .copied()
                .expect("Expected column 0 to be a uint8!"),
            Type: row
                .columns[1]
                .into_u8()
                .copied()
                .expect("Expected column 1 to be a uint8!"),
            Priority: row
                .columns[3]
                .into_u8()
                .copied()
                .expect("Expected column 3 to be a uint8!"),
            Stance: row
                .columns[4]
                .into_u8()
                .copied()
                .expect("Expected column 4 to be a uint8!"),
            Slot: row
                .columns[5]
                .into_u8()
                .copied()
                .expect("Expected column 5 to be a uint8!"),
            LookAtMode: row
                .columns[7]
                .into_u8()
                .copied()
                .expect("Expected column 7 to be a uint8!"),
            ActionTimelineIDMode: row
                .columns[9]
                .into_u8()
                .copied()
                .expect("Expected column 9 to be a uint8!"),
            LoadType: row
                .columns[10]
                .into_u8()
                .copied()
                .expect("Expected column 10 to be a uint8!"),
            StartAttach: row
                .columns[11]
                .into_u8()
                .copied()
                .expect("Expected column 11 to be a uint8!"),
            ResidentPap: row
                .columns[15]
                .into_u8()
                .copied()
                .expect("Expected column 15 to be a uint8!"),
            Unknown6: row
                .columns[19]
                .into_u8()
                .copied()
                .expect("Expected column 19 to be a uint8!"),
            Unknown1: row
                .columns[20]
                .into_u8()
                .copied()
                .expect("Expected column 20 to be a uint8!"),
            Pause: row
                .columns[2]
                .into_bool()
                .copied()
                .expect("Expected column 2 to be a bool!"),
            Resident: row
                .columns[13]
                .into_bool()
                .copied()
                .expect("Expected column 13 to be a bool!"),
            IsMotionCanceledByMoving: row
                .columns[14]
                .into_bool()
                .copied()
                .expect("Expected column 14 to be a bool!"),
            Unknown2: row
                .columns[16]
                .into_bool()
                .copied()
                .expect("Expected column 16 to be a bool!"),
            Unknown3: row
                .columns[17]
                .into_bool()
                .copied()
                .expect("Expected column 17 to be a bool!"),
            IsLoop: row
                .columns[18]
                .into_bool()
                .copied()
                .expect("Expected column 18 to be a bool!"),
            Unknown4: row
                .columns[21]
                .into_bool()
                .copied()
                .expect("Expected column 21 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a ActionTimelineSheet {
    type Item = (u32, Vec<(u16, ActionTimelineRow)>);
    type IntoIter = StructuredSheetIterator<'a, ActionTimelineSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, ActionTimelineSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ActionTimelineRow {
    ///""
    pub Key: String,
    ///""
    pub WeaponTimeline: u16,
    ///""
    pub KillUpper: u16,
    ///""
    pub Unknown_70: u8,
    ///""
    pub Type: u8,
    ///""
    pub Priority: u8,
    ///""
    pub Stance: u8,
    ///""
    pub Slot: u8,
    ///""
    pub LookAtMode: u8,
    ///""
    pub ActionTimelineIDMode: u8,
    ///""
    pub LoadType: u8,
    ///""
    pub StartAttach: u8,
    ///""
    pub ResidentPap: u8,
    ///""
    pub Unknown6: u8,
    ///""
    pub Unknown1: u8,
    ///""
    pub Pause: bool,
    ///""
    pub Resident: bool,
    ///""
    pub IsMotionCanceledByMoving: bool,
    ///""
    pub Unknown2: bool,
    ///""
    pub Unknown3: bool,
    ///""
    pub IsLoop: bool,
    ///""
    pub Unknown4: bool,
}
