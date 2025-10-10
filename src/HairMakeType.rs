#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct CharaMakeStructElement<'a> {
Menu: &'a ColumnData,
SubMenuMask: &'a ColumnData,
Customize: &'a ColumnData,
SubMenuParam: &'a ColumnData,
InitVal: &'a ColumnData,
SubMenuType: &'a ColumnData,
SubMenuNum: &'a ColumnData,
LookAt: &'a ColumnData,
SubMenuGraphic: &'a ColumnData,
}
pub struct FacialFeatureOptionElement<'a> {
Option1: &'a ColumnData,
Option2: &'a ColumnData,
Option3: &'a ColumnData,
Option4: &'a ColumnData,
Option5: &'a ColumnData,
Option6: &'a ColumnData,
Option7: &'a ColumnData,
}
pub struct HairMakeTypeSheet {
pages: Vec<EXD>,
exh: EXH,
row_count: u32,
}
impl HairMakeTypeSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "HairMakeType")?;
let mut pages = Vec::new();
for (i, _) in exh.pages.iter().enumerate() {
pages.push(read_excel_sheet(resource, "HairMakeType", &exh, language, i)?);
}let row_count = exh.header.row_count;
Some(Self {
exh,
pages,
row_count,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<HairMakeTypeRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(HairMakeTypeRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<HairMakeTypeRow> {
for page in &self.pages {
let Some(row) = &page.get_row(row_id) else { continue; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
return self.read_row(row);
}
None
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<HairMakeTypeRow> {
for page in &self.pages {
let Some(row) = &page.get_row(row_id) else { continue; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
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
pub struct HairMakeTypeRow {
columns: Vec<ColumnData>,
}
impl HairMakeTypeRow {
pub fn CharaMakeStruct<'a>(&'a self) -> [CharaMakeStructElement<'a>; 9] {
[CharaMakeStructElement {Menu: &self.columns[0],
SubMenuMask: &self.columns[1],
Customize: &self.columns[2],
SubMenuParam: &self.columns[3],
InitVal: &self.columns[4],
SubMenuType: &self.columns[5],
SubMenuNum: &self.columns[6],
LookAt: &self.columns[7],
SubMenuGraphic: &self.columns[8],
},
CharaMakeStructElement {Menu: &self.columns[9],
SubMenuMask: &self.columns[10],
Customize: &self.columns[11],
SubMenuParam: &self.columns[12],
InitVal: &self.columns[13],
SubMenuType: &self.columns[14],
SubMenuNum: &self.columns[15],
LookAt: &self.columns[16],
SubMenuGraphic: &self.columns[17],
},
CharaMakeStructElement {Menu: &self.columns[18],
SubMenuMask: &self.columns[19],
Customize: &self.columns[20],
SubMenuParam: &self.columns[21],
InitVal: &self.columns[22],
SubMenuType: &self.columns[23],
SubMenuNum: &self.columns[24],
LookAt: &self.columns[25],
SubMenuGraphic: &self.columns[26],
},
CharaMakeStructElement {Menu: &self.columns[27],
SubMenuMask: &self.columns[28],
Customize: &self.columns[29],
SubMenuParam: &self.columns[30],
InitVal: &self.columns[31],
SubMenuType: &self.columns[32],
SubMenuNum: &self.columns[33],
LookAt: &self.columns[34],
SubMenuGraphic: &self.columns[35],
},
CharaMakeStructElement {Menu: &self.columns[36],
SubMenuMask: &self.columns[37],
Customize: &self.columns[38],
SubMenuParam: &self.columns[39],
InitVal: &self.columns[40],
SubMenuType: &self.columns[41],
SubMenuNum: &self.columns[42],
LookAt: &self.columns[43],
SubMenuGraphic: &self.columns[44],
},
CharaMakeStructElement {Menu: &self.columns[45],
SubMenuMask: &self.columns[46],
Customize: &self.columns[47],
SubMenuParam: &self.columns[48],
InitVal: &self.columns[49],
SubMenuType: &self.columns[50],
SubMenuNum: &self.columns[51],
LookAt: &self.columns[52],
SubMenuGraphic: &self.columns[53],
},
CharaMakeStructElement {Menu: &self.columns[54],
SubMenuMask: &self.columns[55],
Customize: &self.columns[56],
SubMenuParam: &self.columns[57],
InitVal: &self.columns[58],
SubMenuType: &self.columns[59],
SubMenuNum: &self.columns[60],
LookAt: &self.columns[61],
SubMenuGraphic: &self.columns[62],
},
CharaMakeStructElement {Menu: &self.columns[63],
SubMenuMask: &self.columns[64],
Customize: &self.columns[65],
SubMenuParam: &self.columns[66],
InitVal: &self.columns[67],
SubMenuType: &self.columns[68],
SubMenuNum: &self.columns[69],
LookAt: &self.columns[70],
SubMenuGraphic: &self.columns[71],
},
CharaMakeStructElement {Menu: &self.columns[72],
SubMenuMask: &self.columns[73],
Customize: &self.columns[74],
SubMenuParam: &self.columns[75],
InitVal: &self.columns[76],
SubMenuType: &self.columns[77],
SubMenuNum: &self.columns[78],
LookAt: &self.columns[79],
SubMenuGraphic: &self.columns[80],
},
]
}
pub fn FacialFeatureOption<'a>(&'a self) -> [FacialFeatureOptionElement<'a>; 8] {
[FacialFeatureOptionElement {Option1: &self.columns[81],
Option2: &self.columns[82],
Option3: &self.columns[83],
Option4: &self.columns[84],
Option5: &self.columns[85],
Option6: &self.columns[86],
Option7: &self.columns[87],
},
FacialFeatureOptionElement {Option1: &self.columns[88],
Option2: &self.columns[89],
Option3: &self.columns[90],
Option4: &self.columns[91],
Option5: &self.columns[92],
Option6: &self.columns[93],
Option7: &self.columns[94],
},
FacialFeatureOptionElement {Option1: &self.columns[95],
Option2: &self.columns[96],
Option3: &self.columns[97],
Option4: &self.columns[98],
Option5: &self.columns[99],
Option6: &self.columns[100],
Option7: &self.columns[101],
},
FacialFeatureOptionElement {Option1: &self.columns[102],
Option2: &self.columns[103],
Option3: &self.columns[104],
Option4: &self.columns[105],
Option5: &self.columns[106],
Option6: &self.columns[107],
Option7: &self.columns[108],
},
FacialFeatureOptionElement {Option1: &self.columns[109],
Option2: &self.columns[110],
Option3: &self.columns[111],
Option4: &self.columns[112],
Option5: &self.columns[113],
Option6: &self.columns[114],
Option7: &self.columns[115],
},
FacialFeatureOptionElement {Option1: &self.columns[116],
Option2: &self.columns[117],
Option3: &self.columns[118],
Option4: &self.columns[119],
Option5: &self.columns[120],
Option6: &self.columns[121],
Option7: &self.columns[122],
},
FacialFeatureOptionElement {Option1: &self.columns[123],
Option2: &self.columns[124],
Option3: &self.columns[125],
Option4: &self.columns[126],
Option5: &self.columns[127],
Option6: &self.columns[128],
Option7: &self.columns[129],
},
FacialFeatureOptionElement {Option1: &self.columns[130],
Option2: &self.columns[131],
Option3: &self.columns[132],
Option4: &self.columns[133],
Option5: &self.columns[134],
Option6: &self.columns[135],
Option7: &self.columns[136],
},
]
}
pub fn Race(&self) -> &ColumnData {
&self.columns[137]
}
pub fn Tribe(&self) -> &ColumnData {
&self.columns[138]
}
pub fn Gender(&self) -> &ColumnData {
&self.columns[139]
}
}
