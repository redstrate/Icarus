//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    resource::{Resource, read_excel_sheet_header, read_excel_sheet},
    exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow},
    exh::{EXH, ExcelColumnDefinition},
    common::Language,
};
pub struct ItemElement<'a> {
    pub ItemId: &'a ColumnData,
    pub XPReward: &'a ColumnData,
    pub Collectability: &'a ColumnData,
    pub GilReward: &'a ColumnData,
    pub Level: &'a ColumnData,
    pub HighXPMultiplier: &'a ColumnData,
    pub HighGilMultiplier: &'a ColumnData,
    pub Unknown8: &'a ColumnData,
    pub ScripReward: &'a ColumnData,
    pub HighScripMultiplier: &'a ColumnData,
}
pub struct BankaCraftWorksSupplySheet {
    pages: Vec<EXD>,
    exh: EXH,
    row_count: u32,
}
impl BankaCraftWorksSupplySheet {
    /// Read the sheet from a `Resource`.
    pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
        let exh = read_excel_sheet_header(resource, "BankaCraftWorksSupply")?;
        let mut pages = Vec::new();
        for (i, _) in exh.pages.iter().enumerate() {
            pages
                .push(
                    read_excel_sheet(
                        resource,
                        "BankaCraftWorksSupply",
                        &exh,
                        language,
                        i,
                    )?,
                );
        }
        let row_count = exh.header.row_count;
        Some(Self { exh, pages, row_count })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<BankaCraftWorksSupplyRow> {
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
        Some(BankaCraftWorksSupplyRow {
            columns,
        })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<BankaCraftWorksSupplyRow> {
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
    pub fn get_subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<BankaCraftWorksSupplyRow> {
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
pub struct BankaCraftWorksSupplyRow {
    columns: Vec<ColumnData>,
}
impl BankaCraftWorksSupplyRow {
    pub fn Item<'a>(&'a self) -> [ItemElement<'a>; 4] {
        [
            ItemElement {
                ItemId: &self.columns[0],
                XPReward: &self.columns[1],
                Collectability: &self.columns[2],
                GilReward: &self.columns[3],
                Level: &self.columns[4],
                HighXPMultiplier: &self.columns[5],
                HighGilMultiplier: &self.columns[6],
                Unknown8: &self.columns[7],
                ScripReward: &self.columns[8],
                HighScripMultiplier: &self.columns[9],
            },
            ItemElement {
                ItemId: &self.columns[10],
                XPReward: &self.columns[11],
                Collectability: &self.columns[12],
                GilReward: &self.columns[13],
                Level: &self.columns[14],
                HighXPMultiplier: &self.columns[15],
                HighGilMultiplier: &self.columns[16],
                Unknown8: &self.columns[17],
                ScripReward: &self.columns[18],
                HighScripMultiplier: &self.columns[19],
            },
            ItemElement {
                ItemId: &self.columns[20],
                XPReward: &self.columns[21],
                Collectability: &self.columns[22],
                GilReward: &self.columns[23],
                Level: &self.columns[24],
                HighXPMultiplier: &self.columns[25],
                HighGilMultiplier: &self.columns[26],
                Unknown8: &self.columns[27],
                ScripReward: &self.columns[28],
                HighScripMultiplier: &self.columns[29],
            },
            ItemElement {
                ItemId: &self.columns[30],
                XPReward: &self.columns[31],
                Collectability: &self.columns[32],
                GilReward: &self.columns[33],
                Level: &self.columns[34],
                HighXPMultiplier: &self.columns[35],
                HighGilMultiplier: &self.columns[36],
                Unknown8: &self.columns[37],
                ScripReward: &self.columns[38],
                HighScripMultiplier: &self.columns[39],
            },
        ]
    }
}
