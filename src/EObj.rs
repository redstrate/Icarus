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
pub struct EObjSheet {
    sheet: Sheet,
}
impl EObjSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("EObj")?;
        let sheet = resolver.read_excel_sheet(&exh, "EObj", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<EObjRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<EObjRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for EObjSheet {
    type Row = EObjRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Data: row
                .columns[9]
                .into_u32()
                .copied()
                .expect("Expected column 9 to be a uint32!"),
            SgbPath: row
                .columns[11]
                .into_u16()
                .copied()
                .expect("Expected column 11 to be a uint16!"),
            PopType: row
                .columns[8]
                .into_u8()
                .copied()
                .expect("Expected column 8 to be a uint8!"),
            Invisibility: row
                .columns[10]
                .into_u8()
                .copied()
                .expect("Expected column 10 to be a uint8!"),
            Unknown0: row
                .columns[18]
                .into_u8()
                .copied()
                .expect("Expected column 18 to be a uint8!"),
            AdditionalInteractRangeX: row
                .columns[15]
                .into_i8()
                .copied()
                .expect("Expected column 15 to be a int8!"),
            AdditionalInteractRangeY: row
                .columns[16]
                .into_i8()
                .copied()
                .expect("Expected column 16 to be a int8!"),
            Unknown1: row
                .columns[0]
                .into_bool()
                .copied()
                .expect("Expected column 0 to be a bool!"),
            Unknown2: row
                .columns[1]
                .into_bool()
                .copied()
                .expect("Expected column 1 to be a bool!"),
            Unknown3: row
                .columns[2]
                .into_bool()
                .copied()
                .expect("Expected column 2 to be a bool!"),
            Unknown4: row
                .columns[3]
                .into_bool()
                .copied()
                .expect("Expected column 3 to be a bool!"),
            Unknown5: row
                .columns[4]
                .into_bool()
                .copied()
                .expect("Expected column 4 to be a bool!"),
            Unknown6: row
                .columns[5]
                .into_bool()
                .copied()
                .expect("Expected column 5 to be a bool!"),
            Unknown7: row
                .columns[6]
                .into_bool()
                .copied()
                .expect("Expected column 6 to be a bool!"),
            Unknown8: row
                .columns[7]
                .into_bool()
                .copied()
                .expect("Expected column 7 to be a bool!"),
            Targetable: row
                .columns[12]
                .into_bool()
                .copied()
                .expect("Expected column 12 to be a bool!"),
            Unknown9: row
                .columns[13]
                .into_bool()
                .copied()
                .expect("Expected column 13 to be a bool!"),
            Unknown10: row
                .columns[14]
                .into_bool()
                .copied()
                .expect("Expected column 14 to be a bool!"),
            Unknown11: row
                .columns[17]
                .into_bool()
                .copied()
                .expect("Expected column 17 to be a bool!"),
            LuaSetup: row
                .columns[19]
                .into_bool()
                .copied()
                .expect("Expected column 19 to be a bool!"),
            SharedTimelineStateUnknown: row
                .columns[20]
                .into_bool()
                .copied()
                .expect("Expected column 20 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a EObjSheet {
    type Item = (u32, Vec<(u16, EObjRow)>);
    type IntoIter = StructuredSheetIterator<'a, EObjSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, EObjSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EObjRow {
    ///""
    pub Data: u32,
    ///""
    pub SgbPath: u16,
    ///""
    pub PopType: u8,
    ///""
    pub Invisibility: u8,
    ///""
    pub Unknown0: u8,
    ///"Used in EventFramework.CheckInteractRangeEx."
    pub AdditionalInteractRangeX: i8,
    ///"Used in EventFramework.CheckInteractRangeEx."
    pub AdditionalInteractRangeY: i8,
    ///""
    pub Unknown1: bool,
    ///""
    pub Unknown2: bool,
    ///""
    pub Unknown3: bool,
    ///""
    pub Unknown4: bool,
    ///""
    pub Unknown5: bool,
    ///""
    pub Unknown6: bool,
    ///""
    pub Unknown7: bool,
    ///""
    pub Unknown8: bool,
    ///""
    pub Targetable: bool,
    ///""
    pub Unknown9: bool,
    ///""
    pub Unknown10: bool,
    ///""
    pub Unknown11: bool,
    ///"If set, EObjs managed by a QuestEventHandler calls into Lua first to set up initial shared group timeline state. This can be used by Quest scripting to animate an SGB during a specific sequence of the quest."
    pub LuaSetup: bool,
    ///"Not sure what the true meaning of this field means, but its checked within EventObject.UpdateSharedTimelineState."
    pub SharedTimelineStateUnknown: bool,
}
