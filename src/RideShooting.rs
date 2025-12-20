//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    resource::{Resource, read_excel_sheet_header, read_excel_sheet},
    exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow},
    exh::{EXH, ExcelColumnDefinition},
    common::Language,
};
pub struct RideShootingParamsElement<'a> {
    pub Unknown0: &'a ColumnData,
    pub PopRange: &'a ColumnData,
    pub ENpc: &'a ColumnData,
    pub Unknown1: &'a ColumnData,
    pub Unknown2: &'a ColumnData,
    pub Unknown3: &'a ColumnData,
    pub Unknown4: &'a ColumnData,
    pub Unknown5: &'a ColumnData,
    pub ENpcScale: &'a ColumnData,
    pub Unknown6: &'a ColumnData,
    pub Unknown7: &'a ColumnData,
    pub Unknown8: &'a ColumnData,
    pub Unknown9: &'a ColumnData,
    pub Unknown10: &'a ColumnData,
}
pub struct RideShootingSheet {
    pages: Vec<EXD>,
    exh: EXH,
    row_count: u32,
}
impl RideShootingSheet {
    /// Read the sheet from a `Resource`.
    pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
        let exh = read_excel_sheet_header(resource, "RideShooting")?;
        let mut pages = Vec::new();
        for (i, _) in exh.pages.iter().enumerate() {
            pages.push(read_excel_sheet(resource, "RideShooting", &exh, language, i)?);
        }
        let row_count = exh.header.row_count;
        Some(Self { exh, pages, row_count })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<RideShootingRow> {
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
        Some(RideShootingRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<RideShootingRow> {
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
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<RideShootingRow> {
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
pub struct RideShootingRow {
    columns: Vec<ColumnData>,
}
impl RideShootingRow {
    pub fn RideShootingParams<'a>(&'a self) -> [RideShootingParamsElement<'a>; 8] {
        [
            RideShootingParamsElement {
                Unknown0: &self.columns[0],
                PopRange: &self.columns[1],
                ENpc: &self.columns[2],
                Unknown1: &self.columns[3],
                Unknown2: &self.columns[4],
                Unknown3: &self.columns[5],
                Unknown4: &self.columns[6],
                Unknown5: &self.columns[7],
                ENpcScale: &self.columns[8],
                Unknown6: &self.columns[9],
                Unknown7: &self.columns[10],
                Unknown8: &self.columns[11],
                Unknown9: &self.columns[12],
                Unknown10: &self.columns[13],
            },
            RideShootingParamsElement {
                Unknown0: &self.columns[14],
                PopRange: &self.columns[15],
                ENpc: &self.columns[16],
                Unknown1: &self.columns[17],
                Unknown2: &self.columns[18],
                Unknown3: &self.columns[19],
                Unknown4: &self.columns[20],
                Unknown5: &self.columns[21],
                ENpcScale: &self.columns[22],
                Unknown6: &self.columns[23],
                Unknown7: &self.columns[24],
                Unknown8: &self.columns[25],
                Unknown9: &self.columns[26],
                Unknown10: &self.columns[27],
            },
            RideShootingParamsElement {
                Unknown0: &self.columns[28],
                PopRange: &self.columns[29],
                ENpc: &self.columns[30],
                Unknown1: &self.columns[31],
                Unknown2: &self.columns[32],
                Unknown3: &self.columns[33],
                Unknown4: &self.columns[34],
                Unknown5: &self.columns[35],
                ENpcScale: &self.columns[36],
                Unknown6: &self.columns[37],
                Unknown7: &self.columns[38],
                Unknown8: &self.columns[39],
                Unknown9: &self.columns[40],
                Unknown10: &self.columns[41],
            },
            RideShootingParamsElement {
                Unknown0: &self.columns[42],
                PopRange: &self.columns[43],
                ENpc: &self.columns[44],
                Unknown1: &self.columns[45],
                Unknown2: &self.columns[46],
                Unknown3: &self.columns[47],
                Unknown4: &self.columns[48],
                Unknown5: &self.columns[49],
                ENpcScale: &self.columns[50],
                Unknown6: &self.columns[51],
                Unknown7: &self.columns[52],
                Unknown8: &self.columns[53],
                Unknown9: &self.columns[54],
                Unknown10: &self.columns[55],
            },
            RideShootingParamsElement {
                Unknown0: &self.columns[56],
                PopRange: &self.columns[57],
                ENpc: &self.columns[58],
                Unknown1: &self.columns[59],
                Unknown2: &self.columns[60],
                Unknown3: &self.columns[61],
                Unknown4: &self.columns[62],
                Unknown5: &self.columns[63],
                ENpcScale: &self.columns[64],
                Unknown6: &self.columns[65],
                Unknown7: &self.columns[66],
                Unknown8: &self.columns[67],
                Unknown9: &self.columns[68],
                Unknown10: &self.columns[69],
            },
            RideShootingParamsElement {
                Unknown0: &self.columns[70],
                PopRange: &self.columns[71],
                ENpc: &self.columns[72],
                Unknown1: &self.columns[73],
                Unknown2: &self.columns[74],
                Unknown3: &self.columns[75],
                Unknown4: &self.columns[76],
                Unknown5: &self.columns[77],
                ENpcScale: &self.columns[78],
                Unknown6: &self.columns[79],
                Unknown7: &self.columns[80],
                Unknown8: &self.columns[81],
                Unknown9: &self.columns[82],
                Unknown10: &self.columns[83],
            },
            RideShootingParamsElement {
                Unknown0: &self.columns[84],
                PopRange: &self.columns[85],
                ENpc: &self.columns[86],
                Unknown1: &self.columns[87],
                Unknown2: &self.columns[88],
                Unknown3: &self.columns[89],
                Unknown4: &self.columns[90],
                Unknown5: &self.columns[91],
                ENpcScale: &self.columns[92],
                Unknown6: &self.columns[93],
                Unknown7: &self.columns[94],
                Unknown8: &self.columns[95],
                Unknown9: &self.columns[96],
                Unknown10: &self.columns[97],
            },
            RideShootingParamsElement {
                Unknown0: &self.columns[98],
                PopRange: &self.columns[99],
                ENpc: &self.columns[100],
                Unknown1: &self.columns[101],
                Unknown2: &self.columns[102],
                Unknown3: &self.columns[103],
                Unknown4: &self.columns[104],
                Unknown5: &self.columns[105],
                ENpcScale: &self.columns[106],
                Unknown6: &self.columns[107],
                Unknown7: &self.columns[108],
                Unknown8: &self.columns[109],
                Unknown9: &self.columns[110],
                Unknown10: &self.columns[111],
            },
        ]
    }
    pub fn GFateRideShooting<'a>(&'a self) -> &'a ColumnData {
        &self.columns[112]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a ColumnData {
        &self.columns[113]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[114]
    }
    pub fn StartText<'a>(&'a self) -> &'a ColumnData {
        &self.columns[115]
    }
    pub fn Unknown2<'a>(&'a self) -> &'a ColumnData {
        &self.columns[116]
    }
    pub fn Unknown3<'a>(&'a self) -> &'a ColumnData {
        &self.columns[117]
    }
}
