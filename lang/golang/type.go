package main

import "encoding/xml"

type RootElement struct {
	XMLName       xml.Name    `xml:"dblp"`
	Incollections []BaseEntry `xml:"incollection"`
	Articles      []BaseEntry `xml:"article"`
	Inproceedings []BaseEntry `xml:"inproceedings"`
	Proceedings   []BaseEntry `xml:"proceedings"`
	Books         []BaseEntry `xml:"book"`
	Phdtheses     []BaseEntry `xml:"phdthesis"`
	Masterstheses []BaseEntry `xml:"mastersthesis"`
	Wwws          []BaseEntry `xml:"www"`
	Persons       []BaseEntry `xml:"person"`
	Datas         []BaseEntry `xml:"data"`
}

func (c *RootElement) Len() int {
	return len(c.Incollections) +
		len(c.Articles) +
		len(c.Inproceedings) +
		len(c.Proceedings) +
		len(c.Books) +
		len(c.Phdtheses) +
		len(c.Masterstheses) +
		len(c.Wwws) +
		len(c.Persons) +
		len(c.Datas)
}

type EntryAttr struct {
	Key      string `xml:"key,attr"`
	Mdate    string `xml:"mdate,attr"`
	Publtype string `xml:"publtype,attr"`
	Reviewid string `xml:"reviewid,attr"`
	Rating   string `xml:"rating,attr"`
	Cdate    string `xml:"cdate,attr"`
	Person   string `xml:"person,attr"`
}
type GeneralField struct {
	Attrs   []xml.Attr `xml:",any,attr"`
	Content string     `xml:",innerxml"`
}
type EntryFields struct {
	Author    GeneralField `xml:"author"`
	Editor    GeneralField `xml:"editor"`
	Title     GeneralField `xml:"title"`
	Booktitle GeneralField `xml:"booktitle"`
	Pages     GeneralField `xml:"pages"`
	Year      GeneralField `xml:"year"`
	Address   GeneralField `xml:"address"`
	Journal   GeneralField `xml:"journal"`
	Volume    GeneralField `xml:"volume"`
	Number    GeneralField `xml:"number"`
	Month     GeneralField `xml:"month"`
	Url       GeneralField `xml:"url"`
	Ee        GeneralField `xml:"ee"`
	Cdrom     GeneralField `xml:"cdrom"`
	Cite      GeneralField `xml:"cite"`
	Publisher GeneralField `xml:"publisher"`
	Note      GeneralField `xml:"note"`
	Crossref  GeneralField `xml:"crossref"`
	Isbn      GeneralField `xml:"isbn"`
	Series    GeneralField `xml:"series"`
	School    GeneralField `xml:"school"`
	Chapter   GeneralField `xml:"chapter"`
	Publnr    GeneralField `xml:"publnr"`
	Stream    GeneralField `xml:"stream"`
	Rel       GeneralField `xml:"rel"`
}

type BaseEntry struct {
	EntryAttr
	EntryFields
}

type Incollection struct {
	BaseEntry
}
type Article struct {
	BaseEntry
}

type UnknownObject struct {
	XMLName xml.Name
	Attrs   []xml.Attr
	Content []byte
}
