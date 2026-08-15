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
pub struct MoveControlSheet {
    sheet: Sheet,
}
impl MoveControlSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("MoveControl")?;
        let sheet = resolver.read_excel_sheet(&exh, "MoveControl", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<MoveControlRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<MoveControlRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for MoveControlSheet {
    type Row = MoveControlRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Acceleration: row
                .columns[0]
                .into_f32()
                .copied()
                .expect("Expected column 0 to be a float32!"),
            Unknown1: row
                .columns[1]
                .into_f32()
                .copied()
                .expect("Expected column 1 to be a float32!"),
            GroundSpeed: row
                .columns[2]
                .into_f32()
                .copied()
                .expect("Expected column 2 to be a float32!"),
            RpWalkTurnSpeed: row
                .columns[3]
                .into_f32()
                .copied()
                .expect("Expected column 3 to be a float32!"),
            RpWalkSpeed: row
                .columns[4]
                .into_f32()
                .copied()
                .expect("Expected column 4 to be a float32!"),
            RpWalkStrafeSpeed: row
                .columns[5]
                .into_f32()
                .copied()
                .expect("Expected column 5 to be a float32!"),
            BackpedalSpeed: row
                .columns[6]
                .into_f32()
                .copied()
                .expect("Expected column 6 to be a float32!"),
            RotationStep: row
                .columns[7]
                .into_u8()
                .copied()
                .expect("Expected column 7 to be a uint8!"),
            Unknown8: row
                .columns[8]
                .into_u8()
                .copied()
                .expect("Expected column 8 to be a uint8!"),
            Unknown9: row
                .columns[9]
                .into_u8()
                .copied()
                .expect("Expected column 9 to be a uint8!"),
        })
    }
}
impl<'a> IntoIterator for &'a MoveControlSheet {
    type Item = (u32, Vec<(u16, MoveControlRow)>);
    type IntoIter = StructuredSheetIterator<'a, MoveControlSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, MoveControlSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MoveControlRow {
    ///""
    pub Acceleration: f32,
    ///""
    pub Unknown1: f32,
    ///""
    pub GroundSpeed: f32,
    ///"Left/Right for the turn control scheme"
    pub RpWalkTurnSpeed: f32,
    ///"Forwards/backwards"
    pub RpWalkSpeed: f32,
    ///"Left/Right for the strafe control scheme"
    pub RpWalkStrafeSpeed: f32,
    ///"Standard controls backwards or legacy back+strafe"
    pub BackpedalSpeed: f32,
    ///"Angle in degrees per step when turning"
    pub RotationStep: u8,
    ///""
    pub Unknown8: u8,
    ///""
    pub Unknown9: u8,
}
