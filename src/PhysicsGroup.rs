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
pub struct PhysicsGroupSheet {
    sheet: Sheet,
}
impl PhysicsGroupSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("PhysicsGroup")?;
        let sheet = resolver.read_excel_sheet(&exh, "PhysicsGroup", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<PhysicsGroupRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<PhysicsGroupRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for PhysicsGroupSheet {
    type Row = PhysicsGroupRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            SimulationTime: [
                row
                    .columns[0]
                    .into_f32()
                    .copied()
                    .expect("Expected column 0 to be a float32!"),
                row
                    .columns[1]
                    .into_f32()
                    .copied()
                    .expect("Expected column 1 to be a float32!"),
                row
                    .columns[2]
                    .into_f32()
                    .copied()
                    .expect("Expected column 2 to be a float32!"),
                row
                    .columns[3]
                    .into_f32()
                    .copied()
                    .expect("Expected column 3 to be a float32!"),
                row
                    .columns[4]
                    .into_f32()
                    .copied()
                    .expect("Expected column 4 to be a float32!"),
                row
                    .columns[5]
                    .into_f32()
                    .copied()
                    .expect("Expected column 5 to be a float32!"),
            ],
            PS3SimulationTime: [
                row
                    .columns[6]
                    .into_f32()
                    .copied()
                    .expect("Expected column 6 to be a float32!"),
                row
                    .columns[7]
                    .into_f32()
                    .copied()
                    .expect("Expected column 7 to be a float32!"),
                row
                    .columns[8]
                    .into_f32()
                    .copied()
                    .expect("Expected column 8 to be a float32!"),
                row
                    .columns[9]
                    .into_f32()
                    .copied()
                    .expect("Expected column 9 to be a float32!"),
                row
                    .columns[10]
                    .into_f32()
                    .copied()
                    .expect("Expected column 10 to be a float32!"),
                row
                    .columns[11]
                    .into_f32()
                    .copied()
                    .expect("Expected column 11 to be a float32!"),
            ],
            RootFollowingGame: row
                .columns[13]
                .into_f32()
                .copied()
                .expect("Expected column 13 to be a float32!"),
            RootFollowingCutScene: row
                .columns[14]
                .into_f32()
                .copied()
                .expect("Expected column 14 to be a float32!"),
            ConfigSwitch: [
                row
                    .columns[15]
                    .into_i8()
                    .copied()
                    .expect("Expected column 15 to be a int8!"),
                row
                    .columns[16]
                    .into_i8()
                    .copied()
                    .expect("Expected column 16 to be a int8!"),
                row
                    .columns[17]
                    .into_i8()
                    .copied()
                    .expect("Expected column 17 to be a int8!"),
            ],
            ResetByLookAt: row
                .columns[12]
                .into_bool()
                .copied()
                .expect("Expected column 12 to be a bool!"),
            ForceAttractByPhysicsOff: row
                .columns[18]
                .into_bool()
                .copied()
                .expect("Expected column 18 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a PhysicsGroupSheet {
    type Item = (u32, Vec<(u16, PhysicsGroupRow)>);
    type IntoIter = StructuredSheetIterator<'a, PhysicsGroupSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, PhysicsGroupSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PhysicsGroupRow {
    ///""
    pub SimulationTime: [f32; 6],
    ///""
    pub PS3SimulationTime: [f32; 6],
    ///""
    pub RootFollowingGame: f32,
    ///""
    pub RootFollowingCutScene: f32,
    ///""
    pub ConfigSwitch: [i8; 3],
    ///""
    pub ResetByLookAt: bool,
    ///""
    pub ForceAttractByPhysicsOff: bool,
}
