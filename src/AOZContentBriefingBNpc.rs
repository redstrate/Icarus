//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    common::Language,
};
#[derive(Debug, Clone)]
pub struct AOZContentBriefingBNpcSheet {
    sheet: Sheet,
}
impl AOZContentBriefingBNpcSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("AOZContentBriefingBNpc")?;
        let sheet = resolver.read_excel_sheet(&exh, "AOZContentBriefingBNpc", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<AOZContentBriefingBNpcRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<AOZContentBriefingBNpcRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for AOZContentBriefingBNpcSheet {
    type Row = AOZContentBriefingBNpcRow;
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
impl<'a> IntoIterator for &'a AOZContentBriefingBNpcSheet {
    type Item = (u32, Vec<(u16, AOZContentBriefingBNpcRow)>);
    type IntoIter = StructuredSheetIterator<'a, AOZContentBriefingBNpcSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, AOZContentBriefingBNpcSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct AOZContentBriefingBNpcRow {
    columns: Vec<Field>,
}
impl AOZContentBriefingBNpcRow {
    pub fn BNpcName<'a>(&'a self) -> &'a Field {
        &self.columns[0]
    }
    pub fn TargetSmall<'a>(&'a self) -> &'a Field {
        &self.columns[1]
    }
    pub fn TargetLarge<'a>(&'a self) -> &'a Field {
        &self.columns[2]
    }
    pub fn Endurance<'a>(&'a self) -> &'a Field {
        &self.columns[3]
    }
    pub fn Fire<'a>(&'a self) -> &'a Field {
        &self.columns[4]
    }
    pub fn Ice<'a>(&'a self) -> &'a Field {
        &self.columns[5]
    }
    pub fn Wind<'a>(&'a self) -> &'a Field {
        &self.columns[6]
    }
    pub fn Earth<'a>(&'a self) -> &'a Field {
        &self.columns[7]
    }
    pub fn Thunder<'a>(&'a self) -> &'a Field {
        &self.columns[8]
    }
    pub fn Water<'a>(&'a self) -> &'a Field {
        &self.columns[9]
    }
    pub fn Slashing<'a>(&'a self) -> &'a Field {
        &self.columns[10]
    }
    pub fn Piercing<'a>(&'a self) -> &'a Field {
        &self.columns[11]
    }
    pub fn Blunt<'a>(&'a self) -> &'a Field {
        &self.columns[12]
    }
    pub fn Magic<'a>(&'a self) -> &'a Field {
        &self.columns[13]
    }
    pub fn HideStats<'a>(&'a self) -> &'a Field {
        &self.columns[14]
    }
    pub fn SlowVuln<'a>(&'a self) -> &'a Field {
        &self.columns[15]
    }
    pub fn PetrificationVuln<'a>(&'a self) -> &'a Field {
        &self.columns[16]
    }
    pub fn ParalysisVuln<'a>(&'a self) -> &'a Field {
        &self.columns[17]
    }
    pub fn InterruptionVuln<'a>(&'a self) -> &'a Field {
        &self.columns[18]
    }
    pub fn BlindVuln<'a>(&'a self) -> &'a Field {
        &self.columns[19]
    }
    pub fn StunVuln<'a>(&'a self) -> &'a Field {
        &self.columns[20]
    }
    pub fn SleepVuln<'a>(&'a self) -> &'a Field {
        &self.columns[21]
    }
    pub fn BindVuln<'a>(&'a self) -> &'a Field {
        &self.columns[22]
    }
    pub fn HeavyVuln<'a>(&'a self) -> &'a Field {
        &self.columns[23]
    }
    pub fn FlatOrDeathVuln<'a>(&'a self) -> &'a Field {
        &self.columns[24]
    }
}
