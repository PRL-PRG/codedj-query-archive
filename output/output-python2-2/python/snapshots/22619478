##############################################################################
#
# Copyright (c) 2004 Zope Corporation and Contributors.
# All Rights Reserved.
#
# This software is subject to the provisions of the Zope Public License,
# Version 2.1 (ZPL).  A copy of the ZPL should accompany this distribution.
# THIS SOFTWARE IS PROVIDED "AS IS" AND ANY AND ALL EXPRESS OR IMPLIED
# WARRANTIES ARE DISCLAIMED, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
# WARRANTIES OF TITLE, MERCHANTABILITY, AGAINST INFRINGEMENT, AND FITNESS
# FOR A PARTICULAR PURPOSE.
#
##############################################################################

from M2Crypto import httpslib
import urllib
import md5

class TransactionResult(object):
    def __init__(self, fields):
        response_code_map = {'1': 'approved', '2': 'declined', '3': 'error'}
        self.response = response_code_map[fields[0]]
        self.response_reason = fields[3]
        TESTING_PREFIX = '(TESTMODE) '
        if self.response_reason.startswith(TESTING_PREFIX):
            self.response_reason = self.response_reason[len(TESTING_PREFIX):]
        self.approval_code = fields[4]
        self.trans_id = fields[6]
        self.amount = fields[9]
        self.hash = fields[37]

    def validateHash(self, login, salt):
        value = ''.join([salt, login, self.trans_id, self.amount])
        return self.hash == md5.new(value).hexdigest()


class AuthorizeNetConnection(object):
    def __init__(self, server, login, key, salt=None):
        self.server = server
        self.login = login
        self.salt = salt
        self.delimiter = '|'
        self.standard_fields = dict(
            x_login = login,
            x_tran_key = key,
            x_version = '3.1',
            x_delim_data = 'TRUE',
            x_delim_char = self.delimiter,
            x_relay_response = 'FALSE',
            x_method = 'CC',
            )

    def sendTransaction(self, **kws):
        # if the card number passed in is the "generate an error" card...
        if kws.get('card_num') == '4222222222222':
            # ... turn on test mode (that's the only time that card works)
            kws['test_request'] = 'TRUE'

        fields = dict(('x_'+key, value) for key, value in kws.iteritems())
        fields.update(self.standard_fields)
        body = urllib.urlencode(fields)

        if self.server.startswith('localhost:'):
            server, port = self.server.split(':')
            conn = httpslib.HTTPConnection(server, port)
        else:
            conn = httpslib.HTTPSConnection(self.server)
        conn.putrequest('POST', '/gateway/transact.dll')
        conn.putheader('content-type', 'application/x-www-form-urlencoded')
        conn.putheader('content-length', len(body))
        conn.endheaders()
        conn.send(body)

        response = conn.getresponse()
        fields = response.read().split(self.delimiter)
        result = TransactionResult(fields)

        if (self.salt is not None
        and not result.validateHash(self.login, self.salt)):
            raise ValueError('MD5 hash is not valid (trans_id = %r)'
                             % result.trans_id)

        return result


class CcProcessor(object):
    def __init__(self, server, login, key, salt=None):
        self.connection = AuthorizeNetConnection(server, login, key, salt)

    def authorize(self, **kws):
        if not isinstance(kws['amount'], basestring):
            raise ValueError('amount must be a string')

        type = 'AUTH_ONLY'
        return self.connection.sendTransaction(type=type, **kws)

    def captureAuthorized(self, **kws):
        type = 'PRIOR_AUTH_CAPTURE'
        return self.connection.sendTransaction(type=type, **kws)

    def void(self, **kws):
        type = 'VOID'
        return self.connection.sendTransaction(type=type, **kws)
