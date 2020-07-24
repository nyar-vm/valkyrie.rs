use polars::frame::DataFrame;

pub struct ValkyrieDataFrame {
    any_frame: LazyFrame,
}
