use ibex::prelude::*;
use ibex::{routes, ssg};

const URL_ROOT: &str = "/recipes/";

fn main() {
    let routes = routes! [
        (/)    => at_index(),
        (/404) => at_404(),
    ];

    ssg::quick_build(routes).unwrap();
    println!("All done!");
}

fn at_index() -> Document {
    view! {
        @use_base[]
    }
    .into()
}

fn at_404() -> Document {
    view! {
        @use_base[]

        center {
            "404 - Not found"
        }
    }
    .into()
}

fn use_base() -> View {
    view! {
        HEAD {
            @use_meta [Meta::new()]
            title { "Recipe Website" }
            link [rel="shortcut icon", href=url!("static/icon.png")]/
            link [rel="stylesheet", href=url!("css/base.css")]/
        }

        center {
            h1 { "Recipe Website" }
        }
    }
}
