import subprocess
import os
import threading
import time
import logging
from subprocess import PIPE
import argparse

logging.basicConfig(level=logging.DEBUG)


class BenchmarkInvoker:
    def __init__(self, cmd: subprocess.Popen) -> None:
        self.cmd = cmd
        pass

    def poll_for_ready(self):
        while True:
            out = self.cmd.stdout.read(6)
            if out.decode().replace("\n", "") == "ready":
                logging.debug("process is ready")
                break

    def bench(self):
        self.poll_stderr()
        self.poll_for_ready()
        logging.debug("sending start mark")
        self.cmd.stdin.write("start\n".encode())
        self.cmd.stdin.flush()
        start_at = time.perf_counter()
        self.cmd.wait()
        end_at = time.perf_counter()
        diff = end_at - start_at
        logging.info(f"{diff=} second")

    def poll_stderr(self):
        def _poll_stderr(p):
            f = self.cmd.stderr
            logging.debug("reading process stderr")
            while True:
                code = self.cmd.poll()
                if isinstance(code, int):
                    logging.debug("the command is existed. stop polling for stderr")
                    return
                if f.readable():
                    b = f.readline()
                    line = b.decode()
                    if len(line) > 0:
                        logging.debug(f"[stderr]: {line}")

        pollth = threading.Thread(target=_poll_stderr, args=[self.cmd])
        pollth.start()


def init_parser():
    parser = argparse.ArgumentParser(description="xml benchmark")
    parser.add_argument(
        "--lang", type=str, required=True, choices=["rust", "golang", "python"]
    )
    return parser


if __name__ == "__main__":
    parser = init_parser()
    args = parser.parse_args()
    bench_args = None
    match args.lang:
        case "rust":
            bench_args = "./lang/rust/target/release/citegraph"
        case "golang":
            bench_args = "./lang/golang/citegraph"
        case "python":
            bench_args = "python3 ./lang/python/main.py"

    cmd = subprocess.Popen(
        bench_args,
        stdin=PIPE,
        stdout=PIPE,
        stderr=PIPE,
        shell=True,
        env={"DATA": os.path.abspath(os.path.join(".", "data", "dblp-2024-04-01.xml"))},
    )

    bm = BenchmarkInvoker(cmd)
    bm.bench()
