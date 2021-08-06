"""
This script use the relax-ng file given to validate the xml file given.
"""

import lxml
from lxml import etree
import os,sys

def Validate(xml_file, rng_file):
    if os.path.exists(xml_file):
        if os.path.exists(rng_file):
            xml_doc=etree.parse(xml_file)
            rng_doc=etree.parse(rng_file)
            
            relaxng=etree.RelaxNG(rng_doc)
            #print xml_doc.relaxng(rng_doc)
            print relaxng.validate(xml_doc)
            if relaxng.validate(xml_doc):
                print "Validating SUCCESSFULL"
            else:
                log=relaxng.error_log
                raise Exception, log.last_error
                
            
        else:
            raise Exception,"FATAL : %s doesn't exixts"%xsl_file
    else:
        raise Exception, "FATAL : %s doesn't exixts"%file_in


if __name__=="__main__":
    #TODO : Parsing argv with getopt
    file_in=sys.argv[1]
    rng_file=sys.argv[2]
    
    print "Validating %s with rng file %s :)"%(file_in, rng_file)
    Validate(file_in, rng_file)
    