//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
#[derive(Clone, Debug, PartialEq)]
pub struct EffectsElement {
    pub Param1: u32,
    pub Param2: u32,
    pub Type: u8,
}
#[derive(Debug, Clone)]
pub struct QuestEffectSheet {
    sheet: Sheet,
}
impl QuestEffectSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("QuestEffect")?;
        let sheet = resolver.read_excel_sheet(&exh, "QuestEffect", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<QuestEffectRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<QuestEffectRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for QuestEffectSheet {
    type Row = QuestEffectRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Effects: [
                EffectsElement {
                    Param1: row
                        .columns[4]
                        .into_u32()
                        .copied()
                        .expect("Expected column 4 to be a uint32!"),
                    Param2: row
                        .columns[8]
                        .into_u32()
                        .copied()
                        .expect("Expected column 8 to be a uint32!"),
                    Type: row
                        .columns[0]
                        .into_u8()
                        .copied()
                        .expect("Expected column 0 to be a uint8!"),
                },
                EffectsElement {
                    Param1: row
                        .columns[5]
                        .into_u32()
                        .copied()
                        .expect("Expected column 5 to be a uint32!"),
                    Param2: row
                        .columns[9]
                        .into_u32()
                        .copied()
                        .expect("Expected column 9 to be a uint32!"),
                    Type: row
                        .columns[1]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1 to be a uint8!"),
                },
                EffectsElement {
                    Param1: row
                        .columns[6]
                        .into_u32()
                        .copied()
                        .expect("Expected column 6 to be a uint32!"),
                    Param2: row
                        .columns[10]
                        .into_u32()
                        .copied()
                        .expect("Expected column 10 to be a uint32!"),
                    Type: row
                        .columns[2]
                        .into_u8()
                        .copied()
                        .expect("Expected column 2 to be a uint8!"),
                },
                EffectsElement {
                    Param1: row
                        .columns[7]
                        .into_u32()
                        .copied()
                        .expect("Expected column 7 to be a uint32!"),
                    Param2: row
                        .columns[11]
                        .into_u32()
                        .copied()
                        .expect("Expected column 11 to be a uint32!"),
                    Type: row
                        .columns[3]
                        .into_u8()
                        .copied()
                        .expect("Expected column 3 to be a uint8!"),
                },
            ],
            OutOfRangeWarningMessage: row
                .columns[12]
                .into_u32()
                .copied()
                .expect("Expected column 12 to be a uint32!"),
            OutOfRangeMessage: row
                .columns[13]
                .into_u32()
                .copied()
                .expect("Expected column 13 to be a uint32!"),
            Unknown_70: row
                .columns[14]
                .into_bool()
                .copied()
                .expect("Expected column 14 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a QuestEffectSheet {
    type Item = (u32, Vec<(u16, QuestEffectRow)>);
    type IntoIter = StructuredSheetIterator<'a, QuestEffectSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, QuestEffectSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct QuestEffectRow {
    ///""
    pub Effects: [EffectsElement; 4],
    ///""
    pub OutOfRangeWarningMessage: u32,
    ///""
    pub OutOfRangeMessage: u32,
    ///""
    pub Unknown_70: bool,
}
