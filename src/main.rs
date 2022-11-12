use wiki_racer::WikiLadderFinder;
use wiki_racer::WikiPageFetcher;

fn main() {
    let finder = WikiLadderFinder::<WikiPageFetcher>::default();

    let start_page = "JavaScript";
    let end_page = "Suicide";

    let ladder = finder.find_ladder(start_page, end_page).unwrap();
    
    println!("To get from '{}' to '{}', you need to go through these pages ({}):", start_page, end_page, ladder.len());
    for page in ladder {
        println!("{}", urlencoding::decode(&page).unwrap());
    }
}
