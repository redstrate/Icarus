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
pub struct AirshipExplorationPointSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl AirshipExplorationPointSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("AirshipExplorationPoint")?;
        let sheet = resolver
            .read_excel_sheet(&exh, "AirshipExplorationPoint", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<AirshipExplorationPointRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<AirshipExplorationPointRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for AirshipExplorationPointSheet {
    type Row = AirshipExplorationPointRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a AirshipExplorationPointSheet {
    type Item = (u32, Vec<(u16, AirshipExplorationPointRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, AirshipExplorationPointSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, AirshipExplorationPointSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct AirshipExplorationPointRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> AirshipExplorationPointRow<'a> {
    pub fn Name(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[0]]
    }
    pub fn NameShort(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1]]
    }
    pub fn ExpReward(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[2]]
    }
    pub fn CeruleumTankReq(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[3]]
    }
    pub fn SurveyDurationmin(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[4]]
    }
    pub fn SurveyDistance(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[5]]
    }
    pub fn X(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[6]]
    }
    pub fn Y(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[7]]
    }
    pub fn RankReq(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[8]]
    }
    pub fn Unknown0(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[9]]
    }
    pub fn Unknown1(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[10]]
    }
    pub fn SurveillanceReq(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[11]]
    }
    pub fn Unknown2(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[12]]
    }
    pub fn Passengers(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[13]]
    }
}
