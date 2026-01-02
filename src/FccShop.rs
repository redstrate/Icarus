//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{ExcelSheet, ColumnData, ExcelRowKind, ExcelSingleRow},
    common::Language,
};
pub struct ItemDataElement<'a> {
    pub Item: &'a ColumnData,
    pub Cost: &'a ColumnData,
    pub FCRankRequired: &'a ColumnData,
}
pub struct FccShopSheet {
    sheet: ExcelSheet,
}
impl FccShopSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("FccShop")?;
        let sheet = resolver.read_excel_sheet(exh, "FccShop", language)?;
        Ok(Self { sheet })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<FccShopRow> {
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
        Some(FccShopRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<FccShopRow> {
        let row = &self.sheet.get_row(row_id)?;
        let row = match row {
            ExcelRowKind::SingleRow(row) => row,
            ExcelRowKind::SubRows(rows) => &rows.first()?.1,
        };
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<FccShopRow> {
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
pub struct FccShopRow {
    columns: Vec<ColumnData>,
}
impl FccShopRow {
    pub fn Name<'a>(&'a self) -> &'a ColumnData {
        &self.columns[0]
    }
    pub fn ItemData<'a>(&'a self) -> [ItemDataElement<'a>; 10] {
        [
            ItemDataElement {
                Item: &self.columns[1],
                Cost: &self.columns[2],
                FCRankRequired: &self.columns[3],
            },
            ItemDataElement {
                Item: &self.columns[4],
                Cost: &self.columns[5],
                FCRankRequired: &self.columns[6],
            },
            ItemDataElement {
                Item: &self.columns[7],
                Cost: &self.columns[8],
                FCRankRequired: &self.columns[9],
            },
            ItemDataElement {
                Item: &self.columns[10],
                Cost: &self.columns[11],
                FCRankRequired: &self.columns[12],
            },
            ItemDataElement {
                Item: &self.columns[13],
                Cost: &self.columns[14],
                FCRankRequired: &self.columns[15],
            },
            ItemDataElement {
                Item: &self.columns[16],
                Cost: &self.columns[17],
                FCRankRequired: &self.columns[18],
            },
            ItemDataElement {
                Item: &self.columns[19],
                Cost: &self.columns[20],
                FCRankRequired: &self.columns[21],
            },
            ItemDataElement {
                Item: &self.columns[22],
                Cost: &self.columns[23],
                FCRankRequired: &self.columns[24],
            },
            ItemDataElement {
                Item: &self.columns[25],
                Cost: &self.columns[26],
                FCRankRequired: &self.columns[27],
            },
            ItemDataElement {
                Item: &self.columns[28],
                Cost: &self.columns[29],
                FCRankRequired: &self.columns[30],
            },
        ]
    }
}
