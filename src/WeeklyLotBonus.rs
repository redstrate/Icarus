//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
pub struct WeeklyLotBonusParamElement<'a> {
    pub Unknown0: &'a Field,
    pub WeeklyLotBonusThreshold: &'a Field,
    pub Unknown1: &'a Field,
}
#[derive(Debug, Clone)]
pub struct WeeklyLotBonusSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl WeeklyLotBonusSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("WeeklyLotBonus")?;
        let sheet = resolver.read_excel_sheet(&exh, "WeeklyLotBonus", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<WeeklyLotBonusRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<WeeklyLotBonusRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for WeeklyLotBonusSheet {
    type Row = WeeklyLotBonusRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a WeeklyLotBonusSheet {
    type Item = (u32, Vec<(u16, WeeklyLotBonusRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, WeeklyLotBonusSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, WeeklyLotBonusSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct WeeklyLotBonusRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> WeeklyLotBonusRow<'a> {
    pub fn WeeklyLotBonusParam(&'a self) -> [WeeklyLotBonusParamElement<'a>; 32] {
        [
            WeeklyLotBonusParamElement {
                Unknown0: &self.row.columns[self.index_mapping[0]],
                WeeklyLotBonusThreshold: &self.row.columns[self.index_mapping[1]],
                Unknown1: &self.row.columns[self.index_mapping[2]],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.row.columns[self.index_mapping[3]],
                WeeklyLotBonusThreshold: &self.row.columns[self.index_mapping[4]],
                Unknown1: &self.row.columns[self.index_mapping[5]],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.row.columns[self.index_mapping[6]],
                WeeklyLotBonusThreshold: &self.row.columns[self.index_mapping[7]],
                Unknown1: &self.row.columns[self.index_mapping[8]],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.row.columns[self.index_mapping[9]],
                WeeklyLotBonusThreshold: &self.row.columns[self.index_mapping[10]],
                Unknown1: &self.row.columns[self.index_mapping[11]],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.row.columns[self.index_mapping[12]],
                WeeklyLotBonusThreshold: &self.row.columns[self.index_mapping[13]],
                Unknown1: &self.row.columns[self.index_mapping[14]],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.row.columns[self.index_mapping[15]],
                WeeklyLotBonusThreshold: &self.row.columns[self.index_mapping[16]],
                Unknown1: &self.row.columns[self.index_mapping[17]],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.row.columns[self.index_mapping[18]],
                WeeklyLotBonusThreshold: &self.row.columns[self.index_mapping[19]],
                Unknown1: &self.row.columns[self.index_mapping[20]],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.row.columns[self.index_mapping[21]],
                WeeklyLotBonusThreshold: &self.row.columns[self.index_mapping[22]],
                Unknown1: &self.row.columns[self.index_mapping[23]],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.row.columns[self.index_mapping[24]],
                WeeklyLotBonusThreshold: &self.row.columns[self.index_mapping[25]],
                Unknown1: &self.row.columns[self.index_mapping[26]],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.row.columns[self.index_mapping[27]],
                WeeklyLotBonusThreshold: &self.row.columns[self.index_mapping[28]],
                Unknown1: &self.row.columns[self.index_mapping[29]],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.row.columns[self.index_mapping[30]],
                WeeklyLotBonusThreshold: &self.row.columns[self.index_mapping[31]],
                Unknown1: &self.row.columns[self.index_mapping[32]],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.row.columns[self.index_mapping[33]],
                WeeklyLotBonusThreshold: &self.row.columns[self.index_mapping[34]],
                Unknown1: &self.row.columns[self.index_mapping[35]],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.row.columns[self.index_mapping[36]],
                WeeklyLotBonusThreshold: &self.row.columns[self.index_mapping[37]],
                Unknown1: &self.row.columns[self.index_mapping[38]],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.row.columns[self.index_mapping[39]],
                WeeklyLotBonusThreshold: &self.row.columns[self.index_mapping[40]],
                Unknown1: &self.row.columns[self.index_mapping[41]],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.row.columns[self.index_mapping[42]],
                WeeklyLotBonusThreshold: &self.row.columns[self.index_mapping[43]],
                Unknown1: &self.row.columns[self.index_mapping[44]],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.row.columns[self.index_mapping[45]],
                WeeklyLotBonusThreshold: &self.row.columns[self.index_mapping[46]],
                Unknown1: &self.row.columns[self.index_mapping[47]],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.row.columns[self.index_mapping[48]],
                WeeklyLotBonusThreshold: &self.row.columns[self.index_mapping[49]],
                Unknown1: &self.row.columns[self.index_mapping[50]],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.row.columns[self.index_mapping[51]],
                WeeklyLotBonusThreshold: &self.row.columns[self.index_mapping[52]],
                Unknown1: &self.row.columns[self.index_mapping[53]],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.row.columns[self.index_mapping[54]],
                WeeklyLotBonusThreshold: &self.row.columns[self.index_mapping[55]],
                Unknown1: &self.row.columns[self.index_mapping[56]],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.row.columns[self.index_mapping[57]],
                WeeklyLotBonusThreshold: &self.row.columns[self.index_mapping[58]],
                Unknown1: &self.row.columns[self.index_mapping[59]],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.row.columns[self.index_mapping[60]],
                WeeklyLotBonusThreshold: &self.row.columns[self.index_mapping[61]],
                Unknown1: &self.row.columns[self.index_mapping[62]],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.row.columns[self.index_mapping[63]],
                WeeklyLotBonusThreshold: &self.row.columns[self.index_mapping[64]],
                Unknown1: &self.row.columns[self.index_mapping[65]],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.row.columns[self.index_mapping[66]],
                WeeklyLotBonusThreshold: &self.row.columns[self.index_mapping[67]],
                Unknown1: &self.row.columns[self.index_mapping[68]],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.row.columns[self.index_mapping[69]],
                WeeklyLotBonusThreshold: &self.row.columns[self.index_mapping[70]],
                Unknown1: &self.row.columns[self.index_mapping[71]],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.row.columns[self.index_mapping[72]],
                WeeklyLotBonusThreshold: &self.row.columns[self.index_mapping[73]],
                Unknown1: &self.row.columns[self.index_mapping[74]],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.row.columns[self.index_mapping[75]],
                WeeklyLotBonusThreshold: &self.row.columns[self.index_mapping[76]],
                Unknown1: &self.row.columns[self.index_mapping[77]],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.row.columns[self.index_mapping[78]],
                WeeklyLotBonusThreshold: &self.row.columns[self.index_mapping[79]],
                Unknown1: &self.row.columns[self.index_mapping[80]],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.row.columns[self.index_mapping[81]],
                WeeklyLotBonusThreshold: &self.row.columns[self.index_mapping[82]],
                Unknown1: &self.row.columns[self.index_mapping[83]],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.row.columns[self.index_mapping[84]],
                WeeklyLotBonusThreshold: &self.row.columns[self.index_mapping[85]],
                Unknown1: &self.row.columns[self.index_mapping[86]],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.row.columns[self.index_mapping[87]],
                WeeklyLotBonusThreshold: &self.row.columns[self.index_mapping[88]],
                Unknown1: &self.row.columns[self.index_mapping[89]],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.row.columns[self.index_mapping[90]],
                WeeklyLotBonusThreshold: &self.row.columns[self.index_mapping[91]],
                Unknown1: &self.row.columns[self.index_mapping[92]],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.row.columns[self.index_mapping[93]],
                WeeklyLotBonusThreshold: &self.row.columns[self.index_mapping[94]],
                Unknown1: &self.row.columns[self.index_mapping[95]],
            },
        ]
    }
}
