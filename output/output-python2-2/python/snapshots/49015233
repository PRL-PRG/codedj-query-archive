# -*- coding: utf-8 -*-

import sys
import os
import gobject
import glob
import syslog

import gettext

from ubiquity import validation
from ubiquity.misc import *

from Queue import Queue
import thread

# Define global path
PATH = '/usr/share/ubiquity'

# Define locale path
LOCALEDIR = "/usr/share/locale"

class Wizard:

    def __init__(self, distro):
        self.distro = distro
        self.pid = False
        self.info = {}
        self.per = 0
        self.parse('/etc/config.cfg',self.info)
     
        # set custom language
        self.set_locales()
        
        
    def run(self):
        error_msg = ['\n']
        error = 0
        for result in validation.check_hostname(self.info['hostname']):
            if result == validation.HOSTNAME_LENGTH:
                error_msg.append("· hostname wrong length (allowed between 3 and 18 chars).\n")
                error = 1
            elif result == validation.HOSTNAME_WHITESPACE:
                error_msg.append("· hostname contains white spaces (they're not allowed).\n")
                error = 1
        if error == 1:
            self.show_error(''.join(error_msg))
        if '/' not in self.info['mountpoints'].values():
             error_msg.append("· mountpoint must start with '/').\n")
             error = 1
        if error == 1:
            self.show_error(''.join(error_msg))
        self.progress_loop()
        self.clean_up()
        return 10 # reboot


    def set_locales(self):
        """internationalization config. Use only once."""
        
        domain = self.distro + '-installer'
        gettext.bindtextdomain(domain, LOCALEDIR)
        gettext.textdomain(domain)
        gettext.install(domain, LOCALEDIR, unicode=1)


    # Methods
    def progress_loop(self):

        def copy_thread(queue):
            """copy thread for copy process."""
            syslog.syslog('Copying the system...')
            cp = copy.Copy()
            if not cp.run(queue):
                syslog.syslog(syslog.LOG_ERR, 'fail the copy phase')
                self.quit()
            else:
                syslog.syslog('Copy: ok')
            queue.put('101')
            
        def config_thread(queue):
            """config thread for config process."""
            syslog.syslog('Configuring the system...')
            cf = config.Config(self)
            if not cf.run(queue):
                syslog.syslog(syslog.LOG_ERR, 'fail the configure phase')
                self.quit()
            else:
                syslog.syslog('Configure: ok')
            queue.put('101')

        for function in [copy_thread,config_thread]:
            # Starting config process
            queue = Queue()
            thread.start_new_thread(function, (queue,))
            
            # setting progress bar status while config process is running
            while True:
                msg = str(queue.get())
                # config process is ended when '101' is pushed
                if msg.startswith('101'):
                    break
                self.set_progress(msg)


    def clean_up(self):
        ex('rm','-f','/cdrom/META/META.squashfs')
        syslog.syslog('Cleaned up')


    def set_progress(self, msg):
        num , text = get_progress(msg)
        if num == self.per:
            return True
        syslog.syslog('%d: %s' % ((num/100.0), text))
        print '%d: %s' % ((num/100.0), text)
        self.per = num
        return True


    def parse(self,name, dict):
        f = open(name)
        for line in f.readlines():
            line = line.strip()
            if line[0] == '#':
                continue
            for word in line.split():
                if '=' in word:
                    name, val = word.split('=', 1)
                    if name == 'mountpoints':
                        mountpoints = {}
                        for each in val.split('-'):
                            mountpoint, device = each.split(':')
                            mountpoints[device] = mountpoint
                        val = mountpoints
                    dict[name] = val
        f.close()


    def show_error(self, msg):
        syslog.syslog(syslog.LOG_ERR, msg)


    def quit(self):
        if self.pid:
            os.kill(self.pid, 9)


    def do_reboot(self, *args):
        """reboot the system after installing process."""

        os.system("reboot")
        self.quit()


    def read_stdout(self, source, condition):
        msg = source.readline()
        if msg.startswith('101'):
            return False
        self.set_progress(msg)
        return True


    def run_main_loop(self):
        pass


    def quit_main_loop(self):
        pass


    def get_hostname(self):
        return self.info['hostname']


# vim:ai:et:sts=4:tw=80:sw=4:
