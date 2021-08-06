"""
This script use the xsl file geo2kml.xsl and transform the input file.
"""

import lxml
from lxml import etree
import os,sys

def Transform(file_in, xsl_file, file_out):
    if os.path.exists(file_in):
        if os.path.exists(xsl_file):
            xml_doc=etree.parse(file_in)
            xsl_doc=etree.parse(xsl_file)
            
            transform=etree.XSLT(xsl_doc)
            result=transform(xml_doc)
            
            
            f=open(file_out,"w")
            f.write(str(result))
            f.close()
        else:
            raise Exception, "%s doesn't exixts"%xsl_file
    else:
        raise Exception, "%s doesn't exixts"%file_in


if __name__=="__main__":
    #TODO : Parsing argv with getopt
    file_in=sys.argv[1]
    xsl_file=sys.argv[2]
    file_out=sys.argv[3]
    
    print "Transforming %s with xsl stylesheet %s :)"%(file_in, xsl_file)
    Transform(file_in, xsl_file, file_out)
    