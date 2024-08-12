
#[derive(Debug)]
pub struct HistoryGroup {
    hs0: Vec::<String>,
    hs1: Vec::<String>,
    hs2: Vec::<String>,
    hs3: Vec::<String>,
    hs4: Vec::<String>,
    hs5: Vec::<String>,
    hs6: Vec::<String>,
    hs7: Vec::<String>,
    hs8: Vec::<String>,
    hs9: Vec::<String>,
    active: i32
}

impl HistoryGroup {
    pub fn new() -> Self {
        HistoryGroup {
            hs0: Vec::<String>::new(),
            hs1: Vec::<String>::new(),
            hs2: Vec::<String>::new(),
            hs3: Vec::<String>::new(),
            hs4: Vec::<String>::new(),
            hs5: Vec::<String>::new(),
            hs6: Vec::<String>::new(),
            hs7: Vec::<String>::new(),
            hs8: Vec::<String>::new(),
            hs9: Vec::<String>::new(),
            active: 0
        }
    }

    pub fn get_active_history_number(&self) -> i32 {
        self.active
    }

    pub fn get_active_history(&mut self) -> &mut Vec<String> {
        if self.active == 1 { return &mut self.hs1 }
        else if self.active == 2 { return &mut self.hs2 }
        else if self.active == 3 { return &mut self.hs3 }
        else if self.active == 4 { return &mut self.hs4 }
        else if self.active == 5 { return &mut self.hs5 }
        else if self.active == 6 { return &mut self.hs6 }
        else if self.active == 7 { return &mut self.hs7 }
        else if self.active == 8 { return &mut self.hs8 }
        else if self.active == 9 { return &mut self.hs9 }
        else { return &mut self.hs0 }
    }

    pub fn set_active(&mut self, n: i32) {
        self.active = n
    }

}
