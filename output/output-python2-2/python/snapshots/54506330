#!/usr/bin/env python

"""
 --------------------------------------------------------------------------
 CASNET(IP Gateway Client for GUCAS)
 Copyright (C) 2008 Wenbo Yang <solrex@gmail.com>
 Official Homepage http://share.solrex.cn/casnet/
 --------------------------------------------------------------------------

 This program is free software: you can redistribute it and/or modify
 it under the terms of the GNU General Public License as published by
 the Free Software Foundation, either version 3 of the License, or
 (at your option) any later version.

 This program is distributed in the hope that it will be useful,
 but WITHOUT ANY WARRANTY; without even the implied warranty of
 MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 GNU General Public License for more details.

 You should have received a copy of the GNU General Public License
 along with this program.  If not, see <http://www.gnu.org/licenses/>.
 --------------------------------------------------------------------------
"""

import sys
import os

# Options dictionary to store user information.
ops = {
  '-u': '',
  '-d': 'mails.gucas.ac.cn',
  '-p': '',
  '-ip': '210.77.16.29',
  '-m': '2',
  '-r': '1',
  '-a': '1'
}

# Print the help information.
def usage():
  print '''Useage: casnetconf [options]
Options:
  -u <user name>\tUser name, like 20072801*******
  -d <domain>\t\tDomain, default: mails.gucas.ac.cn
  -p <password>\t\tPassword
  -ip <server IP>\tServer IP address, default: 210.77.16.29
  -m <login mode>\t0:SchoolNet, 1:ChinaNet, 2:Internet, default: 2
  -r <remeber password>\t0:no, 1:yes, default: 1
  -a <auto login>\t0:no, 1:yes, default: 1
  --help \t\tPrint this message
  --show \t\tPrint account string

Examples:
  casnetconf
  casnetconf -u 20072801******* -p ******
  
Configure file is saved in ~/.casnet/account, use "less ~/.casnet/account"
to view your account setting.

CASNET 1.1 by Wenbo Yang<solrex@gmail.com>
Official Homepage http://share.solrex.cn/casnet/
'''
  sys.exit(0)

# Get user information from account file.
def show():
  # Get the path to account file, location different for POSIX and Windows.
  if os.name == 'nt':
    homedir = os.getenv('HOMEPATH')
  else:
    homedir = os.getenv('HOME')
  casnetfname = homedir + '/.casnet/account'
  # Open and read account information from account file.
  if not os.path.isfile(casnetfname):
    return False
  else:
    casnetfile = open(casnetfname, 'r')
    line = casnetfile.readline()
    if line == '':
      return False
    casnetfile.close()
    return line

# Parse arguments from the command line.
def parse_args(argv):
  i = 1
  while i < len(argv):
    if argv[i].startswith('--'):
      option = argv[i]
      i = i + 1
      if option == '--help':
        usage()
      elif option == '--show':
        print show()
        print 'You have no saved information. Please reconfig.'
        sys.exit(0)
      else:
        print >>sys.stderr, "Unrecognized option \"%s\", ignored!" % option
        continue
    if argv[i].startswith('-'):
      option = argv[i]
      i = i + 1
      # Put the options start with one '-' into options dictionary.
      if option in ops:
        ops[option] = argv[i]
        i = i + 1
      else:
        print >>sys.stderr, "Unrecognized option \"%s\", ignored!" % option
    else:
      print >>sys.stderr, "Poor option value \"%s\", ignored!" % argv[i]
      i = i + 1
  return True

# Put str into options dictionary.
def input_arg(str, option):
  s = raw_input("%s: " % str)
  if s != '':
    ops[option] = s
  return True

# Write account information in options dictionary to account file.
def write_ops():
  # Get the path to account file, location different for POSIX and Windows.
  if os.name == 'nt':
    homedir = os.getenv('HOMEPATH')
  else:
    homedir = os.getenv('HOME')
  casnetdir = homedir + '/.casnet'
  casnetfname = casnetdir + '/account'
  # If application directory does not exist, create it.
  if not os.path.isdir(casnetdir):
    os.mkdir(casnetdir)
    os.chmod(casnetdir, 0700)
  # If account file does not exist, create it.
  if not os.path.isfile(casnetfname):
    casnetfile = open(casnetfname, 'w+')
    os.chmod(casnetfname, 0600)
  else:
    casnetfile = open(casnetfname, 'w+')
  # Format options directory to account string, write it to account file.
  if ops['-r'] == '0':
    line = ops['-u'] + ':' + ops['-d'] + '::'
  else:
    line = ops['-u'] + ':' + ops['-d'] + ':' + ops['-p'] + ':'
  line = line + ops['-ip'] + ':' + ops['-m'] + ':'
  line = line + ops['-r'] + ':'+ ops['-a']
  casnetfile.write(line)
  casnetfile.close()
  return True

# Main function.
def main(argv=sys.argv, verbose=True):
  # Get user input, interactively or non-interactively.
  if len(argv) > 1:
    parse_args(argv)
    while ops['-u'] == '':
      input_arg('user name', '-u')
    while ops['-p'] == '':
      input_arg('password', '-p')
  else:
    input_arg('server ip(default %s)' % ops['-ip'], '-ip')
    input_arg('domain name(default %s)' % ops['-d'], '-d')
    input_arg('login mode(0:SchoolNet, 1:ChinaNet, 2:Internet, default: 2)', '-m')
    while ops['-u'] == '':
      input_arg('user name', '-u')
    while ops['-p'] == '':
      input_arg('password', '-p')
    input_arg('remember password(0:no, 1:yes; default 1)', '-r')
    input_arg('auto login(0:no, 1:yes; default 1)', '-a')
  # If verbose is True(that means 'casnetconf' runs alone), print user's
  # information to confirm. Or(that means 'casnetconf' is called as a module)
  # do nothing. 
  if verbose:
    print 'You settings:'
    print '  User name: \t%s' % ops['-u']
    print '  Domain: \t%s' % ops['-d']
    print '  Password: \t%s' % ops['-p']
    print '  Server IP: \t%s' % ops['-ip']
    print '  Login mode(0:SchoolNet,1:ChinaNet,2:Internet): \t%s' % ops['-m']
    print '  Remember passwd(0:no,1:yes): \t%s' % ops['-r']
    print '  Auto login(0:no,1:yes): \t%s' % ops['-a']
  # Write the options to account file.
  write_ops()
  return True

# If 'casnetconf' invoked in command line, run main function with no argument.
# Else if 'casnetconf' invoked as a module, do nothing.
if __name__ == "__main__":
  main()
