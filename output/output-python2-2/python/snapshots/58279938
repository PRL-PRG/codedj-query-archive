#!/usr/bin/python

"""
move data forward by offset bytes in fileobj leaving offset bytes spare at the
start of the file 
"""

import sys

import StringIO


def shunt(offset, blocksize, fileobj):
    assert blocksize >= offset # TODO: remove this
    datas = []
    start_position = fileobj.tell()
    data = fileobj.read(blocksize)
    if data:
        datas.append(data)
    write_position = start_position + offset
    read_position = start_position + len(data)
    while len(datas) > 0:
        if read_position >= write_position:
            fileobj.seek(read_position)
            data = fileobj.read(blocksize)
            if data:
                datas.append(data)
                read_position += len(data)
        fileobj.seek(write_position)
        write_position += len(datas[0])
        fileobj.write(datas.pop(0))

    
import unittest


class Test(unittest.TestCase):

    def test_simple(self):
        f = StringIO.StringIO("abc")
        shunt(1, 1, f)
        self.assertEqual(f.getvalue(), "aabc")

        f = StringIO.StringIO("abc")
        shunt(1, 2, f)
        self.assertEqual(f.getvalue(), "aabc")

        f = StringIO.StringIO("abc")
        shunt(2, 2, f)
        self.assertEqual(f.getvalue(), "ababc")
        
        f = StringIO.StringIO("abc")
        shunt(2, 3, f)
        self.assertEqual(f.getvalue(), "ababc")


if __name__ == '__main__':
    unittest.main()

