#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct IconsElement<'a> {
AnnounceQuest: &'a ColumnData,
AnnounceQuestLocked: &'a ColumnData,
MapAnnounceQuest1: &'a ColumnData,
MapAnnounceQuestLocked: &'a ColumnData,
MapAnnounceQuest2: &'a ColumnData,
}
pub struct EventCustomIconTypeSheet {
pages: Vec<EXD>,
exh: EXH,
row_count: u32,
}
impl EventCustomIconTypeSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "EventCustomIconType")?;
let mut pages = Vec::new();
for (i, _) in exh.pages.iter().enumerate() {
pages.push(read_excel_sheet(resource, "EventCustomIconType", &exh, language, i)?);
}let row_count = exh.header.row_count;
Some(Self {
exh,
pages,
row_count,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<EventCustomIconTypeRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(EventCustomIconTypeRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<EventCustomIconTypeRow> {
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
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<EventCustomIconTypeRow> {
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
pub struct EventCustomIconTypeRow {
columns: Vec<ColumnData>,
}
impl EventCustomIconTypeRow {
pub fn Icons<'a>(&'a self) -> [IconsElement<'a>; 10] {
[IconsElement {AnnounceQuest: &self.columns[0],
AnnounceQuestLocked: &self.columns[1],
MapAnnounceQuest1: &self.columns[2],
MapAnnounceQuestLocked: &self.columns[3],
MapAnnounceQuest2: &self.columns[4],
},
IconsElement {AnnounceQuest: &self.columns[5],
AnnounceQuestLocked: &self.columns[6],
MapAnnounceQuest1: &self.columns[7],
MapAnnounceQuestLocked: &self.columns[8],
MapAnnounceQuest2: &self.columns[9],
},
IconsElement {AnnounceQuest: &self.columns[10],
AnnounceQuestLocked: &self.columns[11],
MapAnnounceQuest1: &self.columns[12],
MapAnnounceQuestLocked: &self.columns[13],
MapAnnounceQuest2: &self.columns[14],
},
IconsElement {AnnounceQuest: &self.columns[15],
AnnounceQuestLocked: &self.columns[16],
MapAnnounceQuest1: &self.columns[17],
MapAnnounceQuestLocked: &self.columns[18],
MapAnnounceQuest2: &self.columns[19],
},
IconsElement {AnnounceQuest: &self.columns[20],
AnnounceQuestLocked: &self.columns[21],
MapAnnounceQuest1: &self.columns[22],
MapAnnounceQuestLocked: &self.columns[23],
MapAnnounceQuest2: &self.columns[24],
},
IconsElement {AnnounceQuest: &self.columns[25],
AnnounceQuestLocked: &self.columns[26],
MapAnnounceQuest1: &self.columns[27],
MapAnnounceQuestLocked: &self.columns[28],
MapAnnounceQuest2: &self.columns[29],
},
IconsElement {AnnounceQuest: &self.columns[30],
AnnounceQuestLocked: &self.columns[31],
MapAnnounceQuest1: &self.columns[32],
MapAnnounceQuestLocked: &self.columns[33],
MapAnnounceQuest2: &self.columns[34],
},
IconsElement {AnnounceQuest: &self.columns[35],
AnnounceQuestLocked: &self.columns[36],
MapAnnounceQuest1: &self.columns[37],
MapAnnounceQuestLocked: &self.columns[38],
MapAnnounceQuest2: &self.columns[39],
},
IconsElement {AnnounceQuest: &self.columns[40],
AnnounceQuestLocked: &self.columns[41],
MapAnnounceQuest1: &self.columns[42],
MapAnnounceQuestLocked: &self.columns[43],
MapAnnounceQuest2: &self.columns[44],
},
IconsElement {AnnounceQuest: &self.columns[45],
AnnounceQuestLocked: &self.columns[46],
MapAnnounceQuest1: &self.columns[47],
MapAnnounceQuestLocked: &self.columns[48],
MapAnnounceQuest2: &self.columns[49],
},
]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[50]
}
}
