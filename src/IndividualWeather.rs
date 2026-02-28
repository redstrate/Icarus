//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
pub struct IndividualWeatherDataElement<'a> {
    pub Quest: &'a Field,
    pub Unknown0: &'a Field,
    pub Weather: &'a Field,
    pub Unknown1: &'a Field,
}
#[derive(Debug, Clone)]
pub struct IndividualWeatherSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl IndividualWeatherSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("IndividualWeather")?;
        let sheet = resolver.read_excel_sheet(&exh, "IndividualWeather", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<IndividualWeatherRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<IndividualWeatherRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for IndividualWeatherSheet {
    type Row = IndividualWeatherRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a IndividualWeatherSheet {
    type Item = (u32, Vec<(u16, IndividualWeatherRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, IndividualWeatherSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, IndividualWeatherSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct IndividualWeatherRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> IndividualWeatherRow<'a> {
    pub fn IndividualWeatherData(&'a self) -> [IndividualWeatherDataElement<'a>; 7] {
        [
            IndividualWeatherDataElement {
                Quest: &self.row.columns[self.index_mapping[0]],
                Unknown0: &self.row.columns[self.index_mapping[1]],
                Weather: &self.row.columns[self.index_mapping[2]],
                Unknown1: &self.row.columns[self.index_mapping[3]],
            },
            IndividualWeatherDataElement {
                Quest: &self.row.columns[self.index_mapping[4]],
                Unknown0: &self.row.columns[self.index_mapping[5]],
                Weather: &self.row.columns[self.index_mapping[6]],
                Unknown1: &self.row.columns[self.index_mapping[7]],
            },
            IndividualWeatherDataElement {
                Quest: &self.row.columns[self.index_mapping[8]],
                Unknown0: &self.row.columns[self.index_mapping[9]],
                Weather: &self.row.columns[self.index_mapping[10]],
                Unknown1: &self.row.columns[self.index_mapping[11]],
            },
            IndividualWeatherDataElement {
                Quest: &self.row.columns[self.index_mapping[12]],
                Unknown0: &self.row.columns[self.index_mapping[13]],
                Weather: &self.row.columns[self.index_mapping[14]],
                Unknown1: &self.row.columns[self.index_mapping[15]],
            },
            IndividualWeatherDataElement {
                Quest: &self.row.columns[self.index_mapping[16]],
                Unknown0: &self.row.columns[self.index_mapping[17]],
                Weather: &self.row.columns[self.index_mapping[18]],
                Unknown1: &self.row.columns[self.index_mapping[19]],
            },
            IndividualWeatherDataElement {
                Quest: &self.row.columns[self.index_mapping[20]],
                Unknown0: &self.row.columns[self.index_mapping[21]],
                Weather: &self.row.columns[self.index_mapping[22]],
                Unknown1: &self.row.columns[self.index_mapping[23]],
            },
            IndividualWeatherDataElement {
                Quest: &self.row.columns[self.index_mapping[24]],
                Unknown0: &self.row.columns[self.index_mapping[25]],
                Weather: &self.row.columns[self.index_mapping[26]],
                Unknown1: &self.row.columns[self.index_mapping[27]],
            },
        ]
    }
}
