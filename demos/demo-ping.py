#!/usr/bin/env python3
"""Generate stdecor ping demo using pexpect"""

import argparse
import os
import sys
import time

import pexpect


class Session(pexpect.spawn):
    def __init__(self):
        env = dict(os.environ)
        env.update({"PS1": "\\$ ", "PS2": ""})
        pexpect.spawn.__init__(
            self,
            "bash",
            ["--norc"],
            encoding="utf-8",
            timeout=10,
            echo=False,
            logfile=sys.stdout,
            env=env,
        )
        self._prompt = "[#$] $"
        self.exp_prompt()

    def exp_prompt(self):
        self.expect(self._prompt)

    def send(self, string, slow=None):
        if slow:
            prev = self.delaybeforesend
            self.delaybeforesend = slow
            for char in string:
                if char == "\r":
                    time.sleep(slow * 5)
                pexpect.spawn.send(self, char)
            self.delaybeforesend = prev
        else:
            pexpect.spawn.send(self, string)

    def send_cmd(self, cmd, slow=None):
        self.send(cmd, slow=slow)
        if slow:
            time.sleep(slow * 5)
        self.send("\r")
        self.exp_prompt()

    def done(self):
        self.send("exit\r")
        self.expect(pexpect.EOF)
        self.wait()


def do_main():
    p = Session()
    p.delaybeforesend = 0.01
    for line in [
        "cat <<END > test.sh",
        "set -x",
        "stdecor -p '[google]' -- ping -nc8 www.google.com &",
        "stdecor -p '[amazon]' -- ping -nc8 www.amazon.com &",
        "wait",
        "END",
    ]:
        p.send(line + "\r", slow=0.02)
        p.expect("\n")
    p.exp_prompt()
    p.send_cmd("bash test.sh", slow=0.2)
    p.send_cmd("rm -f test.sh")
    p.done()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.parse_args()
    do_main()


if __name__ == "__main__":
    main()
