# pyclene build script

import os, os.path, platform, re, struct, sys

from distutils.core import setup, Extension
import distutils.sysconfig


def detectPlatformBasics(argsLower, argJam):
    global osIsWindows, osIsPosix, osIsLinux, osIs64bit, \
           compilerIsGCC, compilerIsMinGW

    # OS:
    osIsWindows = sys.platform.lower().startswith('win')
    # Assume that any non-Windows platform supported by pyclene is POSIX-
    # compliant:
    osIsPosix = not osIsWindows
    osIsLinux = sys.platform.lower().startswith('linux')
    # If the size of a native C long is > 32 bits, we term the OS "64-bit":
    osIs64bit = struct.calcsize('l') > 4

    # COMPILER:
    compilerIsMinGW = '-cmingw32' in argJam or '--compiler=mingw32' in argJam
    # If compiling on a platform other than Windows, assume that the compiler
    # is GCC (or compatible with GCC):
    compilerIsGCC = ((compilerIsMinGW or not osIsWindows) and True) or False

    if compilerIsMinGW:
        # Create MinGW-compatible library file libpythonVV.a on the basis of
        # the MSVC-oriented library file pythonVV.lib if that file isn't
        # already present in this Python installation.
        import mingw_support
        mingw_support.generateMinGWPythonLib(out=sys.stderr)

        macroDefs.append(('COMPILER_MINGW32', '1'))
    elif osIsWindows:
        # If platform is Windows but compiler is not MinGW, assume it's MSVC.
        macroDefs.append(('COMPILER_MSVC', '1'))


    if osIsWindows:
        # If compiling from a *source* distribution of Python on Windows, add a
        # couple of necessary include and lib directories:
        pcbuildDir = os.path.join(
            os.path.dirname(distutils.sysconfig.get_python_inc()), 'PCBuild'
          )
        if os.path.exists(pcbuildDir):
            libDirs.append(pcbuildDir)
            pySrcDistExtraIncludeDir = os.path.join(
                os.path.dirname(distutils.sysconfig.get_python_inc()), 'PC'
              )
            includeDirs.append(pySrcDistExtraIncludeDir)

        # Compiler-specific C++ library config:
        if compilerIsMinGW:
            # Need to explicitly link to the standard C++ library:
            libs.append('stdc++')
        else:
            # If compiling on Windows and not using MinGW, assume using MSVC.

            # Enable C++ RTTI (with MSVC 7.1, must do this explicitly or prog
            # generates unhandled exception):
            extraCompilerArgs.append('/GR')
    else: # not Windows
        # This build script takes the place of CLucene's make-based build, yet
        # there are places in CLucene's non-Windows code paths where it assumes
        # that make-generated support files will be present unless the symbol
        # _SUPPRESS_MAKE_BASED_CONFIG is defined:
        macroDefs.append(('_SUPPRESS_MAKE_BASED_CONFIG', '1'))

        # Use distutils to replace some of the config detection functionality
        # of the make-based build:
        import distutils.command.config as cmd_conf
        import distutils.dist as dist_dist

        class _ConfigUglyHack(cmd_conf.config):
            # _ConfigUglyHack circumvents a distutils problem brought to light
            # on Unix by this script's abuse of the distutils.
            def try_link(self, *args, **kwargs):
                self.compiler.exe_extension = '' # ('' rather than None)
                return cmd_conf.config.try_link(self, *args, **kwargs)
        cfg = _ConfigUglyHack(dist_dist.Distribution())

        for (headerName, preprocSymbol) in (
            ('dirent.h', 'HAVE_DIRENT_H'),
          ):
            if cfg.check_header(headerName, lang='c++'):
                macroDefs.append((preprocSymbol, '1'))


def detectOptLocal():
    # By default, this build script examines the local system and tries to
    # choose the most aggressive optimizations, even if they make the binaries
    # incompatible with other CPUs.
    # This behavior can be disabled with command-line switch '--no-opt-local'.

    if osIsWindows:
        # Define a shortcut function 'reg' to allow us to read the registry:
        import _winreg
        _reg = _winreg.ConnectRegistry(None, _winreg.HKEY_LOCAL_MACHINE)
        def reg(keyName, valueName):
            key = _winreg.OpenKey(_reg, keyName)
            try:
                return _winreg.QueryValueEx(key, valueName)[0]
            finally:
                _winreg.CloseKey(key)

    procArch = None
    compIsMultiprocessor = False
    if osIsWindows:
        # YYY: Are these registry values present on Win9x?
        procLoc = r'HARDWARE\DESCRIPTION\System\CentralProcessor\0'
        procVendor = reg(procLoc, 'VendorIdentifier').lower()
        procName = reg(procLoc, 'ProcessorNameString').lower()

        # Multiprocessor?
        compIsMultiprocessor = 'uniprocessor' not in platform.win32_ver()[3].lower()

        # Specfic vendor/model flags:
        # (There are no Intel-oriented settings here because I don't own any
        # Intel machines; contributors are welcome to add them.)
        if 'amd' in procVendor:
            if 'athlon' in procName:
                if osIs64bit:
                    procArch = 'k8'
                else:
                    procArch = 'athlon'
                    if compIsMultiprocessor:
                        procArch += '-mp'
                    # YYY: Should this check for SSE support instead of "'xp'
                    # in procName"?  For example, a Sempron should probably
                    # match arch 'athlon-xp'.
                    elif 'xp' in procName or '64' in procName:
                        procArch += '-xp'
            elif 'opteron' in procName:
                if osIs64bit:
                    procArch = 'k8'
                else:
                    procArch = 'athlon-xp'
    elif osIsLinux:
        cpuInfo = file('/proc/cpuinfo').read().lower().replace('(', ' ').replace(')', ' ')
        cpuInfoWords = cpuInfo.split()

        # Multiprocessor?
        processorIds = re.findall(r'processor\s*:\s*\d+', cpuInfo)
        compIsMultiprocessor = len(processorIds) > 1

        # Specfic vendor/model flags:
        # (There are no Intel-oriented settings here because I don't own any
        # Intel machines; contributors are welcome to add them.)
        procVendor = re.search(r'vendor_id\s*:\s*(?P<id>(\w+)+)', cpuInfo)
        if procVendor is not None and 'amd' in procVendor.group('id'):
            if 'athlon' in cpuInfoWords:
                if osIs64bit:
                    procArch = 'k8'
                else:
                    procArch = 'athlon'
                    if compIsMultiprocessor:
                        procArch += '-mp'
                    elif 'sse' in cpuInfoWords:
                        procArch += '-xp'
            elif 'opteron' in cpuInfoWords:
                if osIs64bit:
                    procArch = 'k8'
                else:
                    procArch = 'athlon-xp'

    return procArch, compIsMultiprocessor


def setOptimizations(optDisableAll, optDisableLocal, procArch):
    optArgList = []

    if optDisableAll:
        if compilerIsGCC:
            optArgList.append('-O0') # Opt-compile-speed
        elif osIsWindows: # Assume MSVC:
            optArgList.append('/Od') # Opt-compile-speed

            # Distutils passes /DNDEBUG to disable asserts; we counteract that
            # in the unoptimized build by undefining the same symbol.
            optArgList.append('/UNDEBUG')

            # Tell MSVC to insert integrity-checking code, such as checks for
            # use of unitialized locals.
            optArgList.extend(['/RTCsu', '/GSe'])
    else:
        if compilerIsGCC:
            if not optDisableLocal:
                if procArch:
                    optArgList += ['-march=' + procArch]

            optArgList.append('-DNDEBUG')

            optArgList.append('-O3')

            optArgList.append('-fmove-all-movables')
            optArgList.append('-freduce-all-givs')
            optArgList.append('-ftracer')

            # Omitting the frame pointer delivers a speed gain, but can
            # interfere with debugging tools:
            optArgList.append('-fomit-frame-pointer')

            # Small speed gain:
            optArgList.append('-ffast-math')

            # Debatable effect on speed, and only supported by GCC 3.4 and later:
            # optArgList.append('-fbranch-target-load-optimize2')

            # Has little impact on speed (on a CPU with 1MB cache), but
            # increases code size immensely:
            # optArgList.append('-funroll-loops')

            # Enabling new register allocation (-fnew-ra) is an *experimental*
            # option that's not intended for production code with any current
            # GCC release.
            # It delivers a slight speed gain with GCC-3.3, but causes GCC-3.4
            # to generate crashing code.
            # optArgList.append('-fnew-ra')
        else:
            # Assume MSVC 7.1:

            # G7 optimizes for Pentium IV / Athlon processors:
            optArgList.append('/G7')

            # Link-time code generation creates code that's some 5% faster.
            optArgList.append('/GL')
            extraLinkArgs.append('/LTCG')

            # Allow the compiler to choose what to inline.
            optArgList.append('/Ob2')


    return optArgList


