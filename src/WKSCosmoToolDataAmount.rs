#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct StagesElement<'a> {
RequiredAmount: &'a ColumnData,
MaxAmount: &'a ColumnData,
}
pub struct WKSCosmoToolDataAmountSheet {
pages: Vec<EXD>,
exh: EXH,
row_count: u32,
}
impl WKSCosmoToolDataAmountSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "WKSCosmoToolDataAmount")?;
let mut pages = Vec::new();
for (i, _) in exh.pages.iter().enumerate() {
pages.push(read_excel_sheet(resource, "WKSCosmoToolDataAmount", &exh, language, i)?);
}let row_count = exh.header.row_count;
Some(Self {
exh,
pages,
row_count,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<WKSCosmoToolDataAmountRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(WKSCosmoToolDataAmountRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<WKSCosmoToolDataAmountRow> {
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
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<WKSCosmoToolDataAmountRow> {
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
pub struct WKSCosmoToolDataAmountRow {
columns: Vec<ColumnData>,
}
impl WKSCosmoToolDataAmountRow {
pub fn Stages<'a>(&'a self) -> [StagesElement<'a>; 14] {
[StagesElement {RequiredAmount: &self.columns[0],
MaxAmount: &self.columns[1],
},
StagesElement {RequiredAmount: &self.columns[2],
MaxAmount: &self.columns[3],
},
StagesElement {RequiredAmount: &self.columns[4],
MaxAmount: &self.columns[5],
},
StagesElement {RequiredAmount: &self.columns[6],
MaxAmount: &self.columns[7],
},
StagesElement {RequiredAmount: &self.columns[8],
MaxAmount: &self.columns[9],
},
StagesElement {RequiredAmount: &self.columns[10],
MaxAmount: &self.columns[11],
},
StagesElement {RequiredAmount: &self.columns[12],
MaxAmount: &self.columns[13],
},
StagesElement {RequiredAmount: &self.columns[14],
MaxAmount: &self.columns[15],
},
StagesElement {RequiredAmount: &self.columns[16],
MaxAmount: &self.columns[17],
},
StagesElement {RequiredAmount: &self.columns[18],
MaxAmount: &self.columns[19],
},
StagesElement {RequiredAmount: &self.columns[20],
MaxAmount: &self.columns[21],
},
StagesElement {RequiredAmount: &self.columns[22],
MaxAmount: &self.columns[23],
},
StagesElement {RequiredAmount: &self.columns[24],
MaxAmount: &self.columns[25],
},
StagesElement {RequiredAmount: &self.columns[26],
MaxAmount: &self.columns[27],
},
]
}
}
