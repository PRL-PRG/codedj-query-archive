"""
This use the xsl file geo2kml.xsl and transform the input file.
"""

import lxml
from lxml import etree
import os

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
            print "%s doesn't exixts"%xsl_file
    else:
        print "%s doesn't exixts"%file_in

    
    


if __name__=="__main__":
    file_in="../xml/examples/geoflights1.xml"
    xsl_file="../xsl/geo2kml.xsl"
    file_out="../kml/out_py.kml"
    
    print "Transforming %s with xsl stylesheet %s :)"%(file_in, xsl_file)
    Transform(file_in, xsl_file, file_out)
    