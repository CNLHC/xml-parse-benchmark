# benchmark-xml

This repo containes utilities to benchmark the performance of parsing large xml file of mainstream programming languages, including `Rust`,`Golang` and `Python`.

## Benchmark Result

### **environment**

- `CPU&OS`: MBP with M3 Max 128GiB / macOS 14.4.1
- `python`: 3.11.8 [Clang 15.0.0 (clang-1500.1.0.2.5)] on darwin
  - **lxml**: 5.1.0
- `rust`: rustc 1.74.0 (79e9716c9 2023-11-13)
  - **quick-xml**: [this version](https://github.com/tafia/quick-xml/tree/2b2b773bd9bdcbb3f5e37adb10110df34c3bcff3)
- `go`: go1.21.0 darwin/arm64

### **result**

| lang   | duration |
| ------ | -------- |
| python | 83.52s   |
| golang | 93.13s   |
| rust   | 12.67s   |

## How To Run

### Prerequisites

1. `rust`: rust 2021 toolchain is installed
2. `golang`: golang 1.21 is installed
3. `python`: python 3.10 or newer version is installed

### Build and Run

1. **prepare the data**: you can manually download data from [dblp xml release](https://dblp.org/xml/release/) and place data in the `./data` folder. ( currently, the `data/dblp-2024-04-01.xml` is hard-code to the `bench.py`)

2. **build binaries**: `bash build.sh`. (This is a simple script and is error-prone, but the logic is simple i believe you can find way to fix the problems)

3. **benchmark**: `python3 bench.py --lang [rust|golang|python]`
