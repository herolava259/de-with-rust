

#[derive(Debug)]
enum WineRegions {
    Bordeaux,
    Burgundy,
    Champagne,
    Rioja,
    Tuscany,
    NapaValley,
}

struct Wine {
    name: String,
    region: WineRegions,
    year: u16,
}

fn supported_regions(w: &WineRegions) {
    match w {
        WineRegions::Rioja => println!("Rioja is supported!"),
        _ => println!("{:?} is not supported yet.", w),
    }
}

fn get_popularity(region: &WineRegions) -> String {
    match region {
        WineRegions::Bordeaux => String::from("Highly popular"),
        WineRegions::Burgundy => String::from("Very popular"),
        WineRegions::Champagne => String::from("Popular for celebrations"),
        WineRegions::Rioja => String::from("Gaining popularity"),
        WineRegions::Tuscany => String::from("Well-known and loved"),
        WineRegions::NapaValley => String::from("Popular in the US"),
    }
}

fn main() {


    let wine1 = Wine {
        name: String::from("Chateau Margaux"),
        region: WineRegions::Bordeaux,
        year: 2015
    };

    let wine2 = Wine {
        name: String::from("Barolo"),
        region: WineRegions::Tuscany,
        year: 2018
    };

    let wine3: Wine = Wine {
        name: String::from("Marques de Riscal"),
        region: WineRegions::NapaValley,
        year: 2016
    };

    // println!("Wine 1: {} from {:?}", wine1.name, wine1.region);
    // println!("Wine 2: {} from {:?}", wine2.name, wine2.region);
    supported_regions(&wine1.region);
    supported_regions(&WineRegions::Rioja);
    println!("Popularity of {}: {}", wine1.name, get_popularity(&wine1.region));
    println!("Popularity of {}: {}", wine2.name, get_popularity(&wine2.region));
    println!("Popularity of {}: {}", wine3.name, get_popularity(&wine3.region));
}
