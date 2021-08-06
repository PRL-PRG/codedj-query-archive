import numpy


class Orbit(object):
    """
    orbitId
    q
    e
    i
    node
    argPeri
    timePeri
    epoch
    hv
    g
    src
	"""
    def __init__(self, orbitId, q, e, i, node, argPeri, timePeri, epoch, hv, g,
                 src=None):
        """
        orbitId: integer
        q (AU)
        e
        i (deg)
        node (deg)
        argPeri (deg)
        timePeri (UTC MJD)
        epoch: orbit epoch (UTC MJD)
        hv: absolute magnitude (V mag)
        g: slope parameter
        src: 21 element array (covariance matrix in diagonal form).
        """
        self.orbitId = orbitId
        self.q = q
        self.e = e
        self.i = i
        self.node = node
        self.argPeri = argPeri
        self.timePeri = timePeri
        self.epoch = epoch
        self.hv = hv
        self.g = g
        self.src = self.setSrc(src)
        return


    def setSrc(self, src):
        """
        If all elements of the covariance list are not None, then cast that
        list into a numpy.array. Return the casted array or None in case the
        covariance is invalid (i.e. has null elements).
        """
        if(None in src):
            return(None)
        return(numpy.array([float(e) for e in src]))





