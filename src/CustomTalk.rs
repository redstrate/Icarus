//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
pub struct ScriptElement<'a> {
    pub ScriptInstruction: &'a Field,
    pub ScriptArg: &'a Field,
}
#[derive(Debug, Clone)]
pub struct CustomTalkSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl CustomTalkSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("CustomTalk")?;
        let sheet = resolver.read_excel_sheet(&exh, "CustomTalk", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<CustomTalkRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<CustomTalkRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for CustomTalkSheet {
    type Row = CustomTalkRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a CustomTalkSheet {
    type Item = (u32, Vec<(u16, CustomTalkRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, CustomTalkSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, CustomTalkSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct CustomTalkRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> CustomTalkRow<'a> {
    pub fn Script(&'a self) -> [ScriptElement<'a>; 30] {
        [
            ScriptElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[0]],
                ScriptArg: &self.row.columns[self.index_mapping[1]],
            },
            ScriptElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[2]],
                ScriptArg: &self.row.columns[self.index_mapping[3]],
            },
            ScriptElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[4]],
                ScriptArg: &self.row.columns[self.index_mapping[5]],
            },
            ScriptElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[6]],
                ScriptArg: &self.row.columns[self.index_mapping[7]],
            },
            ScriptElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[8]],
                ScriptArg: &self.row.columns[self.index_mapping[9]],
            },
            ScriptElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[10]],
                ScriptArg: &self.row.columns[self.index_mapping[11]],
            },
            ScriptElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[12]],
                ScriptArg: &self.row.columns[self.index_mapping[13]],
            },
            ScriptElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[14]],
                ScriptArg: &self.row.columns[self.index_mapping[15]],
            },
            ScriptElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[16]],
                ScriptArg: &self.row.columns[self.index_mapping[17]],
            },
            ScriptElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[18]],
                ScriptArg: &self.row.columns[self.index_mapping[19]],
            },
            ScriptElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[20]],
                ScriptArg: &self.row.columns[self.index_mapping[21]],
            },
            ScriptElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[22]],
                ScriptArg: &self.row.columns[self.index_mapping[23]],
            },
            ScriptElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[24]],
                ScriptArg: &self.row.columns[self.index_mapping[25]],
            },
            ScriptElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[26]],
                ScriptArg: &self.row.columns[self.index_mapping[27]],
            },
            ScriptElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[28]],
                ScriptArg: &self.row.columns[self.index_mapping[29]],
            },
            ScriptElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[30]],
                ScriptArg: &self.row.columns[self.index_mapping[31]],
            },
            ScriptElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[32]],
                ScriptArg: &self.row.columns[self.index_mapping[33]],
            },
            ScriptElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[34]],
                ScriptArg: &self.row.columns[self.index_mapping[35]],
            },
            ScriptElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[36]],
                ScriptArg: &self.row.columns[self.index_mapping[37]],
            },
            ScriptElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[38]],
                ScriptArg: &self.row.columns[self.index_mapping[39]],
            },
            ScriptElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[40]],
                ScriptArg: &self.row.columns[self.index_mapping[41]],
            },
            ScriptElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[42]],
                ScriptArg: &self.row.columns[self.index_mapping[43]],
            },
            ScriptElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[44]],
                ScriptArg: &self.row.columns[self.index_mapping[45]],
            },
            ScriptElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[46]],
                ScriptArg: &self.row.columns[self.index_mapping[47]],
            },
            ScriptElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[48]],
                ScriptArg: &self.row.columns[self.index_mapping[49]],
            },
            ScriptElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[50]],
                ScriptArg: &self.row.columns[self.index_mapping[51]],
            },
            ScriptElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[52]],
                ScriptArg: &self.row.columns[self.index_mapping[53]],
            },
            ScriptElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[54]],
                ScriptArg: &self.row.columns[self.index_mapping[55]],
            },
            ScriptElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[56]],
                ScriptArg: &self.row.columns[self.index_mapping[57]],
            },
            ScriptElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[58]],
                ScriptArg: &self.row.columns[self.index_mapping[59]],
            },
        ]
    }
    pub fn MainOption(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[60]]
    }
    pub fn SubOption(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[61]]
    }
    pub fn Name(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[62]]
    }
    pub fn IconActor(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[63]]
    }
    pub fn IconMap(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[64]]
    }
    pub fn SpecialLinks(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[65]]
    }
    pub fn Unknown0(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[66]]
    }
    pub fn Unknown1(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[67]]
    }
    pub fn Unknown2(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[68]]
    }
    pub fn Unknown3(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[69]]
    }
    pub fn Unknown4(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[70]]
    }
    pub fn Unknown5(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[71]]
    }
    pub fn Unknown6(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[72]]
    }
    pub fn Unknown7(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[73]]
    }
    pub fn Unknown8(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[74]]
    }
    pub fn Unknown9(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[75]]
    }
    pub fn Unknown10(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[76]]
    }
    pub fn Unknown11(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[77]]
    }
    pub fn Unknown12(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[78]]
    }
}
