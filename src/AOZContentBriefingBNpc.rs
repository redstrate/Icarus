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
pub struct AOZContentBriefingBNpcSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl AOZContentBriefingBNpcSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("AOZContentBriefingBNpc")?;
        let sheet = resolver.read_excel_sheet(&exh, "AOZContentBriefingBNpc", language)?;
        let mut index_mapping: Vec<(usize, &ExcelColumnDefinition)> = sheet
            .exh
            .column_definitions
            .iter()
            .enumerate()
            .collect();
        index_mapping.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
        let index_mapping: Vec<usize> = index_mapping
            .iter()
            .map(|(index, _)| *index)
            .collect();
        Ok(Self { sheet, index_mapping })
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
impl<'a> StructuredSheet<'a> for AOZContentBriefingBNpcSheet {
    type Row = AOZContentBriefingBNpcRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a AOZContentBriefingBNpcSheet {
    type Item = (u32, Vec<(u16, AOZContentBriefingBNpcRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, AOZContentBriefingBNpcSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, AOZContentBriefingBNpcSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct AOZContentBriefingBNpcRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> AOZContentBriefingBNpcRow<'a> {
    pub fn BNpcName(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[0]]
    }
    pub fn TargetSmall(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1]]
    }
    pub fn TargetLarge(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[2]]
    }
    pub fn Endurance(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[3]]
    }
    pub fn Fire(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[4]]
    }
    pub fn Ice(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[5]]
    }
    pub fn Wind(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[6]]
    }
    pub fn Earth(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[7]]
    }
    pub fn Thunder(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[8]]
    }
    pub fn Water(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[9]]
    }
    pub fn Slashing(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[10]]
    }
    pub fn Piercing(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[11]]
    }
    pub fn Blunt(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[12]]
    }
    pub fn Magic(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[13]]
    }
    pub fn HideStats(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[14]]
    }
    pub fn SlowVuln(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[15]]
    }
    pub fn PetrificationVuln(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[16]]
    }
    pub fn ParalysisVuln(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[17]]
    }
    pub fn InterruptionVuln(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[18]]
    }
    pub fn BlindVuln(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[19]]
    }
    pub fn StunVuln(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[20]]
    }
    pub fn SleepVuln(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[21]]
    }
    pub fn BindVuln(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[22]]
    }
    pub fn HeavyVuln(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[23]]
    }
    pub fn FlatOrDeathVuln(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[24]]
    }
}
