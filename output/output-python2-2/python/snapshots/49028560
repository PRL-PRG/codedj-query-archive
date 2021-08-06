#!/usr/bin/env python
# This program is free software; you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation; either version 2 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Library General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program; if not, write to the Free Software
# write to the Free Software Foundation, Inc., 
# 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA

# Copyright (C) 2006 Ghe Rivero

import shutil
from datetime import date
import os
import re
import commands
import pwd


def first_use():
    example_path="/usr/share/guadalinex-noodle/examples"
    username = pwd.getpwuid(os.getuid())[0]
    home = pwd.getpwuid(os.getuid())[5]
    multisync_path = home + "/.multisync/"

    if not os.path.exists(multisync_path):
        os.mkdir (multisync_path)

    if not os.path.exists(multisync_path + "/old"):
        os.mkdir (multisync_path + "/old")

    if os.path.exists (multisync_path + "/1"):
            shutil.move (multisync_path + "/1", multisync_path + date.today().strftime("%Y%m%d"))

    os.mkdir (multisync_path + "/1")

    if not os.path.exists(multisync_path + "/localsettings"):
         cregex=re.compile("__HOME__")
         readlines=open(example_path + "/localsettings" ,'r').readlines()
         write_file = open(multisync_path + "/1/localsettings",'w')
         for currentline in readlines:
             if cregex.search(currentline):
                 currentline = re.sub("__HOME__",home,currentline) 
             write_file.write(currentline)
         write_file.close()

    shutil.copy (example_path + "/syncpair", multisync_path + "/1/syncpair" )
    
    write_file = open(home + "/.psuite",'w')
    write_file.write(date.today().strftime("%Y%m%d"))
    write_file.close()

    return

def main():
    home = pwd.getpwuid(os.getuid())[5]
    username = pwd.getpwuid(os.getuid())[0]
    
    if not os.path.exists(home):
        return 1
    
    if not os.path.exists(home + "/.psuite"):
        first_use()
    running =  not commands.getstatusoutput("pgrep -u " + username + " synce-trayicon")[0]
    if running:
         os.system ("killall -HUP synce-trayicon")
    else:
        os.system ("synce-trayicon")
    
    return

if __name__ == '__main__':
    main()
