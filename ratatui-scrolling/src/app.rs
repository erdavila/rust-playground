use ratatui::widgets::ListState;

pub(crate) struct App {
    pub(crate) items_count: usize,
    pub(crate) list_state: ListState,
    pub(crate) page_size: usize,
}

impl App {
    pub(crate) fn new(items_count: usize) -> Self {
        App {
            items_count,
            list_state: ListState::default().with_selected(Some(0)),
            page_size: 0,
        }
    }

    pub(crate) fn select_up(&mut self, delta: usize) {
        let index = self.list_state.selected().unwrap();
        self.list_state.select(Some(index.saturating_sub(delta)));
    }

    pub(crate) fn select_down(&mut self, delta: usize) {
        if self.items_count > 0 {
            let index = self.list_state.selected().unwrap();
            self.list_state
                .select(Some(index.saturating_add(delta).min(self.items_count - 1)));
        }
    }

    pub(crate) fn select_first(&mut self) {
        self.list_state.select(Some(0));
    }

    pub(crate) fn select_last(&mut self) {
        self.list_state.select(Some(self.items_count - 1));
    }
}
