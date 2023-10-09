#[allow(non_upper_case_globals)]

// (&'static str, Option<&'static str>, bool    )
// (name        , namespace           , volatile)
// volatile is for stuff that needs to be updated regardless of value change (like input value)

pub trait GlobalAttributes {
    const display: (&'static str, Option<&'static str>, bool) = ("display", None, false);
    const position_type: (&'static str, Option<&'static str>, bool) = ("position_type", None, false);
    const overflow: (&'static str, Option<&'static str>, bool) = ("overflow", None, false);
    const direction: (&'static str, Option<&'static str>, bool) = ("direction", None, false);
    const left: (&'static str, Option<&'static str>, bool) = ("left", None, false);
    const right: (&'static str, Option<&'static str>, bool) = ("right", None, false);
    const top: (&'static str, Option<&'static str>, bool) = ("top", None, false);
    const bottom: (&'static str, Option<&'static str>, bool) = ("bottom", None, false);
    const width: (&'static str, Option<&'static str>, bool) = ("width", None, false);
    const height: (&'static str, Option<&'static str>, bool) = ("height", None, false);
    const min_width: (&'static str, Option<&'static str>, bool) = ("min_width", None, false);
    const min_height: (&'static str, Option<&'static str>, bool) = ("min_height", None, false);
    const max_width: (&'static str, Option<&'static str>, bool) = ("max_width", None, false);
    const max_height: (&'static str, Option<&'static str>, bool) = ("max_height", None, false);
    const aspect_ratio: (&'static str, Option<&'static str>, bool) = ("aspect_ratio", None, false);
    const align_items: (&'static str, Option<&'static str>, bool) = ("align_items", None, false);
    const justify_items: (&'static str, Option<&'static str>, bool) = ("justify_items", None, false);
    const align_self: (&'static str, Option<&'static str>, bool) = ("align_self", None, false);
    const justify_self: (&'static str, Option<&'static str>, bool) = ("justify_self", None, false);
    const align_content: (&'static str, Option<&'static str>, bool) = ("align_content", None, false);
    const justify_content: (&'static str, Option<&'static str>, bool) = ("justify_content", None, false);
    const margin: (&'static str, Option<&'static str>, bool) = ("margin", None, false);
    const padding: (&'static str, Option<&'static str>, bool) = ("padding", None, false);
    const border: (&'static str, Option<&'static str>, bool) = ("border", None, false);
    const flex_direction: (&'static str, Option<&'static str>, bool) = ("flex_direction", None, false);
    const flex_wrap: (&'static str, Option<&'static str>, bool) = ("flex_wrap", None, false);
    const flex_grow: (&'static str, Option<&'static str>, bool) = ("flex_grow", None, false);
    const flex_shrink: (&'static str, Option<&'static str>, bool) = ("flex_shrink", None, false);
    const flex_basis: (&'static str, Option<&'static str>, bool) = ("flex_basis", None, false);
    const row_gap: (&'static str, Option<&'static str>, bool) = ("row_gap", None, false);
    const column_gap: (&'static str, Option<&'static str>, bool) = ("column_gap", None, false);
    const grid_auto_flow: (&'static str, Option<&'static str>, bool) = ("grid_auto_flow", None, false);
    const grid_template_rows: (&'static str, Option<&'static str>, bool) = ("grid_template_rows", None, false);
    const grid_template_columns: (&'static str, Option<&'static str>, bool) = ("grid_template_columns", None, false);
    const grid_auto_rows: (&'static str, Option<&'static str>, bool) = ("grid_auto_rows", None, false);
    const grid_auto_columns: (&'static str, Option<&'static str>, bool) = ("grid_auto_columns", None, false);
    const grid_row: (&'static str, Option<&'static str>, bool) = ("grid_row", None, false);
    const grid_column: (&'static str, Option<&'static str>, bool) = ("grid_column", None, false);
}



pub struct div;
impl div {
    pub const TAG_NAME: &'static str = "div";
    pub const NAME_SPACE: Option<&'static str> = None;
}

impl GlobalAttributes for div {}

pub struct img;
impl img {
    pub const TAG_NAME: &'static str = "img";
    pub const NAME_SPACE: Option<&'static str> = None;
}

impl GlobalAttributes for img {}

pub struct button;
impl button {
    pub const TAG_NAME: &'static str = "button";
    pub const NAME_SPACE: Option<&'static str> = None;
}

impl GlobalAttributes for button {}