//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    resource::{Resource, read_excel_sheet_header, read_excel_sheet},
    exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow},
    exh::{EXH, ExcelColumnDefinition},
    common::Language,
};
pub struct SnipeDataElement<'a> {
    DataEventNPC: &'a ColumnData,
    Unknown0: &'a ColumnData,
    Unknown1: &'a ColumnData,
    Unknown2: &'a ColumnData,
    Unknown3: &'a ColumnData,
    Unknown4: &'a ColumnData,
    Unknown5: &'a ColumnData,
}
pub struct SnipeSheet {
    pages: Vec<EXD>,
    exh: EXH,
    row_count: u32,
}
impl SnipeSheet {
    /// Read the sheet from a `Resource`.
    pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
        let exh = read_excel_sheet_header(resource, "Snipe")?;
        let mut pages = Vec::new();
        for (i, _) in exh.pages.iter().enumerate() {
            pages.push(read_excel_sheet(resource, "Snipe", &exh, language, i)?);
        }
        let row_count = exh.header.row_count;
        Some(Self { exh, pages, row_count })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<SnipeRow> {
        let column_defs = &self.exh.column_definitions;
        let mut zipped: Vec<_> = row
            .columns
            .clone()
            .into_iter()
            .zip(column_defs)
            .collect();
        zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
        let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition>) = zipped
            .into_iter()
            .unzip();
        Some(SnipeRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<SnipeRow> {
        for page in &self.pages {
            let Some(row) = &page.get_row(row_id) else {
                continue;
            };
            let row = match row {
                ExcelRowKind::SingleRow(row) => row,
                ExcelRowKind::SubRows(rows) => &rows.first()?.1,
            };
            return self.read_row(row);
        }
        None
    }
    /// Fetches the specified subrow from the sheet.
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<SnipeRow> {
        for page in &self.pages {
            let Some(row) = &page.get_row(row_id) else {
                continue;
            };
            let row = match row {
                ExcelRowKind::SingleRow(row) => return None,
                ExcelRowKind::SubRows(subrows) => {
                    &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1
                }
            };
            return self.read_row(row);
        }
        None
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.row_count
    }
}
pub struct SnipeRow {
    columns: Vec<ColumnData>,
}
impl SnipeRow {
    pub fn SnipeData<'a>(&'a self) -> [SnipeDataElement<'a>; 8] {
        [
            SnipeDataElement {
                DataEventNPC: &self.columns[0],
                Unknown0: &self.columns[1],
                Unknown1: &self.columns[2],
                Unknown2: &self.columns[3],
                Unknown3: &self.columns[4],
                Unknown4: &self.columns[5],
                Unknown5: &self.columns[6],
            },
            SnipeDataElement {
                DataEventNPC: &self.columns[7],
                Unknown0: &self.columns[8],
                Unknown1: &self.columns[9],
                Unknown2: &self.columns[10],
                Unknown3: &self.columns[11],
                Unknown4: &self.columns[12],
                Unknown5: &self.columns[13],
            },
            SnipeDataElement {
                DataEventNPC: &self.columns[14],
                Unknown0: &self.columns[15],
                Unknown1: &self.columns[16],
                Unknown2: &self.columns[17],
                Unknown3: &self.columns[18],
                Unknown4: &self.columns[19],
                Unknown5: &self.columns[20],
            },
            SnipeDataElement {
                DataEventNPC: &self.columns[21],
                Unknown0: &self.columns[22],
                Unknown1: &self.columns[23],
                Unknown2: &self.columns[24],
                Unknown3: &self.columns[25],
                Unknown4: &self.columns[26],
                Unknown5: &self.columns[27],
            },
            SnipeDataElement {
                DataEventNPC: &self.columns[28],
                Unknown0: &self.columns[29],
                Unknown1: &self.columns[30],
                Unknown2: &self.columns[31],
                Unknown3: &self.columns[32],
                Unknown4: &self.columns[33],
                Unknown5: &self.columns[34],
            },
            SnipeDataElement {
                DataEventNPC: &self.columns[35],
                Unknown0: &self.columns[36],
                Unknown1: &self.columns[37],
                Unknown2: &self.columns[38],
                Unknown3: &self.columns[39],
                Unknown4: &self.columns[40],
                Unknown5: &self.columns[41],
            },
            SnipeDataElement {
                DataEventNPC: &self.columns[42],
                Unknown0: &self.columns[43],
                Unknown1: &self.columns[44],
                Unknown2: &self.columns[45],
                Unknown3: &self.columns[46],
                Unknown4: &self.columns[47],
                Unknown5: &self.columns[48],
            },
            SnipeDataElement {
                DataEventNPC: &self.columns[49],
                Unknown0: &self.columns[50],
                Unknown1: &self.columns[51],
                Unknown2: &self.columns[52],
                Unknown3: &self.columns[53],
                Unknown4: &self.columns[54],
                Unknown5: &self.columns[55],
            },
        ]
    }
    pub fn EventNPC<'a>(&'a self) -> [&'a ColumnData; 8] {
        [
            &self.columns[56],
            &self.columns[57],
            &self.columns[58],
            &self.columns[59],
            &self.columns[60],
            &self.columns[61],
            &self.columns[62],
            &self.columns[63],
        ]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a ColumnData {
        &self.columns[64]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[65]
    }
    pub fn Unknown2<'a>(&'a self) -> &'a ColumnData {
        &self.columns[66]
    }
    pub fn Unknown3<'a>(&'a self) -> &'a ColumnData {
        &self.columns[67]
    }
    pub fn Unknown4<'a>(&'a self) -> &'a ColumnData {
        &self.columns[68]
    }
    pub fn Unknown5<'a>(&'a self) -> &'a ColumnData {
        &self.columns[69]
    }
    pub fn Unknown6<'a>(&'a self) -> &'a ColumnData {
        &self.columns[70]
    }
    pub fn Unknown7<'a>(&'a self) -> &'a ColumnData {
        &self.columns[71]
    }
    pub fn Unknown8<'a>(&'a self) -> &'a ColumnData {
        &self.columns[72]
    }
    pub fn Unknown9<'a>(&'a self) -> &'a ColumnData {
        &self.columns[73]
    }
    pub fn Unknown10<'a>(&'a self) -> &'a ColumnData {
        &self.columns[74]
    }
    pub fn Unknown11<'a>(&'a self) -> &'a ColumnData {
        &self.columns[75]
    }
    pub fn Objective0<'a>(&'a self) -> &'a ColumnData {
        &self.columns[76]
    }
    pub fn Hint0<'a>(&'a self) -> &'a ColumnData {
        &self.columns[77]
    }
    pub fn Objective1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[78]
    }
    pub fn Hint1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[79]
    }
    pub fn Unknown12<'a>(&'a self) -> &'a ColumnData {
        &self.columns[80]
    }
    pub fn Unknown13<'a>(&'a self) -> &'a ColumnData {
        &self.columns[81]
    }
    pub fn Unknown14<'a>(&'a self) -> &'a ColumnData {
        &self.columns[82]
    }
    pub fn Unknown15<'a>(&'a self) -> &'a ColumnData {
        &self.columns[83]
    }
    pub fn Unknown16<'a>(&'a self) -> &'a ColumnData {
        &self.columns[84]
    }
    pub fn Unknown17<'a>(&'a self) -> &'a ColumnData {
        &self.columns[85]
    }
    pub fn Unknown18<'a>(&'a self) -> &'a ColumnData {
        &self.columns[86]
    }
    pub fn ActionText<'a>(&'a self) -> &'a ColumnData {
        &self.columns[87]
    }
    pub fn Unknown19<'a>(&'a self) -> &'a ColumnData {
        &self.columns[88]
    }
    pub fn Unknown20<'a>(&'a self) -> &'a ColumnData {
        &self.columns[89]
    }
    pub fn VFXFire<'a>(&'a self) -> &'a ColumnData {
        &self.columns[90]
    }
    pub fn VFXHit<'a>(&'a self) -> &'a ColumnData {
        &self.columns[91]
    }
    pub fn VFXMiss<'a>(&'a self) -> &'a ColumnData {
        &self.columns[92]
    }
    pub fn VFXAdditional<'a>(&'a self) -> &'a ColumnData {
        &self.columns[93]
    }
    pub fn LGBTargetMarker<'a>(&'a self) -> &'a ColumnData {
        &self.columns[94]
    }
    pub fn Unknown21<'a>(&'a self) -> &'a ColumnData {
        &self.columns[95]
    }
    pub fn Unknown22<'a>(&'a self) -> &'a ColumnData {
        &self.columns[96]
    }
    pub fn Unknown23<'a>(&'a self) -> &'a ColumnData {
        &self.columns[97]
    }
    pub fn Unknown24<'a>(&'a self) -> &'a ColumnData {
        &self.columns[98]
    }
    pub fn Unknown25<'a>(&'a self) -> &'a ColumnData {
        &self.columns[99]
    }
    pub fn Unknown26<'a>(&'a self) -> &'a ColumnData {
        &self.columns[100]
    }
    pub fn Unknown27<'a>(&'a self) -> &'a ColumnData {
        &self.columns[101]
    }
    pub fn Unknown28<'a>(&'a self) -> &'a ColumnData {
        &self.columns[102]
    }
    pub fn Unknown29<'a>(&'a self) -> &'a ColumnData {
        &self.columns[103]
    }
    pub fn Unknown30<'a>(&'a self) -> &'a ColumnData {
        &self.columns[104]
    }
    pub fn Unknown31<'a>(&'a self) -> &'a ColumnData {
        &self.columns[105]
    }
    pub fn Unknown32<'a>(&'a self) -> &'a ColumnData {
        &self.columns[106]
    }
}
