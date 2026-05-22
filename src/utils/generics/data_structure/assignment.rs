use scraper::{Html, Selector};
use std::collections::HashMap;

/// Fetches the published Google Doc at `url`, parses the embedded table of
/// (x, character, y) triples, and prints the resulting 2-D character grid.
///
/// Coordinate system:
///   • (0, 0) is the **top-left** corner.
///   • x increases to the **right**.
///   • y increases **downward**.
///
/// Grid cells with no entry are rendered as spaces, so the characters
/// together form a readable graphic when displayed in a fixed-width font.
pub fn print_grid(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    // ------------------------------------------------------------------ //
    // 1. Fetch the published HTML page
    // ------------------------------------------------------------------ //
    let html: String = ureq::get(url).call()?.into_string()?;

    // ------------------------------------------------------------------ //
    // 2. Parse every data <tr> inside a <table>
    //    The table header is: x-coordinate | Character | y-coordinate
    // ------------------------------------------------------------------ //
    let document  = Html::parse_document(&html);
    let row_sel   = Selector::parse("table tr").unwrap();
    let cell_sel  = Selector::parse("td").unwrap();

    let mut grid: HashMap<(i32, i32), char> = HashMap::new();
    let mut max_x = 0i32;
    let mut max_y = 0i32;

    for row in document.select(&row_sel).skip(1) {   // skip header <tr>
        let cells: Vec<_> = row.select(&cell_sel).collect();
        if cells.len() < 3 {
            continue;
        }

        // Plain text of each cell, stripped of leading/trailing whitespace.
        let cell_text = |i: usize| -> String {
            cells[i].text().collect::<String>().trim().to_string()
        };

        let x_str  = cell_text(0);
        let ch_str = cell_text(1);
        let y_str  = cell_text(2);

        // Rows that don't have numeric coordinates are skipped (e.g. extra headers).
        let (Ok(x), Ok(y)) = (x_str.parse::<i32>(), y_str.parse::<i32>()) else {
            continue;
        };

        // Take the first Unicode scalar in the character cell.
        let Some(ch) = ch_str.chars().next() else {
            continue;
        };

        grid.insert((x, y), ch);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }

    if grid.is_empty() {
        eprintln!("Warning: no character data found – check the URL or document format.");
        return Ok(());
    }

    // ------------------------------------------------------------------ //
    // 3. Render: rows run top-to-bottom (y = 0 … max_y),
    //            columns left-to-right (x = 0 … max_x).
    // ------------------------------------------------------------------ //
    for y in 0..=max_y {
        let row: String = (0..=max_x)
            .map(|x| grid.get(&(x, y)).copied().unwrap_or(' '))
            .collect();
        println!("{}", row);
    }

    Ok(())
}

#[test]
fn test_link(){

    let url = " https://docs.google.com/document/d/e/2PACX-1vSvM5gDlNvt7npYHhp_XfsJvuntUhq184By5xO_pA4b_gCWeXb6dM6ZxwN8rE6S4ghUsCj2VKR21oEP/pub";

    if let Err(e) = print_grid(url) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

// fn main() {
//     let url = "https://docs.google.com/document/d/e/\
//                2PACX-1vTMOmshQe8YvaRXi6gEPKKlsC6UpFJSMAk4mQjLm_u1gmHdVVTaeh7nBNFBRlui0sTZ-snGwZM4DBCT/pub";
// 
//     if let Err(e) = print_grid(url) {
//         eprintln!("Error: {e}");
//         std::process::exit(1);
//     }
// }