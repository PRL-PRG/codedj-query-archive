#!/usr/bin/env python

import poplib
import S3
import httplib
import sys
import re
import string
import socket
import GMailBackup_Config
import AWS_Config

if (not (hasattr(AWS_Config, 'AWS_ACCESS_KEY_ID') and hasattr(AWS_Config, 'AWS_SECRET_ACCESS_KEY')) ):
    print "You need to add your AWS credentials to AWS_Config.py"
    sys.exit()

BUCKET_NAME = '%(key)s-%(email)s-bucket' % { "key" : AWS_Config.AWS_ACCESS_KEY_ID, "email" : re.compile('(@|\.)').sub('_', GMailBackup_Config.GMAIL_USER) }

conn = S3.AWSAuthConnection(AWS_Config.AWS_ACCESS_KEY_ID, AWS_Config.AWS_SECRET_ACCESS_KEY)

foundit = 0

for i in conn.list_all_my_buckets().entries:
    if (i.name == BUCKET_NAME):
        foundit = 1
        break

if (not foundit):
    print "Creating gmail bucket", BUCKET_NAME
    if (conn.create_bucket(BUCKET_NAME).http_response.status != 200):
        print "Failed to create gmail bucket", BUCKET_NAME
        print "aborting"
        sys.exit()
    else:
        print "Bucket", BUCKET_NAME, "created!"

if hasattr(GMailBackup_Config, 'SOCKSHOST'):
    import socks
    socks.setdefaultproxy(GMailBackup_Config.SOCKSTYPE, GMailBackup_Config.SOCKSHOST)
    socket.socket = socks.socksocket

print "Attempting to connect to GMail..."

def parseHeader(h):
    key = None
    title = 'No subject'
    for i in h:
        if i.lstrip().startswith('X-Gmail-Received:'):
            key = i[len('X-Gmail-Received:'):].lstrip()
        elif i.lstrip().startswith('Subject:'):
            title = i[len('Subject:'):].lstrip()

    return [key, title]


try: 
    pop = poplib.POP3_SSL("pop.gmail.com")

    pop.user(GMailBackup_Config.GMAIL_USER)
    pop.pass_(GMailBackup_Config.GMAIL_PASS)
    
    print
    print "We're in!  There is/are", pop.stat()[0], "email(s) on the system."

    for i in range(len(pop.list()[1])):
        [key, title] = parseHeader(pop.top(i + 1, 1)[1])
        if (key != None):
            print "--->Message", i, "ID:", key, "(%s)" % title
            response =  conn.put(   BUCKET_NAME
                                ,   key
                                ,   S3.S3Object(string.join(pop.top(i + 1, pop.list(i + 1).split(' ')[2])[1], '\n')
                                            ,   {'title': title })
                                ,   {'Content-Type': 'text/plain'}
                                )
            if (response.http_response.status == 200):
                print "Saved!"
            else:
                print "Failed!"
    
    pop.quit()

except poplib.error_proto, detail:
    print "POP3 Protocol Error:", detail

except socket.error, detail:
    print "Socket Error:", detail
