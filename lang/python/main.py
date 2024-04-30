import os
import sys
from lxml import etree

test = """<?xml version="1.0" encoding="ISO-8859-1"?>
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
"""


def prepare():
    with open(os.getenv("DATA"), "r") as fp:
        raw: str = fp.read()
        t = raw.encode("utf-8")
        return t


def wait():
    sys.stdout.write("ready\n")
    sys.stdout.flush()
    s = sys.stdin.read(5)
    if s == "start":
        sys.stderr.write("start parsing\n")
        return
    else:
        raise Exception("unknown")


raw = prepare()
wait()

root = etree.fromstring(
    raw, parser=etree.XMLParser(resolve_entities=False, recover=True)
)
sys.stderr.write(f"total: {len(root.getchildren())}")
