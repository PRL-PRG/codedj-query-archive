#!/usr/bin/env python
"""
 --------------------------------------------------------------------------
 CAS NET 1.1
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


import httplib
import re
import sys
import casnetconf

conn_info = []

def usage():
  print '''Useage: casnet [options]
Options:
  [None]\tPrint this message
  on\t\tOnline
  off\t\tOffline
  query\t\tPrint account statistics
  forceoff\tForce offline
  --help\tPrint this message

Examples:
  casnet on
  casnet query

*NOTE*: Before use "casnet", you must configure your account with
        "casnetconf" command. 

CAS NET 1.1 by Wenbo Yang<solrex@gmail.com>
Official Homepage http://share.solrex.cn/casnet/
'''
  sys.exit(0)

def login(account):
  #return True
  if len(conn_info) == 0:
    conn = httplib.HTTPSConnection(account[3])
    conn_info.insert(0, conn)
  else:
    conn = conn_info[0]
  conn.connect()
  data = 'password=%s&domainid=%s&loginuser=%s' % (account[2],{'mails.gucas.ac.cn':'2', 'gucas.ac.cn':'1'}[account[1]],account[0])
  headers = {'Host':account[3],'User-Agent':'casnet_python',
             'Content-Length':str(len(data)),
             'Content-Type':'application/x-www-form-urlencoded'}
  conn.request('POST','/php/user_login.php', data, headers)
  res = conn.getresponse()
  res_html = res.read()
  if(res_html.find('\xB5\xC7\xC2\xBC\xB4\xED\xCE\xF3') != -1):
    return False
  else:
    cookie = res.getheader('Set-Cookie').split(';')[0]
    headers = {'Host':account[3],'User-Agent':'casnet_python',
               'Cookie':cookie,'Cookie2':'$Version="1"'}
    conn_info.insert(1, headers)
    return True

#Global functions
def online(mode):
  #return (True, 'Online succeeded.')
  conn = conn_info[0]
  headers = conn_info[1]
  conn.request('GET','/php/login_net?mode=%s' % mode, None, headers)
  res=conn.getresponse()
  res_html=res.read()
  if(res_html.find('\xC1\xAC\xCF\xDF\xB3\xC9\xB9\xA6') != -1):
    return (True, 'Online succeeded.')
  elif(res_html.find('\xD2\xD1\xBE\xAD\xD4\xDA\xB4\xCB\x20\x49\x50\x20\xC1\xAC\xCF\xDF')):
    return (False, 'Duplicate request!')
  elif(res_html.find('\x27\xD2\xD1\xB4\xEF\xB5\xBD\xD7\xEE\xB4\xF3\xC1\xAC\xCF\xDF\xCA\xFD')):
    return (False, 'Online at other IP!\n "casnet forceoff" to force offline.')
  else:
    return (False, 'Online failed, unknown error!')

def offline():
  #return (True, 'Offline succeeded.')
  conn = conn_info[0]
  headers = conn_info[1]
  conn.request('GET','/php/logout_net.php', None, headers)
  res=conn.getresponse()
  res_html=res.read()
  if(res_html.find('\xC0\xEB\xCF\xDF\xB3\xC9\xB9\xA6')!=-1):
    return (True, 'Offline succeeded.')
  else:
    return (False, 'Offline failed.')

def query():
  #return (True, ('1', '2', '3', '4', '5', '6', '7', '8', '9'))
  modes_dic = {'\xB3\xC7\xD3\xF2':'GucasNet','\xB9\xFA\xC4\xDA':'ChinaNet','\xB9\xFA\xBC\xCA':'Internet'}
  conn = conn_info[0]
  headers = conn_info[1]
  conn.request('GET','/php/onlinestatus.php', None, headers)
  res = conn.getresponse()
  res_html = res.read()
  if(res_html.find('\xD3\xC3\xBB\xA7\xC1\xAC\xCF\xDF\xD7\xB4\xCC\xAC')!=-1):
    a = re.search(r"\xC1\xAC\xCF\xDF\xCA\xB1\xBC\xE4.*?center\">(.*?)</div>.*?center.*?\xB3\xC7\xD3\xF2\xC1\xF7\xC1\xBF.*?right\">(.*?)&nb.*?\xA1\xFC<br>(.*?)&nbsp;.*?\xC1\xAC\xCF\xDF\xB7\xBD\xCA\xBD.*?<div align=\"center\">(.*?)</div>.*?\xB9\xFA\xC4\xDA\xC1\xF7\xC1\xBF.*?right\">(.*?)&nb.*?\xA1\xFC<br>(.*?)&nb.*?\xD7\xDC\xB7\xD1\xD3\xC3.*?center\">(.*?)\xD4\xAA.*?\xB9\xFA\xBC\xCA\xC1\xF7\xC1\xBF.*?right\">(.*?)&nb.*?\xA1\xFC<br>(.*?)&nbsp",
    res_html, re.S)
    if(a != None):
      b = a.groups()
      stat = (b[0], modes_dic[b[3]], b[1], b[2], b[4], b[5], b[7], b[8], b[6])
      return (True, stat)
    else:
      return (False, 'Query failed, online first please!')
  else:
    return (False, 'Query failed, unknown error!')

def forceoff(account):
  #return (True, 'Previous connection')
  conn = conn_info[0]
  headers = conn_info[1]
  conn.request('GET', '/php/useronlinelist.php', None, headers)
  res = conn.getresponse()
  res_html = res.read()
  if(res_html.find('\xB5\xC7\xC2\xBC\xC1\xD0\xB1\xED') != -1):
    a = re.search(r"tokickself\.php\?ip=(.*?)>", res_html, re.S)
    if(a != None):
      b = a.groups()
      c = '/php/tokickself.php?ip=%s' % b[0]
      conn.request("GET", c, None, headers)
      res = conn.getresponse()
      res_html = res.read()
      if(res_html.find('\xD3\xC3\xBB\xA7\xC7\xBF\xD6\xC6\xCD\xCB\xB3\xF6\xCD\xF8\xC2\xE7') != -1):
        cookie = res.getheader('Set-Cookie').split(';')[0]
        c = 'ip=%s&password=%s' % (b[0], account[2])
        d = {'Host':account[3],'User-Agent':'casnet_python','Cookie':cookie,
             'Cookie2':'$Version="1"','Content-Length':str(len(c)),
             'Content-Type':'application/x-www-form-urlencoded'}
        conn.request('POST','/php/kickself.php', c, d)
        res = conn.getresponse()
        res_html = res.read()
        if(res_html.find('\xD3\xC3\xBB\xA7\xC7\xBF\xD6\xC6\xC0\xEB\xCF\xDF\xB3\xC9\xB9\xA6') != -1):
          return (True, 'Previous connection from %s is forced offline!' % b[0])
        elif(res_html.find('\xC3\xDC\xC2\xEB\xB4\xED\xCD\xF3')!=-1):
          return (False, 'Force offline failed, incorrect password!')
    else:
      return (True, 'No other IP onlining.')
  return (False, 'Force offline failed, unkown error.')

def main(account=[], verbose=True):
  if len(account) != 7:
    s = casnetconf.show()
    account = s.split(':')

  #Global settings
  result = ''
  if(login(account) == False):
    return (False, 'Login Failed')
  else:
    if len(sys.argv) == 1:
      usage()
    elif sys.argv[1] == 'on':
      ret, retstr = online(account[4])
      result += retstr
      ret, retstr = query()
      if ret:
        result += '''\nOnline Time: %s, Mode: %s
Statistics:
\tGucasNet: %sMB(up)\t%sMB(down)
\tChinaNet: %sMB(up)\t%sMB(down)
\tInternet: %sMB(up)\t%sMB(down)
\tNet  Fee: %s RMB
''' % retstr
#(retstr[0], retstr[1], retstr[2], retstr[3], retstr[4], retstr[5], 
#       retstr[6], retstr[7], retstr[8])
      else:
        result += retstr
    elif(sys.argv[1] == 'query'):
      ret, retstr = query()
      if ret:
        result += '''\nOnline Time: %s, Mode: %s
Statistics:
\tGucasNet: %sMB(up)\t%sMB(down)
\tChinaNet: %sMB(up)\t%sMB(down)
\tInternet: %sMB(up)\t%sMB(down)
\tNet  Fee: %s RMB
''' % (retstr[0], retstr[1], retstr[2], retstr[3], retstr[4], retstr[5], 
       retstr[6], retstr[7], retstr[8])
    elif(sys.argv[1] == 'off'):
      ret, retstr = query()
      if ret:
        result += '''\nOnline Time: %s, Mode: %s
Statistics:
\tGucasNet: %sMB(up)\t%sMB(down)
\tChinaNet: %sMB(up)\t%sMB(down)
\tInternet: %sMB(up)\t%sMB(down)
\tNet  Fee: %s RMB
''' % (retstr[0], retstr[1], retstr[2], retstr[3], retstr[4],retstr[5], 
       retstr[6], retstr[7], retstr[8])
      ret, retstr = offline()
      result += '\n' + retstr
    elif(sys.argv[1] == 'forceoff'):
      ret, retstr = forceoff(account)
      result += retstr
    else:
      if verbose:
        conn_info[0].close()
        print 'Unknow option!'
        usage()
      else:
        conn_info[0].close()
        return False
  conn_info[0].close()
  if verbose:
    print result

if __name__ == "__main__":
  main()
