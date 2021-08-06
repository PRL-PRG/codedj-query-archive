# Released to the public domain by David Rushby, 2 September 2004.
# Last modified 2004/09/02 13:55.

import os, os.path, shutil, sys
import distutils.sysconfig


class BuildError(Exception): pass


def generateMinGWPythonLib(out=sys.stdout):
    """
      generateMinGWPythonLib uses the MinGW tools pexports and dlltool to
    create a GCC-compatible library file for Python
    (pythonVV.lib -> libpythonVV.a).
    """
    pythonHomeDir = sys.exec_prefix
    pyVersionSuffix = ''.join( [str(n) for n in sys.version_info[:2]] ) # '23', '24', ...
    pyDLLName = 'python%s.dll' % pyVersionSuffix
    pyLibName = 'python%s.lib' % pyVersionSuffix

    pcbuildDir = os.path.join(
        os.path.dirname(distutils.sysconfig.get_python_inc()), 'PCBuild'
      )

    winSysDir = _determineWindowsSystemDir(out=out)

    # Accommodate source dists of Python on Windows (give the
    # PCBuild\pythonVV.lib file (if any) precedence over the libs\pythonVV.lib
    # file (if any)):
    pyLibsDir = os.path.join(pythonHomeDir, 'PCbuild')
    if not os.path.exists(os.path.join(pyLibsDir, pyLibName)):
        pyLibsDir = os.path.join(pythonHomeDir, 'libs')

    # Find the Python DLL:
    pyDLLPathPossibilies = [os.path.join(d, pyDLLName) for d in
        (pythonHomeDir, pcbuildDir, winSysDir)
      ]
    for pyDLLPath in pyDLLPathPossibilies:
        if os.path.isfile(pyDLLPath):
            break
    else:
        raise BuildError("Can't find Python DLL '%s'." % pyDLLName)

    libName = 'libpython%s.a' % pyVersionSuffix
    libUltimateDest = os.path.join(pyLibsDir, libName)
    defFilename = 'python%s.def' % pyVersionSuffix
    if os.path.isfile(libUltimateDest):
        print >> out, ('MinGW-compatible Python library already exists at:\n  %s'
            % libUltimateDest
          )
    else:
        print >> out, (
            'Trying to create MinGW-compatible Python library at:\n  "%s"'
            % libUltimateDest
          )
        origWorkingDir = os.path.abspath(os.curdir)
        os.chdir(os.path.dirname(pyDLLPath))
        try:
            _execCommand('pexports %s > %s' % (pyDLLName, defFilename), out=out)
            _execCommand(
                'dlltool --dllname %s --def %s --output-lib %s'
                % (pyDLLName, defFilename, libName),
                out=out
              )
            os.remove(defFilename)
            # With source builds of some versions of Python, the Python DLL
            # is located in the same directory that distutils declares to
            # be the "library directory", so the generated library file
            # shouldn't be moved.
            if os.path.dirname(libUltimateDest).lower() != os.path.abspath(os.curdir).lower():
                shutil.copyfile(libName, libUltimateDest)
                os.remove(libName)
        finally:
            os.chdir(origWorkingDir)

    assert os.path.isfile(libUltimateDest)


# UTILITY FUNCTIONS:
def _execCommand(cmd, out=sys.stdout):
    print >> out, cmd
    taskOutStream = os.popen(cmd)
    taskOutput = taskOutStream.read()
    taskRetCode = taskOutStream.close()
    if taskRetCode is not None:
        raise BuildError('Command [%s] died with error:\n[%s]'
            % (cmd, taskOutput)
          )
    return taskOutput


def _determineWindowsSystemDir(out=sys.stdout):
    if sys.platform == 'cygwin':
        return _execCommand('cygpath --sysdir', out=out)[:-1] # Trailing newline.
    else:
        # (normal win32)
        # If I were willing to introduce a win32all dependency into this build
        # script, this function would be replaced by win32api.GetSystemDirectory.
        winDir = os.environ.get('SYSTEMROOT', os.environ.get('WINDIR', 'C:\\Windows'))
        winSysDir = os.path.join(winDir, 'system32')
        if not os.path.isdir(winSysDir):
            winSysDir = os.path.join(winDir, 'system')
        return winSysDir


if __name__ == '__main__':
    generateMinGWPythonLib()