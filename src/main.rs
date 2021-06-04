use chronology::Category;
use iced::{Application, Command, Settings, Text};
use sqlx::{Error, Result, SqlitePool};

struct State {
    pool: SqlitePool,
    categories: Option<Vec<Category>>,
}

enum Ui {
    Loading,
    Loaded(State),
    Error(Error),
}

#[derive(Debug)]
enum Msg {
    Init(Result<SqlitePool>),

    FetchCategoryAll(Result<Vec<Category>>),
}

impl Application for Ui {
    type Executor = iced::executor::Default;

    type Message = Msg;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Ui::Loading,
            Command::perform(SqlitePool::connect("sqlite:chronology.db"), Msg::Init),
        )
    }

    fn title(&self) -> String {
        "Chronology".to_string()
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut iced::Clipboard,
    ) -> Command<Self::Message> {
        match self {
            Ui::Loading => match message {
                Msg::Init(Ok(pool)) => {
                    *self = Ui::Loaded(State {
                        pool: pool.clone(),
                        categories: None,
                    });
                    Command::perform(Category::all(pool), Msg::FetchCategoryAll)
                }
                Msg::Init(Err(err)) => {
                    *self = Ui::Error(err);

                    Command::none()
                }
                _ => Command::none(),
            },

            Ui::Loaded(state) => match message {
                Msg::Init(_) => panic!("Impossible"),
                Msg::FetchCategoryAll(Ok(categories)) => {
                    state.categories = Some(categories);
                    Command::none()
                }
                Msg::FetchCategoryAll(Err(err)) => {
                    *self = Ui::Error(err);
                    Command::none()
                }
            },

            Ui::Error(_) => Command::none(),
        }
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let label = match &self {
            Ui::Loading => "Loading".to_string(),
            Ui::Loaded(state) => {
                if let Some(ref categories) = state.categories {
                    format!("{} categories", categories.len())
                } else {
                    format!("No categories found")
                }
            }
            Ui::Error(err) => err.to_string(),
        };

        Text::new(label).into()
    }
}

fn main() -> iced::Result {
    Ui::run(Settings::default())
}
