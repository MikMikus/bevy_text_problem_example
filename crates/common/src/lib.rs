pub const COLS_NUM: usize = 70;
pub const ROWS_NUM: usize = 50;

pub const INIT_CAMERA_ZOOM: f32 = 8.;

// pub const COLS_NUM: usize = 100;
// pub const ROWS_NUM: usize = 100;

// pub const INIT_CAMERA_ZOOM: f32 = 17.;

pub const LABEL_BODY_A: f32 = 100.;
pub const LABEL_BODY_R: f32 = 50.;
pub const MARGIN: f32 = 10.;

pub const FONT_SIZE: f32 = 24.;

pub const MULTI_SECTION_TEXT: [&str; 3] = ["Lorem ipsum", "\ndolor sit", "\namet"];
pub const ONE_SECTION_TEXT: &str = "Lorem ipsum\ndolor sit\namet";

pub struct CommonCalc;

impl CommonCalc {
    pub fn camera_center_x() -> f32 {
        (LABEL_BODY_A + LABEL_BODY_R * 2. + MARGIN) * (COLS_NUM + 1) as f32 * 0.5
    }

    pub fn camera_center_y() -> f32 {
        (LABEL_BODY_R * 2. + MARGIN) * ROWS_NUM as f32 * 0.5
    }

    pub fn label_x_pos(index: usize) -> f32 {
        (LABEL_BODY_A + LABEL_BODY_R * 2. + MARGIN) * index as f32
    }

    pub fn label_y_pos(index: usize) -> f32 {
        (LABEL_BODY_R * 2. + MARGIN) * index as f32
    }
}
