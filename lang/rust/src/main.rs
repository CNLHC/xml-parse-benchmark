use quick_xml::{self, DeError};

mod dblp;

use std::{
    self,
    io::{Read, Write},
};

use crate::dblp::Dblp;

fn prepare() -> String {
    let fp = std::fs::read(std::env::var("DATA").unwrap()).expect("should read the file");
    return String::from_utf8(fp).unwrap();
}

fn wait_for_start() {
    std::io::stdout().write(b"ready\n").unwrap();
    let mut buf = vec![0; 5];
    std::io::stdin().read_exact(buf.as_mut()).unwrap();
    if String::from_utf8(buf).unwrap().as_str() == "start" {
        std::io::stderr()
            .write("start parsing\n".as_bytes())
            .unwrap();
        std::io::stderr().flush().unwrap();
        return;
    } else {
        panic!("unknown mark")
    }
}

fn work(inp: &String) {
    let res: Result<Dblp, DeError> = quick_xml::de::from_str(inp.as_str());
    let res = res.unwrap();
    std::io::stderr()
        .write(format!("total: {:}\n", res.fields.len()).as_bytes())
        .unwrap();
}

fn main() {
    let inp = prepare();
    wait_for_start();
    work(&inp);
}

#[cfg(test)]
mod test {
    use crate::dblp::Dblp;

    #[test]
    fn test_basic() {
        let sample = r#"
        <?xml version="1.0" encoding="ISO-8859-1"?>
        <!DOCTYPE dblp SYSTEM "dblp-2023-06-28.dtd">
        <dblp>
        <incollection mdate="2017-07-12" key="reference/vision/X14bd" publtype="encyclopedia">
            <title foo="bar" aux="asdj">Curvedness.&lt; &uuml;</title>
            <pages>159</pages>
            <year>2014</year>
            <booktitle>Computer Vision, A Reference Guide</booktitle>
            <ee>https://doi.org/10.1007/978-0-387-31439-6_100117</ee>
            <url>db/reference/vision/vision2014.html#X14bd</url>
        </incollection>
        <incollection mdate="2019-01-09" key="reference/vision/Singh14" publtype="encyclopedia">
            <author>Manish Singh 0001</author>
            <title>Transparency and Translucency.</title>
            <pages>815-819</pages>
            <year>2014</year>
            <booktitle>Computer Vision, A Reference Guide</booktitle>
            <ee>https://doi.org/10.1007/978-0-387-31439-6_559</ee>
            <url>db/reference/vision/vision2014.html#Singh14</url>
        </incollection>
        <article mdate="2018-07-05" key="persons/Ley95" publtype="informal">
            <author>Michael Ley</author>
            <title>DB&amp;LP: A WWW Bibliography on Databases and Logic Programming</title>
            <journal>Compulog Newsletter</journal>
            <year>1995</year>
        </article>
        </dblp>
        "#;

        let res: Dblp = quick_xml::de::from_str(sample).unwrap();
        dbg!(res);
    }
}
