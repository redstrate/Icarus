//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScriptElement<'a> {
    pub ScriptInstruction: &'a str,
    pub ScriptArg: u32,
}
#[derive(Debug, Clone)]
pub struct CustomTalkSheet {
    sheet: Sheet,
}
impl CustomTalkSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("CustomTalk")?;
        let sheet = resolver.read_excel_sheet(&exh, "CustomTalk", language)?;
        Ok(Self { sheet })
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
        Some(Self::Row { row })
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
}
impl<'a> CustomTalkRow<'a> {
    pub fn Script(&'a self) -> [ScriptElement<'a>; 30] {
        [
            ScriptElement {
                ScriptInstruction: self.row.columns[3].into_string().unwrap(),
                ScriptArg: self.row.columns[33].into_u32().copied().unwrap(),
            },
            ScriptElement {
                ScriptInstruction: self.row.columns[4].into_string().unwrap(),
                ScriptArg: self.row.columns[34].into_u32().copied().unwrap(),
            },
            ScriptElement {
                ScriptInstruction: self.row.columns[5].into_string().unwrap(),
                ScriptArg: self.row.columns[35].into_u32().copied().unwrap(),
            },
            ScriptElement {
                ScriptInstruction: self.row.columns[6].into_string().unwrap(),
                ScriptArg: self.row.columns[36].into_u32().copied().unwrap(),
            },
            ScriptElement {
                ScriptInstruction: self.row.columns[7].into_string().unwrap(),
                ScriptArg: self.row.columns[37].into_u32().copied().unwrap(),
            },
            ScriptElement {
                ScriptInstruction: self.row.columns[8].into_string().unwrap(),
                ScriptArg: self.row.columns[38].into_u32().copied().unwrap(),
            },
            ScriptElement {
                ScriptInstruction: self.row.columns[9].into_string().unwrap(),
                ScriptArg: self.row.columns[39].into_u32().copied().unwrap(),
            },
            ScriptElement {
                ScriptInstruction: self.row.columns[10].into_string().unwrap(),
                ScriptArg: self.row.columns[40].into_u32().copied().unwrap(),
            },
            ScriptElement {
                ScriptInstruction: self.row.columns[11].into_string().unwrap(),
                ScriptArg: self.row.columns[41].into_u32().copied().unwrap(),
            },
            ScriptElement {
                ScriptInstruction: self.row.columns[12].into_string().unwrap(),
                ScriptArg: self.row.columns[42].into_u32().copied().unwrap(),
            },
            ScriptElement {
                ScriptInstruction: self.row.columns[13].into_string().unwrap(),
                ScriptArg: self.row.columns[43].into_u32().copied().unwrap(),
            },
            ScriptElement {
                ScriptInstruction: self.row.columns[14].into_string().unwrap(),
                ScriptArg: self.row.columns[44].into_u32().copied().unwrap(),
            },
            ScriptElement {
                ScriptInstruction: self.row.columns[15].into_string().unwrap(),
                ScriptArg: self.row.columns[45].into_u32().copied().unwrap(),
            },
            ScriptElement {
                ScriptInstruction: self.row.columns[16].into_string().unwrap(),
                ScriptArg: self.row.columns[46].into_u32().copied().unwrap(),
            },
            ScriptElement {
                ScriptInstruction: self.row.columns[17].into_string().unwrap(),
                ScriptArg: self.row.columns[47].into_u32().copied().unwrap(),
            },
            ScriptElement {
                ScriptInstruction: self.row.columns[18].into_string().unwrap(),
                ScriptArg: self.row.columns[48].into_u32().copied().unwrap(),
            },
            ScriptElement {
                ScriptInstruction: self.row.columns[19].into_string().unwrap(),
                ScriptArg: self.row.columns[49].into_u32().copied().unwrap(),
            },
            ScriptElement {
                ScriptInstruction: self.row.columns[20].into_string().unwrap(),
                ScriptArg: self.row.columns[50].into_u32().copied().unwrap(),
            },
            ScriptElement {
                ScriptInstruction: self.row.columns[21].into_string().unwrap(),
                ScriptArg: self.row.columns[51].into_u32().copied().unwrap(),
            },
            ScriptElement {
                ScriptInstruction: self.row.columns[22].into_string().unwrap(),
                ScriptArg: self.row.columns[52].into_u32().copied().unwrap(),
            },
            ScriptElement {
                ScriptInstruction: self.row.columns[23].into_string().unwrap(),
                ScriptArg: self.row.columns[53].into_u32().copied().unwrap(),
            },
            ScriptElement {
                ScriptInstruction: self.row.columns[24].into_string().unwrap(),
                ScriptArg: self.row.columns[54].into_u32().copied().unwrap(),
            },
            ScriptElement {
                ScriptInstruction: self.row.columns[25].into_string().unwrap(),
                ScriptArg: self.row.columns[55].into_u32().copied().unwrap(),
            },
            ScriptElement {
                ScriptInstruction: self.row.columns[26].into_string().unwrap(),
                ScriptArg: self.row.columns[56].into_u32().copied().unwrap(),
            },
            ScriptElement {
                ScriptInstruction: self.row.columns[27].into_string().unwrap(),
                ScriptArg: self.row.columns[57].into_u32().copied().unwrap(),
            },
            ScriptElement {
                ScriptInstruction: self.row.columns[28].into_string().unwrap(),
                ScriptArg: self.row.columns[58].into_u32().copied().unwrap(),
            },
            ScriptElement {
                ScriptInstruction: self.row.columns[29].into_string().unwrap(),
                ScriptArg: self.row.columns[59].into_u32().copied().unwrap(),
            },
            ScriptElement {
                ScriptInstruction: self.row.columns[30].into_string().unwrap(),
                ScriptArg: self.row.columns[60].into_u32().copied().unwrap(),
            },
            ScriptElement {
                ScriptInstruction: self.row.columns[31].into_string().unwrap(),
                ScriptArg: self.row.columns[61].into_u32().copied().unwrap(),
            },
            ScriptElement {
                ScriptInstruction: self.row.columns[32].into_string().unwrap(),
                ScriptArg: self.row.columns[62].into_u32().copied().unwrap(),
            },
        ]
    }
    pub fn MainOption(&'a self) -> &'a str {
        self.row.columns[64].into_string().unwrap()
    }
    pub fn SubOption(&'a self) -> &'a str {
        self.row.columns[65].into_string().unwrap()
    }
    pub fn Name(&'a self) -> &'a str {
        self.row.columns[2].into_string().unwrap()
    }
    pub fn IconActor(&'a self) -> u32 {
        self.row.columns[0].into_u32().copied().unwrap()
    }
    pub fn IconMap(&'a self) -> u32 {
        self.row.columns[1].into_u32().copied().unwrap()
    }
    pub fn SpecialLinks(&'a self) -> u32 {
        self.row.columns[75].into_u32().copied().unwrap()
    }
    pub fn Unknown0(&'a self) -> u8 {
        self.row.columns[76].into_u8().copied().unwrap()
    }
    pub fn Unknown1(&'a self) -> u8 {
        self.row.columns[77].into_u8().copied().unwrap()
    }
    pub fn Unknown2(&'a self) -> bool {
        self.row.columns[63].into_bool().copied().unwrap()
    }
    pub fn Unknown3(&'a self) -> bool {
        self.row.columns[66].into_bool().copied().unwrap()
    }
    pub fn Unknown4(&'a self) -> bool {
        self.row.columns[67].into_bool().copied().unwrap()
    }
    pub fn Unknown5(&'a self) -> bool {
        self.row.columns[68].into_bool().copied().unwrap()
    }
    pub fn Unknown6(&'a self) -> bool {
        self.row.columns[69].into_bool().copied().unwrap()
    }
    pub fn Unknown7(&'a self) -> bool {
        self.row.columns[70].into_bool().copied().unwrap()
    }
    pub fn Unknown8(&'a self) -> bool {
        self.row.columns[71].into_bool().copied().unwrap()
    }
    pub fn Unknown9(&'a self) -> bool {
        self.row.columns[72].into_bool().copied().unwrap()
    }
    pub fn Unknown10(&'a self) -> bool {
        self.row.columns[73].into_bool().copied().unwrap()
    }
    pub fn Unknown11(&'a self) -> bool {
        self.row.columns[74].into_bool().copied().unwrap()
    }
    pub fn Unknown12(&'a self) -> bool {
        self.row.columns[78].into_bool().copied().unwrap()
    }
}
