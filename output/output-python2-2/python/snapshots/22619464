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

from M2Crypto import httpslib, SSL
import urllib
import md5

class HTTPSConnection(httpslib.HTTPSConnection):
    """HTTPS connection with a timeout

    We'll need a connection stub:

        >>> class StubConnection(object):
        ...     def set_socket_read_timeout(self, tm):
        ...          print 'Read timeout set to %d seconds, %d microseconds' % (tm.sec, tm.microsec)
        ...     def set_socket_write_timeout(self, tm):
        ...          print 'Write timeout set to %d seconds, %d microseconds' % (tm.sec, tm.microsec)
        ...     def connect(self, address):
        ...          print 'Connecting to %s' % (address, )

    Now, let's create a HTTPS connection with this stub SSL connection:

        >>> conn = HTTPSConnection('localhost')
        >>> conn._createConnection = lambda: StubConnection()

        >>> conn.connect()
        Connecting to ('localhost', 443)

    If the timeout keyword attribute is provided, it is passed to the
    SSL connection:

        >>> conn = HTTPSConnection('localhost', timeout=4.5)
        >>> conn._createConnection = lambda: StubConnection()

        >>> conn.connect()
        Read timeout set to 4 seconds, 500000000 microseconds
        Write timeout set to 4 seconds, 500000000 microseconds
        Connecting to ('localhost', 443)

    """
    # why do we use the HTTPS connection in the MCrypto2 httpslib module
    # rather than the one in the standard library's httplib module?  Answer
    # (gathered from Benji): the version in MCrypto2 verifies certificates,
    # while the one in the standard library does not.

    def __init__(self, host, port=None, strict=None, timeout=None):
        # timeout is None or float
        self.timeout = timeout
        httpslib.HTTPSConnection.__init__(self, host, port, strict)

    def _createConnection(self):
        return SSL.Connection(self.ssl_ctx)

    def _getTimeout(self, timeout):
        """Create an SSL.timeout object out of a float of seconds

            >>> conn = HTTPSConnection('localhost')
            >>> tm = conn._getTimeout(1.85)
            >>> tm
            <M2Crypto.SSL.timeout.timeout instance at ...>
            >>> tm.sec
            1
            >>> tm.microsec
            850000000

            >>> tm = conn._getTimeout(2.1)
            >>> tm.sec
            2
            >>> tm.microsec
            100000000

        """
        seconds = int(timeout)
        useconds = int((timeout - seconds) * 10 ** 9)
        return SSL.timeout(seconds, useconds)

    def connect(self):
        self.sock = self._createConnection()

        if self.timeout:
            timeout = self._getTimeout(self.timeout)
            self.sock.set_socket_read_timeout(timeout)
            self.sock.set_socket_write_timeout(timeout)
            self.sock.blocking = True

        self.sock.connect((self.host, self.port))


class TransactionResult(object):
    def __init__(self, fields):
        self.response_code = fields[0]
        self.response = {'1': 'approved', '2': 'declined', '3': 'error',
                         '4': 'held for review'}[self.response_code]
        self.response_reason_code = fields[2]
        self.response_reason = fields[3]
        TESTING_PREFIX = '(TESTMODE) '
        if self.response_reason.startswith(TESTING_PREFIX):
            self.test = True
            self.response_reason = self.response_reason[len(TESTING_PREFIX):]
        else:
            self.test = False
        self.approval_code = fields[4]
        self.trans_id = fields[6]
        self.amount = fields[9]
        self.hash = fields[37]

    def validateHash(self, login, salt):
        value = ''.join([salt, login, self.trans_id, self.amount])
        return self.hash.upper() == md5.new(value).hexdigest().upper()


class AuthorizeNetConnection(object):
    def __init__(self, server, login, key, salt=None, timeout=None):
        self.server = server
        self.login = login
        self.salt = salt
        self.timeout = timeout
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
            conn = HTTPSConnection(self.server, timeout=self.timeout)
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
    def __init__(self, server, login, key, salt=None, timeout=None):
        self.connection = AuthorizeNetConnection(
            server, login, key, salt, timeout)

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