def detectCLuceneSources():
    # Find all operative "C/C++ source files" (excluding header files) in
    # ../../src:
    cluceneSources = []
    for dirPath, dirNames, fileNames in os.walk(CLuceneSrcDir):
        for fn in fileNames:
            if os.path.splitext(fn)[1] in (
                '.c', '.cpp', '.cxx',
                # Don't need to explicitly include headers:
                # '.h', '.hpp', '.hxx'
              ):
                cluceneSources.append( os.path.join(dirPath, fn) )

    cluceneSources.sort()
    return cluceneSources


def createPycleneSourcesManifest():
    # Generates a distutils-compatible MANIFEST file, which is used chiefly
    # for creating source distributions with 'setup.py sdist'.

    def allFilesWithExt(inDir, extensions=(), recursive=False):
        extensions = [ext.lower() for ext in extensions]
        def fileMatches(f):
            if os.path.isdir(f):
                return False
            # Prefix:
            if f.startswith('nodist') or f.startswith('.'):
                return False
            # Suffix:
            ext = os.path.splitext(f)[1].lower()
            if ext in ('.pyc', '.pyo'):
                return False
            if len(extensions) == 0:
                return True
            else:
                return ext in extensions

        if not recursive:
            return [os.path.join(inDir, f) for f in os.listdir(inDir) if fileMatches(f)]
        else:
            files = []
            for dirPath, dirNames, fileNames in os.walk(inDir):
                for f in fileNames:
                    if fileMatches(f):
                        files.append( os.path.join(dirPath, f) )
            return files


    allPycleneSourceFiles = []
    allPycleneSourceFiles.extend( allFilesWithExt(os.curdir) )

    # Note that the following 'CLucene' directory structure contains snippets
    # of CLucene headers massaged for SWIG compatibility, rather than CLucene's
    # actual C++ code:
    allPycleneSourceFiles.extend( allFilesWithExt('CLucene', recursive=True) )

    # Here's the "actual C++ code":
    allPycleneSourceFiles.extend(allFilesWithExt(CLuceneSrcDir, recursive=True))

    allPycleneSourceFiles.extend( allFilesWithExt('demo', recursive=True) )
    allPycleneSourceFiles.extend( allFilesWithExt('doc', recursive=True) )
    allPycleneSourceFiles.extend( allFilesWithExt('tests', recursive=True) )

    allPycleneSourceFiles.sort()

    # Don't include the file named 'MANIFEST'; this script autogenerates it.
    try:
        allPycleneSourceFiles.remove(os.curdir + os.sep + 'MANIFEST')
    except ValueError:
        pass

    maniFile = file('MANIFEST', 'wb')
    for fn in allPycleneSourceFiles:
        print >> maniFile, fn
    maniFile.close()


## GLOBAL VARS : Begin ##
# Normally would frown on using globals so extensively, but this is a mere
# build script.
CLuceneSrcDir = os.path.abspath(os.path.join(os.pardir, os.pardir, 'src'))

osIsWindows, osIsPosix, osIsLinux, osIs64bit = False, False, False, False
compilerIsGCC, compilerIsMinGW = False, False

extraCompilerArgs = []
macroDefs = []
includeDirs = []
libDirs = []
libs = []
extraLinkArgs = []
## GLOBAL VARS : End ##

