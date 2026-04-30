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
pub struct CraftActionSheet {
    sheet: Sheet,
}
impl CraftActionSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("CraftAction")?;
        let sheet = resolver.read_excel_sheet(&exh, "CraftAction", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<CraftActionRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<CraftActionRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for CraftActionSheet {
    type Row = CraftActionRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Name: row
                .columns[0]
                .into_string()
                .cloned()
                .expect("Expected column 0 to be a string!"),
            Description: row
                .columns[1]
                .into_string()
                .cloned()
                .expect("Expected column 1 to be a string!"),
            QuestRequirement: row
                .columns[8]
                .into_u32()
                .copied()
                .expect("Expected column 8 to be a uint32!"),
            CRP: row
                .columns[12]
                .into_i32()
                .copied()
                .expect("Expected column 12 to be a int32!"),
            BSM: row
                .columns[13]
                .into_i32()
                .copied()
                .expect("Expected column 13 to be a int32!"),
            ARM: row
                .columns[14]
                .into_i32()
                .copied()
                .expect("Expected column 14 to be a int32!"),
            GSM: row
                .columns[15]
                .into_i32()
                .copied()
                .expect("Expected column 15 to be a int32!"),
            LTW: row
                .columns[16]
                .into_i32()
                .copied()
                .expect("Expected column 16 to be a int32!"),
            WVR: row
                .columns[17]
                .into_i32()
                .copied()
                .expect("Expected column 17 to be a int32!"),
            ALC: row
                .columns[18]
                .into_i32()
                .copied()
                .expect("Expected column 18 to be a int32!"),
            CUL: row
                .columns[19]
                .into_i32()
                .copied()
                .expect("Expected column 19 to be a int32!"),
            AnimationStart: row
                .columns[2]
                .into_u16()
                .copied()
                .expect("Expected column 2 to be a uint16!"),
            AnimationEnd: row
                .columns[3]
                .into_u16()
                .copied()
                .expect("Expected column 3 to be a uint16!"),
            Icon: row
                .columns[4]
                .into_u16()
                .copied()
                .expect("Expected column 4 to be a uint16!"),
            RequiredStatus: row
                .columns[10]
                .into_u16()
                .copied()
                .expect("Expected column 10 to be a uint16!"),
            ClassJobCategory: row
                .columns[6]
                .into_u8()
                .copied()
                .expect("Expected column 6 to be a uint8!"),
            ClassJobLevel: row
                .columns[7]
                .into_u8()
                .copied()
                .expect("Expected column 7 to be a uint8!"),
            Cost: row
                .columns[11]
                .into_u8()
                .copied()
                .expect("Expected column 11 to be a uint8!"),
            ClassJob: row
                .columns[5]
                .into_i8()
                .copied()
                .expect("Expected column 5 to be a int8!"),
            Specialist: row
                .columns[9]
                .into_bool()
                .copied()
                .expect("Expected column 9 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a CraftActionSheet {
    type Item = (u32, Vec<(u16, CraftActionRow)>);
    type IntoIter = StructuredSheetIterator<'a, CraftActionSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, CraftActionSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CraftActionRow {
    ///""
    pub Name: String,
    ///""
    pub Description: String,
    ///""
    pub QuestRequirement: u32,
    ///""
    pub CRP: i32,
    ///""
    pub BSM: i32,
    ///""
    pub ARM: i32,
    ///""
    pub GSM: i32,
    ///""
    pub LTW: i32,
    ///""
    pub WVR: i32,
    ///""
    pub ALC: i32,
    ///""
    pub CUL: i32,
    ///""
    pub AnimationStart: u16,
    ///""
    pub AnimationEnd: u16,
    ///""
    pub Icon: u16,
    ///""
    pub RequiredStatus: u16,
    ///""
    pub ClassJobCategory: u8,
    ///""
    pub ClassJobLevel: u8,
    ///""
    pub Cost: u8,
    ///""
    pub ClassJob: i8,
    ///""
    pub Specialist: bool,
}
