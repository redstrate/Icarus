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
pub struct CutsceneMotionSheet {
    sheet: Sheet,
}
impl CutsceneMotionSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("CutsceneMotion")?;
        let sheet = resolver.read_excel_sheet(&exh, "CutsceneMotion", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<CutsceneMotionRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<CutsceneMotionRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for CutsceneMotionSheet {
    type Row = CutsceneMotionRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            WALK_LOOP_SPEED: row
                .columns[0]
                .into_f32()
                .copied()
                .expect("Expected column 0 to be a float32!"),
            RUN_LOOP_SPEED: row
                .columns[1]
                .into_f32()
                .copied()
                .expect("Expected column 1 to be a float32!"),
            SLOWWALK_LOOP_SPEED: row
                .columns[2]
                .into_f32()
                .copied()
                .expect("Expected column 2 to be a float32!"),
            SLOWRUN_LOOP_SPEED: row
                .columns[3]
                .into_f32()
                .copied()
                .expect("Expected column 3 to be a float32!"),
            BATTLEWALK_LOOP_SPEED: row
                .columns[4]
                .into_f32()
                .copied()
                .expect("Expected column 4 to be a float32!"),
            BATTLERUN_LOOP_SPEED: row
                .columns[5]
                .into_f32()
                .copied()
                .expect("Expected column 5 to be a float32!"),
            DASH_LOOP_SPEED: row
                .columns[6]
                .into_f32()
                .copied()
                .expect("Expected column 6 to be a float32!"),
            TURN_CW90_FRAME: row
                .columns[7]
                .into_u8()
                .copied()
                .expect("Expected column 7 to be a uint8!"),
            TURN_CCW90_FRAME: row
                .columns[8]
                .into_u8()
                .copied()
                .expect("Expected column 8 to be a uint8!"),
            TURN_CW180_FRAME: row
                .columns[9]
                .into_u8()
                .copied()
                .expect("Expected column 9 to be a uint8!"),
            TURN_CCW180_FRAME: row
                .columns[10]
                .into_u8()
                .copied()
                .expect("Expected column 10 to be a uint8!"),
        })
    }
}
impl<'a> IntoIterator for &'a CutsceneMotionSheet {
    type Item = (u32, Vec<(u16, CutsceneMotionRow)>);
    type IntoIter = StructuredSheetIterator<'a, CutsceneMotionSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, CutsceneMotionSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CutsceneMotionRow {
    ///""
    pub WALK_LOOP_SPEED: f32,
    ///""
    pub RUN_LOOP_SPEED: f32,
    ///""
    pub SLOWWALK_LOOP_SPEED: f32,
    ///""
    pub SLOWRUN_LOOP_SPEED: f32,
    ///""
    pub BATTLEWALK_LOOP_SPEED: f32,
    ///""
    pub BATTLERUN_LOOP_SPEED: f32,
    ///""
    pub DASH_LOOP_SPEED: f32,
    ///""
    pub TURN_CW90_FRAME: u8,
    ///""
    pub TURN_CCW90_FRAME: u8,
    ///""
    pub TURN_CW180_FRAME: u8,
    ///""
    pub TURN_CCW180_FRAME: u8,
}