def main():
    pycleneVersion = file('version.txt', 'rb').read().strip()

  ## COMMAND-LINE ARGUMENTS : Begin ##
    # Under this primitive arg-parsing scheme, we remove the args we're
    # interested in from sys.argv after detecting them, so that distutils
    # won't reject them as unrecognized.

    argsLower = [arg.lower() for arg in sys.argv[1:]]
    argJam = ('|'.join(argsLower).replace(' ', '').replace('\t', '')) + '|'

    optDisableAll = '--no-opt|' in argJam
    if optDisableAll: sys.argv.remove('--no-opt')

    optDisableLocal = '--no-opt-local|' in argJam
    if optDisableLocal: sys.argv.remove('--no-opt-local')

    monolithicCompilation = '--no-monolithic-compilation|' not in argJam
    if not monolithicCompilation: sys.argv.remove('--no-monolithic-compilation')
  ## COMMAND-LINE ARGUMENTS : End ##

  ## PLATFORM AUTODETECTION : Begin ##
    detectPlatformBasics(argsLower, argJam)

    procArch, compIsMultiprocessor = detectOptLocal()
    extraCompilerArgs.extend(setOptimizations(optDisableAll, optDisableLocal, procArch))
  ## PLATFORM AUTODETECTION : End ##

  ## CONCURRENCY : Begin ##
    # YYY:CONCURRENCY:
    # Before enabling any sort of concurrency, need to:
    #   - Remodel CLucene's own MT code to use proper concurrency constructs
    #     for Win32/POSIX platforms.
    #   - Determine how to cause SWIG to generate GIL-manipulation code at the
    #     appropriate places.
    #     - Looks like GIL-manipulation code can be rolled in the %exception
    #       code so that every call to the wrapped library is surrounded by
    #       GIL release/acquire pairs, but there are problems with this (see
    #       DSR's e-mail to CLucene-dev on 2004.11.22).

    # Make best guess for the local machine, but override that guess with the
    # user's choice, if any was specified.
    if compIsMultiprocessor:
        concurrencyLevel = 2
    else:
        concurrencyLevel = 1
    # Determine whether user specified a choice:
    mConcLev = re.search('--concurrency=(\d+)', argJam)
    if mConcLev:
        concurrencyLevel = int(mConcLev.group(1))

    # YYY:CONCURRENCY: Since the aforementioned tasks are incomplete, force
    # concurrencyLevel to zero, disabling concurrency altogether.
    concurrencyLevel = 0

    if concurrencyLevel == 0:
        macroDefs.append(('LUCENE_DISABLE_MULTITHREADING', '1'))
  ## CONCURRENCY : End ##

  ## UNICODE : Begin ##
    # YYY:UNICODE: Unicode support has yet to be added to pyclene.

    #if '--with-unicode|' in argJam:
        #macroDefs.append(('_UNICODE', '1'))
        # Python can be compiled to use 2-byte unicode chars or 4-byte.  By
        # default, Red Hat uses 4-byte, while the Python.org Windows builds and
        # most Linux distributions use 2-byte.  Anyone attempting to add
        # Unicode support to pyclene needs to be aware of this.
        #macroDefs.append(('_UNICODE_SIZE_PYTHON', ((sys.maxunicode > 65535 and '4') or '2')))
  ## UNICODE : End ##

  ## COMPILATION CONFIG : Begin ##
    # Using SWIG_COBJECT_TYPES yields a slight-to-moderate performance gain.
    # (A significant disadvantage is that SWIG_COBJECT_TYPES complicates the
    # use of a debug-Python-build's COUNT_ALLOCS facility.)
    macroDefs.append(('SWIG_COBJECT_TYPES', '1'))

    # By default (disableable with command-line switch
    # '--no-monolithic-compilation'), this build script includes the entire
    # set of CLucene C++ files into _clucene_wrap.cpp (indirectly, via
    # nodist__clucene_source_files__include_all.h) rather than submitting each
    # C++ file as a separate entry to distutils.core.Extension.
    #
    # This causes the compilation to use a much larger amount of RAM, but to
    # execute far more quickly with compilers that don't support precompiled
    # headers.  More importantly:
    #   - Compilers such as GCC 3.4 that support unit-level optimization but
    #     not whole-program optimization are able to optimize better this way
    #     because the entire program is stuffed into a single compilation unit.
    #   - Some compilers (e.g., GCC) generate much smaller binaries when only
    #     one compilation unit is involved.
    distutilsExtensionSources = ['_clucene_wrap.cpp']

    cluceneSources = detectCLuceneSources()
    # f is filled with include directives if we're compiling monolithically;
    # cleared if not.
    f = file('nodist__clucene_source_files__include_all.h', 'wb')
    try:
        if monolithicCompilation:
            macroDefs.append(('MONOLITHIC_COMPILATION', '1'))
            for src in cluceneSources:
                print >> f, '#include "%s"' % src.replace(os.sep, '/')
        else:
            distutilsExtensionSources.extend(cluceneSources)
    finally:
        f.close()


    # Create the MANIFEST file that specifies which files to include in a
    # distutils source distribution.
    createPycleneSourcesManifest()
  ## COMPILATION CONFIG : End ##

  ## ACTUAL BUILD : Begin ##
    setup(
        name='pyclene',
        version=pycleneVersion,
        package_dir={'pyclene': os.curdir},

        # The following Python modules are required to operate CLucene (the
        # remaining Python modules are included only in the source distribution):
        py_modules=['pyclene.__init__', 'pyclene.lucene', 'pyclene._cl_py'],

        ext_modules=[
          Extension('pyclene._cl_c',
            language='c++',
            sources=distutilsExtensionSources,
            include_dirs=includeDirs + [CLuceneSrcDir],
            library_dirs=libDirs,
            libraries=libs,
            extra_link_args=extraLinkArgs,
            extra_compile_args=extraCompilerArgs,
            define_macros=macroDefs
          ),
        ],
      )
  ## ACTUAL BUILD : End ##


if __name__ == '__main__':
    main()
