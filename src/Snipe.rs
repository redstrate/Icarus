//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
pub struct SnipeDataElement<'a> {
    pub DataEventNPC: &'a Field,
    pub Unknown0: &'a Field,
    pub Unknown1: &'a Field,
    pub Unknown2: &'a Field,
    pub Unknown3: &'a Field,
    pub Unknown4: &'a Field,
    pub Unknown5: &'a Field,
}
#[derive(Debug, Clone)]
pub struct SnipeSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl SnipeSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Snipe")?;
        let sheet = resolver.read_excel_sheet(&exh, "Snipe", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<SnipeRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<SnipeRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for SnipeSheet {
    type Row = SnipeRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a SnipeSheet {
    type Item = (u32, Vec<(u16, SnipeRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, SnipeSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, SnipeSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct SnipeRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> SnipeRow<'a> {
    pub fn SnipeData(&'a self) -> [SnipeDataElement<'a>; 8] {
        [
            SnipeDataElement {
                DataEventNPC: &self.row.columns[self.index_mapping[0]],
                Unknown0: &self.row.columns[self.index_mapping[1]],
                Unknown1: &self.row.columns[self.index_mapping[2]],
                Unknown2: &self.row.columns[self.index_mapping[3]],
                Unknown3: &self.row.columns[self.index_mapping[4]],
                Unknown4: &self.row.columns[self.index_mapping[5]],
                Unknown5: &self.row.columns[self.index_mapping[6]],
            },
            SnipeDataElement {
                DataEventNPC: &self.row.columns[self.index_mapping[7]],
                Unknown0: &self.row.columns[self.index_mapping[8]],
                Unknown1: &self.row.columns[self.index_mapping[9]],
                Unknown2: &self.row.columns[self.index_mapping[10]],
                Unknown3: &self.row.columns[self.index_mapping[11]],
                Unknown4: &self.row.columns[self.index_mapping[12]],
                Unknown5: &self.row.columns[self.index_mapping[13]],
            },
            SnipeDataElement {
                DataEventNPC: &self.row.columns[self.index_mapping[14]],
                Unknown0: &self.row.columns[self.index_mapping[15]],
                Unknown1: &self.row.columns[self.index_mapping[16]],
                Unknown2: &self.row.columns[self.index_mapping[17]],
                Unknown3: &self.row.columns[self.index_mapping[18]],
                Unknown4: &self.row.columns[self.index_mapping[19]],
                Unknown5: &self.row.columns[self.index_mapping[20]],
            },
            SnipeDataElement {
                DataEventNPC: &self.row.columns[self.index_mapping[21]],
                Unknown0: &self.row.columns[self.index_mapping[22]],
                Unknown1: &self.row.columns[self.index_mapping[23]],
                Unknown2: &self.row.columns[self.index_mapping[24]],
                Unknown3: &self.row.columns[self.index_mapping[25]],
                Unknown4: &self.row.columns[self.index_mapping[26]],
                Unknown5: &self.row.columns[self.index_mapping[27]],
            },
            SnipeDataElement {
                DataEventNPC: &self.row.columns[self.index_mapping[28]],
                Unknown0: &self.row.columns[self.index_mapping[29]],
                Unknown1: &self.row.columns[self.index_mapping[30]],
                Unknown2: &self.row.columns[self.index_mapping[31]],
                Unknown3: &self.row.columns[self.index_mapping[32]],
                Unknown4: &self.row.columns[self.index_mapping[33]],
                Unknown5: &self.row.columns[self.index_mapping[34]],
            },
            SnipeDataElement {
                DataEventNPC: &self.row.columns[self.index_mapping[35]],
                Unknown0: &self.row.columns[self.index_mapping[36]],
                Unknown1: &self.row.columns[self.index_mapping[37]],
                Unknown2: &self.row.columns[self.index_mapping[38]],
                Unknown3: &self.row.columns[self.index_mapping[39]],
                Unknown4: &self.row.columns[self.index_mapping[40]],
                Unknown5: &self.row.columns[self.index_mapping[41]],
            },
            SnipeDataElement {
                DataEventNPC: &self.row.columns[self.index_mapping[42]],
                Unknown0: &self.row.columns[self.index_mapping[43]],
                Unknown1: &self.row.columns[self.index_mapping[44]],
                Unknown2: &self.row.columns[self.index_mapping[45]],
                Unknown3: &self.row.columns[self.index_mapping[46]],
                Unknown4: &self.row.columns[self.index_mapping[47]],
                Unknown5: &self.row.columns[self.index_mapping[48]],
            },
            SnipeDataElement {
                DataEventNPC: &self.row.columns[self.index_mapping[49]],
                Unknown0: &self.row.columns[self.index_mapping[50]],
                Unknown1: &self.row.columns[self.index_mapping[51]],
                Unknown2: &self.row.columns[self.index_mapping[52]],
                Unknown3: &self.row.columns[self.index_mapping[53]],
                Unknown4: &self.row.columns[self.index_mapping[54]],
                Unknown5: &self.row.columns[self.index_mapping[55]],
            },
        ]
    }
    pub fn EventNPC(&'a self) -> [&'a Field; 8] {
        [
            &self.row.columns[self.index_mapping[56]],
            &self.row.columns[self.index_mapping[57]],
            &self.row.columns[self.index_mapping[58]],
            &self.row.columns[self.index_mapping[59]],
            &self.row.columns[self.index_mapping[60]],
            &self.row.columns[self.index_mapping[61]],
            &self.row.columns[self.index_mapping[62]],
            &self.row.columns[self.index_mapping[63]],
        ]
    }
    pub fn Unknown0(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[64]]
    }
    pub fn Unknown1(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[65]]
    }
    pub fn Unknown2(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[66]]
    }
    pub fn Unknown3(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[67]]
    }
    pub fn Unknown4(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[68]]
    }
    pub fn Unknown5(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[69]]
    }
    pub fn Unknown6(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[70]]
    }
    pub fn Unknown7(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[71]]
    }
    pub fn Unknown8(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[72]]
    }
    pub fn Unknown9(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[73]]
    }
    pub fn Unknown10(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[74]]
    }
    pub fn Unknown11(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[75]]
    }
    pub fn Objective0(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[76]]
    }
    pub fn Hint0(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[77]]
    }
    pub fn Objective1(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[78]]
    }
    pub fn Hint1(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[79]]
    }
    pub fn Unknown12(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[80]]
    }
    pub fn Unknown13(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[81]]
    }
    pub fn Unknown14(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[82]]
    }
    pub fn Unknown15(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[83]]
    }
    pub fn Unknown16(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[84]]
    }
    pub fn Unknown17(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[85]]
    }
    pub fn Unknown18(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[86]]
    }
    pub fn ActionText(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[87]]
    }
    pub fn Unknown19(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[88]]
    }
    pub fn Unknown20(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[89]]
    }
    pub fn VFXFire(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[90]]
    }
    pub fn VFXHit(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[91]]
    }
    pub fn VFXMiss(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[92]]
    }
    pub fn VFXAdditional(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[93]]
    }
    pub fn LGBTargetMarker(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[94]]
    }
    pub fn Unknown21(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[95]]
    }
    pub fn Unknown22(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[96]]
    }
    pub fn Unknown23(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[97]]
    }
    pub fn Unknown24(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[98]]
    }
    pub fn Unknown25(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[99]]
    }
    pub fn Unknown26(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[100]]
    }
    pub fn Unknown27(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[101]]
    }
    pub fn Unknown28(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[102]]
    }
    pub fn Unknown29(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[103]]
    }
    pub fn Unknown30(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[104]]
    }
    pub fn Unknown31(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[105]]
    }
    pub fn Unknown32(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[106]]
    }
}
