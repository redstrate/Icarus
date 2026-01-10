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
}
impl CalendarSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Calendar")?;
        let sheet = resolver.read_excel_sheet(&exh, "Calendar", language)?;
        Ok(Self { sheet })
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
impl StructuredSheet for CalendarSheet {
    type Row = CalendarRow;
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
impl<'a> IntoIterator for &'a CalendarSheet {
    type Item = (u32, Vec<(u16, CalendarRow)>);
    type IntoIter = StructuredSheetIterator<'a, CalendarSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, CalendarSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct CalendarRow {
    columns: Vec<Field>,
}
impl CalendarRow {
    pub fn CalendarStruct<'a>(&'a self) -> [CalendarStructElement<'a>; 32] {
        [
            CalendarStructElement {
                Month: &self.columns[0],
                Day: &self.columns[1],
            },
            CalendarStructElement {
                Month: &self.columns[2],
                Day: &self.columns[3],
            },
            CalendarStructElement {
                Month: &self.columns[4],
                Day: &self.columns[5],
            },
            CalendarStructElement {
                Month: &self.columns[6],
                Day: &self.columns[7],
            },
            CalendarStructElement {
                Month: &self.columns[8],
                Day: &self.columns[9],
            },
            CalendarStructElement {
                Month: &self.columns[10],
                Day: &self.columns[11],
            },
            CalendarStructElement {
                Month: &self.columns[12],
                Day: &self.columns[13],
            },
            CalendarStructElement {
                Month: &self.columns[14],
                Day: &self.columns[15],
            },
            CalendarStructElement {
                Month: &self.columns[16],
                Day: &self.columns[17],
            },
            CalendarStructElement {
                Month: &self.columns[18],
                Day: &self.columns[19],
            },
            CalendarStructElement {
                Month: &self.columns[20],
                Day: &self.columns[21],
            },
            CalendarStructElement {
                Month: &self.columns[22],
                Day: &self.columns[23],
            },
            CalendarStructElement {
                Month: &self.columns[24],
                Day: &self.columns[25],
            },
            CalendarStructElement {
                Month: &self.columns[26],
                Day: &self.columns[27],
            },
            CalendarStructElement {
                Month: &self.columns[28],
                Day: &self.columns[29],
            },
            CalendarStructElement {
                Month: &self.columns[30],
                Day: &self.columns[31],
            },
            CalendarStructElement {
                Month: &self.columns[32],
                Day: &self.columns[33],
            },
            CalendarStructElement {
                Month: &self.columns[34],
                Day: &self.columns[35],
            },
            CalendarStructElement {
                Month: &self.columns[36],
                Day: &self.columns[37],
            },
            CalendarStructElement {
                Month: &self.columns[38],
                Day: &self.columns[39],
            },
            CalendarStructElement {
                Month: &self.columns[40],
                Day: &self.columns[41],
            },
            CalendarStructElement {
                Month: &self.columns[42],
                Day: &self.columns[43],
            },
            CalendarStructElement {
                Month: &self.columns[44],
                Day: &self.columns[45],
            },
            CalendarStructElement {
                Month: &self.columns[46],
                Day: &self.columns[47],
            },
            CalendarStructElement {
                Month: &self.columns[48],
                Day: &self.columns[49],
            },
            CalendarStructElement {
                Month: &self.columns[50],
                Day: &self.columns[51],
            },
            CalendarStructElement {
                Month: &self.columns[52],
                Day: &self.columns[53],
            },
            CalendarStructElement {
                Month: &self.columns[54],
                Day: &self.columns[55],
            },
            CalendarStructElement {
                Month: &self.columns[56],
                Day: &self.columns[57],
            },
            CalendarStructElement {
                Month: &self.columns[58],
                Day: &self.columns[59],
            },
            CalendarStructElement {
                Month: &self.columns[60],
                Day: &self.columns[61],
            },
            CalendarStructElement {
                Month: &self.columns[62],
                Day: &self.columns[63],
            },
        ]
    }
}
