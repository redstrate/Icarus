//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
pub struct GFATEParamsElement<'a> {
    pub LGBPopRange: &'a Field,
    pub Icon: &'a Field,
    pub Unknown0: &'a Field,
    pub Unknown1: &'a Field,
    pub Unknown2: &'a Field,
}
#[derive(Debug, Clone)]
pub struct GFATESheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl GFATESheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("GFATE")?;
        let sheet = resolver.read_excel_sheet(&exh, "GFATE", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<GFATERow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<GFATERow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for GFATESheet {
    type Row = GFATERow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a GFATESheet {
    type Item = (u32, Vec<(u16, GFATERow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, GFATESheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, GFATESheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct GFATERow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> GFATERow<'a> {
    pub fn GFATEParams(&'a self) -> [GFATEParamsElement<'a>; 15] {
        [
            GFATEParamsElement {
                LGBPopRange: &self.row.columns[self.index_mapping[0]],
                Icon: &self.row.columns[self.index_mapping[1]],
                Unknown0: &self.row.columns[self.index_mapping[2]],
                Unknown1: &self.row.columns[self.index_mapping[3]],
                Unknown2: &self.row.columns[self.index_mapping[4]],
            },
            GFATEParamsElement {
                LGBPopRange: &self.row.columns[self.index_mapping[5]],
                Icon: &self.row.columns[self.index_mapping[6]],
                Unknown0: &self.row.columns[self.index_mapping[7]],
                Unknown1: &self.row.columns[self.index_mapping[8]],
                Unknown2: &self.row.columns[self.index_mapping[9]],
            },
            GFATEParamsElement {
                LGBPopRange: &self.row.columns[self.index_mapping[10]],
                Icon: &self.row.columns[self.index_mapping[11]],
                Unknown0: &self.row.columns[self.index_mapping[12]],
                Unknown1: &self.row.columns[self.index_mapping[13]],
                Unknown2: &self.row.columns[self.index_mapping[14]],
            },
            GFATEParamsElement {
                LGBPopRange: &self.row.columns[self.index_mapping[15]],
                Icon: &self.row.columns[self.index_mapping[16]],
                Unknown0: &self.row.columns[self.index_mapping[17]],
                Unknown1: &self.row.columns[self.index_mapping[18]],
                Unknown2: &self.row.columns[self.index_mapping[19]],
            },
            GFATEParamsElement {
                LGBPopRange: &self.row.columns[self.index_mapping[20]],
                Icon: &self.row.columns[self.index_mapping[21]],
                Unknown0: &self.row.columns[self.index_mapping[22]],
                Unknown1: &self.row.columns[self.index_mapping[23]],
                Unknown2: &self.row.columns[self.index_mapping[24]],
            },
            GFATEParamsElement {
                LGBPopRange: &self.row.columns[self.index_mapping[25]],
                Icon: &self.row.columns[self.index_mapping[26]],
                Unknown0: &self.row.columns[self.index_mapping[27]],
                Unknown1: &self.row.columns[self.index_mapping[28]],
                Unknown2: &self.row.columns[self.index_mapping[29]],
            },
            GFATEParamsElement {
                LGBPopRange: &self.row.columns[self.index_mapping[30]],
                Icon: &self.row.columns[self.index_mapping[31]],
                Unknown0: &self.row.columns[self.index_mapping[32]],
                Unknown1: &self.row.columns[self.index_mapping[33]],
                Unknown2: &self.row.columns[self.index_mapping[34]],
            },
            GFATEParamsElement {
                LGBPopRange: &self.row.columns[self.index_mapping[35]],
                Icon: &self.row.columns[self.index_mapping[36]],
                Unknown0: &self.row.columns[self.index_mapping[37]],
                Unknown1: &self.row.columns[self.index_mapping[38]],
                Unknown2: &self.row.columns[self.index_mapping[39]],
            },
            GFATEParamsElement {
                LGBPopRange: &self.row.columns[self.index_mapping[40]],
                Icon: &self.row.columns[self.index_mapping[41]],
                Unknown0: &self.row.columns[self.index_mapping[42]],
                Unknown1: &self.row.columns[self.index_mapping[43]],
                Unknown2: &self.row.columns[self.index_mapping[44]],
            },
            GFATEParamsElement {
                LGBPopRange: &self.row.columns[self.index_mapping[45]],
                Icon: &self.row.columns[self.index_mapping[46]],
                Unknown0: &self.row.columns[self.index_mapping[47]],
                Unknown1: &self.row.columns[self.index_mapping[48]],
                Unknown2: &self.row.columns[self.index_mapping[49]],
            },
            GFATEParamsElement {
                LGBPopRange: &self.row.columns[self.index_mapping[50]],
                Icon: &self.row.columns[self.index_mapping[51]],
                Unknown0: &self.row.columns[self.index_mapping[52]],
                Unknown1: &self.row.columns[self.index_mapping[53]],
                Unknown2: &self.row.columns[self.index_mapping[54]],
            },
            GFATEParamsElement {
                LGBPopRange: &self.row.columns[self.index_mapping[55]],
                Icon: &self.row.columns[self.index_mapping[56]],
                Unknown0: &self.row.columns[self.index_mapping[57]],
                Unknown1: &self.row.columns[self.index_mapping[58]],
                Unknown2: &self.row.columns[self.index_mapping[59]],
            },
            GFATEParamsElement {
                LGBPopRange: &self.row.columns[self.index_mapping[60]],
                Icon: &self.row.columns[self.index_mapping[61]],
                Unknown0: &self.row.columns[self.index_mapping[62]],
                Unknown1: &self.row.columns[self.index_mapping[63]],
                Unknown2: &self.row.columns[self.index_mapping[64]],
            },
            GFATEParamsElement {
                LGBPopRange: &self.row.columns[self.index_mapping[65]],
                Icon: &self.row.columns[self.index_mapping[66]],
                Unknown0: &self.row.columns[self.index_mapping[67]],
                Unknown1: &self.row.columns[self.index_mapping[68]],
                Unknown2: &self.row.columns[self.index_mapping[69]],
            },
            GFATEParamsElement {
                LGBPopRange: &self.row.columns[self.index_mapping[70]],
                Icon: &self.row.columns[self.index_mapping[71]],
                Unknown0: &self.row.columns[self.index_mapping[72]],
                Unknown1: &self.row.columns[self.index_mapping[73]],
                Unknown2: &self.row.columns[self.index_mapping[74]],
            },
        ]
    }
    pub fn Unknown0(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[75]]
    }
    pub fn Unknown1(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[76]]
    }
    pub fn Unknown2(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[77]]
    }
    pub fn Unknown3(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[78]]
    }
    pub fn Unknown4(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[79]]
    }
    pub fn Unknown5(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[80]]
    }
    pub fn Unknown6(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[81]]
    }
    pub fn Unknown7(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[82]]
    }
    pub fn Unknown8(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[83]]
    }
    pub fn Unknown9(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[84]]
    }
    pub fn Unknown10(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[85]]
    }
    pub fn Unknown11(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[86]]
    }
}
