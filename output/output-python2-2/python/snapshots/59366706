#!/usr/bin/python
#-*- coding: utf-8 -*-

import telnetlib

from socket_abstract import Socket

class FakeSocket(Socket):

    def __init__(self):
        zen = """
The Zen of Python, by Tim Peters

Beautiful is better than ugly.
Explicit is better than implicit.
Simple is better than complex.
Complex is better than complicated.
Flat is better than nested.
Sparse is better than dense.
Readability counts.
Special cases aren't special enough to break the rules.
Although practicality beats purity.
Errors should never pass silently.
Unless explicitly silenced.
In the face of ambiguity, refuse the temptation to guess.
There should be one-- and preferably only one --obvious way to do it.
Although that way may not be obvious at first unless you're Dutch.
Now is better than never.
Although never is often better than *right* now.
If the implementation is hard to explain, it's a bad idea.
If the implementation is easy to explain, it may be a good idea.
Namespaces are one honking great idea -- let's do more of those!"""


        self.msg = zen.split("\n")
        self.count = 0

    def read(self):
        self.count += 1

        if self.count > len(self.msg)-1:
            self.count = 0

        return self.msg[self.count]