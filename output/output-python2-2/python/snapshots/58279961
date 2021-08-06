#!/usr/bin/python


import ctypes
import os


def caps_on(display=None):
    if display is None:
        display = os.environ["DISPLAY"]
    libx11 = ctypes.cdll.LoadLibrary("libX11.so.6")
    d = libx11.XOpenDisplay(os.environ["DISPLAY"])
    if not d:
        raise RuntimeError("couldn't open display")
    w = libx11.XDefaultRootWindow(d)
    if not w:
        raise RuntimeError("couldn't get root window")
    root = ctypes.c_int()
    child = ctypes.c_int()
    root_x = ctypes.c_int()
    root_y = ctypes.c_int()
    win_x = ctypes.c_int()
    win_y = ctypes.c_int()
    mask = ctypes.c_int()
    byref = ctypes.byref
    b = libx11.XQueryPointer(d, w, byref(root), byref(child), byref(root_x),
                             byref(root_y), byref(win_x), byref(win_y),
                             byref(mask))
    libx11.XCloseDisplay(d)
    lock_mask = 0x2 # see LockMask in X.h
    return bool(b and mask.value & lock_mask)


for i in range(1024):
    print caps_on()

