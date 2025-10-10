#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct SupplyDataElement<'a> {
Item: &'a ColumnData,
ItemCount: &'a ColumnData,
}
pub struct GCSupplyDutySheet {
pages: Vec<EXD>,
exh: EXH,
row_count: u32,
}
impl GCSupplyDutySheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "GCSupplyDuty")?;
let mut pages = Vec::new();
for (i, _) in exh.pages.iter().enumerate() {
pages.push(read_excel_sheet(resource, "GCSupplyDuty", &exh, language, i)?);
}let row_count = exh.header.row_count;
Some(Self {
exh,
pages,
row_count,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<GCSupplyDutyRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(GCSupplyDutyRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<GCSupplyDutyRow> {
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
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<GCSupplyDutyRow> {
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
pub struct GCSupplyDutyRow {
columns: Vec<ColumnData>,
}
impl GCSupplyDutyRow {
pub fn SupplyData<'a>(&'a self) -> [SupplyDataElement<'a>; 11] {
[SupplyDataElement {Item: &self.columns[0],
ItemCount: &self.columns[1],
},
SupplyDataElement {Item: &self.columns[2],
ItemCount: &self.columns[3],
},
SupplyDataElement {Item: &self.columns[4],
ItemCount: &self.columns[5],
},
SupplyDataElement {Item: &self.columns[6],
ItemCount: &self.columns[7],
},
SupplyDataElement {Item: &self.columns[8],
ItemCount: &self.columns[9],
},
SupplyDataElement {Item: &self.columns[10],
ItemCount: &self.columns[11],
},
SupplyDataElement {Item: &self.columns[12],
ItemCount: &self.columns[13],
},
SupplyDataElement {Item: &self.columns[14],
ItemCount: &self.columns[15],
},
SupplyDataElement {Item: &self.columns[16],
ItemCount: &self.columns[17],
},
SupplyDataElement {Item: &self.columns[18],
ItemCount: &self.columns[19],
},
SupplyDataElement {Item: &self.columns[20],
ItemCount: &self.columns[21],
},
]
}
}
