#!/usr/bin/env python
# -*- coding: utf-8 -*-

# Authors: 
#     Juan Jesús Ojeda Croissier (juanje) <jojeda@emergya.es>   
#
# Last modified: 
#     $Date$ 
#     $Author$
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
import utils


__revision__ = "0.01"
STATUS_FILE = '/tmp/status'

class Depends:
    """Depends

    This Class provide of a bunch of methods for dealing with the repositories
    we like to use. We can choice the mirror, the components, the distribution
    or codename, even the architecture.

    And we can get lists or sets of packages dependencies for that especific 
    repository we has chosen.

    """

    def __init__(self, extra_pkgs, mirror, codename,
                 components='main,restricted,universe,multiverse', arch='i386'):

        self.extra_pkgs = extra_pkgs
        self.mirror = mirror
        self.codename = codename
        self.components = components
        self.arch = arch
        self.live_packages = ['ubiquity', 'casper',
                              'grub',
                              'linux-image-generic',
                              'linux-restricted-modules-generic']
        self.base = set()

        if not self._get_base_dependencies():
            print >> sys.stderr, \
                     "Error: It was imposible to get the base dependencies"

        if not self._get_packages_file():
            print >> sys.stderr, \
                     "Error: It was imposible to get the Packages file"

        # Set the Packages file as apt's status file
        if not os.path.isfile(STATUS_FILE):
            print >> sys.stderr, "Error: %s doesn't exist" % STATUS_FILE

        apt_pkg.Config.Set("Dir::State::status", STATUS_FILE)
        self.cache = apt.Cache()


    def _get_packages_file(self):
        """_get_packages_file(self) -> bool
        
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
        status_file = open(STATUS_FILE, 'w')
        status_file.write(status_content)
        status_file.close()
    
        return True
    
    
    def _get_base_dependencies(self):
        """_get_base_dependencies(self) -> bool
    
        Get the list of packages which the debootstrap try to install
        
        """
    
        from subprocess import Popen, PIPE
    
        # Simulate a standar debootstrap for getting the list of packages
        # are going to be installed as a base system
        temp_dir = '/tmp/fake_chroot'
        debootstrap = utils.get_path('debootstrap')
        if debootstrap is None:
            return False
        proc = Popen([debootstrap, '--print-debs', self.codename, 
                     temp_dir, self.mirror], stdin=PIPE, stdout=PIPE, 
                     stderr=PIPE, close_fds=True)
        ret = proc.wait()
        if ret != 0:
            return False

        output = proc.stdout.readline().strip()
        self.base = set(output.split())   # Put the list into a set

        return True
    
    
    def get_base_dependencies(self):
        """get_base_dependencies(self) -> set

        Return the self.base set of dpenendecies with the debootstrap install
        as a base system.

        """

        return self.base


    def set_live_packages(self, packages_list=None):
        """set_live_packages(self, packages_list)

        Set the list of packages the live system need

        """

        # Don't change the self.live_packages if packages_list is empty
        if packages_list is None:
            return False
        self.live_packages = packages_list

        return True

     
    def _get_dependencies(self, pkg, deps, key):
        """_get_dependencies(self, pkg, deps, key) -> set

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
                            self._get_dependencies( \
                                      self.cache[dep.TargetPkg.Name], deps, key)
        return deps
 

    def get_live_dependencies(self):
        """get_live_dependencies(self) -> set

        """

        # depends = set()

        # for package_name in self.live_packages:
        #     pkg = self.cache[package_name]
        #     depends.add(package_name)
        #     for key in ['Depends', 'Predepends']:
        #         depends = self._get_dependencies(pkg, depends, key)

        # return depends

        depends = self.get_dependencies_for_list(self.live_packages)
        return depends
    

    def get_all_dependencies(self, package_name):
        """get_all_dependencies(self, package_name) -> set
    
        Get a set of packages from which the package
        package_name depends.
    
        """
        
        pkg = self.cache[package_name]
        
        deps = set()
       
        for key in ['Depends', 'PreDepends']:
            deps = self._get_dependencies(pkg, deps, key)
    
        return deps
    

    def get_dependencies_for_list(self, package_list=None):
        """get_dependencies_for_list(self, package_list=None) -> set

        Get a set with the list of dependencies for a list of packages

        """

        deps = set()
        # Get the dependencies of all the passed packages
        if package_list is not None:
            for pkg in package_list:
                sub_deps = self.get_all_dependencies(pkg)
                sub_deps.add(pkg)
                deps.update(sub_deps)
 
        return deps


    def merge_lists(self):
        """merge_list(self) -> set

        Merge the list from the base package list with the extra package list.

        """

        base_list = self.get_base_dependencies()
        base_deps = set(base_list)

        extra_deps = self.get_dependencies_for_list(self.extra_pkgs)

        if extra_deps is None:
            return base_deps

        total_deps = extra_deps - base_deps

        live_deps = self.get_live_dependencies()

        total_deps = total_deps.union(live_deps)

        return total_deps


    def create_manifests(self):
        """create_manifests(self) -> bool

        Create the filesystem.manifest and filesystem.manifest-desktp.

        """

        dest_dir = '/tmp'
        manifest_file = utils.join_path(dest_dir, 'filesystem.manifest')


        manifest = open(manifest_file, 'w')

        base_list = self.get_base_dependencies()
        base_deps = set(base_list)
        print "base_deps size: %d" % len(base_deps)

        extra_deps = self.get_dependencies_for_list(self.extra_pkgs)
        print "extra_deps size: %d" % len(extra_deps)

        live_depends = self.get_live_dependencies()

        manifest_set = base_deps.union(extra_deps)
        manifest_set.update(live_depends)
        print "manifest_sets size: %d" % len(manifest_set)
        manifest_list = list(manifest_set)
        print "manifest_list size: %d" % len(manifest_list)
        manifest.write('\n'.join(manifest_list))
        manifest.close()


        manifest_desktop_file = utils.join_path(dest_dir,
                                                'filesystem.manifest-desktop')


        manifest_desktop = open(manifest_desktop_file, 'w')

        print "live_depends size: %d" % len(live_depends)
        manifest_desktop_list = manifest_set.difference(live_depends)
        print "manifest_desktop_list size: %d" % len(manifest_desktop_list)
        manifest_desktop_list = list(manifest_desktop_list)
        manifest_desktop.write('\n'.join(manifest_desktop_list))
        manifest_desktop.close()


        # FIXME: Just for testing purposes
        f = open('/tmp/live.pkgs', 'w')
        f.write('\n'.join(list(live_depends.difference(manifest_set))))
        f.close()
        return True

# vim:ai:et:sts=4:tw=80:sw=4:
