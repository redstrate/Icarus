//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
pub struct OpenContentDataElement<'a> {
    pub CandidateName: &'a Field,
    pub Content: &'a Field,
}
#[derive(Debug, Clone)]
pub struct OpenContentSheet {
    sheet: Sheet,
}
impl OpenContentSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("OpenContent")?;
        let sheet = resolver.read_excel_sheet(&exh, "OpenContent", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<OpenContentRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<OpenContentRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for OpenContentSheet {
    type Row = OpenContentRow;
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
impl<'a> IntoIterator for &'a OpenContentSheet {
    type Item = (u32, Vec<(u16, OpenContentRow)>);
    type IntoIter = StructuredSheetIterator<'a, OpenContentSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, OpenContentSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct OpenContentRow {
    columns: Vec<Field>,
}
impl OpenContentRow {
    pub fn OpenContentData<'a>(&'a self) -> [OpenContentDataElement<'a>; 16] {
        [
            OpenContentDataElement {
                CandidateName: &self.columns[0],
                Content: &self.columns[1],
            },
            OpenContentDataElement {
                CandidateName: &self.columns[2],
                Content: &self.columns[3],
            },
            OpenContentDataElement {
                CandidateName: &self.columns[4],
                Content: &self.columns[5],
            },
            OpenContentDataElement {
                CandidateName: &self.columns[6],
                Content: &self.columns[7],
            },
            OpenContentDataElement {
                CandidateName: &self.columns[8],
                Content: &self.columns[9],
            },
            OpenContentDataElement {
                CandidateName: &self.columns[10],
                Content: &self.columns[11],
            },
            OpenContentDataElement {
                CandidateName: &self.columns[12],
                Content: &self.columns[13],
            },
            OpenContentDataElement {
                CandidateName: &self.columns[14],
                Content: &self.columns[15],
            },
            OpenContentDataElement {
                CandidateName: &self.columns[16],
                Content: &self.columns[17],
            },
            OpenContentDataElement {
                CandidateName: &self.columns[18],
                Content: &self.columns[19],
            },
            OpenContentDataElement {
                CandidateName: &self.columns[20],
                Content: &self.columns[21],
            },
            OpenContentDataElement {
                CandidateName: &self.columns[22],
                Content: &self.columns[23],
            },
            OpenContentDataElement {
                CandidateName: &self.columns[24],
                Content: &self.columns[25],
            },
            OpenContentDataElement {
                CandidateName: &self.columns[26],
                Content: &self.columns[27],
            },
            OpenContentDataElement {
                CandidateName: &self.columns[28],
                Content: &self.columns[29],
            },
            OpenContentDataElement {
                CandidateName: &self.columns[30],
                Content: &self.columns[31],
            },
        ]
    }
}
