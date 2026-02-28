//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
pub struct ObjectiveIconElement<'a> {
    pub LayoutId: &'a Field,
    pub Icon: &'a Field,
}
#[derive(Debug, Clone)]
pub struct FateSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl FateSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Fate")?;
        let sheet = resolver.read_excel_sheet(&exh, "Fate", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<FateRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<FateRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for FateSheet {
    type Row = FateRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a FateSheet {
    type Item = (u32, Vec<(u16, FateRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, FateSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, FateSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct FateRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> FateRow<'a> {
    pub fn Name(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[0]]
    }
    pub fn Description(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1]]
    }
    pub fn Objective(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[2]]
    }
    pub fn StatusText(&'a self) -> [&'a Field; 3] {
        [
            &self.row.columns[self.index_mapping[3]],
            &self.row.columns[self.index_mapping[4]],
            &self.row.columns[self.index_mapping[5]],
        ]
    }
    pub fn Unknown0(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[6]]
    }
    pub fn Unknown1(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[7]]
    }
    pub fn ReqEventItem(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[8]]
    }
    pub fn TurnInEventItem(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[9]]
    }
    pub fn Unknown2(&'a self) -> [&'a Field; 3] {
        [
            &self.row.columns[self.index_mapping[10]],
            &self.row.columns[self.index_mapping[11]],
            &self.row.columns[self.index_mapping[12]],
        ]
    }
    pub fn Unknown10(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[13]]
    }
    pub fn Unknown11(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[14]]
    }
    pub fn Unknown12(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[15]]
    }
    pub fn ObjectiveIcon(&'a self) -> [ObjectiveIconElement<'a>; 32] {
        [
            ObjectiveIconElement {
                LayoutId: &self.row.columns[self.index_mapping[16]],
                Icon: &self.row.columns[self.index_mapping[17]],
            },
            ObjectiveIconElement {
                LayoutId: &self.row.columns[self.index_mapping[18]],
                Icon: &self.row.columns[self.index_mapping[19]],
            },
            ObjectiveIconElement {
                LayoutId: &self.row.columns[self.index_mapping[20]],
                Icon: &self.row.columns[self.index_mapping[21]],
            },
            ObjectiveIconElement {
                LayoutId: &self.row.columns[self.index_mapping[22]],
                Icon: &self.row.columns[self.index_mapping[23]],
            },
            ObjectiveIconElement {
                LayoutId: &self.row.columns[self.index_mapping[24]],
                Icon: &self.row.columns[self.index_mapping[25]],
            },
            ObjectiveIconElement {
                LayoutId: &self.row.columns[self.index_mapping[26]],
                Icon: &self.row.columns[self.index_mapping[27]],
            },
            ObjectiveIconElement {
                LayoutId: &self.row.columns[self.index_mapping[28]],
                Icon: &self.row.columns[self.index_mapping[29]],
            },
            ObjectiveIconElement {
                LayoutId: &self.row.columns[self.index_mapping[30]],
                Icon: &self.row.columns[self.index_mapping[31]],
            },
            ObjectiveIconElement {
                LayoutId: &self.row.columns[self.index_mapping[32]],
                Icon: &self.row.columns[self.index_mapping[33]],
            },
            ObjectiveIconElement {
                LayoutId: &self.row.columns[self.index_mapping[34]],
                Icon: &self.row.columns[self.index_mapping[35]],
            },
            ObjectiveIconElement {
                LayoutId: &self.row.columns[self.index_mapping[36]],
                Icon: &self.row.columns[self.index_mapping[37]],
            },
            ObjectiveIconElement {
                LayoutId: &self.row.columns[self.index_mapping[38]],
                Icon: &self.row.columns[self.index_mapping[39]],
            },
            ObjectiveIconElement {
                LayoutId: &self.row.columns[self.index_mapping[40]],
                Icon: &self.row.columns[self.index_mapping[41]],
            },
            ObjectiveIconElement {
                LayoutId: &self.row.columns[self.index_mapping[42]],
                Icon: &self.row.columns[self.index_mapping[43]],
            },
            ObjectiveIconElement {
                LayoutId: &self.row.columns[self.index_mapping[44]],
                Icon: &self.row.columns[self.index_mapping[45]],
            },
            ObjectiveIconElement {
                LayoutId: &self.row.columns[self.index_mapping[46]],
                Icon: &self.row.columns[self.index_mapping[47]],
            },
            ObjectiveIconElement {
                LayoutId: &self.row.columns[self.index_mapping[48]],
                Icon: &self.row.columns[self.index_mapping[49]],
            },
            ObjectiveIconElement {
                LayoutId: &self.row.columns[self.index_mapping[50]],
                Icon: &self.row.columns[self.index_mapping[51]],
            },
            ObjectiveIconElement {
                LayoutId: &self.row.columns[self.index_mapping[52]],
                Icon: &self.row.columns[self.index_mapping[53]],
            },
            ObjectiveIconElement {
                LayoutId: &self.row.columns[self.index_mapping[54]],
                Icon: &self.row.columns[self.index_mapping[55]],
            },
            ObjectiveIconElement {
                LayoutId: &self.row.columns[self.index_mapping[56]],
                Icon: &self.row.columns[self.index_mapping[57]],
            },
            ObjectiveIconElement {
                LayoutId: &self.row.columns[self.index_mapping[58]],
                Icon: &self.row.columns[self.index_mapping[59]],
            },
            ObjectiveIconElement {
                LayoutId: &self.row.columns[self.index_mapping[60]],
                Icon: &self.row.columns[self.index_mapping[61]],
            },
            ObjectiveIconElement {
                LayoutId: &self.row.columns[self.index_mapping[62]],
                Icon: &self.row.columns[self.index_mapping[63]],
            },
            ObjectiveIconElement {
                LayoutId: &self.row.columns[self.index_mapping[64]],
                Icon: &self.row.columns[self.index_mapping[65]],
            },
            ObjectiveIconElement {
                LayoutId: &self.row.columns[self.index_mapping[66]],
                Icon: &self.row.columns[self.index_mapping[67]],
            },
            ObjectiveIconElement {
                LayoutId: &self.row.columns[self.index_mapping[68]],
                Icon: &self.row.columns[self.index_mapping[69]],
            },
            ObjectiveIconElement {
                LayoutId: &self.row.columns[self.index_mapping[70]],
                Icon: &self.row.columns[self.index_mapping[71]],
            },
            ObjectiveIconElement {
                LayoutId: &self.row.columns[self.index_mapping[72]],
                Icon: &self.row.columns[self.index_mapping[73]],
            },
            ObjectiveIconElement {
                LayoutId: &self.row.columns[self.index_mapping[74]],
                Icon: &self.row.columns[self.index_mapping[75]],
            },
            ObjectiveIconElement {
                LayoutId: &self.row.columns[self.index_mapping[76]],
                Icon: &self.row.columns[self.index_mapping[77]],
            },
            ObjectiveIconElement {
                LayoutId: &self.row.columns[self.index_mapping[78]],
                Icon: &self.row.columns[self.index_mapping[79]],
            },
        ]
    }
    pub fn Location(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[80]]
    }
    pub fn EventItem(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[81]]
    }
    pub fn Icon(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[82]]
    }
    pub fn MapIcon(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[83]]
    }
    pub fn InactiveMapIcon(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[84]]
    }
    pub fn LGBGuardNPCLocation(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[85]]
    }
    pub fn RequiredQuest(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[86]]
    }
    pub fn FATEChain(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[87]]
    }
    pub fn Unknown13(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[88]]
    }
    pub fn FateRuleEx(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[89]]
    }
    pub fn Music(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[90]]
    }
    pub fn ScreenImageAccept(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[91]]
    }
    pub fn ScreenImageComplete(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[92]]
    }
    pub fn ScreenImageFailed(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[93]]
    }
    pub fn GivenStatus(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[94]]
    }
    pub fn Unknown4(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[95]]
    }
    pub fn Unknown5(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[96]]
    }
    pub fn EurekaFate(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[97]]
    }
    pub fn Rule(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[98]]
    }
    pub fn ClassJobLevel(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[99]]
    }
    pub fn ClassJobLevelMax(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[100]]
    }
    pub fn StatusValue(&'a self) -> [&'a Field; 3] {
        [
            &self.row.columns[self.index_mapping[101]],
            &self.row.columns[self.index_mapping[102]],
            &self.row.columns[self.index_mapping[103]],
        ]
    }
    pub fn Unknown6(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[104]]
    }
    pub fn Unknown7(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[105]]
    }
    pub fn SpecialFate(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[106]]
    }
    pub fn Unknown8(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[107]]
    }
    pub fn AdventEvent(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[108]]
    }
    pub fn MoonFaireEvent(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[109]]
    }
    pub fn Unknown9(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[110]]
    }
}
