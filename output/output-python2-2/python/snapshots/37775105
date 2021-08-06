try:
    # lxml
    from lxml import etree
    from etree import XMLSyntaxError
    HAVE_LXML = True
except ImportError:
    HAVE_LXML = False
    try:
        # Python 2.5
        import xml.etree.cElementTree as etree
        from exceptions import SyntaxError as XMLSyntaxError
    except ImportError:
        try:
            # Python 2.5
            import xml.etree.ElementTree as etree
            import xml.parsers.expat.ExpatError as XMLSyntaxError
        except ImportError:
            etree = None

def check():
    """
    Check wether any implementation of ElementTree library is installed, or not
    
    @raise AssertionError: if ElementTree is not installed
    """
    
    assert etree is not None, "No implementation of ElementTree library installed"
    
    if not HAVE_LXML:
        print "WARNING: lxml library is not installed. Data format validation will not be used"
