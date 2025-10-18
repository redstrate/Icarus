//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    resource::{Resource, read_excel_sheet_header, read_excel_sheet},
    exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow},
    exh::{EXH, ExcelColumnDefinition},
    common::Language,
};
pub struct CompanionSheet {
    pages: Vec<EXD>,
    exh: EXH,
    row_count: u32,
}
impl CompanionSheet {
    /// Read the sheet from a `Resource`.
    pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
        let exh = read_excel_sheet_header(resource, "Companion")?;
        let mut pages = Vec::new();
        for (i, _) in exh.pages.iter().enumerate() {
            pages.push(read_excel_sheet(resource, "Companion", &exh, language, i)?);
        }
        let row_count = exh.header.row_count;
        Some(Self { exh, pages, row_count })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<CompanionRow> {
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
        Some(CompanionRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<CompanionRow> {
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
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<CompanionRow> {
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
pub struct CompanionRow {
    columns: Vec<ColumnData>,
}
impl CompanionRow {
    pub fn Singular<'a>(&'a self) -> &'a ColumnData {
        &self.columns[0]
    }
    pub fn Plural<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1]
    }
    pub fn Adjective<'a>(&'a self) -> &'a ColumnData {
        &self.columns[2]
    }
    pub fn PossessivePronoun<'a>(&'a self) -> &'a ColumnData {
        &self.columns[3]
    }
    pub fn StartsWithVowel<'a>(&'a self) -> &'a ColumnData {
        &self.columns[4]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a ColumnData {
        &self.columns[5]
    }
    pub fn Pronoun<'a>(&'a self) -> &'a ColumnData {
        &self.columns[6]
    }
    pub fn Article<'a>(&'a self) -> &'a ColumnData {
        &self.columns[7]
    }
    pub fn Model<'a>(&'a self) -> &'a ColumnData {
        &self.columns[8]
    }
    pub fn Priority<'a>(&'a self) -> &'a ColumnData {
        &self.columns[9]
    }
    pub fn Enemy<'a>(&'a self) -> &'a ColumnData {
        &self.columns[10]
    }
    pub fn Icon<'a>(&'a self) -> &'a ColumnData {
        &self.columns[11]
    }
    pub fn Order<'a>(&'a self) -> &'a ColumnData {
        &self.columns[12]
    }
    pub fn HP<'a>(&'a self) -> &'a ColumnData {
        &self.columns[13]
    }
    pub fn SkillAngle<'a>(&'a self) -> &'a ColumnData {
        &self.columns[14]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[15]
    }
    pub fn Scale<'a>(&'a self) -> &'a ColumnData {
        &self.columns[16]
    }
    pub fn InactiveIdle0<'a>(&'a self) -> &'a ColumnData {
        &self.columns[17]
    }
    pub fn InactiveIdle1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[18]
    }
    pub fn InactiveBattle<'a>(&'a self) -> &'a ColumnData {
        &self.columns[19]
    }
    pub fn InactiveWandering<'a>(&'a self) -> &'a ColumnData {
        &self.columns[20]
    }
    pub fn Behavior<'a>(&'a self) -> &'a ColumnData {
        &self.columns[21]
    }
    pub fn Special<'a>(&'a self) -> &'a ColumnData {
        &self.columns[22]
    }
    pub fn Unknown10<'a>(&'a self) -> &'a ColumnData {
        &self.columns[23]
    }
    pub fn Unknown11<'a>(&'a self) -> &'a ColumnData {
        &self.columns[24]
    }
    pub fn WanderingWait<'a>(&'a self) -> &'a ColumnData {
        &self.columns[25]
    }
    pub fn Unknown2<'a>(&'a self) -> &'a ColumnData {
        &self.columns[26]
    }
    pub fn Cost<'a>(&'a self) -> &'a ColumnData {
        &self.columns[27]
    }
    pub fn Unknown3<'a>(&'a self) -> &'a ColumnData {
        &self.columns[28]
    }
    pub fn SkillCost<'a>(&'a self) -> &'a ColumnData {
        &self.columns[29]
    }
    pub fn Unknown4<'a>(&'a self) -> &'a ColumnData {
        &self.columns[30]
    }
    pub fn MinionRace<'a>(&'a self) -> &'a ColumnData {
        &self.columns[31]
    }
    pub fn Unknown5<'a>(&'a self) -> &'a ColumnData {
        &self.columns[32]
    }
    pub fn Unknown6<'a>(&'a self) -> &'a ColumnData {
        &self.columns[33]
    }
    pub fn Unknown7<'a>(&'a self) -> &'a ColumnData {
        &self.columns[34]
    }
    pub fn Unknown8<'a>(&'a self) -> &'a ColumnData {
        &self.columns[35]
    }
    pub fn Unknown9<'a>(&'a self) -> &'a ColumnData {
        &self.columns[36]
    }
    pub fn Battle<'a>(&'a self) -> &'a ColumnData {
        &self.columns[37]
    }
    pub fn Roulette<'a>(&'a self) -> &'a ColumnData {
        &self.columns[38]
    }
    pub fn IdleAnimation<'a>(&'a self) -> &'a ColumnData {
        &self.columns[39]
    }
}
