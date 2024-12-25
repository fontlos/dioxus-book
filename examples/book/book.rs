#[derive(
    Clone,
    Copy,
    dioxus_router::prelude::Routable,
    PartialEq,
    Eq,
    Hash,
    Debug,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum BookRoute {
    #[route("/chapter")]
    Chapter {},
    #[route("/sub/sub")]
    SubSub {},
}
impl BookRoute {
    pub fn sections(&self) -> &'static [dioxus_book::Section] {
        &self.page().sections
    }
    pub fn page(&self) -> &'static dioxus_book::Page<Self> {
        LAZY_BOOK.get_page(self)
    }
    pub fn page_id(&self) -> dioxus_book::PageId {
        match self {
            BookRoute::Chapter {} => dioxus_book::PageId(0usize),
            BookRoute::SubSub {} => dioxus_book::PageId(1usize),
        }
    }
}
impl Default for BookRoute {
    fn default() -> Self {
        BookRoute::Chapter {}
    }
}
pub static LAZY_BOOK: std::sync::LazyLock<dioxus_book::Book<BookRoute>> =
    std::sync::LazyLock::new(|| {
        let mut page_id_mapping = ::std::collections::HashMap::new();
        let mut pages = Vec::new();
        pages.push((0usize, {
            ::dioxus_book::Page {
                title: "Chapter".to_string(),
                url: BookRoute::Chapter {},
                segments: vec![],
                sections: vec![
                    ::dioxus_book::Section {
                        title: "title = \"\"".to_string(),
                        id: "title-=-\"\"".to_string(),
                        level: 2usize,
                    },
                    ::dioxus_book::Section {
                        title: "Chapter".to_string(),
                        id: "chapter".to_string(),
                        level: 1usize,
                    },
                    ::dioxus_book::Section {
                        title: "H2 Title".to_string(),
                        id: "h2-title".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::dioxus_book::PageId(0usize),
            }
        }));
        page_id_mapping.insert(BookRoute::Chapter {}, ::dioxus_book::PageId(0usize));
        pages.push((1usize, {
            ::dioxus_book::Page {
                title: "Sub Chapter".to_string(),
                url: BookRoute::SubSub {},
                segments: vec![],
                sections: vec![::dioxus_book::Section {
                    title: "Sub Chapter".to_string(),
                    id: "sub-chapter".to_string(),
                    level: 1usize,
                }],
                raw: String::new(),
                id: ::dioxus_book::PageId(1usize),
            }
        }));
        page_id_mapping.insert(BookRoute::SubSub {}, ::dioxus_book::PageId(1usize));
        ::dioxus_book::Book {
            summary: ::dioxus_book::Summary {
                title: Some("Summary".to_string()),
                prefix_chapters: vec![],
                numbered_chapters: vec![::dioxus_book::SummaryItem::Link(::dioxus_book::Link {
                    name: "Chapter".to_string(),
                    location: Some(BookRoute::Chapter {}),
                    number: Some(::dioxus_book::SectionNumber(vec![1u32])),
                    nested_items: vec![::dioxus_book::SummaryItem::Link(::dioxus_book::Link {
                        name: "Sub Chapter".to_string(),
                        location: Some(BookRoute::SubSub {}),
                        number: Some(::dioxus_book::SectionNumber(vec![1u32, 1u32])),
                        nested_items: vec![],
                    })],
                })],
                suffix_chapters: vec![],
            },
            pages: pages.into_iter().collect(),
            page_id_mapping,
        }
    });
#[component(no_case_check)]
pub fn Chapter() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "chapter",
            a { href: "#chapter", class: "header", "Chapter" }
        }
        h2 { id: "h2-title",
            a { href: "#h2-title", class: "header", "H2 Title" }
        }
        p { "Text from Chapter" }
    }
}
#[component(no_case_check)]
pub fn SubSub() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "sub-chapter",
            a { href: "#sub-chapter", class: "header", "Sub Chapter" }
        }
        p { "Text from Sub Chapter" }
    }
}
