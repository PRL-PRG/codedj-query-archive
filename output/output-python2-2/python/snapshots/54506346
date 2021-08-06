#!/usr/bin/env python
# -*- coding: gbk -*-
# This file should be opened with encoding GBK[GB18030, GB2312].
"""
 --------------------------------------------------------------------------
 CAS NET 1.0
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

CAS NET 1.0 by Wenbo Yang<solrex@gmail.com>
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
  if(res_html.find("登录错误") != -1):
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
  if(res_html.find('连线成功') != -1):
    return (True, 'Online succeeded.')
  elif(res_html.find('已经在此 IP 连线')):
    return (False, 'Duplicate request!')
  elif(res_html.find('已达到最大连线数')):
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
  if(res_html.find('离线成功')!=-1):
    return (True, 'Offline succeeded.')
  else:
    return (False, 'Offline failed.')

def query():
  #return (True, ('1', '2', '3', '4', '5', '6', '7', '8', '9'))
  modes_dic = {'城域':'GucasNet','国内':'ChinaNet','国际':'Internet'}
  conn = conn_info[0]
  headers = conn_info[1]
  conn.request('GET','/php/onlinestatus.php', None, headers)
  res = conn.getresponse()
  res_html = res.read()
  if(res_html.find('用户连线状态')!=-1):
    a = re.search(r"连线时间.*?center\">(.*?)</div>.*?center.*?城域流量.*?right\">(.*?)&nb.*?↑<br>(.*?)&nbsp;.*?连线方式.*?<div align=\"center\">(.*?)</div>.*?国内流量.*?right\">(.*?)&nb.*?↑<br>(.*?)&nb.*?总费用.*?center\">(.*?)元.*?国际流量.*?right\">(.*?)&nb.*?↑<br>(.*?)&nbsp",
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
  if(res_html.find('登录列表') != -1):
    a = re.search(r"tokickself\.php\?ip=(.*?)>", res_html, re.S)
    if(a != None):
      b = a.groups()
      c = '/php/tokickself.php?ip=%s' % b[0]
      conn.request("GET", c, None, headers)
      res = conn.getresponse()
      res_html = res.read()
      if(res_html.find('用户强制退出网络') != -1):
        cookie = res.getheader('Set-Cookie').split(';')[0]
        c = 'ip=%s&password=%s' % (b[0], account[2])
        d = {'Host':account[3],'User-Agent':'casnet_python','Cookie':cookie,
             'Cookie2':'$Version="1"','Content-Length':str(len(c)),
             'Content-Type':'application/x-www-form-urlencoded'}
        conn.request('POST','/php/kickself.php', c, d)
        res = conn.getresponse()
        res_html = res.read()
        if(res_html.find('用户强制离线成功') != -1):
          return (True, 'Previous connection from %s is forced offline!' % b[0])
        elif(res_html.find('密码错误')!=-1):
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
