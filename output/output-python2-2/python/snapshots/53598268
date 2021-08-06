"""
ephem
F. Pierfederici (fpierfed@lsst.org)

Ephemerides generation module. It wraps custon fortran code developed using
OrbFit libraries.
"""
import _ephem
_ephem.initialize()


__all__ = ['ephemerides', ]


def ephemerides(elementsType,
                orbitElements,
                orbitEpoch,
                absMag,
                times,
                obsCode,
                covariance=None,
                normal=None):
    """ephemerides(elementsType, orbitElements, orbitEpoch, absMag, times,
                   obsCode, covariance, normal)

    Given one orbit (described by its orbital elements), a list of times, and a
    position on Earth (specified by an observatory code), compute the predictied
    position (i.e. ephemerides) of that orbit at each of the input times. Return
    positions (i.e. RA, DEC), time t, magnitude mag and positional error ellipse
    (err1, err2, PA).
    
    INPUT (all angles in degrees)
      elementsType: Orbital elements type (i.e. 'CAR', 'EQU', 'KEP', 'COM',
        'ATT').
      orbitElements: numpy.array containing the orbital elements
        (dimension <= 6). All angles in degrees.
      orbitEpoch: epoch of the orbit (UTC MJD).
      absMag: absolute H magnitude. If not known, use G mag or 0.0.
      obsCode: MPC observatory code.
      times: list of ephemerides times (UTC MJD).
      covariance: optional covariance matrix in diagonal form as 21-element
        numpy.array.
      normal: optional normal matrix in diagonal form as 21-element numpy.array.
    
    OUTPUT
      numpy.array of numpy.arrays of the form [RA, Dec, mag, t, err1, err2, PA].
      RA: Right Ascension (deg).
      Dec: Declination (deg).
      t: ephemerides time (UTC MJD).
      mag: apparent magnitude.
      err1: semi-major axis of the positional error ellipse (deg).
      err1: semi-major axis of the positional error ellipse (deg).
      PA: position angle of the positional error ellipse (deg).
    """
    if(covariance != None and normal != None):
        return(_ephem.ephemerides(typ=elementsType,
                                  ndim=len(orbitElements),
                                  elements=orbitElements,
                                  t=orbitEpoch,
                                  h_mag=absMag,
                                  g_mag=0.,
                                  obscode=obsCode,
                                  times=times,
                                  covar=covariance,
                                  norm=normal))
    elif(covariance != None):
        return(_ephem.ephemerides(typ=elementsType,
                                  ndim=len(orbitElements),
                                  elements=orbitElements,
                                  t=orbitEpoch,
                                  h_mag=absMag,
                                  g_mag=0.,
                                  obscode=obsCode,
                                  times=times,
                                  covar=covariance))
    elif(normal != None):
        return(_ephem.ephemerides(typ=elementsType,
                                  ndim=len(orbitElements),
                                  elements=orbitElements,
                                  t=orbitEpoch,
                                  h_mag=absMag,
                                  g_mag=0.,
                                  obscode=obsCode,
                                  times=times,
                                  norm=normal))
    # else:
    return(_ephem.ephemerides(typ=elementsType,
                              ndim=len(orbitElements),
                              elements=orbitElements,
                              t=orbitEpoch,
                              h_mag=absMag,
                              g_mag=0.,
                              obscode=obsCode,
                              times=times))
