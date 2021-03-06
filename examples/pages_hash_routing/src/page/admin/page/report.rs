use crate::Context;
use seed::{prelude::*, *};

const DAILY: &str = "daily";
const WEEKLY: &str = "weekly";

// ------ ------
//     Init
// ------ ------

pub fn init(mut url: Url) -> Option<Model> {
    let base_url = url.to_hash_base_url();

    let frequency = match url.remaining_hash_path_parts().as_slice() {
        [] => {
            Urls::new(&base_url).default().go_and_replace();
            Frequency::default()
        }
        [DAILY] => Frequency::Daily,
        [WEEKLY] => Frequency::Weekly,
        _ => None?,
    };

    Some(Model {
        base_url,
        frequency,
    })
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    base_url: Url,
    frequency: Frequency,
}

// ------ Frequency ------

enum Frequency {
    Daily,
    Weekly,
}

impl Default for Frequency {
    fn default() -> Self {
        Self::Daily
    }
}

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    pub fn default(self) -> Url {
        self.daily()
    }
    pub fn daily(self) -> Url {
        self.base_url().add_hash_path_part(DAILY)
    }
    pub fn weekly(self) -> Url {
        self.base_url().add_hash_path_part(WEEKLY)
    }
}

// ------ ------
//     View
// ------ ------

pub fn view<Ms>(model: &Model, ctx: &Context) -> Node<Ms> {
    let (frequency, link) = match &model.frequency {
        Frequency::Daily => (
            "daily",
            a![
                "Switch to weekly",
                attrs! {
                    At::Href => Urls::new(&model.base_url).weekly()
                }
            ],
        ),
        Frequency::Weekly => (
            "weekly",
            a![
                "Switch to daily",
                attrs! {
                    At::Href => Urls::new(&model.base_url).daily()
                }
            ],
        ),
    };
    div![
        format!(
            "Hello {}! This is your {} report.",
            ctx.logged_user, frequency
        ),
        link,
    ]
}
