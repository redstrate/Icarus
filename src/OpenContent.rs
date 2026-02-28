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
    index_mapping: Vec<usize>,
}
impl OpenContentSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("OpenContent")?;
        let sheet = resolver.read_excel_sheet(&exh, "OpenContent", language)?;
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
impl<'a> StructuredSheet<'a> for OpenContentSheet {
    type Row = OpenContentRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a OpenContentSheet {
    type Item = (u32, Vec<(u16, OpenContentRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, OpenContentSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, OpenContentSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct OpenContentRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> OpenContentRow<'a> {
    pub fn OpenContentData(&'a self) -> [OpenContentDataElement<'a>; 16] {
        [
            OpenContentDataElement {
                CandidateName: &self.row.columns[self.index_mapping[0]],
                Content: &self.row.columns[self.index_mapping[1]],
            },
            OpenContentDataElement {
                CandidateName: &self.row.columns[self.index_mapping[2]],
                Content: &self.row.columns[self.index_mapping[3]],
            },
            OpenContentDataElement {
                CandidateName: &self.row.columns[self.index_mapping[4]],
                Content: &self.row.columns[self.index_mapping[5]],
            },
            OpenContentDataElement {
                CandidateName: &self.row.columns[self.index_mapping[6]],
                Content: &self.row.columns[self.index_mapping[7]],
            },
            OpenContentDataElement {
                CandidateName: &self.row.columns[self.index_mapping[8]],
                Content: &self.row.columns[self.index_mapping[9]],
            },
            OpenContentDataElement {
                CandidateName: &self.row.columns[self.index_mapping[10]],
                Content: &self.row.columns[self.index_mapping[11]],
            },
            OpenContentDataElement {
                CandidateName: &self.row.columns[self.index_mapping[12]],
                Content: &self.row.columns[self.index_mapping[13]],
            },
            OpenContentDataElement {
                CandidateName: &self.row.columns[self.index_mapping[14]],
                Content: &self.row.columns[self.index_mapping[15]],
            },
            OpenContentDataElement {
                CandidateName: &self.row.columns[self.index_mapping[16]],
                Content: &self.row.columns[self.index_mapping[17]],
            },
            OpenContentDataElement {
                CandidateName: &self.row.columns[self.index_mapping[18]],
                Content: &self.row.columns[self.index_mapping[19]],
            },
            OpenContentDataElement {
                CandidateName: &self.row.columns[self.index_mapping[20]],
                Content: &self.row.columns[self.index_mapping[21]],
            },
            OpenContentDataElement {
                CandidateName: &self.row.columns[self.index_mapping[22]],
                Content: &self.row.columns[self.index_mapping[23]],
            },
            OpenContentDataElement {
                CandidateName: &self.row.columns[self.index_mapping[24]],
                Content: &self.row.columns[self.index_mapping[25]],
            },
            OpenContentDataElement {
                CandidateName: &self.row.columns[self.index_mapping[26]],
                Content: &self.row.columns[self.index_mapping[27]],
            },
            OpenContentDataElement {
                CandidateName: &self.row.columns[self.index_mapping[28]],
                Content: &self.row.columns[self.index_mapping[29]],
            },
            OpenContentDataElement {
                CandidateName: &self.row.columns[self.index_mapping[30]],
                Content: &self.row.columns[self.index_mapping[31]],
            },
        ]
    }
}
