//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
pub struct CalendarStructElement<'a> {
    pub Month: &'a Field,
    pub Day: &'a Field,
}
#[derive(Debug, Clone)]
pub struct CalendarSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl CalendarSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Calendar")?;
        let sheet = resolver.read_excel_sheet(&exh, "Calendar", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<CalendarRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<CalendarRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for CalendarSheet {
    type Row = CalendarRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a CalendarSheet {
    type Item = (u32, Vec<(u16, CalendarRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, CalendarSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, CalendarSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct CalendarRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> CalendarRow<'a> {
    pub fn CalendarStruct(&'a self) -> [CalendarStructElement<'a>; 32] {
        [
            CalendarStructElement {
                Month: &self.row.columns[self.index_mapping[0]],
                Day: &self.row.columns[self.index_mapping[1]],
            },
            CalendarStructElement {
                Month: &self.row.columns[self.index_mapping[2]],
                Day: &self.row.columns[self.index_mapping[3]],
            },
            CalendarStructElement {
                Month: &self.row.columns[self.index_mapping[4]],
                Day: &self.row.columns[self.index_mapping[5]],
            },
            CalendarStructElement {
                Month: &self.row.columns[self.index_mapping[6]],
                Day: &self.row.columns[self.index_mapping[7]],
            },
            CalendarStructElement {
                Month: &self.row.columns[self.index_mapping[8]],
                Day: &self.row.columns[self.index_mapping[9]],
            },
            CalendarStructElement {
                Month: &self.row.columns[self.index_mapping[10]],
                Day: &self.row.columns[self.index_mapping[11]],
            },
            CalendarStructElement {
                Month: &self.row.columns[self.index_mapping[12]],
                Day: &self.row.columns[self.index_mapping[13]],
            },
            CalendarStructElement {
                Month: &self.row.columns[self.index_mapping[14]],
                Day: &self.row.columns[self.index_mapping[15]],
            },
            CalendarStructElement {
                Month: &self.row.columns[self.index_mapping[16]],
                Day: &self.row.columns[self.index_mapping[17]],
            },
            CalendarStructElement {
                Month: &self.row.columns[self.index_mapping[18]],
                Day: &self.row.columns[self.index_mapping[19]],
            },
            CalendarStructElement {
                Month: &self.row.columns[self.index_mapping[20]],
                Day: &self.row.columns[self.index_mapping[21]],
            },
            CalendarStructElement {
                Month: &self.row.columns[self.index_mapping[22]],
                Day: &self.row.columns[self.index_mapping[23]],
            },
            CalendarStructElement {
                Month: &self.row.columns[self.index_mapping[24]],
                Day: &self.row.columns[self.index_mapping[25]],
            },
            CalendarStructElement {
                Month: &self.row.columns[self.index_mapping[26]],
                Day: &self.row.columns[self.index_mapping[27]],
            },
            CalendarStructElement {
                Month: &self.row.columns[self.index_mapping[28]],
                Day: &self.row.columns[self.index_mapping[29]],
            },
            CalendarStructElement {
                Month: &self.row.columns[self.index_mapping[30]],
                Day: &self.row.columns[self.index_mapping[31]],
            },
            CalendarStructElement {
                Month: &self.row.columns[self.index_mapping[32]],
                Day: &self.row.columns[self.index_mapping[33]],
            },
            CalendarStructElement {
                Month: &self.row.columns[self.index_mapping[34]],
                Day: &self.row.columns[self.index_mapping[35]],
            },
            CalendarStructElement {
                Month: &self.row.columns[self.index_mapping[36]],
                Day: &self.row.columns[self.index_mapping[37]],
            },
            CalendarStructElement {
                Month: &self.row.columns[self.index_mapping[38]],
                Day: &self.row.columns[self.index_mapping[39]],
            },
            CalendarStructElement {
                Month: &self.row.columns[self.index_mapping[40]],
                Day: &self.row.columns[self.index_mapping[41]],
            },
            CalendarStructElement {
                Month: &self.row.columns[self.index_mapping[42]],
                Day: &self.row.columns[self.index_mapping[43]],
            },
            CalendarStructElement {
                Month: &self.row.columns[self.index_mapping[44]],
                Day: &self.row.columns[self.index_mapping[45]],
            },
            CalendarStructElement {
                Month: &self.row.columns[self.index_mapping[46]],
                Day: &self.row.columns[self.index_mapping[47]],
            },
            CalendarStructElement {
                Month: &self.row.columns[self.index_mapping[48]],
                Day: &self.row.columns[self.index_mapping[49]],
            },
            CalendarStructElement {
                Month: &self.row.columns[self.index_mapping[50]],
                Day: &self.row.columns[self.index_mapping[51]],
            },
            CalendarStructElement {
                Month: &self.row.columns[self.index_mapping[52]],
                Day: &self.row.columns[self.index_mapping[53]],
            },
            CalendarStructElement {
                Month: &self.row.columns[self.index_mapping[54]],
                Day: &self.row.columns[self.index_mapping[55]],
            },
            CalendarStructElement {
                Month: &self.row.columns[self.index_mapping[56]],
                Day: &self.row.columns[self.index_mapping[57]],
            },
            CalendarStructElement {
                Month: &self.row.columns[self.index_mapping[58]],
                Day: &self.row.columns[self.index_mapping[59]],
            },
            CalendarStructElement {
                Month: &self.row.columns[self.index_mapping[60]],
                Day: &self.row.columns[self.index_mapping[61]],
            },
            CalendarStructElement {
                Month: &self.row.columns[self.index_mapping[62]],
                Day: &self.row.columns[self.index_mapping[63]],
            },
        ]
    }
}
