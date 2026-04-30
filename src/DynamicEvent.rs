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
pub struct DynamicEventSheet {
    sheet: Sheet,
}
impl DynamicEventSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("DynamicEvent")?;
        let sheet = resolver.read_excel_sheet(&exh, "DynamicEvent", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<DynamicEventRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<DynamicEventRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for DynamicEventSheet {
    type Row = DynamicEventRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Name: row
                .columns[11]
                .into_string()
                .cloned()
                .expect("Expected column 11 to be a string!"),
            Description: row
                .columns[12]
                .into_string()
                .cloned()
                .expect("Expected column 12 to be a string!"),
            LGBEventObject: row
                .columns[5]
                .into_u32()
                .copied()
                .expect("Expected column 5 to be a uint32!"),
            LGBMapRange: row
                .columns[6]
                .into_u32()
                .copied()
                .expect("Expected column 6 to be a uint32!"),
            Quest: row
                .columns[7]
                .into_u32()
                .copied()
                .expect("Expected column 7 to be a uint32!"),
            Announce: row
                .columns[10]
                .into_u32()
                .copied()
                .expect("Expected column 10 to be a uint32!"),
            IconObjective0: row
                .columns[16]
                .into_u32()
                .copied()
                .expect("Expected column 16 to be a uint32!"),
            IconObjective1: row
                .columns[17]
                .into_u32()
                .copied()
                .expect("Expected column 17 to be a uint32!"),
            Unknown6: row
                .columns[13]
                .into_i16()
                .copied()
                .expect("Expected column 13 to be a int16!"),
            Unknown7: row
                .columns[14]
                .into_i16()
                .copied()
                .expect("Expected column 14 to be a int16!"),
            Unknown2: row
                .columns[15]
                .into_i16()
                .copied()
                .expect("Expected column 15 to be a int16!"),
            EventType: row
                .columns[0]
                .into_u8()
                .copied()
                .expect("Expected column 0 to be a uint8!"),
            EnemyType: row
                .columns[1]
                .into_u8()
                .copied()
                .expect("Expected column 1 to be a uint8!"),
            MaxParticipants: row
                .columns[2]
                .into_u8()
                .copied()
                .expect("Expected column 2 to be a uint8!"),
            Duration: row
                .columns[3]
                .into_u8()
                .copied()
                .expect("Expected column 3 to be a uint8!"),
            Unknown5: row
                .columns[8]
                .into_u8()
                .copied()
                .expect("Expected column 8 to be a uint8!"),
            SingleBattle: row
                .columns[9]
                .into_u8()
                .copied()
                .expect("Expected column 9 to be a uint8!"),
            Unknown9: row
                .columns[18]
                .into_u8()
                .copied()
                .expect("Expected column 18 to be a uint8!"),
            Unknown8: row
                .columns[4]
                .into_bool()
                .copied()
                .expect("Expected column 4 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a DynamicEventSheet {
    type Item = (u32, Vec<(u16, DynamicEventRow)>);
    type IntoIter = StructuredSheetIterator<'a, DynamicEventSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, DynamicEventSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DynamicEventRow {
    ///""
    pub Name: String,
    ///""
    pub Description: String,
    ///""
    pub LGBEventObject: u32,
    ///""
    pub LGBMapRange: u32,
    ///""
    pub Quest: u32,
    ///""
    pub Announce: u32,
    ///""
    pub IconObjective0: u32,
    ///""
    pub IconObjective1: u32,
    ///""
    pub Unknown6: i16,
    ///""
    pub Unknown7: i16,
    ///""
    pub Unknown2: i16,
    ///""
    pub EventType: u8,
    ///""
    pub EnemyType: u8,
    ///""
    pub MaxParticipants: u8,
    ///"In minutes."
    pub Duration: u8,
    ///""
    pub Unknown5: u8,
    ///""
    pub SingleBattle: u8,
    ///""
    pub Unknown9: u8,
    ///""
    pub Unknown8: bool,
}
