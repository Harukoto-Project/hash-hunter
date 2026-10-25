use std::io::IsTerminal;

use owo_colors::OwoColorize;

use crate::detect::{Hit, ScoredHash};

/// スコア降順の上位を色付きで標準出力する。
pub fn print_ranking(scored: &[ScoredHash], top: usize, min_score: f64) {
    let color = std::io::stdout().is_terminal();
    print!("{}", render_ranking(scored, top, min_score, color));
}

/// スコア降順・同点はハッシュ昇順で、`min_score` 以上の上位 `top` 件を返す。
pub fn rank(scored: &[ScoredHash], top: usize, min_score: f64) -> Vec<ScoredHash> {
    select_ranked(scored, top, min_score)
        .into_iter()
        .cloned()
        .collect()
}

/// フィルタ・ソート済みのランキング文字列を返す。
pub fn render_ranking(scored: &[ScoredHash], top: usize, min_score: f64, color: bool) -> String {
    let ranked = select_ranked(scored, top, min_score);
    if ranked.is_empty() {
        return String::new();
    }
    let rank_width = ranked.len().to_string().len();
    let mut out = String::new();
    for (index, item) in ranked.iter().enumerate() {
        let hash = format_hash(&item.hash, &item.hits, color);
        out.push_str(&format!(
            "{:>width$}. {:>7.2}  {hash}\n",
            index + 1,
            item.score,
            width = rank_width
        ));
        let mut hits = item.hits.clone();
        hits.sort_by(|a, b| {
            a.range
                .start
                .cmp(&b.range.start)
                .then_with(|| a.range.end.cmp(&b.range.end))
                .then_with(|| a.detector_id.cmp(b.detector_id))
        });
        for hit in hits {
            out.push_str(&format!(
                "     {:<16} {}..{}  {:>7.2}\n",
                hit.detector_id, hit.range.start, hit.range.end, hit.score
            ));
        }
    }
    out
}

fn select_ranked(scored: &[ScoredHash], top: usize, min_score: f64) -> Vec<&ScoredHash> {
    let mut items: Vec<_> = scored
        .iter()
        .filter(|item| item.score >= min_score)
        .collect();
    items.sort_by(|a, b| {
        b.score
            .total_cmp(&a.score)
            .then_with(|| a.hash.cmp(&b.hash))
    });
    items.into_iter().take(top).collect()
}

fn format_hash(hash: &str, hits: &[Hit], color: bool) -> String {
    if !color {
        return hash.to_string();
    }
    let mut marked = vec![false; hash.len()];
    for hit in hits {
        for index in hit.range.clone() {
            if let Some(flag) = marked.get_mut(index) {
                *flag = true;
            }
        }
    }
    let mut out = String::new();
    let mut index = 0;
    while index < hash.len() {
        let on = marked[index];
        let start = index;
        index += 1;
        while index < hash.len() && marked[index] == on {
            index += 1;
        }
        let slice = &hash[start..index];
        if on {
            out.push_str(&format!("{}", slice.yellow().bold()));
        } else {
            out.push_str(slice);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scored(hash: &str, score: f64, hits: Vec<Hit>) -> ScoredHash {
        ScoredHash {
            hash: hash.to_string(),
            hits,
            score,
        }
    }

    fn hit(id: &'static str, start: usize, end: usize, score: f64) -> Hit {
        Hit {
            detector_id: id,
            range: start..end,
            score,
        }
    }

    #[test]
    fn orders_by_score_then_hash() {
        let items = [
            scored("bbbb", 4.0, vec![]),
            scored("aaaa", 8.0, vec![]),
            scored("cccc", 8.0, vec![]),
            scored("dddd", 0.0, vec![]),
        ];
        let ranked: Vec<_> = select_ranked(&items, 10, 0.0)
            .iter()
            .map(|item| item.hash.as_str())
            .collect();
        assert_eq!(ranked, ["aaaa", "cccc", "bbbb", "dddd"]);
    }

    #[test]
    fn filters_min_score_and_top() {
        let items = [
            scored("high", 12.0, vec![]),
            scored("mid", 4.0, vec![]),
            scored("low", 0.0, vec![]),
        ];
        let ranked: Vec<_> = select_ranked(&items, 1, 4.0)
            .iter()
            .map(|item| item.hash.as_str())
            .collect();
        assert_eq!(ranked, ["high"]);
    }

    #[test]
    fn renders_plain_ranking() {
        let items = [scored(
            "0000ab",
            16.0,
            vec![
                hit("repeat-run", 0, 4, 12.0),
                hit("leading-zeros", 0, 4, 16.0),
            ],
        )];
        let rendered = render_ranking(&items, 20, 0.0, false);
        assert_eq!(
            rendered,
            concat!(
                "1.   16.00  0000ab\n",
                "     leading-zeros    0..4    16.00\n",
                "     repeat-run       0..4    12.00\n"
            )
        );
    }

    #[test]
    fn empty_when_nothing_matches() {
        let items = [scored("aaaa", 2.0, vec![])];
        assert!(render_ranking(&items, 20, 8.0, false).is_empty());
    }
}
