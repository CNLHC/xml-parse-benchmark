package main

import (
	"bytes"
	"encoding/xml"
	"fmt"
	"io"
	"os"

	"golang.org/x/net/html/charset"
)

func prepare() []byte {
	fp, err := os.OpenFile(os.Getenv("DATA"), os.O_RDONLY, os.ModePerm)
	if err != nil {
		panic(err)
	}
	raw, err := io.ReadAll(fp)
	if err != nil {
		panic(err)
	}
	return raw
}

func waitForStart() {
	os.Stdout.WriteString("ready\n")
	buf := make([]byte, 5)
	for {
		n, err := os.Stdin.Read(buf)
		if err != nil {
			panic(err)
		}
		if n == 5 && string(buf) == "start" {
			os.Stderr.WriteString("start parsing\n")
			return
		} else {
			panic("unknown mark")
		}
	}
}

func main() {
	var (
		raw     = prepare()
		reader  = bytes.NewBuffer(raw)
		v       = &RootElement{}
		decoder = xml.NewDecoder(reader)
	)
	decoder.CharsetReader = charset.NewReaderLabel
	decoder.Strict = false
	waitForStart()
	err := decoder.Decode(v)
	if err != nil {
		panic(err)
	}
	os.Stderr.WriteString(fmt.Sprintf("total: %d\n", v.Len()))
	_ = v
}
