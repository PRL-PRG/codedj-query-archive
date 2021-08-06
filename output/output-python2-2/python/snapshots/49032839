#!/usr/bin/python
# -*- coding: utf-8 -*-

import os
import logging

def get_sudo():
    """
    Return true if user has login as sudoer. False in other way.
    """
    logger = logging.getLogger() 
    #Try for password. Three times.
    res = 768 
    attemps = 0

    # Errno 768: Bad password
    while res == 768 and attemps < 3:
        res = os.system('gksudo -m "Introduzca contraseña" /bin/true')
        # Errno 512: User press cancel
        if res == 512:
            logger.debug("User press cancel")
            return False
        attemps += 1

    if res == 768:
        logger.debug("Three attemps for password")
        return False

    return True

