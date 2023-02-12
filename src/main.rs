use std::error::Error;
use std::fmt::format;
use druid::{AppLauncher, Color, Data, Lens, LocalizedString, Widget, WidgetExt, WindowDesc};
use druid::widget::{Button, Container, Flex, Label, LensWrap, List, Split, TextBox};
use crate::infra::memory_repository::MemoryRepository;
use crate::infra::uuid_id_generator::UuidIdGenerator;
use crate::usecase::simple_usecase::SimpleUsecase;
use crate::usecase::usecase::{TimeRecord, Usecase};
use druid::im::{Vector, vector};

mod domain;
mod usecase;
mod infra;

#[derive(Clone, Data, Lens)]
struct TodoList {
    items: Vector<String>,
    next_item: String,
}

fn ui_builder() -> impl Widget<TodoList> {
    Split::columns(
        Container::new(
            LensWrap::new(
                List::new(|| Label::dynamic(|data, _| format!("List item: {data}"))),
                TodoList::items,
            )
        )
            .border(Color::grey(0.6), 2.0),
        Container::new(
            Flex::column()
                .with_flex_child(
                    Button::new("Add item").on_click(|_, data: &mut TodoList, _| {
                        data.items.push_back(data.next_item.clone());
                        data.next_item = String::new();
                    }),
                    1.0,
                )
                .with_flex_child(LensWrap::new(TextBox::new(), TodoList::next_item), 1.0),
        )
            .border(Color::grey(0.6), 2.0),
    )
}

fn main() -> Result<(), Box<dyn Error>> {
    let window = WindowDesc::new(ui_builder());
    let data = TodoList {
        items: vector!["a".into(), "b".into()],
        next_item: String::default(),
    };
    AppLauncher::with_window(window)
        .log_to_console()
        .launch(data)?;


    let mut usecase = SimpleUsecase::new(Box::new(MemoryRepository::new()), Box::new(UuidIdGenerator::new()));
    match usecase.create_time_record(TimeRecord {
        start: "2023-02-10 21:24:50".to_string(),
        end: "2023-02-10 21:24:50".to_string(),
        memo: "".to_string(),
    }) {
        Ok(_) => {}
        Err(err) => {
            println!("{}", err);
        }
    }

    for record in usecase.list_time_records()? {
        println!("{} {} ~ {} {}", record.id, record.start, record.end, record.memo);
    }
    Ok(())
}
