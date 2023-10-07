pub struct div;
impl div {
    pub const TAG_NAME: &'static str = "div";
    pub const NAME_SPACE: Option<&'static str> = None;

    pub const style: (&'static str, Option<&'static str>, bool) = ("style", None, false);
}

pub struct img;
impl img {
    pub const TAG_NAME: &'static str = "img";
    pub const NAME_SPACE: Option<&'static str> = None;

    pub const style: (&'static str, Option<&'static str>, bool) = ("style", None, false);
}

pub struct button;
impl button {
    pub const TAG_NAME: &'static str = "button";
    pub const NAME_SPACE: Option<&'static str> = None;

    pub const style: (&'static str, Option<&'static str>, bool) = ("style", None, false);
}