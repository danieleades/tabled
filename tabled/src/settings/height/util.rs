use papergrid::{
    grid::spanned::{ExactDimension, GridConfig},
    records::Records,
};

use crate::records::ExactRecords;

pub(crate) fn get_table_height<R: Records + ExactRecords>(
    records: R,
    cfg: &GridConfig,
) -> (usize, Vec<usize>) {
    let count_horizontals = cfg.count_horizontal(records.count_rows());

    let margin = cfg.get_margin();
    let margin_size = margin.top.size + margin.bottom.size;

    let list = ExactDimension::height(records, cfg);
    let total = list.iter().sum::<usize>();

    let total = total + count_horizontals + margin_size;

    (total, list)
}
