#!/usr/bin/env python
#-*- coding: utf-8 -*-

class DevClient(Exception):
    """
    Base class for all exceptions from DevClient.
    """
    pass

class ClassNotFound(DevClient):
    """
    Class for handling error on finding class to load.
    """

    def __init__(self, msg):
        self.message = msg

    def __str__(self):
        return self.message

class BufferUnderSize(DevClient):
    """
    Handling error on losing data because buffer size is not sufficient.
    """
    pass

class ConnectionRefused(DevClient):
    """
    Handling error on establish connection.
    """
    pass
