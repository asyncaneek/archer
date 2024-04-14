use cursive::views::{DummyView, ListView, OnEventView, Panel, ScrollView};

static SEL_VIEW_NAME: &str = "PIPELINE_SEL_VIEW::SEL_VIEW";

pub struct PipelineSelectView {
    pub view: OnEventView<ScrollView<ListView>>,
}

impl PipelineSelectView {
    pub fn new() -> Self {
        let mut view = OnEventView::new(ScrollView::new(ListView::new()));
        let list_view = view.get_inner_mut().get_inner_mut();

        for _i in 0..50 {
            let panel = Panel::new(DummyView).title("");

            list_view.add_child("", panel);
        }

        // let mut list_view = OnEventView::new(ScrollView::new(
        //     SelectView::<String>::new().with_name(SEL_VIEW_NAME),
        // ));
        // let mut sel_view = list_view.get_inner_mut().get_inner_mut().get_mut();
        // sel_view.set_autojump(true);

        // for i in 0..5 {
        //     sel_view.add_item(format!("pipeline {}", i), format!("pipeline {}", i));
        // }

        // list_view = list_view.on_event(Key::Enter, |s| {
        //     if let Some(sel_view) = s.find_name::<SelectView<String>>(SEL_VIEW_NAME) {
        //         let _selection = sel_view.selection();
        //         // run a pipeline based on the selection
        //         log::info!(
        //             "PipelineSelectView::Selection = {}",
        //             _selection.unwrap().to_string()
        //         )
        //     }
        // });

        Self { view }
    }
}

impl Default for PipelineSelectView {
    fn default() -> Self {
        Self::new()
    }
}
