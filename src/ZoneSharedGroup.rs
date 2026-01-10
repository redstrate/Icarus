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
        let column_defs = &self.sheet.exh.column_definitions;
        let mut zipped: Vec<_> = row
            .columns
            .clone()
            .into_iter()
            .zip(column_defs)
            .collect();
        zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
        let (columns, _): (Vec<Field>, Vec<ExcelColumnDefinition>) = zipped
            .into_iter()
            .unzip();
        Some(Self::Row { columns })
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
#[derive(Debug, Clone)]
pub struct ZoneSharedGroupRow {
    columns: Vec<Field>,
}
impl ZoneSharedGroupRow {
    pub fn LGBSharedGroup<'a>(&'a self) -> &'a Field {
        &self.columns[0]
    }
    pub fn RequirementRow<'a>(&'a self) -> [&'a Field; 6] {
        [
            &self.columns[1],
            &self.columns[2],
            &self.columns[3],
            &self.columns[4],
            &self.columns[5],
            &self.columns[6],
        ]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a Field {
        &self.columns[7]
    }
    pub fn RequirementQuestSequence<'a>(&'a self) -> [&'a Field; 6] {
        [
            &self.columns[8],
            &self.columns[9],
            &self.columns[10],
            &self.columns[11],
            &self.columns[12],
            &self.columns[13],
        ]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a Field {
        &self.columns[14]
    }
    /// 1 = Quest
    /// 2 = Quest with specific Sequence
    /// 3 = AetherCurrent
    /// 4 = EurekaStoryProgress
    /// 5 = DomaStoryProgress
    ///
    pub fn RequirementType<'a>(&'a self) -> [&'a Field; 6] {
        [
            &self.columns[15],
            &self.columns[16],
            &self.columns[17],
            &self.columns[18],
            &self.columns[19],
            &self.columns[20],
        ]
    }
    pub fn Unknown8<'a>(&'a self) -> &'a Field {
        &self.columns[21]
    }
    pub fn Unknown9<'a>(&'a self) -> &'a Field {
        &self.columns[22]
    }
    pub fn Unknown10<'a>(&'a self) -> &'a Field {
        &self.columns[23]
    }
    pub fn Unknown11<'a>(&'a self) -> &'a Field {
        &self.columns[24]
    }
    pub fn Unknown12<'a>(&'a self) -> &'a Field {
        &self.columns[25]
    }
    pub fn Unknown13<'a>(&'a self) -> &'a Field {
        &self.columns[26]
    }
    pub fn Unknown14<'a>(&'a self) -> &'a Field {
        &self.columns[27]
    }
    pub fn Unknown15<'a>(&'a self) -> &'a Field {
        &self.columns[28]
    }
}
