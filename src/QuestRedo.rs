//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
pub struct QuestRedoParamElement<'a> {
    pub Quest: &'a Field,
    pub UnknownParam: &'a Field,
}
#[derive(Debug, Clone)]
pub struct QuestRedoSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl QuestRedoSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("QuestRedo")?;
        let sheet = resolver.read_excel_sheet(&exh, "QuestRedo", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<QuestRedoRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<QuestRedoRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for QuestRedoSheet {
    type Row = QuestRedoRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a QuestRedoSheet {
    type Item = (u32, Vec<(u16, QuestRedoRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, QuestRedoSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, QuestRedoSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct QuestRedoRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> QuestRedoRow<'a> {
    pub fn QuestRedoParam(&'a self) -> [QuestRedoParamElement<'a>; 32] {
        [
            QuestRedoParamElement {
                Quest: &self.row.columns[self.index_mapping[0]],
                UnknownParam: &self.row.columns[self.index_mapping[1]],
            },
            QuestRedoParamElement {
                Quest: &self.row.columns[self.index_mapping[2]],
                UnknownParam: &self.row.columns[self.index_mapping[3]],
            },
            QuestRedoParamElement {
                Quest: &self.row.columns[self.index_mapping[4]],
                UnknownParam: &self.row.columns[self.index_mapping[5]],
            },
            QuestRedoParamElement {
                Quest: &self.row.columns[self.index_mapping[6]],
                UnknownParam: &self.row.columns[self.index_mapping[7]],
            },
            QuestRedoParamElement {
                Quest: &self.row.columns[self.index_mapping[8]],
                UnknownParam: &self.row.columns[self.index_mapping[9]],
            },
            QuestRedoParamElement {
                Quest: &self.row.columns[self.index_mapping[10]],
                UnknownParam: &self.row.columns[self.index_mapping[11]],
            },
            QuestRedoParamElement {
                Quest: &self.row.columns[self.index_mapping[12]],
                UnknownParam: &self.row.columns[self.index_mapping[13]],
            },
            QuestRedoParamElement {
                Quest: &self.row.columns[self.index_mapping[14]],
                UnknownParam: &self.row.columns[self.index_mapping[15]],
            },
            QuestRedoParamElement {
                Quest: &self.row.columns[self.index_mapping[16]],
                UnknownParam: &self.row.columns[self.index_mapping[17]],
            },
            QuestRedoParamElement {
                Quest: &self.row.columns[self.index_mapping[18]],
                UnknownParam: &self.row.columns[self.index_mapping[19]],
            },
            QuestRedoParamElement {
                Quest: &self.row.columns[self.index_mapping[20]],
                UnknownParam: &self.row.columns[self.index_mapping[21]],
            },
            QuestRedoParamElement {
                Quest: &self.row.columns[self.index_mapping[22]],
                UnknownParam: &self.row.columns[self.index_mapping[23]],
            },
            QuestRedoParamElement {
                Quest: &self.row.columns[self.index_mapping[24]],
                UnknownParam: &self.row.columns[self.index_mapping[25]],
            },
            QuestRedoParamElement {
                Quest: &self.row.columns[self.index_mapping[26]],
                UnknownParam: &self.row.columns[self.index_mapping[27]],
            },
            QuestRedoParamElement {
                Quest: &self.row.columns[self.index_mapping[28]],
                UnknownParam: &self.row.columns[self.index_mapping[29]],
            },
            QuestRedoParamElement {
                Quest: &self.row.columns[self.index_mapping[30]],
                UnknownParam: &self.row.columns[self.index_mapping[31]],
            },
            QuestRedoParamElement {
                Quest: &self.row.columns[self.index_mapping[32]],
                UnknownParam: &self.row.columns[self.index_mapping[33]],
            },
            QuestRedoParamElement {
                Quest: &self.row.columns[self.index_mapping[34]],
                UnknownParam: &self.row.columns[self.index_mapping[35]],
            },
            QuestRedoParamElement {
                Quest: &self.row.columns[self.index_mapping[36]],
                UnknownParam: &self.row.columns[self.index_mapping[37]],
            },
            QuestRedoParamElement {
                Quest: &self.row.columns[self.index_mapping[38]],
                UnknownParam: &self.row.columns[self.index_mapping[39]],
            },
            QuestRedoParamElement {
                Quest: &self.row.columns[self.index_mapping[40]],
                UnknownParam: &self.row.columns[self.index_mapping[41]],
            },
            QuestRedoParamElement {
                Quest: &self.row.columns[self.index_mapping[42]],
                UnknownParam: &self.row.columns[self.index_mapping[43]],
            },
            QuestRedoParamElement {
                Quest: &self.row.columns[self.index_mapping[44]],
                UnknownParam: &self.row.columns[self.index_mapping[45]],
            },
            QuestRedoParamElement {
                Quest: &self.row.columns[self.index_mapping[46]],
                UnknownParam: &self.row.columns[self.index_mapping[47]],
            },
            QuestRedoParamElement {
                Quest: &self.row.columns[self.index_mapping[48]],
                UnknownParam: &self.row.columns[self.index_mapping[49]],
            },
            QuestRedoParamElement {
                Quest: &self.row.columns[self.index_mapping[50]],
                UnknownParam: &self.row.columns[self.index_mapping[51]],
            },
            QuestRedoParamElement {
                Quest: &self.row.columns[self.index_mapping[52]],
                UnknownParam: &self.row.columns[self.index_mapping[53]],
            },
            QuestRedoParamElement {
                Quest: &self.row.columns[self.index_mapping[54]],
                UnknownParam: &self.row.columns[self.index_mapping[55]],
            },
            QuestRedoParamElement {
                Quest: &self.row.columns[self.index_mapping[56]],
                UnknownParam: &self.row.columns[self.index_mapping[57]],
            },
            QuestRedoParamElement {
                Quest: &self.row.columns[self.index_mapping[58]],
                UnknownParam: &self.row.columns[self.index_mapping[59]],
            },
            QuestRedoParamElement {
                Quest: &self.row.columns[self.index_mapping[60]],
                UnknownParam: &self.row.columns[self.index_mapping[61]],
            },
            QuestRedoParamElement {
                Quest: &self.row.columns[self.index_mapping[62]],
                UnknownParam: &self.row.columns[self.index_mapping[63]],
            },
        ]
    }
    pub fn FinalQuest(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[64]]
    }
    pub fn Unknown0(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[65]]
    }
    pub fn Chapter(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[66]]
    }
    pub fn Unknown1(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[67]]
    }
}
