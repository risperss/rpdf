#[derive(Debug)]
enum Subtype {
    Type0,
    Type1,
}

#[derive(Debug)]
struct Font {
    subtype: Subtype,
    basefont: String,
}

#[derive(Debug)]
struct Offsets(usize, usize);

#[derive(Debug)]
struct Stream {
    length: usize,
    fontsize: usize,
    body: String,
    font: Font,
    offsets: Offsets,
}

#[derive(Debug)]
struct MediaBox(usize, usize, usize, usize);

#[derive(Debug)]
struct Page {
    contents: Stream,
    mediabox: MediaBox,
}

#[derive(Debug)]
struct Pages {
    kids: Vec<Page>,
}

#[derive(Debug)]
struct Catalog {
    pages: Pages,
}

fn main() {
    let helvetica = Font {
        subtype: Subtype::Type1,
        basefont: String::from("Helvetica"),
    };

    let stream = Stream {
        length: 44,
        fontsize: 24,
        body: String::from("Hello World!"),
        font: helvetica,
        offsets: Offsets(175, 720),
    };

    let page = Page {
        contents: stream,
        mediabox: MediaBox(0, 0, 500, 800),
    };

    let pages = Pages { kids: vec![page] };

    let catalog = Catalog { pages: pages };

    println!("{:#?}", catalog);
}
