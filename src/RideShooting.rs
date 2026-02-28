//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
pub struct RideShootingParamsElement<'a> {
    pub Unknown0: &'a Field,
    pub PopRange: &'a Field,
    pub ENpc: &'a Field,
    pub Unknown1: &'a Field,
    pub Unknown2: &'a Field,
    pub Unknown3: &'a Field,
    pub Unknown4: &'a Field,
    pub Unknown5: &'a Field,
    pub ENpcScale: &'a Field,
    pub Unknown6: &'a Field,
    pub Unknown7: &'a Field,
    pub Unknown8: &'a Field,
    pub Unknown9: &'a Field,
    pub Unknown10: &'a Field,
}
#[derive(Debug, Clone)]
pub struct RideShootingSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl RideShootingSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("RideShooting")?;
        let sheet = resolver.read_excel_sheet(&exh, "RideShooting", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<RideShootingRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<RideShootingRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for RideShootingSheet {
    type Row = RideShootingRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a RideShootingSheet {
    type Item = (u32, Vec<(u16, RideShootingRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, RideShootingSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, RideShootingSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct RideShootingRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> RideShootingRow<'a> {
    pub fn RideShootingParams(&'a self) -> [RideShootingParamsElement<'a>; 8] {
        [
            RideShootingParamsElement {
                Unknown0: &self.row.columns[self.index_mapping[0]],
                PopRange: &self.row.columns[self.index_mapping[1]],
                ENpc: &self.row.columns[self.index_mapping[2]],
                Unknown1: &self.row.columns[self.index_mapping[3]],
                Unknown2: &self.row.columns[self.index_mapping[4]],
                Unknown3: &self.row.columns[self.index_mapping[5]],
                Unknown4: &self.row.columns[self.index_mapping[6]],
                Unknown5: &self.row.columns[self.index_mapping[7]],
                ENpcScale: &self.row.columns[self.index_mapping[8]],
                Unknown6: &self.row.columns[self.index_mapping[9]],
                Unknown7: &self.row.columns[self.index_mapping[10]],
                Unknown8: &self.row.columns[self.index_mapping[11]],
                Unknown9: &self.row.columns[self.index_mapping[12]],
                Unknown10: &self.row.columns[self.index_mapping[13]],
            },
            RideShootingParamsElement {
                Unknown0: &self.row.columns[self.index_mapping[14]],
                PopRange: &self.row.columns[self.index_mapping[15]],
                ENpc: &self.row.columns[self.index_mapping[16]],
                Unknown1: &self.row.columns[self.index_mapping[17]],
                Unknown2: &self.row.columns[self.index_mapping[18]],
                Unknown3: &self.row.columns[self.index_mapping[19]],
                Unknown4: &self.row.columns[self.index_mapping[20]],
                Unknown5: &self.row.columns[self.index_mapping[21]],
                ENpcScale: &self.row.columns[self.index_mapping[22]],
                Unknown6: &self.row.columns[self.index_mapping[23]],
                Unknown7: &self.row.columns[self.index_mapping[24]],
                Unknown8: &self.row.columns[self.index_mapping[25]],
                Unknown9: &self.row.columns[self.index_mapping[26]],
                Unknown10: &self.row.columns[self.index_mapping[27]],
            },
            RideShootingParamsElement {
                Unknown0: &self.row.columns[self.index_mapping[28]],
                PopRange: &self.row.columns[self.index_mapping[29]],
                ENpc: &self.row.columns[self.index_mapping[30]],
                Unknown1: &self.row.columns[self.index_mapping[31]],
                Unknown2: &self.row.columns[self.index_mapping[32]],
                Unknown3: &self.row.columns[self.index_mapping[33]],
                Unknown4: &self.row.columns[self.index_mapping[34]],
                Unknown5: &self.row.columns[self.index_mapping[35]],
                ENpcScale: &self.row.columns[self.index_mapping[36]],
                Unknown6: &self.row.columns[self.index_mapping[37]],
                Unknown7: &self.row.columns[self.index_mapping[38]],
                Unknown8: &self.row.columns[self.index_mapping[39]],
                Unknown9: &self.row.columns[self.index_mapping[40]],
                Unknown10: &self.row.columns[self.index_mapping[41]],
            },
            RideShootingParamsElement {
                Unknown0: &self.row.columns[self.index_mapping[42]],
                PopRange: &self.row.columns[self.index_mapping[43]],
                ENpc: &self.row.columns[self.index_mapping[44]],
                Unknown1: &self.row.columns[self.index_mapping[45]],
                Unknown2: &self.row.columns[self.index_mapping[46]],
                Unknown3: &self.row.columns[self.index_mapping[47]],
                Unknown4: &self.row.columns[self.index_mapping[48]],
                Unknown5: &self.row.columns[self.index_mapping[49]],
                ENpcScale: &self.row.columns[self.index_mapping[50]],
                Unknown6: &self.row.columns[self.index_mapping[51]],
                Unknown7: &self.row.columns[self.index_mapping[52]],
                Unknown8: &self.row.columns[self.index_mapping[53]],
                Unknown9: &self.row.columns[self.index_mapping[54]],
                Unknown10: &self.row.columns[self.index_mapping[55]],
            },
            RideShootingParamsElement {
                Unknown0: &self.row.columns[self.index_mapping[56]],
                PopRange: &self.row.columns[self.index_mapping[57]],
                ENpc: &self.row.columns[self.index_mapping[58]],
                Unknown1: &self.row.columns[self.index_mapping[59]],
                Unknown2: &self.row.columns[self.index_mapping[60]],
                Unknown3: &self.row.columns[self.index_mapping[61]],
                Unknown4: &self.row.columns[self.index_mapping[62]],
                Unknown5: &self.row.columns[self.index_mapping[63]],
                ENpcScale: &self.row.columns[self.index_mapping[64]],
                Unknown6: &self.row.columns[self.index_mapping[65]],
                Unknown7: &self.row.columns[self.index_mapping[66]],
                Unknown8: &self.row.columns[self.index_mapping[67]],
                Unknown9: &self.row.columns[self.index_mapping[68]],
                Unknown10: &self.row.columns[self.index_mapping[69]],
            },
            RideShootingParamsElement {
                Unknown0: &self.row.columns[self.index_mapping[70]],
                PopRange: &self.row.columns[self.index_mapping[71]],
                ENpc: &self.row.columns[self.index_mapping[72]],
                Unknown1: &self.row.columns[self.index_mapping[73]],
                Unknown2: &self.row.columns[self.index_mapping[74]],
                Unknown3: &self.row.columns[self.index_mapping[75]],
                Unknown4: &self.row.columns[self.index_mapping[76]],
                Unknown5: &self.row.columns[self.index_mapping[77]],
                ENpcScale: &self.row.columns[self.index_mapping[78]],
                Unknown6: &self.row.columns[self.index_mapping[79]],
                Unknown7: &self.row.columns[self.index_mapping[80]],
                Unknown8: &self.row.columns[self.index_mapping[81]],
                Unknown9: &self.row.columns[self.index_mapping[82]],
                Unknown10: &self.row.columns[self.index_mapping[83]],
            },
            RideShootingParamsElement {
                Unknown0: &self.row.columns[self.index_mapping[84]],
                PopRange: &self.row.columns[self.index_mapping[85]],
                ENpc: &self.row.columns[self.index_mapping[86]],
                Unknown1: &self.row.columns[self.index_mapping[87]],
                Unknown2: &self.row.columns[self.index_mapping[88]],
                Unknown3: &self.row.columns[self.index_mapping[89]],
                Unknown4: &self.row.columns[self.index_mapping[90]],
                Unknown5: &self.row.columns[self.index_mapping[91]],
                ENpcScale: &self.row.columns[self.index_mapping[92]],
                Unknown6: &self.row.columns[self.index_mapping[93]],
                Unknown7: &self.row.columns[self.index_mapping[94]],
                Unknown8: &self.row.columns[self.index_mapping[95]],
                Unknown9: &self.row.columns[self.index_mapping[96]],
                Unknown10: &self.row.columns[self.index_mapping[97]],
            },
            RideShootingParamsElement {
                Unknown0: &self.row.columns[self.index_mapping[98]],
                PopRange: &self.row.columns[self.index_mapping[99]],
                ENpc: &self.row.columns[self.index_mapping[100]],
                Unknown1: &self.row.columns[self.index_mapping[101]],
                Unknown2: &self.row.columns[self.index_mapping[102]],
                Unknown3: &self.row.columns[self.index_mapping[103]],
                Unknown4: &self.row.columns[self.index_mapping[104]],
                Unknown5: &self.row.columns[self.index_mapping[105]],
                ENpcScale: &self.row.columns[self.index_mapping[106]],
                Unknown6: &self.row.columns[self.index_mapping[107]],
                Unknown7: &self.row.columns[self.index_mapping[108]],
                Unknown8: &self.row.columns[self.index_mapping[109]],
                Unknown9: &self.row.columns[self.index_mapping[110]],
                Unknown10: &self.row.columns[self.index_mapping[111]],
            },
        ]
    }
    pub fn GFateRideShooting(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[112]]
    }
    pub fn Unknown0(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[113]]
    }
    pub fn Unknown1(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[114]]
    }
    pub fn StartText(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[115]]
    }
    pub fn Unknown2(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[116]]
    }
    pub fn Unknown3(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[117]]
    }
}
