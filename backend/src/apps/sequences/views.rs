use askama::Template;
use axum::response::Html;
use askama_axum::IntoResponse;
use axum::extract::Path;

use crate::urls;

use super::models::{Sequence, SequenceUrl, AminoAcid};


#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}

#[derive(Template)]
#[template(path = "sequences/detail.html")]
struct DetailTemplate<'a> {
    sequence: &'a Sequence,
    edit_url: &'a str,
    delete_url: &'a str,
    list_url: &'a str,
}

#[derive(Template)]
#[template(path = "sequences/list.html")]
struct ListTemplate<'a> {
    sequence_urls: &'a Vec<SequenceUrl>,
    positions: &'a Vec<i32>,
}

pub async fn detail(Path(id): Path<i32>) -> impl IntoResponse {
    let edit_url = format!("{}/{}/edit", urls::sequences(), &id);
    let delete_url = format!("{}/{}/delete", urls::sequences(), &id);
    let list_url = urls::sequences();
    let sequence = Sequence::new(
        id,
        "test".to_string(),
        "test".to_string(),
        "DIVMTQSPLSSVTPGKSISCKASGFTFSYAYYMDVWGQGTTVTVSS".to_string(),
    );
    let detail = DetailTemplate {
        sequence: &sequence,
        edit_url: edit_url.as_str(),
        delete_url: delete_url.as_str(),
        list_url,
    };
    Html(detail.render().unwrap())
}


pub async fn list() -> impl IntoResponse {
    let sequence1 = "DIVMTQSPLSSVTPGKSISCKASGFTFSYAYYMDVWGQGTTVTVSS".to_string();
    let sequence2 = "DIVMTQSPLSQVTPGKSIKCKASGSTFSYAYYMDVTGYGTTQVPIDSJFPOISDJFPOIDJSPVSS".to_string();
    let sequence3 = "EVRGJIODJSOFIJDSDIVMTQSPLSQVTPGKSIFJDSOIFJODSPIJFPIDSJPFIDJSPFIKCKASGSTFSYAYYMDVTGYGTTQVVSS".to_string();
    let sequences = vec![
        SequenceUrl::new(Sequence::new(
            1,
            "C0001".to_string(),
            "test".to_string(),
            sequence1.clone(),
        )),
        SequenceUrl::new(Sequence::new(
            2,
            "C0002".to_string(),
            "test".to_string(),
            sequence2.clone(),
        )),
        SequenceUrl::new(Sequence::new(
            3,
            "C0003".to_string(),
            "test".to_string(),
            sequence3.clone(),
        )),
    ];
    let positions = &sequence1.chars().enumerate().map(|(i, _)| (i as i32) + 1).collect();
    let list = ListTemplate { 
        sequence_urls: &sequences,
        positions,
    };
    Html(list.render().unwrap())
}
