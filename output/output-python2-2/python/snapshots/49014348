# -*- coding: utf-8 -*-

"""
Noui Frontend

Noui frontend implementation for the installer
This UI implementation consists of no UI at all: it retrieves answers
noninteractively. We don't ask because the answers already exist.

Some of the answers will need to be preseeded in advance, but most will be
determined from the running system.
"""

import debconf
    
from ubiquity.backend.part import call_autoparted


class Wizard:
    '''
    This is a wizard interface to interact with the user and the 
    main program. It has some basic methods:
     - set_progress()
     - get_info()
     - get_partitions()
    '''
    def __init__(self):
        debconf.runFrontEnd()
        self.db = debconf.Debconf()

    def set_progress(self,num,msg=''):
        '''set_progress(num, msg='') -> none

        Put the progress bar in the 'num' percent and if
        there is any value in 'msg', this method print it.
        '''
        print "%d\t%s" % (num,msg)

    def get_info(self):
        '''get_info() -> [hostname, fullname, name, password]

        Get from the Debconf database the information about
        hostname and user. Return a list with those values.
        '''
        info = []
        hostname = open('/etc/hostname')
        info.append(hostname.readline().strip())
        hostname.close()
        info.append(self.db.get('passwd/user-fullname'))
        info.append(self.db.get('passwd/username'))
        info.append(self.db.get('passwd/user-password'))
        return info
        
    def get_partitions(self):
        '''get_partitions() -> dict {'mount point' : 'dev'}

        Get the information to be able to partitioning the disk.
        Partitioning the disk and return a dict with the pairs
        mount point and device.
        At least, there must be 2 partitions: / and swap.
        '''
        #FIXME: We've to put here the autopartitioning stuff
        
        # This is just a example info.
        # We should take that info from the debconf
        # Something like:
        # re = self.db.get('ubiquity/mountpoints')
        # for path, dev in re:
        #     mountpoints[path] = dev
        self.mountpoints = {'/'         : '/dev/hda1',
                                                'swap'    : '/dev/hda2',
                                                '/home' : '/dev/hda3'}
        # TODO cjwatson 2006-02-01: convert this to debconffiltered partman
        self.mountpoints = call_autoparted()
        if self.mountpoints is None:
            print 'Autopartioning fail!'

        return self.mountpoints

    def run_main_loop(self):
        pass

    def quit_main_loop(self):
        pass

    def get_hostname(self):
        return self.get_info()[0]

    def get_mountpoints(self):
        # TODO cjwatson 2006-03-08: partman now expects
        # {mountpoint -> (partition, format?, fstype)}; this frontend should
        # be fixed to work with that internally (see gtkui).
        dummy_mountpoints = {}
        for mountpoint, partition in self.mountpoints.iteritems():
            dummy_mountpoints[mountpoint] = (partition, True, None)
        return dummy_mountpoints

if __name__ == '__main__':
    w = Wizard()
    hostname, fullname, name, password = w.get_info()
    print '''
    Hostname: %s
    User Full name: %s
    Username: %s
    Password: %s
    Mountpoints : %s
    ''' % (hostname, fullname, name, password, w.get_partitions())

# vim:ai:et:sts=4:tw=80:sw=4:
