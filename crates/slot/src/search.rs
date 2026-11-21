use slot_store::{Cart, Platform};
use slot_ui::{NetFace, NetLine, TexId};

const MAX_RESULTS: usize = 300;

#[derive(Debug)]
pub struct SearchHit {
    pub title: String,
    pub platform: Platform,
    pub stem: String,
}

pub(crate) struct SearchFaces {
    pub first: usize,
    pub lines: Vec<NetLine>,
    pub empty: NetFace,
    pub go: [(TexId, u32); 2],
}

fn words(text: &str) -> Vec<String> {
    text.split(|c: char| !c.is_alphanumeric())
        .filter(|word| !word.is_empty())
        .map(str::to_lowercase)
        .collect()
}

fn rank(title: &str, query: &[String], phrase: &str) -> Option<u8> {
    let title = words(title);
    if !query
        .iter()
        .all(|term| title.iter().any(|word| word.starts_with(term)))
    {
        return None;
    }
    Some(if title.join(" ").starts_with(phrase) {
        0
    } else if title[0].starts_with(&query[0]) {
        1
    } else {
        2
    })
}

fn matches<'a>(titles: impl Iterator<Item = &'a str>, query: &str) -> Vec<usize> {
    let query = words(query);
    if query.is_empty() {
        return Vec::new();
    }
    let phrase = query.join(" ");
    let mut hits: Vec<_> = titles
        .enumerate()
        .filter_map(|(i, title)| rank(title, &query, &phrase).map(|rank| (rank, i)))
        .collect();
    hits.sort_unstable();
    hits.into_iter().take(MAX_RESULTS).map(|(_, i)| i).collect()
}

pub(crate) fn search<'a>(carts: impl Iterator<Item = &'a Cart>, query: &str) -> Vec<SearchHit> {
    let carts: Vec<_> = carts
        .filter(|cart| matches!(cart.platform, Platform::Gb | Platform::Gbc | Platform::Gba))
        .collect();
    let titles: Vec<_> = carts.iter().map(|cart| slot_ui::label_text(cart)).collect();
    matches(titles.iter().map(String::as_str), query)
        .into_iter()
        .map(|i| SearchHit {
            title: titles[i].clone(),
            platform: carts[i].platform,
            stem: carts[i].stem.clone(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::matches;

    #[test]
    fn each_query_word_matches_a_title_word_prefix() {
        assert_eq!(matches(["The Legend of Zelda"].into_iter(), "leg zel"), [0]);
        assert!(matches(["The Legend of Zelda"].into_iter(), "elda").is_empty());
        assert!(matches(["Metal Slug"].into_iter(), " - ").is_empty());
    }

    #[test]
    fn matching_ignores_case_and_punctuation_between_words() {
        assert_eq!(
            matches(["Pokémon: Emerald"].into_iter(), "POKÉMON, em"),
            [0]
        );
    }

    #[test]
    fn title_prefixes_rank_first_and_ties_keep_library_order() {
        let titles = ["The Metal Slug Story", "Mega Man", "Metal Slug", "Metroid"];
        assert_eq!(matches(titles.into_iter(), "met"), [2, 3, 0]);
        assert_eq!(
            matches(["B Metal", "Metal B", "Metal A B"].into_iter(), "metal b"),
            [1, 2, 0]
        );
    }
}
