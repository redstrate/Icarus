//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    common::Language,
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
}
impl IndividualWeatherSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("IndividualWeather")?;
        let sheet = resolver.read_excel_sheet(&exh, "IndividualWeather", language)?;
        Ok(Self { sheet })
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
impl StructuredSheet for IndividualWeatherSheet {
    type Row = IndividualWeatherRow;
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
impl<'a> IntoIterator for &'a IndividualWeatherSheet {
    type Item = (u32, Vec<(u16, IndividualWeatherRow)>);
    type IntoIter = StructuredSheetIterator<'a, IndividualWeatherSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, IndividualWeatherSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct IndividualWeatherRow {
    columns: Vec<Field>,
}
impl IndividualWeatherRow {
    pub fn IndividualWeatherData<'a>(&'a self) -> [IndividualWeatherDataElement<'a>; 7] {
        [
            IndividualWeatherDataElement {
                Quest: &self.columns[0],
                Unknown0: &self.columns[1],
                Weather: &self.columns[2],
                Unknown1: &self.columns[3],
            },
            IndividualWeatherDataElement {
                Quest: &self.columns[4],
                Unknown0: &self.columns[5],
                Weather: &self.columns[6],
                Unknown1: &self.columns[7],
            },
            IndividualWeatherDataElement {
                Quest: &self.columns[8],
                Unknown0: &self.columns[9],
                Weather: &self.columns[10],
                Unknown1: &self.columns[11],
            },
            IndividualWeatherDataElement {
                Quest: &self.columns[12],
                Unknown0: &self.columns[13],
                Weather: &self.columns[14],
                Unknown1: &self.columns[15],
            },
            IndividualWeatherDataElement {
                Quest: &self.columns[16],
                Unknown0: &self.columns[17],
                Weather: &self.columns[18],
                Unknown1: &self.columns[19],
            },
            IndividualWeatherDataElement {
                Quest: &self.columns[20],
                Unknown0: &self.columns[21],
                Weather: &self.columns[22],
                Unknown1: &self.columns[23],
            },
            IndividualWeatherDataElement {
                Quest: &self.columns[24],
                Unknown0: &self.columns[25],
                Weather: &self.columns[26],
                Unknown1: &self.columns[27],
            },
        ]
    }
}
