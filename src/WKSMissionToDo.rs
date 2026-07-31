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
pub struct WKSMissionToDoSheet {
    sheet: Sheet,
}
impl WKSMissionToDoSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("WKSMissionToDo")?;
        let sheet = resolver.read_excel_sheet(&exh, "WKSMissionToDo", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<WKSMissionToDoRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<WKSMissionToDoRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for WKSMissionToDoSheet {
    type Row = WKSMissionToDoRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            TemporaryAction: row
                .columns[0]
                .into_u32()
                .copied()
                .expect("Expected column 0 to be a uint32!"),
            BronzeScore: row
                .columns[3]
                .into_u32()
                .copied()
                .expect("Expected column 3 to be a uint32!"),
            Unknown1: row
                .columns[13]
                .into_u32()
                .copied()
                .expect("Expected column 13 to be a uint32!"),
            RequiredItem: [
                row
                    .columns[4]
                    .into_u16()
                    .copied()
                    .expect("Expected column 4 to be a uint16!"),
                row
                    .columns[6]
                    .into_u16()
                    .copied()
                    .expect("Expected column 6 to be a uint16!"),
                row
                    .columns[8]
                    .into_u16()
                    .copied()
                    .expect("Expected column 8 to be a uint16!"),
            ],
            RequiredItemQuantity: [
                row
                    .columns[5]
                    .into_u16()
                    .copied()
                    .expect("Expected column 5 to be a uint16!"),
                row
                    .columns[7]
                    .into_u16()
                    .copied()
                    .expect("Expected column 7 to be a uint16!"),
                row
                    .columns[9]
                    .into_u16()
                    .copied()
                    .expect("Expected column 9 to be a uint16!"),
            ],
            RequiredFishVariety: row
                .columns[11]
                .into_u16()
                .copied()
                .expect("Expected column 11 to be a uint16!"),
            Unknown10: row
                .columns[15]
                .into_u16()
                .copied()
                .expect("Expected column 15 to be a uint16!"),
            Unknown11: row
                .columns[16]
                .into_u16()
                .copied()
                .expect("Expected column 16 to be a uint16!"),
            Unknown12: row
                .columns[17]
                .into_u16()
                .copied()
                .expect("Expected column 17 to be a uint16!"),
            MapMarker: row
                .columns[18]
                .into_u16()
                .copied()
                .expect("Expected column 18 to be a uint16!"),
            Unknown18: row
                .columns[19]
                .into_u16()
                .copied()
                .expect("Expected column 19 to be a uint16!"),
            Unknown19: row
                .columns[20]
                .into_u16()
                .copied()
                .expect("Expected column 20 to be a uint16!"),
            WKSMissionText: row
                .columns[21]
                .into_u16()
                .copied()
                .expect("Expected column 21 to be a uint16!"),
            TemporaryActionCount: row
                .columns[1]
                .into_u8()
                .copied()
                .expect("Expected column 1 to be a uint8!"),
            Unknown15: row
                .columns[2]
                .into_u8()
                .copied()
                .expect("Expected column 2 to be a uint8!"),
            Unknown16: row
                .columns[10]
                .into_u8()
                .copied()
                .expect("Expected column 10 to be a uint8!"),
            RequiredFishQuantity: row
                .columns[12]
                .into_u8()
                .copied()
                .expect("Expected column 12 to be a uint8!"),
            MissionType: row
                .columns[14]
                .into_u8()
                .copied()
                .expect("Expected column 14 to be a uint8!"),
        })
    }
}
impl<'a> IntoIterator for &'a WKSMissionToDoSheet {
    type Item = (u32, Vec<(u16, WKSMissionToDoRow)>);
    type IntoIter = StructuredSheetIterator<'a, WKSMissionToDoSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, WKSMissionToDoSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct WKSMissionToDoRow {
    ///""
    pub TemporaryAction: u32,
    ///""
    pub BronzeScore: u32,
    ///""
    pub Unknown1: u32,
    ///""
    pub RequiredItem: [u16; 3],
    ///""
    pub RequiredItemQuantity: [u16; 3],
    ///""
    pub RequiredFishVariety: u16,
    ///""
    pub Unknown10: u16,
    ///""
    pub Unknown11: u16,
    ///""
    pub Unknown12: u16,
    ///""
    pub MapMarker: u16,
    ///""
    pub Unknown18: u16,
    ///""
    pub Unknown19: u16,
    ///""
    pub WKSMissionText: u16,
    ///""
    pub TemporaryActionCount: u8,
    ///""
    pub Unknown15: u8,
    ///""
    pub Unknown16: u8,
    ///""
    pub RequiredFishQuantity: u8,
    ///""
    pub MissionType: u8,
}
