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
pub struct SystemGraphicPresetSheet {
    sheet: Sheet,
}
impl SystemGraphicPresetSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("SystemGraphicPreset")?;
        let sheet = resolver.read_excel_sheet(&exh, "SystemGraphicPreset", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<SystemGraphicPresetRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<SystemGraphicPresetRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for SystemGraphicPresetSheet {
    type Row = SystemGraphicPresetRow;
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
            Unknown2: row
                .columns[2]
                .into_i8()
                .copied()
                .expect("Expected column 2 to be a int8!"),
            Unknown3: row
                .columns[3]
                .into_i8()
                .copied()
                .expect("Expected column 3 to be a int8!"),
            Unknown4: row
                .columns[4]
                .into_i8()
                .copied()
                .expect("Expected column 4 to be a int8!"),
            Unknown5: row
                .columns[5]
                .into_i8()
                .copied()
                .expect("Expected column 5 to be a int8!"),
            Unknown6: row
                .columns[6]
                .into_i8()
                .copied()
                .expect("Expected column 6 to be a int8!"),
            Unknown7: row
                .columns[7]
                .into_i8()
                .copied()
                .expect("Expected column 7 to be a int8!"),
            Unknown8: row
                .columns[8]
                .into_i8()
                .copied()
                .expect("Expected column 8 to be a int8!"),
            Unknown9: row
                .columns[9]
                .into_i8()
                .copied()
                .expect("Expected column 9 to be a int8!"),
            Unknown10: row
                .columns[10]
                .into_i8()
                .copied()
                .expect("Expected column 10 to be a int8!"),
            Unknown11: row
                .columns[11]
                .into_i8()
                .copied()
                .expect("Expected column 11 to be a int8!"),
            Unknown12: row
                .columns[12]
                .into_i8()
                .copied()
                .expect("Expected column 12 to be a int8!"),
            Unknown13: row
                .columns[13]
                .into_i8()
                .copied()
                .expect("Expected column 13 to be a int8!"),
            Unknown14: row
                .columns[14]
                .into_i8()
                .copied()
                .expect("Expected column 14 to be a int8!"),
            Unknown15: row
                .columns[15]
                .into_i8()
                .copied()
                .expect("Expected column 15 to be a int8!"),
            Unknown16: row
                .columns[16]
                .into_i8()
                .copied()
                .expect("Expected column 16 to be a int8!"),
            Unknown17: row
                .columns[17]
                .into_i8()
                .copied()
                .expect("Expected column 17 to be a int8!"),
            Unknown18: row
                .columns[18]
                .into_i8()
                .copied()
                .expect("Expected column 18 to be a int8!"),
            Unknown19: row
                .columns[19]
                .into_i8()
                .copied()
                .expect("Expected column 19 to be a int8!"),
            Unknown20: row
                .columns[20]
                .into_i8()
                .copied()
                .expect("Expected column 20 to be a int8!"),
            Unknown21: row
                .columns[21]
                .into_i8()
                .copied()
                .expect("Expected column 21 to be a int8!"),
            Unknown22: row
                .columns[22]
                .into_i8()
                .copied()
                .expect("Expected column 22 to be a int8!"),
            Unknown23: row
                .columns[23]
                .into_i8()
                .copied()
                .expect("Expected column 23 to be a int8!"),
            Unknown24: row
                .columns[24]
                .into_i8()
                .copied()
                .expect("Expected column 24 to be a int8!"),
            Unknown25: row
                .columns[25]
                .into_i8()
                .copied()
                .expect("Expected column 25 to be a int8!"),
            Unknown26: row
                .columns[26]
                .into_i8()
                .copied()
                .expect("Expected column 26 to be a int8!"),
            Unknown27: row
                .columns[27]
                .into_i8()
                .copied()
                .expect("Expected column 27 to be a int8!"),
            Unknown28: row
                .columns[28]
                .into_i8()
                .copied()
                .expect("Expected column 28 to be a int8!"),
            Unknown29: row
                .columns[29]
                .into_i8()
                .copied()
                .expect("Expected column 29 to be a int8!"),
            Unknown30: row
                .columns[30]
                .into_i8()
                .copied()
                .expect("Expected column 30 to be a int8!"),
            Unknown31: row
                .columns[31]
                .into_i8()
                .copied()
                .expect("Expected column 31 to be a int8!"),
            Unknown32: row
                .columns[32]
                .into_i8()
                .copied()
                .expect("Expected column 32 to be a int8!"),
            Unknown_70_1: row
                .columns[33]
                .into_i8()
                .copied()
                .expect("Expected column 33 to be a int8!"),
            Unknown_70_2: row
                .columns[34]
                .into_i8()
                .copied()
                .expect("Expected column 34 to be a int8!"),
            Unknown_70_3: row
                .columns[35]
                .into_i8()
                .copied()
                .expect("Expected column 35 to be a int8!"),
            Unknown_70_4: row
                .columns[36]
                .into_i8()
                .copied()
                .expect("Expected column 36 to be a int8!"),
            Unknown_70_5: row
                .columns[37]
                .into_i8()
                .copied()
                .expect("Expected column 37 to be a int8!"),
            Unknown_70_6: row
                .columns[38]
                .into_i8()
                .copied()
                .expect("Expected column 38 to be a int8!"),
            Unknown_70_7: row
                .columns[39]
                .into_i8()
                .copied()
                .expect("Expected column 39 to be a int8!"),
        })
    }
}
impl<'a> IntoIterator for &'a SystemGraphicPresetSheet {
    type Item = (u32, Vec<(u16, SystemGraphicPresetRow)>);
    type IntoIter = StructuredSheetIterator<'a, SystemGraphicPresetSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, SystemGraphicPresetSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SystemGraphicPresetRow {
    ///""
    pub Unknown0: i8,
    ///""
    pub Unknown1: i8,
    ///""
    pub Unknown2: i8,
    ///""
    pub Unknown3: i8,
    ///""
    pub Unknown4: i8,
    ///""
    pub Unknown5: i8,
    ///""
    pub Unknown6: i8,
    ///""
    pub Unknown7: i8,
    ///""
    pub Unknown8: i8,
    ///""
    pub Unknown9: i8,
    ///""
    pub Unknown10: i8,
    ///""
    pub Unknown11: i8,
    ///""
    pub Unknown12: i8,
    ///""
    pub Unknown13: i8,
    ///""
    pub Unknown14: i8,
    ///""
    pub Unknown15: i8,
    ///""
    pub Unknown16: i8,
    ///""
    pub Unknown17: i8,
    ///""
    pub Unknown18: i8,
    ///""
    pub Unknown19: i8,
    ///""
    pub Unknown20: i8,
    ///""
    pub Unknown21: i8,
    ///""
    pub Unknown22: i8,
    ///""
    pub Unknown23: i8,
    ///""
    pub Unknown24: i8,
    ///""
    pub Unknown25: i8,
    ///""
    pub Unknown26: i8,
    ///""
    pub Unknown27: i8,
    ///""
    pub Unknown28: i8,
    ///""
    pub Unknown29: i8,
    ///""
    pub Unknown30: i8,
    ///""
    pub Unknown31: i8,
    ///""
    pub Unknown32: i8,
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
}
