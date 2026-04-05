//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
pub struct SnipeDataElement {
    pub DataEventNPC: u32,
    pub Unknown0: u16,
    pub Unknown1: u16,
    pub Unknown2: u16,
    pub Unknown3: u16,
    pub Unknown4: u8,
    pub Unknown5: u8,
}
#[derive(Debug, Clone)]
pub struct SnipeSheet {
    sheet: Sheet,
}
impl SnipeSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Snipe")?;
        let sheet = resolver.read_excel_sheet(&exh, "Snipe", language)?;
        Ok(Self { sheet })
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
        Some(Self::Row { row })
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
}
impl<'a> SnipeRow<'a> {
    pub fn SnipeData(&'a self) -> [SnipeDataElement; 8] {
        [
            SnipeDataElement {
                DataEventNPC: self.row.columns[17].into_u32().copied().unwrap(),
                Unknown0: self.row.columns[25].into_u16().copied().unwrap(),
                Unknown1: self.row.columns[33].into_u16().copied().unwrap(),
                Unknown2: self.row.columns[49].into_u16().copied().unwrap(),
                Unknown3: self.row.columns[57].into_u16().copied().unwrap(),
                Unknown4: self.row.columns[41].into_u8().copied().unwrap(),
                Unknown5: self.row.columns[65].into_u8().copied().unwrap(),
            },
            SnipeDataElement {
                DataEventNPC: self.row.columns[18].into_u32().copied().unwrap(),
                Unknown0: self.row.columns[26].into_u16().copied().unwrap(),
                Unknown1: self.row.columns[34].into_u16().copied().unwrap(),
                Unknown2: self.row.columns[50].into_u16().copied().unwrap(),
                Unknown3: self.row.columns[58].into_u16().copied().unwrap(),
                Unknown4: self.row.columns[42].into_u8().copied().unwrap(),
                Unknown5: self.row.columns[66].into_u8().copied().unwrap(),
            },
            SnipeDataElement {
                DataEventNPC: self.row.columns[19].into_u32().copied().unwrap(),
                Unknown0: self.row.columns[27].into_u16().copied().unwrap(),
                Unknown1: self.row.columns[35].into_u16().copied().unwrap(),
                Unknown2: self.row.columns[51].into_u16().copied().unwrap(),
                Unknown3: self.row.columns[59].into_u16().copied().unwrap(),
                Unknown4: self.row.columns[43].into_u8().copied().unwrap(),
                Unknown5: self.row.columns[67].into_u8().copied().unwrap(),
            },
            SnipeDataElement {
                DataEventNPC: self.row.columns[20].into_u32().copied().unwrap(),
                Unknown0: self.row.columns[28].into_u16().copied().unwrap(),
                Unknown1: self.row.columns[36].into_u16().copied().unwrap(),
                Unknown2: self.row.columns[52].into_u16().copied().unwrap(),
                Unknown3: self.row.columns[60].into_u16().copied().unwrap(),
                Unknown4: self.row.columns[44].into_u8().copied().unwrap(),
                Unknown5: self.row.columns[68].into_u8().copied().unwrap(),
            },
            SnipeDataElement {
                DataEventNPC: self.row.columns[21].into_u32().copied().unwrap(),
                Unknown0: self.row.columns[29].into_u16().copied().unwrap(),
                Unknown1: self.row.columns[37].into_u16().copied().unwrap(),
                Unknown2: self.row.columns[53].into_u16().copied().unwrap(),
                Unknown3: self.row.columns[61].into_u16().copied().unwrap(),
                Unknown4: self.row.columns[45].into_u8().copied().unwrap(),
                Unknown5: self.row.columns[69].into_u8().copied().unwrap(),
            },
            SnipeDataElement {
                DataEventNPC: self.row.columns[22].into_u32().copied().unwrap(),
                Unknown0: self.row.columns[30].into_u16().copied().unwrap(),
                Unknown1: self.row.columns[38].into_u16().copied().unwrap(),
                Unknown2: self.row.columns[54].into_u16().copied().unwrap(),
                Unknown3: self.row.columns[62].into_u16().copied().unwrap(),
                Unknown4: self.row.columns[46].into_u8().copied().unwrap(),
                Unknown5: self.row.columns[70].into_u8().copied().unwrap(),
            },
            SnipeDataElement {
                DataEventNPC: self.row.columns[23].into_u32().copied().unwrap(),
                Unknown0: self.row.columns[31].into_u16().copied().unwrap(),
                Unknown1: self.row.columns[39].into_u16().copied().unwrap(),
                Unknown2: self.row.columns[55].into_u16().copied().unwrap(),
                Unknown3: self.row.columns[63].into_u16().copied().unwrap(),
                Unknown4: self.row.columns[47].into_u8().copied().unwrap(),
                Unknown5: self.row.columns[71].into_u8().copied().unwrap(),
            },
            SnipeDataElement {
                DataEventNPC: self.row.columns[24].into_u32().copied().unwrap(),
                Unknown0: self.row.columns[32].into_u16().copied().unwrap(),
                Unknown1: self.row.columns[40].into_u16().copied().unwrap(),
                Unknown2: self.row.columns[56].into_u16().copied().unwrap(),
                Unknown3: self.row.columns[64].into_u16().copied().unwrap(),
                Unknown4: self.row.columns[48].into_u8().copied().unwrap(),
                Unknown5: self.row.columns[72].into_u8().copied().unwrap(),
            },
        ]
    }
    pub fn EventNPC(&'a self) -> [u32; 8] {
        [
            self.row.columns[73].into_u32().copied().unwrap(),
            self.row.columns[74].into_u32().copied().unwrap(),
            self.row.columns[75].into_u32().copied().unwrap(),
            self.row.columns[76].into_u32().copied().unwrap(),
            self.row.columns[77].into_u32().copied().unwrap(),
            self.row.columns[78].into_u32().copied().unwrap(),
            self.row.columns[79].into_u32().copied().unwrap(),
            self.row.columns[80].into_u32().copied().unwrap(),
        ]
    }
    pub fn Unknown0(&'a self) -> u32 {
        self.row.columns[89].into_u32().copied().unwrap()
    }
    pub fn Unknown1(&'a self) -> u16 {
        self.row.columns[81].into_u16().copied().unwrap()
    }
    pub fn Unknown2(&'a self) -> u8 {
        self.row.columns[85].into_u8().copied().unwrap()
    }
    pub fn Unknown3(&'a self) -> u32 {
        self.row.columns[90].into_u32().copied().unwrap()
    }
    pub fn Unknown4(&'a self) -> u16 {
        self.row.columns[82].into_u16().copied().unwrap()
    }
    pub fn Unknown5(&'a self) -> u8 {
        self.row.columns[86].into_u8().copied().unwrap()
    }
    pub fn Unknown6(&'a self) -> u32 {
        self.row.columns[91].into_u32().copied().unwrap()
    }
    pub fn Unknown7(&'a self) -> u16 {
        self.row.columns[83].into_u16().copied().unwrap()
    }
    pub fn Unknown8(&'a self) -> u8 {
        self.row.columns[87].into_u8().copied().unwrap()
    }
    pub fn Unknown9(&'a self) -> u32 {
        self.row.columns[92].into_u32().copied().unwrap()
    }
    pub fn Unknown10(&'a self) -> u16 {
        self.row.columns[84].into_u16().copied().unwrap()
    }
    pub fn Unknown11(&'a self) -> u8 {
        self.row.columns[88].into_u8().copied().unwrap()
    }
    pub fn Objective0(&'a self) -> &'a str {
        self.row.columns[93].into_string().unwrap()
    }
    pub fn Hint0(&'a self) -> &'a str {
        self.row.columns[94].into_string().unwrap()
    }
    pub fn Objective1(&'a self) -> &'a str {
        self.row.columns[95].into_string().unwrap()
    }
    pub fn Hint1(&'a self) -> &'a str {
        self.row.columns[96].into_string().unwrap()
    }
    pub fn Unknown12(&'a self) -> &'a str {
        self.row.columns[97].into_string().unwrap()
    }
    pub fn Unknown13(&'a self) -> &'a str {
        self.row.columns[98].into_string().unwrap()
    }
    pub fn Unknown14(&'a self) -> &'a str {
        self.row.columns[99].into_string().unwrap()
    }
    pub fn Unknown15(&'a self) -> &'a str {
        self.row.columns[100].into_string().unwrap()
    }
    pub fn Unknown16(&'a self) -> &'a str {
        self.row.columns[101].into_string().unwrap()
    }
    pub fn Unknown17(&'a self) -> &'a str {
        self.row.columns[102].into_string().unwrap()
    }
    pub fn Unknown18(&'a self) -> &'a str {
        self.row.columns[103].into_string().unwrap()
    }
    pub fn ActionText(&'a self) -> &'a str {
        self.row.columns[104].into_string().unwrap()
    }
    pub fn Unknown19(&'a self) -> u8 {
        self.row.columns[105].into_u8().copied().unwrap()
    }
    pub fn Unknown20(&'a self) -> u8 {
        self.row.columns[106].into_u8().copied().unwrap()
    }
    pub fn VFXFire(&'a self) -> &'a str {
        self.row.columns[11].into_string().unwrap()
    }
    pub fn VFXHit(&'a self) -> &'a str {
        self.row.columns[12].into_string().unwrap()
    }
    pub fn VFXMiss(&'a self) -> &'a str {
        self.row.columns[13].into_string().unwrap()
    }
    pub fn VFXAdditional(&'a self) -> &'a str {
        self.row.columns[14].into_string().unwrap()
    }
    pub fn LGBTargetMarker(&'a self) -> u32 {
        self.row.columns[0].into_u32().copied().unwrap()
    }
    pub fn Unknown21(&'a self) -> u16 {
        self.row.columns[1].into_u16().copied().unwrap()
    }
    pub fn Unknown22(&'a self) -> u16 {
        self.row.columns[8].into_u16().copied().unwrap()
    }
    pub fn Unknown23(&'a self) -> u16 {
        self.row.columns[15].into_u16().copied().unwrap()
    }
    pub fn Unknown24(&'a self) -> u8 {
        self.row.columns[3].into_u8().copied().unwrap()
    }
    pub fn Unknown25(&'a self) -> u8 {
        self.row.columns[4].into_u8().copied().unwrap()
    }
    pub fn Unknown26(&'a self) -> u8 {
        self.row.columns[5].into_u8().copied().unwrap()
    }
    pub fn Unknown27(&'a self) -> u8 {
        self.row.columns[6].into_u8().copied().unwrap()
    }
    pub fn Unknown28(&'a self) -> u8 {
        self.row.columns[7].into_u8().copied().unwrap()
    }
    pub fn Unknown29(&'a self) -> u8 {
        self.row.columns[9].into_u8().copied().unwrap()
    }
    pub fn Unknown30(&'a self) -> u8 {
        self.row.columns[10].into_u8().copied().unwrap()
    }
    pub fn Unknown31(&'a self) -> bool {
        self.row.columns[2].into_bool().copied().unwrap()
    }
    pub fn Unknown32(&'a self) -> bool {
        self.row.columns[16].into_bool().copied().unwrap()
    }
}
