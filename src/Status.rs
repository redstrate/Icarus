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
pub struct StatusSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl StatusSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Status")?;
        let sheet = resolver.read_excel_sheet(&exh, "Status", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<StatusRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<StatusRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for StatusSheet {
    type Row = StatusRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a StatusSheet {
    type Item = (u32, Vec<(u16, StatusRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, StatusSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, StatusSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct StatusRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> StatusRow<'a> {
    pub fn Name(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[0]]
    }
    pub fn Description(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1]]
    }
    pub fn Icon(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[2]]
    }
    pub fn ParamModifier(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[3]]
    }
    pub fn VFX(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[4]]
    }
    pub fn Log(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[5]]
    }
    pub fn Unknown0(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[6]]
    }
    pub fn MaxStacks(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[7]]
    }
    pub fn ClassJobCategory(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[8]]
    }
    pub fn StatusCategory(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[9]]
    }
    pub fn HitEffect(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[10]]
    }
    pub fn PartyListPriority(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[11]]
    }
    pub fn CanIncreaseRewards(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[12]]
    }
    pub fn ParamEffect(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[13]]
    }
    pub fn TargetType(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[14]]
    }
    /// actually an index of the flag
    pub fn Flags(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[15]]
    }
    pub fn Flag2(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[16]]
    }
    pub fn Unknown_70_1(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[17]]
    }
    pub fn Unknown2(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[18]]
    }
    pub fn LockMovement(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[19]]
    }
    pub fn Unknown3(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[20]]
    }
    pub fn LockActions(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[21]]
    }
    pub fn LockControl(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[22]]
    }
    pub fn Transfiguration(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[23]]
    }
    pub fn IsGaze(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[24]]
    }
    pub fn CanDispel(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[25]]
    }
    pub fn InflictedByActor(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[26]]
    }
    pub fn IsPermanent(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[27]]
    }
    pub fn NoLogVfx(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[28]]
    }
    pub fn Unknown5(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[29]]
    }
    pub fn CanStatusOff(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[30]]
    }
    pub fn IsFcBuff(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[31]]
    }
    pub fn Invisibility(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[32]]
    }
    pub fn Unknown6(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[33]]
    }
    pub fn Unknown_70_2(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[34]]
    }
    pub fn Unknown7(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[35]]
    }
}
