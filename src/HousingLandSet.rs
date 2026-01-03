//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{ExcelSheet, ColumnData, ExcelRowKind, ExcelSingleRow},
    common::Language,
};
pub struct LandSetElement<'a> {
    pub UnknownRange1: &'a ColumnData,
    pub PlacardId: &'a ColumnData,
    pub UnknownRange2: &'a ColumnData,
    pub InitialPrice: &'a ColumnData,
    pub PlotSize: &'a ColumnData,
}
#[derive(Debug, Clone)]
pub struct HousingLandSetSheet {
    sheet: ExcelSheet,
}
impl HousingLandSetSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("HousingLandSet")?;
        let sheet = resolver.read_excel_sheet(exh, "HousingLandSet", language)?;
        Ok(Self { sheet })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<HousingLandSetRow> {
        let column_defs = &self.sheet.exh.column_definitions;
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
        Some(HousingLandSetRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<HousingLandSetRow> {
        let row = &self.sheet.get_row(row_id)?;
        let row = match row {
            ExcelRowKind::SingleRow(row) => row,
            ExcelRowKind::SubRows(rows) => &rows.first()?.1,
        };
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<HousingLandSetRow> {
        let row = &self.sheet.get_row(row_id)?;
        let row = match row {
            ExcelRowKind::SingleRow(row) => return None,
            ExcelRowKind::SubRows(subrows) => {
                &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1
            }
        };
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
pub struct HousingLandSetRow {
    columns: Vec<ColumnData>,
}
impl HousingLandSetRow {
    pub fn LandSet<'a>(&'a self) -> [LandSetElement<'a>; 60] {
        [
            LandSetElement {
                UnknownRange1: &self.columns[0],
                PlacardId: &self.columns[1],
                UnknownRange2: &self.columns[2],
                InitialPrice: &self.columns[3],
                PlotSize: &self.columns[4],
            },
            LandSetElement {
                UnknownRange1: &self.columns[5],
                PlacardId: &self.columns[6],
                UnknownRange2: &self.columns[7],
                InitialPrice: &self.columns[8],
                PlotSize: &self.columns[9],
            },
            LandSetElement {
                UnknownRange1: &self.columns[10],
                PlacardId: &self.columns[11],
                UnknownRange2: &self.columns[12],
                InitialPrice: &self.columns[13],
                PlotSize: &self.columns[14],
            },
            LandSetElement {
                UnknownRange1: &self.columns[15],
                PlacardId: &self.columns[16],
                UnknownRange2: &self.columns[17],
                InitialPrice: &self.columns[18],
                PlotSize: &self.columns[19],
            },
            LandSetElement {
                UnknownRange1: &self.columns[20],
                PlacardId: &self.columns[21],
                UnknownRange2: &self.columns[22],
                InitialPrice: &self.columns[23],
                PlotSize: &self.columns[24],
            },
            LandSetElement {
                UnknownRange1: &self.columns[25],
                PlacardId: &self.columns[26],
                UnknownRange2: &self.columns[27],
                InitialPrice: &self.columns[28],
                PlotSize: &self.columns[29],
            },
            LandSetElement {
                UnknownRange1: &self.columns[30],
                PlacardId: &self.columns[31],
                UnknownRange2: &self.columns[32],
                InitialPrice: &self.columns[33],
                PlotSize: &self.columns[34],
            },
            LandSetElement {
                UnknownRange1: &self.columns[35],
                PlacardId: &self.columns[36],
                UnknownRange2: &self.columns[37],
                InitialPrice: &self.columns[38],
                PlotSize: &self.columns[39],
            },
            LandSetElement {
                UnknownRange1: &self.columns[40],
                PlacardId: &self.columns[41],
                UnknownRange2: &self.columns[42],
                InitialPrice: &self.columns[43],
                PlotSize: &self.columns[44],
            },
            LandSetElement {
                UnknownRange1: &self.columns[45],
                PlacardId: &self.columns[46],
                UnknownRange2: &self.columns[47],
                InitialPrice: &self.columns[48],
                PlotSize: &self.columns[49],
            },
            LandSetElement {
                UnknownRange1: &self.columns[50],
                PlacardId: &self.columns[51],
                UnknownRange2: &self.columns[52],
                InitialPrice: &self.columns[53],
                PlotSize: &self.columns[54],
            },
            LandSetElement {
                UnknownRange1: &self.columns[55],
                PlacardId: &self.columns[56],
                UnknownRange2: &self.columns[57],
                InitialPrice: &self.columns[58],
                PlotSize: &self.columns[59],
            },
            LandSetElement {
                UnknownRange1: &self.columns[60],
                PlacardId: &self.columns[61],
                UnknownRange2: &self.columns[62],
                InitialPrice: &self.columns[63],
                PlotSize: &self.columns[64],
            },
            LandSetElement {
                UnknownRange1: &self.columns[65],
                PlacardId: &self.columns[66],
                UnknownRange2: &self.columns[67],
                InitialPrice: &self.columns[68],
                PlotSize: &self.columns[69],
            },
            LandSetElement {
                UnknownRange1: &self.columns[70],
                PlacardId: &self.columns[71],
                UnknownRange2: &self.columns[72],
                InitialPrice: &self.columns[73],
                PlotSize: &self.columns[74],
            },
            LandSetElement {
                UnknownRange1: &self.columns[75],
                PlacardId: &self.columns[76],
                UnknownRange2: &self.columns[77],
                InitialPrice: &self.columns[78],
                PlotSize: &self.columns[79],
            },
            LandSetElement {
                UnknownRange1: &self.columns[80],
                PlacardId: &self.columns[81],
                UnknownRange2: &self.columns[82],
                InitialPrice: &self.columns[83],
                PlotSize: &self.columns[84],
            },
            LandSetElement {
                UnknownRange1: &self.columns[85],
                PlacardId: &self.columns[86],
                UnknownRange2: &self.columns[87],
                InitialPrice: &self.columns[88],
                PlotSize: &self.columns[89],
            },
            LandSetElement {
                UnknownRange1: &self.columns[90],
                PlacardId: &self.columns[91],
                UnknownRange2: &self.columns[92],
                InitialPrice: &self.columns[93],
                PlotSize: &self.columns[94],
            },
            LandSetElement {
                UnknownRange1: &self.columns[95],
                PlacardId: &self.columns[96],
                UnknownRange2: &self.columns[97],
                InitialPrice: &self.columns[98],
                PlotSize: &self.columns[99],
            },
            LandSetElement {
                UnknownRange1: &self.columns[100],
                PlacardId: &self.columns[101],
                UnknownRange2: &self.columns[102],
                InitialPrice: &self.columns[103],
                PlotSize: &self.columns[104],
            },
            LandSetElement {
                UnknownRange1: &self.columns[105],
                PlacardId: &self.columns[106],
                UnknownRange2: &self.columns[107],
                InitialPrice: &self.columns[108],
                PlotSize: &self.columns[109],
            },
            LandSetElement {
                UnknownRange1: &self.columns[110],
                PlacardId: &self.columns[111],
                UnknownRange2: &self.columns[112],
                InitialPrice: &self.columns[113],
                PlotSize: &self.columns[114],
            },
            LandSetElement {
                UnknownRange1: &self.columns[115],
                PlacardId: &self.columns[116],
                UnknownRange2: &self.columns[117],
                InitialPrice: &self.columns[118],
                PlotSize: &self.columns[119],
            },
            LandSetElement {
                UnknownRange1: &self.columns[120],
                PlacardId: &self.columns[121],
                UnknownRange2: &self.columns[122],
                InitialPrice: &self.columns[123],
                PlotSize: &self.columns[124],
            },
            LandSetElement {
                UnknownRange1: &self.columns[125],
                PlacardId: &self.columns[126],
                UnknownRange2: &self.columns[127],
                InitialPrice: &self.columns[128],
                PlotSize: &self.columns[129],
            },
            LandSetElement {
                UnknownRange1: &self.columns[130],
                PlacardId: &self.columns[131],
                UnknownRange2: &self.columns[132],
                InitialPrice: &self.columns[133],
                PlotSize: &self.columns[134],
            },
            LandSetElement {
                UnknownRange1: &self.columns[135],
                PlacardId: &self.columns[136],
                UnknownRange2: &self.columns[137],
                InitialPrice: &self.columns[138],
                PlotSize: &self.columns[139],
            },
            LandSetElement {
                UnknownRange1: &self.columns[140],
                PlacardId: &self.columns[141],
                UnknownRange2: &self.columns[142],
                InitialPrice: &self.columns[143],
                PlotSize: &self.columns[144],
            },
            LandSetElement {
                UnknownRange1: &self.columns[145],
                PlacardId: &self.columns[146],
                UnknownRange2: &self.columns[147],
                InitialPrice: &self.columns[148],
                PlotSize: &self.columns[149],
            },
            LandSetElement {
                UnknownRange1: &self.columns[150],
                PlacardId: &self.columns[151],
                UnknownRange2: &self.columns[152],
                InitialPrice: &self.columns[153],
                PlotSize: &self.columns[154],
            },
            LandSetElement {
                UnknownRange1: &self.columns[155],
                PlacardId: &self.columns[156],
                UnknownRange2: &self.columns[157],
                InitialPrice: &self.columns[158],
                PlotSize: &self.columns[159],
            },
            LandSetElement {
                UnknownRange1: &self.columns[160],
                PlacardId: &self.columns[161],
                UnknownRange2: &self.columns[162],
                InitialPrice: &self.columns[163],
                PlotSize: &self.columns[164],
            },
            LandSetElement {
                UnknownRange1: &self.columns[165],
                PlacardId: &self.columns[166],
                UnknownRange2: &self.columns[167],
                InitialPrice: &self.columns[168],
                PlotSize: &self.columns[169],
            },
            LandSetElement {
                UnknownRange1: &self.columns[170],
                PlacardId: &self.columns[171],
                UnknownRange2: &self.columns[172],
                InitialPrice: &self.columns[173],
                PlotSize: &self.columns[174],
            },
            LandSetElement {
                UnknownRange1: &self.columns[175],
                PlacardId: &self.columns[176],
                UnknownRange2: &self.columns[177],
                InitialPrice: &self.columns[178],
                PlotSize: &self.columns[179],
            },
            LandSetElement {
                UnknownRange1: &self.columns[180],
                PlacardId: &self.columns[181],
                UnknownRange2: &self.columns[182],
                InitialPrice: &self.columns[183],
                PlotSize: &self.columns[184],
            },
            LandSetElement {
                UnknownRange1: &self.columns[185],
                PlacardId: &self.columns[186],
                UnknownRange2: &self.columns[187],
                InitialPrice: &self.columns[188],
                PlotSize: &self.columns[189],
            },
            LandSetElement {
                UnknownRange1: &self.columns[190],
                PlacardId: &self.columns[191],
                UnknownRange2: &self.columns[192],
                InitialPrice: &self.columns[193],
                PlotSize: &self.columns[194],
            },
            LandSetElement {
                UnknownRange1: &self.columns[195],
                PlacardId: &self.columns[196],
                UnknownRange2: &self.columns[197],
                InitialPrice: &self.columns[198],
                PlotSize: &self.columns[199],
            },
            LandSetElement {
                UnknownRange1: &self.columns[200],
                PlacardId: &self.columns[201],
                UnknownRange2: &self.columns[202],
                InitialPrice: &self.columns[203],
                PlotSize: &self.columns[204],
            },
            LandSetElement {
                UnknownRange1: &self.columns[205],
                PlacardId: &self.columns[206],
                UnknownRange2: &self.columns[207],
                InitialPrice: &self.columns[208],
                PlotSize: &self.columns[209],
            },
            LandSetElement {
                UnknownRange1: &self.columns[210],
                PlacardId: &self.columns[211],
                UnknownRange2: &self.columns[212],
                InitialPrice: &self.columns[213],
                PlotSize: &self.columns[214],
            },
            LandSetElement {
                UnknownRange1: &self.columns[215],
                PlacardId: &self.columns[216],
                UnknownRange2: &self.columns[217],
                InitialPrice: &self.columns[218],
                PlotSize: &self.columns[219],
            },
            LandSetElement {
                UnknownRange1: &self.columns[220],
                PlacardId: &self.columns[221],
                UnknownRange2: &self.columns[222],
                InitialPrice: &self.columns[223],
                PlotSize: &self.columns[224],
            },
            LandSetElement {
                UnknownRange1: &self.columns[225],
                PlacardId: &self.columns[226],
                UnknownRange2: &self.columns[227],
                InitialPrice: &self.columns[228],
                PlotSize: &self.columns[229],
            },
            LandSetElement {
                UnknownRange1: &self.columns[230],
                PlacardId: &self.columns[231],
                UnknownRange2: &self.columns[232],
                InitialPrice: &self.columns[233],
                PlotSize: &self.columns[234],
            },
            LandSetElement {
                UnknownRange1: &self.columns[235],
                PlacardId: &self.columns[236],
                UnknownRange2: &self.columns[237],
                InitialPrice: &self.columns[238],
                PlotSize: &self.columns[239],
            },
            LandSetElement {
                UnknownRange1: &self.columns[240],
                PlacardId: &self.columns[241],
                UnknownRange2: &self.columns[242],
                InitialPrice: &self.columns[243],
                PlotSize: &self.columns[244],
            },
            LandSetElement {
                UnknownRange1: &self.columns[245],
                PlacardId: &self.columns[246],
                UnknownRange2: &self.columns[247],
                InitialPrice: &self.columns[248],
                PlotSize: &self.columns[249],
            },
            LandSetElement {
                UnknownRange1: &self.columns[250],
                PlacardId: &self.columns[251],
                UnknownRange2: &self.columns[252],
                InitialPrice: &self.columns[253],
                PlotSize: &self.columns[254],
            },
            LandSetElement {
                UnknownRange1: &self.columns[255],
                PlacardId: &self.columns[256],
                UnknownRange2: &self.columns[257],
                InitialPrice: &self.columns[258],
                PlotSize: &self.columns[259],
            },
            LandSetElement {
                UnknownRange1: &self.columns[260],
                PlacardId: &self.columns[261],
                UnknownRange2: &self.columns[262],
                InitialPrice: &self.columns[263],
                PlotSize: &self.columns[264],
            },
            LandSetElement {
                UnknownRange1: &self.columns[265],
                PlacardId: &self.columns[266],
                UnknownRange2: &self.columns[267],
                InitialPrice: &self.columns[268],
                PlotSize: &self.columns[269],
            },
            LandSetElement {
                UnknownRange1: &self.columns[270],
                PlacardId: &self.columns[271],
                UnknownRange2: &self.columns[272],
                InitialPrice: &self.columns[273],
                PlotSize: &self.columns[274],
            },
            LandSetElement {
                UnknownRange1: &self.columns[275],
                PlacardId: &self.columns[276],
                UnknownRange2: &self.columns[277],
                InitialPrice: &self.columns[278],
                PlotSize: &self.columns[279],
            },
            LandSetElement {
                UnknownRange1: &self.columns[280],
                PlacardId: &self.columns[281],
                UnknownRange2: &self.columns[282],
                InitialPrice: &self.columns[283],
                PlotSize: &self.columns[284],
            },
            LandSetElement {
                UnknownRange1: &self.columns[285],
                PlacardId: &self.columns[286],
                UnknownRange2: &self.columns[287],
                InitialPrice: &self.columns[288],
                PlotSize: &self.columns[289],
            },
            LandSetElement {
                UnknownRange1: &self.columns[290],
                PlacardId: &self.columns[291],
                UnknownRange2: &self.columns[292],
                InitialPrice: &self.columns[293],
                PlotSize: &self.columns[294],
            },
            LandSetElement {
                UnknownRange1: &self.columns[295],
                PlacardId: &self.columns[296],
                UnknownRange2: &self.columns[297],
                InitialPrice: &self.columns[298],
                PlotSize: &self.columns[299],
            },
        ]
    }
    pub fn UnknownRange1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[300]
    }
    pub fn UnknownRange2<'a>(&'a self) -> &'a ColumnData {
        &self.columns[301]
    }
}
