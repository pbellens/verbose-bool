pub struct Truth {
    pub value: bool,
    pub trace: Option<Vec<String>>
}

impl Truth {
    pub fn new(v: bool) -> Self {
        Truth {
            value: v,
            trace: None
        }
    }
}

