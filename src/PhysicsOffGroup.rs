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
pub struct PhysicsOffGroupSheet {
    sheet: Sheet,
}
impl PhysicsOffGroupSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("PhysicsOffGroup")?;
        let sheet = resolver.read_excel_sheet(&exh, "PhysicsOffGroup", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<PhysicsOffGroupRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<PhysicsOffGroupRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for PhysicsOffGroupSheet {
    type Row = PhysicsOffGroupRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Unknown0: row
                .columns[0]
                .into_i8()
                .copied()
                .expect("Expected column 0 to be a int8!"),
            Unknown1: row
                .columns[1]
                .into_i8()
                .copied()
                .expect("Expected column 1 to be a int8!"),
            Unknown_70_1: row
                .columns[2]
                .into_i8()
                .copied()
                .expect("Expected column 2 to be a int8!"),
            Unknown_70_2: row
                .columns[3]
                .into_i8()
                .copied()
                .expect("Expected column 3 to be a int8!"),
            Unknown_70_3: row
                .columns[4]
                .into_i8()
                .copied()
                .expect("Expected column 4 to be a int8!"),
            Unknown_70_4: row
                .columns[5]
                .into_i8()
                .copied()
                .expect("Expected column 5 to be a int8!"),
            Unknown_70_5: row
                .columns[6]
                .into_i8()
                .copied()
                .expect("Expected column 6 to be a int8!"),
            Unknown_70_6: row
                .columns[7]
                .into_i8()
                .copied()
                .expect("Expected column 7 to be a int8!"),
            Unknown_70_7: row
                .columns[8]
                .into_i8()
                .copied()
                .expect("Expected column 8 to be a int8!"),
            Unknown_70_8: row
                .columns[9]
                .into_i8()
                .copied()
                .expect("Expected column 9 to be a int8!"),
            Unknown_70_9: row
                .columns[10]
                .into_i8()
                .copied()
                .expect("Expected column 10 to be a int8!"),
            Unknown_70_10: row
                .columns[11]
                .into_i8()
                .copied()
                .expect("Expected column 11 to be a int8!"),
            Unknown_70_11: row
                .columns[12]
                .into_i8()
                .copied()
                .expect("Expected column 12 to be a int8!"),
            Unknown_70_12: row
                .columns[13]
                .into_i8()
                .copied()
                .expect("Expected column 13 to be a int8!"),
            Unknown_70_13: row
                .columns[14]
                .into_i8()
                .copied()
                .expect("Expected column 14 to be a int8!"),
            Unknown_70_14: row
                .columns[15]
                .into_i8()
                .copied()
                .expect("Expected column 15 to be a int8!"),
        })
    }
}
impl<'a> IntoIterator for &'a PhysicsOffGroupSheet {
    type Item = (u32, Vec<(u16, PhysicsOffGroupRow)>);
    type IntoIter = StructuredSheetIterator<'a, PhysicsOffGroupSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, PhysicsOffGroupSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PhysicsOffGroupRow {
    ///""
    pub Unknown0: i8,
    ///""
    pub Unknown1: i8,
    ///""
    pub Unknown_70_1: i8,
    ///""
    pub Unknown_70_2: i8,
    ///""
    pub Unknown_70_3: i8,
    ///""
    pub Unknown_70_4: i8,
    ///""
    pub Unknown_70_5: i8,
    ///""
    pub Unknown_70_6: i8,
    ///""
    pub Unknown_70_7: i8,
    ///""
    pub Unknown_70_8: i8,
    ///""
    pub Unknown_70_9: i8,
    ///""
    pub Unknown_70_10: i8,
    ///""
    pub Unknown_70_11: i8,
    ///""
    pub Unknown_70_12: i8,
    ///""
    pub Unknown_70_13: i8,
    ///""
    pub Unknown_70_14: i8,
}
