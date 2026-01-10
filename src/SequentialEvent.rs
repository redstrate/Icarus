//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
pub struct UnknownStructElement<'a> {
    pub Unknown1: &'a Field,
    pub Unknown2: &'a Field,
    pub Unknown3: &'a Field,
    pub Unknown4: &'a Field,
    pub Unknown5: &'a Field,
}
#[derive(Debug, Clone)]
pub struct SequentialEventSheet {
    sheet: Sheet,
}
impl SequentialEventSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("SequentialEvent")?;
        let sheet = resolver.read_excel_sheet(&exh, "SequentialEvent", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<SequentialEventRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<SequentialEventRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for SequentialEventSheet {
    type Row = SequentialEventRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        let column_defs = &self.sheet.exh.column_definitions;
        let mut zipped: Vec<_> = row
            .columns
            .clone()
            .into_iter()
            .zip(column_defs)
            .collect();
        zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
        let (columns, _): (Vec<Field>, Vec<ExcelColumnDefinition>) = zipped
            .into_iter()
            .unzip();
        Some(Self::Row { columns })
    }
}
impl<'a> IntoIterator for &'a SequentialEventSheet {
    type Item = (u32, Vec<(u16, SequentialEventRow)>);
    type IntoIter = StructuredSheetIterator<'a, SequentialEventSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, SequentialEventSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct SequentialEventRow {
    columns: Vec<Field>,
}
impl SequentialEventRow {
    pub fn UnknownStruct<'a>(&'a self) -> [UnknownStructElement<'a>; 64] {
        [
            UnknownStructElement {
                Unknown1: &self.columns[0],
                Unknown2: &self.columns[1],
                Unknown3: &self.columns[2],
                Unknown4: &self.columns[3],
                Unknown5: &self.columns[4],
            },
            UnknownStructElement {
                Unknown1: &self.columns[5],
                Unknown2: &self.columns[6],
                Unknown3: &self.columns[7],
                Unknown4: &self.columns[8],
                Unknown5: &self.columns[9],
            },
            UnknownStructElement {
                Unknown1: &self.columns[10],
                Unknown2: &self.columns[11],
                Unknown3: &self.columns[12],
                Unknown4: &self.columns[13],
                Unknown5: &self.columns[14],
            },
            UnknownStructElement {
                Unknown1: &self.columns[15],
                Unknown2: &self.columns[16],
                Unknown3: &self.columns[17],
                Unknown4: &self.columns[18],
                Unknown5: &self.columns[19],
            },
            UnknownStructElement {
                Unknown1: &self.columns[20],
                Unknown2: &self.columns[21],
                Unknown3: &self.columns[22],
                Unknown4: &self.columns[23],
                Unknown5: &self.columns[24],
            },
            UnknownStructElement {
                Unknown1: &self.columns[25],
                Unknown2: &self.columns[26],
                Unknown3: &self.columns[27],
                Unknown4: &self.columns[28],
                Unknown5: &self.columns[29],
            },
            UnknownStructElement {
                Unknown1: &self.columns[30],
                Unknown2: &self.columns[31],
                Unknown3: &self.columns[32],
                Unknown4: &self.columns[33],
                Unknown5: &self.columns[34],
            },
            UnknownStructElement {
                Unknown1: &self.columns[35],
                Unknown2: &self.columns[36],
                Unknown3: &self.columns[37],
                Unknown4: &self.columns[38],
                Unknown5: &self.columns[39],
            },
            UnknownStructElement {
                Unknown1: &self.columns[40],
                Unknown2: &self.columns[41],
                Unknown3: &self.columns[42],
                Unknown4: &self.columns[43],
                Unknown5: &self.columns[44],
            },
            UnknownStructElement {
                Unknown1: &self.columns[45],
                Unknown2: &self.columns[46],
                Unknown3: &self.columns[47],
                Unknown4: &self.columns[48],
                Unknown5: &self.columns[49],
            },
            UnknownStructElement {
                Unknown1: &self.columns[50],
                Unknown2: &self.columns[51],
                Unknown3: &self.columns[52],
                Unknown4: &self.columns[53],
                Unknown5: &self.columns[54],
            },
            UnknownStructElement {
                Unknown1: &self.columns[55],
                Unknown2: &self.columns[56],
                Unknown3: &self.columns[57],
                Unknown4: &self.columns[58],
                Unknown5: &self.columns[59],
            },
            UnknownStructElement {
                Unknown1: &self.columns[60],
                Unknown2: &self.columns[61],
                Unknown3: &self.columns[62],
                Unknown4: &self.columns[63],
                Unknown5: &self.columns[64],
            },
            UnknownStructElement {
                Unknown1: &self.columns[65],
                Unknown2: &self.columns[66],
                Unknown3: &self.columns[67],
                Unknown4: &self.columns[68],
                Unknown5: &self.columns[69],
            },
            UnknownStructElement {
                Unknown1: &self.columns[70],
                Unknown2: &self.columns[71],
                Unknown3: &self.columns[72],
                Unknown4: &self.columns[73],
                Unknown5: &self.columns[74],
            },
            UnknownStructElement {
                Unknown1: &self.columns[75],
                Unknown2: &self.columns[76],
                Unknown3: &self.columns[77],
                Unknown4: &self.columns[78],
                Unknown5: &self.columns[79],
            },
            UnknownStructElement {
                Unknown1: &self.columns[80],
                Unknown2: &self.columns[81],
                Unknown3: &self.columns[82],
                Unknown4: &self.columns[83],
                Unknown5: &self.columns[84],
            },
            UnknownStructElement {
                Unknown1: &self.columns[85],
                Unknown2: &self.columns[86],
                Unknown3: &self.columns[87],
                Unknown4: &self.columns[88],
                Unknown5: &self.columns[89],
            },
            UnknownStructElement {
                Unknown1: &self.columns[90],
                Unknown2: &self.columns[91],
                Unknown3: &self.columns[92],
                Unknown4: &self.columns[93],
                Unknown5: &self.columns[94],
            },
            UnknownStructElement {
                Unknown1: &self.columns[95],
                Unknown2: &self.columns[96],
                Unknown3: &self.columns[97],
                Unknown4: &self.columns[98],
                Unknown5: &self.columns[99],
            },
            UnknownStructElement {
                Unknown1: &self.columns[100],
                Unknown2: &self.columns[101],
                Unknown3: &self.columns[102],
                Unknown4: &self.columns[103],
                Unknown5: &self.columns[104],
            },
            UnknownStructElement {
                Unknown1: &self.columns[105],
                Unknown2: &self.columns[106],
                Unknown3: &self.columns[107],
                Unknown4: &self.columns[108],
                Unknown5: &self.columns[109],
            },
            UnknownStructElement {
                Unknown1: &self.columns[110],
                Unknown2: &self.columns[111],
                Unknown3: &self.columns[112],
                Unknown4: &self.columns[113],
                Unknown5: &self.columns[114],
            },
            UnknownStructElement {
                Unknown1: &self.columns[115],
                Unknown2: &self.columns[116],
                Unknown3: &self.columns[117],
                Unknown4: &self.columns[118],
                Unknown5: &self.columns[119],
            },
            UnknownStructElement {
                Unknown1: &self.columns[120],
                Unknown2: &self.columns[121],
                Unknown3: &self.columns[122],
                Unknown4: &self.columns[123],
                Unknown5: &self.columns[124],
            },
            UnknownStructElement {
                Unknown1: &self.columns[125],
                Unknown2: &self.columns[126],
                Unknown3: &self.columns[127],
                Unknown4: &self.columns[128],
                Unknown5: &self.columns[129],
            },
            UnknownStructElement {
                Unknown1: &self.columns[130],
                Unknown2: &self.columns[131],
                Unknown3: &self.columns[132],
                Unknown4: &self.columns[133],
                Unknown5: &self.columns[134],
            },
            UnknownStructElement {
                Unknown1: &self.columns[135],
                Unknown2: &self.columns[136],
                Unknown3: &self.columns[137],
                Unknown4: &self.columns[138],
                Unknown5: &self.columns[139],
            },
            UnknownStructElement {
                Unknown1: &self.columns[140],
                Unknown2: &self.columns[141],
                Unknown3: &self.columns[142],
                Unknown4: &self.columns[143],
                Unknown5: &self.columns[144],
            },
            UnknownStructElement {
                Unknown1: &self.columns[145],
                Unknown2: &self.columns[146],
                Unknown3: &self.columns[147],
                Unknown4: &self.columns[148],
                Unknown5: &self.columns[149],
            },
            UnknownStructElement {
                Unknown1: &self.columns[150],
                Unknown2: &self.columns[151],
                Unknown3: &self.columns[152],
                Unknown4: &self.columns[153],
                Unknown5: &self.columns[154],
            },
            UnknownStructElement {
                Unknown1: &self.columns[155],
                Unknown2: &self.columns[156],
                Unknown3: &self.columns[157],
                Unknown4: &self.columns[158],
                Unknown5: &self.columns[159],
            },
            UnknownStructElement {
                Unknown1: &self.columns[160],
                Unknown2: &self.columns[161],
                Unknown3: &self.columns[162],
                Unknown4: &self.columns[163],
                Unknown5: &self.columns[164],
            },
            UnknownStructElement {
                Unknown1: &self.columns[165],
                Unknown2: &self.columns[166],
                Unknown3: &self.columns[167],
                Unknown4: &self.columns[168],
                Unknown5: &self.columns[169],
            },
            UnknownStructElement {
                Unknown1: &self.columns[170],
                Unknown2: &self.columns[171],
                Unknown3: &self.columns[172],
                Unknown4: &self.columns[173],
                Unknown5: &self.columns[174],
            },
            UnknownStructElement {
                Unknown1: &self.columns[175],
                Unknown2: &self.columns[176],
                Unknown3: &self.columns[177],
                Unknown4: &self.columns[178],
                Unknown5: &self.columns[179],
            },
            UnknownStructElement {
                Unknown1: &self.columns[180],
                Unknown2: &self.columns[181],
                Unknown3: &self.columns[182],
                Unknown4: &self.columns[183],
                Unknown5: &self.columns[184],
            },
            UnknownStructElement {
                Unknown1: &self.columns[185],
                Unknown2: &self.columns[186],
                Unknown3: &self.columns[187],
                Unknown4: &self.columns[188],
                Unknown5: &self.columns[189],
            },
            UnknownStructElement {
                Unknown1: &self.columns[190],
                Unknown2: &self.columns[191],
                Unknown3: &self.columns[192],
                Unknown4: &self.columns[193],
                Unknown5: &self.columns[194],
            },
            UnknownStructElement {
                Unknown1: &self.columns[195],
                Unknown2: &self.columns[196],
                Unknown3: &self.columns[197],
                Unknown4: &self.columns[198],
                Unknown5: &self.columns[199],
            },
            UnknownStructElement {
                Unknown1: &self.columns[200],
                Unknown2: &self.columns[201],
                Unknown3: &self.columns[202],
                Unknown4: &self.columns[203],
                Unknown5: &self.columns[204],
            },
            UnknownStructElement {
                Unknown1: &self.columns[205],
                Unknown2: &self.columns[206],
                Unknown3: &self.columns[207],
                Unknown4: &self.columns[208],
                Unknown5: &self.columns[209],
            },
            UnknownStructElement {
                Unknown1: &self.columns[210],
                Unknown2: &self.columns[211],
                Unknown3: &self.columns[212],
                Unknown4: &self.columns[213],
                Unknown5: &self.columns[214],
            },
            UnknownStructElement {
                Unknown1: &self.columns[215],
                Unknown2: &self.columns[216],
                Unknown3: &self.columns[217],
                Unknown4: &self.columns[218],
                Unknown5: &self.columns[219],
            },
            UnknownStructElement {
                Unknown1: &self.columns[220],
                Unknown2: &self.columns[221],
                Unknown3: &self.columns[222],
                Unknown4: &self.columns[223],
                Unknown5: &self.columns[224],
            },
            UnknownStructElement {
                Unknown1: &self.columns[225],
                Unknown2: &self.columns[226],
                Unknown3: &self.columns[227],
                Unknown4: &self.columns[228],
                Unknown5: &self.columns[229],
            },
            UnknownStructElement {
                Unknown1: &self.columns[230],
                Unknown2: &self.columns[231],
                Unknown3: &self.columns[232],
                Unknown4: &self.columns[233],
                Unknown5: &self.columns[234],
            },
            UnknownStructElement {
                Unknown1: &self.columns[235],
                Unknown2: &self.columns[236],
                Unknown3: &self.columns[237],
                Unknown4: &self.columns[238],
                Unknown5: &self.columns[239],
            },
            UnknownStructElement {
                Unknown1: &self.columns[240],
                Unknown2: &self.columns[241],
                Unknown3: &self.columns[242],
                Unknown4: &self.columns[243],
                Unknown5: &self.columns[244],
            },
            UnknownStructElement {
                Unknown1: &self.columns[245],
                Unknown2: &self.columns[246],
                Unknown3: &self.columns[247],
                Unknown4: &self.columns[248],
                Unknown5: &self.columns[249],
            },
            UnknownStructElement {
                Unknown1: &self.columns[250],
                Unknown2: &self.columns[251],
                Unknown3: &self.columns[252],
                Unknown4: &self.columns[253],
                Unknown5: &self.columns[254],
            },
            UnknownStructElement {
                Unknown1: &self.columns[255],
                Unknown2: &self.columns[256],
                Unknown3: &self.columns[257],
                Unknown4: &self.columns[258],
                Unknown5: &self.columns[259],
            },
            UnknownStructElement {
                Unknown1: &self.columns[260],
                Unknown2: &self.columns[261],
                Unknown3: &self.columns[262],
                Unknown4: &self.columns[263],
                Unknown5: &self.columns[264],
            },
            UnknownStructElement {
                Unknown1: &self.columns[265],
                Unknown2: &self.columns[266],
                Unknown3: &self.columns[267],
                Unknown4: &self.columns[268],
                Unknown5: &self.columns[269],
            },
            UnknownStructElement {
                Unknown1: &self.columns[270],
                Unknown2: &self.columns[271],
                Unknown3: &self.columns[272],
                Unknown4: &self.columns[273],
                Unknown5: &self.columns[274],
            },
            UnknownStructElement {
                Unknown1: &self.columns[275],
                Unknown2: &self.columns[276],
                Unknown3: &self.columns[277],
                Unknown4: &self.columns[278],
                Unknown5: &self.columns[279],
            },
            UnknownStructElement {
                Unknown1: &self.columns[280],
                Unknown2: &self.columns[281],
                Unknown3: &self.columns[282],
                Unknown4: &self.columns[283],
                Unknown5: &self.columns[284],
            },
            UnknownStructElement {
                Unknown1: &self.columns[285],
                Unknown2: &self.columns[286],
                Unknown3: &self.columns[287],
                Unknown4: &self.columns[288],
                Unknown5: &self.columns[289],
            },
            UnknownStructElement {
                Unknown1: &self.columns[290],
                Unknown2: &self.columns[291],
                Unknown3: &self.columns[292],
                Unknown4: &self.columns[293],
                Unknown5: &self.columns[294],
            },
            UnknownStructElement {
                Unknown1: &self.columns[295],
                Unknown2: &self.columns[296],
                Unknown3: &self.columns[297],
                Unknown4: &self.columns[298],
                Unknown5: &self.columns[299],
            },
            UnknownStructElement {
                Unknown1: &self.columns[300],
                Unknown2: &self.columns[301],
                Unknown3: &self.columns[302],
                Unknown4: &self.columns[303],
                Unknown5: &self.columns[304],
            },
            UnknownStructElement {
                Unknown1: &self.columns[305],
                Unknown2: &self.columns[306],
                Unknown3: &self.columns[307],
                Unknown4: &self.columns[308],
                Unknown5: &self.columns[309],
            },
            UnknownStructElement {
                Unknown1: &self.columns[310],
                Unknown2: &self.columns[311],
                Unknown3: &self.columns[312],
                Unknown4: &self.columns[313],
                Unknown5: &self.columns[314],
            },
            UnknownStructElement {
                Unknown1: &self.columns[315],
                Unknown2: &self.columns[316],
                Unknown3: &self.columns[317],
                Unknown4: &self.columns[318],
                Unknown5: &self.columns[319],
            },
        ]
    }
    pub fn Unknown320<'a>(&'a self) -> &'a Field {
        &self.columns[320]
    }
    pub fn Unknown_70<'a>(&'a self) -> &'a Field {
        &self.columns[321]
    }
    pub fn Unknown321<'a>(&'a self) -> &'a Field {
        &self.columns[322]
    }
}
