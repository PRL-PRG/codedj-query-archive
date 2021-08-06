#!/usr/bin/env python
# -*- coding: utf-8 -*-

# Authors: 
#     Juan Jesús Ojeda Croissier (juanje) <jojeda@emergya.es>   
#
# Last modified: 
#     $Date:  $ 
#     $Author:  $
#

""" Module Depends.

This is a module for get dependencies os a package or group of packages.
Also it has moethods for getting the dependencies of the base system minus
the extra packages and so on.

"""  

import sys
import apt
import apt_pkg
import os.path
from decompress import get_content


__revision__ = "0.01"

class Depends:
    """Depends

    This Class provide of a bunch of methods for dealing with the repositories
    we like to use. We can choice the mirror, the components, the distribution
    or codename, even the architecture.

    And we can get lists or sets of packages dependencies for that especific 
    repository we has chosen.

    """

    def __init__(self, mirror, codename,
                 components='main,restricted,universe,multiverse', arch='i386'):

        self.mirror = mirror
        self.codename = codename
        self.components = components
        self.arch = arch
        self.live_packages = ['ubiquity', 'casper']
        self.status_file = '/tmp/status'

        if not self.get_packages_file():
            print >> sys.stderr, \
                     "Error: It was imposible to get the Packages file"

        # Set the Packages file as apt's status file
        if not os.path.isfile(self.status_file):
            print >> sys.stderr, "Error: %s doesn't exist" % self.status_file

        apt_pkg.Config.Set("Dir::State::status", self.status_file)
        self.cache = apt.Cache()


    def get_packages_file(self):
        """get_packages_file(self) -> bool
        
        get_packages_file get the Packages[.bz2|gz] from the mirror. Either 
        local or remote. After decompress the file it writes a file /tmp/status
        with the content.
        
        """
        
        # Create a URI like 
        # http://www.server.org/ubuntu/dists/edgy/main/binary-i386/Packages.bz2
        status_content = None
        for component in self.components.split(','):
            for ext in ['.bz2', '.gz', '']:
                uri = os.path.join(self.mirror,   # Server or local repository
                                   'dists',       # Standard name
                                   self.codename, # edgy, dapper, etc
                                   component,     # main, resticted, universe..
                                   'binary-' + self.arch, # i386...
                                   'Packages' + ext)      # bz2, gz, no one...
    
                content = get_content(uri)
                if content is not None:
                    break
    
            if content is not None:
                if status_content is None:
                    status_content = ''
                status_content += "\n" + content
    
        if status_content is None:
            return False
    
        # Create the status file
        status_file = open(self.status_file, 'w')
        status_file.write(status_content)
        status_file.close()
    
        return True
    
    
    def get_base_dependencies(self):
        """get_base_dpendencies(self) -> set
    
        Get the list of packages which the debootstrap try to install
        
        """
    
        from subprocess import Popen, PIPE
    
        # Simulate a standar debootstrap for getting the list of packages
        # are going to be installed as a base system
        temp_dir = '/tmp/fake_chroot'
        proc = Popen(["/usr/sbin/debootstrap", "--print-debs", self.codename, \
                     temp_dir, self.mirror], stdin=PIPE, stdout=PIPE, \
                     stderr=PIPE, close_fds=True)
        output = proc.stdout.readline().strip()
        packages = set(output.split())   # Put the list into a set
    
        return packages
    
    
    def set_live_packages(self, packages_list=None):
        """set_live_packages(self, packages_list)

        Set the list of packages the live system need

        """

        # Don't change the self.live_packages if packages_list is empty
        if packages_list is None:
            return False
        self.live_packages = packages_list

        return True

     
    def __get_dependencies(self, pkg, deps, key):
        """get_dependencies(self, pkg, deps, key) -> set

        Get the dependencies or predependencies for a specific package.

        """

        candidate_ver = self.cache._depcache.GetCandidateVer(pkg._pkg)
        if candidate_ver == None:
            return deps
        depends_list = candidate_ver.DependsList
        if depends_list.has_key(key):
            for depends_ver_list in depends_list[key]:
                for dep in depends_ver_list:
                    if self.cache.has_key(dep.TargetPkg.Name):
                        if pkg.name != dep.TargetPkg.Name and \
                           not dep.TargetPkg.Name in deps:
                            deps.add(dep.TargetPkg.Name)
                            self.__get_dependencies(self.cache, \
                            self.cache[dep.TargetPkg.Name], deps, key)
        return deps
 

    def get_live_dependecies(self):
        """get_live_dependencies(self) -> set

        """

        depends = set()

        for package_name in self.live_packages:
            pkg = self.cache[package_name]
            for key in ['Depends', 'Predepends']:
                depends = self.__get_dependencies(pkg, depends, key)

        return depends
    

    def get_all_dependencies(self, package_name):
        """get_all_dependencies(self, package_name) -> set
    
        Get a set of packages from which the package
        package_name depends.
    
        """
        
        pkg = self.cache[package_name]
        
        deps = set()
       
        for key in ['Depends', 'PreDepends']:
            deps = self.__get_dependencies(pkg, deps, key)
    
        return deps
    

# vim:ai:et:sts=4:tw=80:sw=4:
