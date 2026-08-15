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
pub struct ZoneSharedGroupSheet {
    sheet: Sheet,
}
impl ZoneSharedGroupSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("ZoneSharedGroup")?;
        let sheet = resolver.read_excel_sheet(&exh, "ZoneSharedGroup", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<ZoneSharedGroupRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<ZoneSharedGroupRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for ZoneSharedGroupSheet {
    type Row = ZoneSharedGroupRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            LGBSharedGroup: row
                .columns[0]
                .into_u32()
                .copied()
                .expect("Expected column 0 to be a uint32!"),
            RequirementRow: [
                row
                    .columns[2]
                    .into_u32()
                    .copied()
                    .expect("Expected column 2 to be a uint32!"),
                row
                    .columns[6]
                    .into_u32()
                    .copied()
                    .expect("Expected column 6 to be a uint32!"),
                row
                    .columns[10]
                    .into_u32()
                    .copied()
                    .expect("Expected column 10 to be a uint32!"),
                row
                    .columns[14]
                    .into_u32()
                    .copied()
                    .expect("Expected column 14 to be a uint32!"),
                row
                    .columns[18]
                    .into_u32()
                    .copied()
                    .expect("Expected column 18 to be a uint32!"),
                row
                    .columns[22]
                    .into_u32()
                    .copied()
                    .expect("Expected column 22 to be a uint32!"),
            ],
            Unknown0: row
                .columns[26]
                .into_u32()
                .copied()
                .expect("Expected column 26 to be a uint32!"),
            RequirementQuestSequence: [
                row
                    .columns[3]
                    .into_u32()
                    .copied()
                    .expect("Expected column 3 to be a uint32!"),
                row
                    .columns[7]
                    .into_u32()
                    .copied()
                    .expect("Expected column 7 to be a uint32!"),
                row
                    .columns[11]
                    .into_u32()
                    .copied()
                    .expect("Expected column 11 to be a uint32!"),
                row
                    .columns[15]
                    .into_u32()
                    .copied()
                    .expect("Expected column 15 to be a uint32!"),
                row
                    .columns[19]
                    .into_u32()
                    .copied()
                    .expect("Expected column 19 to be a uint32!"),
                row
                    .columns[23]
                    .into_u32()
                    .copied()
                    .expect("Expected column 23 to be a uint32!"),
            ],
            Unknown1: row
                .columns[27]
                .into_u32()
                .copied()
                .expect("Expected column 27 to be a uint32!"),
            RequirementType: [
                row
                    .columns[1]
                    .into_u8()
                    .copied()
                    .expect("Expected column 1 to be a uint8!"),
                row
                    .columns[5]
                    .into_u8()
                    .copied()
                    .expect("Expected column 5 to be a uint8!"),
                row
                    .columns[9]
                    .into_u8()
                    .copied()
                    .expect("Expected column 9 to be a uint8!"),
                row
                    .columns[13]
                    .into_u8()
                    .copied()
                    .expect("Expected column 13 to be a uint8!"),
                row
                    .columns[17]
                    .into_u8()
                    .copied()
                    .expect("Expected column 17 to be a uint8!"),
                row
                    .columns[21]
                    .into_u8()
                    .copied()
                    .expect("Expected column 21 to be a uint8!"),
            ],
            Unknown8: row
                .columns[25]
                .into_u8()
                .copied()
                .expect("Expected column 25 to be a uint8!"),
            Unknown9: row
                .columns[4]
                .into_bool()
                .copied()
                .expect("Expected column 4 to be a bool!"),
            Unknown10: row
                .columns[8]
                .into_bool()
                .copied()
                .expect("Expected column 8 to be a bool!"),
            Unknown11: row
                .columns[12]
                .into_bool()
                .copied()
                .expect("Expected column 12 to be a bool!"),
            Unknown12: row
                .columns[16]
                .into_bool()
                .copied()
                .expect("Expected column 16 to be a bool!"),
            Unknown13: row
                .columns[20]
                .into_bool()
                .copied()
                .expect("Expected column 20 to be a bool!"),
            Unknown14: row
                .columns[24]
                .into_bool()
                .copied()
                .expect("Expected column 24 to be a bool!"),
            Unknown15: row
                .columns[28]
                .into_bool()
                .copied()
                .expect("Expected column 28 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a ZoneSharedGroupSheet {
    type Item = (u32, Vec<(u16, ZoneSharedGroupRow)>);
    type IntoIter = StructuredSheetIterator<'a, ZoneSharedGroupSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, ZoneSharedGroupSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ZoneSharedGroupRow {
    ///""
    pub LGBSharedGroup: u32,
    ///""
    pub RequirementRow: [u32; 6],
    ///""
    pub Unknown0: u32,
    ///""
    pub RequirementQuestSequence: [u32; 6],
    ///""
    pub Unknown1: u32,
    ///"1 = Quest\n /// 2 = Quest with specific Sequence\n /// 3 = AetherCurrent\n /// 4 = EurekaStoryProgress\n /// 5 = DomaStoryProgress\n /// "
    pub RequirementType: [u8; 6],
    ///""
    pub Unknown8: u8,
    ///""
    pub Unknown9: bool,
    ///""
    pub Unknown10: bool,
    ///""
    pub Unknown11: bool,
    ///""
    pub Unknown12: bool,
    ///""
    pub Unknown13: bool,
    ///""
    pub Unknown14: bool,
    ///""
    pub Unknown15: bool,
}
