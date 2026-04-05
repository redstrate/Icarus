//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
pub struct UnknownStructElement {
    pub Unknown1: u32,
    pub Unknown_70: u32,
    pub Unknown2: u8,
}
#[derive(Debug, Clone)]
pub struct QuestEffectSheet {
    sheet: Sheet,
}
impl QuestEffectSheet {
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
impl<'a> StructuredSheet<'a> for QuestEffectSheet {
    type Row = QuestEffectRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a QuestEffectSheet {
    type Item = (u32, Vec<(u16, QuestEffectRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, QuestEffectSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, QuestEffectSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct QuestEffectRow<'a> {
    row: &'a Row,
}
impl<'a> QuestEffectRow<'a> {
    pub fn UnknownStruct(&'a self) -> [UnknownStructElement; 4] {
        [
            UnknownStructElement {
                Unknown1: self.row.columns[4].into_u32().copied().unwrap(),
                Unknown_70: self.row.columns[8].into_u32().copied().unwrap(),
                Unknown2: self.row.columns[0].into_u8().copied().unwrap(),
            },
            UnknownStructElement {
                Unknown1: self.row.columns[5].into_u32().copied().unwrap(),
                Unknown_70: self.row.columns[9].into_u32().copied().unwrap(),
                Unknown2: self.row.columns[1].into_u8().copied().unwrap(),
            },
            UnknownStructElement {
                Unknown1: self.row.columns[6].into_u32().copied().unwrap(),
                Unknown_70: self.row.columns[10].into_u32().copied().unwrap(),
                Unknown2: self.row.columns[2].into_u8().copied().unwrap(),
            },
            UnknownStructElement {
                Unknown1: self.row.columns[7].into_u32().copied().unwrap(),
                Unknown_70: self.row.columns[11].into_u32().copied().unwrap(),
                Unknown2: self.row.columns[3].into_u8().copied().unwrap(),
            },
        ]
    }
    pub fn Unknown8(&'a self) -> u32 {
        self.row.columns[12].into_u32().copied().unwrap()
    }
    pub fn Unknown9(&'a self) -> u32 {
        self.row.columns[13].into_u32().copied().unwrap()
    }
    pub fn Unknown_70(&'a self) -> bool {
        self.row.columns[14].into_bool().copied().unwrap()
    }
}
