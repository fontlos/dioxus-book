use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;

use book_tool::Book;
use book_tool::Page;
use book_tool::Section;
use book_tool::Summary;
use book_tool::SummaryItem;
// use book_tool::get_book_content_path;
// use book_tool::get_summary_path;

use std::path::Path;
use std::path::PathBuf;

use crate::compiler::path_to_route_enum;

/// Transforms the book to use enum routes instead of paths
pub fn write_book_with_routes(
    book_path: PathBuf,
    book: &book_tool::Book<PathBuf>,
) -> TokenStream {
    // let summary_path = get_summary_path(&book_path).expect("SUMMARY.md path not found");
    // let index_path = summary_path.to_string_lossy();

    let Book { summary, .. } = book;
    let summary = write_summary_with_routes(summary);
    let pages = book.pages().iter().map(|(id, v)| {
        let name = path_to_route_enum(&v.url);
        let page = write_page_with_routes(&book_path, v);
        quote! {
            pages.push((#id, #page));
            page_id_mapping.insert(#name, ::dioxus_book::PageId(#id));
        }
    });

    let out = quote! {
        {
            // Let the compiler know that we care about the index file
            // const _: &[u8] = include_bytes!(#index_path);
            let mut page_id_mapping = ::std::collections::HashMap::new();
            let mut pages = Vec::new();
            #(#pages)*
            ::dioxus_book::Book {
                summary: #summary,
                pages: pages.into_iter().collect(),
                page_id_mapping,
            }
        }
    };

    out.to_token_stream()
}

fn write_summary_with_routes(book: &book_tool::Summary<PathBuf>) -> TokenStream {
    let Summary {
        title,
        prefix_chapters,
        numbered_chapters,
        suffix_chapters,
    } = book;

    let prefix_chapters = prefix_chapters.iter().map(write_summary_item_with_routes);
    let numbered_chapters = numbered_chapters.iter().map(write_summary_item_with_routes);
    let suffix_chapters = suffix_chapters.iter().map(write_summary_item_with_routes);
    let title = match title {
        Some(title) => quote! { Some(#title.to_string()) },
        None => quote! { None },
    };

    quote! {
        ::dioxus_book::Summary {
            title: #title,
            prefix_chapters: vec![#(#prefix_chapters),*],
            numbered_chapters: vec![#(#numbered_chapters),*],
            suffix_chapters: vec![#(#suffix_chapters),*],
        }
    }
}

fn write_summary_item_with_routes(item: &SummaryItem<PathBuf>) -> TokenStream {
    match item {
        SummaryItem::Link(link) => {
            let link = write_link_with_routes(link);
            quote! {
                ::dioxus_book::SummaryItem::Link(#link)
            }
        }
        SummaryItem::Separator => {
            quote! {
                ::dioxus_book::SummaryItem::Separator
            }
        }
        SummaryItem::PartTitle(title) => {
            quote! {
                ::dioxus_book::SummaryItem::PartTitle(#title.to_string())
            }
        }
    }
}

fn write_link_with_routes(book: &book_tool::Link<PathBuf>) -> TokenStream {
    let book_tool::Link {
        name,
        location,
        number,
        nested_items,
    } = book;

    let location = match location {
        Some(loc) => {
            let inner = path_to_route_enum(loc);
            quote! { Some(#inner) }
        }
        None => quote! { None },
    };
    let number = match number {
        Some(number) => {
            let inner = write_number_with_routes(number);
            quote! { Some(#inner) }
        }
        None => quote! {None},
    };

    let nested_items = nested_items.iter().map(write_summary_item_with_routes);

    quote! {
        ::dioxus_book::Link {
            name: #name.to_string(),
            location: #location,
            number: #number,
            nested_items: vec![#(#nested_items,)*],
        }
    }
}

fn write_number_with_routes(number: &book_tool::SectionNumber) -> TokenStream {
    let book_tool::SectionNumber(number) = number;
    let numbers = number.iter().map(|num| {
        quote! {
            #num
        }
    });

    quote! {
        ::dioxus_book::SectionNumber(vec![#(#numbers),*])
    }
}

fn write_page_with_routes(_book_path: &Path, book: &book_tool::Page<PathBuf>) -> TokenStream {
    let Page {
        title,
        url,
        segments,
        sections,
        raw: _,
        id,
    } = book;

    let segments = segments.iter().map(|segment| {
        quote! {
            #segment.to_string()
        }
    });

    let sections = sections.iter().map(write_section_with_routes);

    let path = url;
    let url = path_to_route_enum(path);
    // let full_path = get_book_content_path(book_path)
    //     .expect("No book content path found")
    //     .join(path);
    // let path_str = full_path.to_str().unwrap();
    let id = id.0;

    quote! {
        {
            // This lets the rust compile know that we read the file
            // const _: &[u8] = include_bytes!(#path_str);
            ::dioxus_book::Page {
                title: #title.to_string(),
                url: #url,
                segments: vec![#(#segments,)*],
                sections: vec![#(#sections,)*],
                raw: String::new(),
                id: ::dioxus_book::PageId(#id),
            }
        }
    }
}

fn write_section_with_routes(book: &book_tool::Section) -> TokenStream {
    let Section { title, id, level } = book;

    quote! {
        ::dioxus_book::Section {
            title: #title.to_string(),
            id: #id.to_string(),
            level: #level,
        }
    }
}
