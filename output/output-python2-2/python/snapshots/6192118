# tigerutils.py

# NOTES
# * Add query scopes (county, city or ZIP code) to config files!
# * What else?
# * Where does this belong?
#
#The data dictionary used to generate these schemas is here
#[http://www.census.gov/geo/www/tiger/tiger2006se/a6sech6.txt]'''
#

# TODO (FEATURES)
# * Right now (Sunday 1/7/08) the only thing that I can think of that would be
#   nice to have is a directory walker that picks up all the SQL files and
#   imports them into the database.  It would have to be interactive, for two
#   reasons: need credentials, and the user may not want to import all of
#   them.  Also, need to decide whether to drop the existing tables first.
#
#   Actually, the way the SQL code is currently written, that plan is not
#   going to work.  MakeRtSqlFiles::generateSqlFiles() says
#
#        DROP TABLE IF EXISTS `tiger_01`;
#          and
#        DROP TABLE IF EXISTS `tiger_02`;
#
#   So the newest table always wins.  It's destructive, and probably not the
#   best way to go.  I'll have to think about that.

import ConfigParser
import networkx
import networkx.component
import networkx.path
import os, os.path
import pylab
import random, re
import shutil, sys
import sqlalchemy # can I delete this?
import urllib

from decimal import *
from sqlalchemy import Column, Integer # ForeignKey,
from sqlalchemy import MetaData, Numeric, String, Table
from sqlalchemy.orm import sessionmaker, mapper
from sqlalchemy.sql import select


# crude but necessary
min_sqlver='0.4.0'
if sqlalchemy.__version__ < min_sqlver:
    print "SQLAlchemy v%s or later required." % min_sqlver
    sys.exit(0)

# Locate and read the config files
config=ConfigParser.SafeConfigParser()
config.read(os.path.join('agents','conf','graphs','defaults.ini'))
config.read(os.path.join('agents','conf','graphs','overrides.ini'))

# dataprep config values
#
# Late note: I could move these into G().  It wouldn't be consistent with
# agents, but it's something to consider.
FETCH_COMMAND=os.path.join('agents','bin',config.get('dataprep','fetchCommand'))
ZIP_COMMAND=os.path.join('agents','bin',config.get('dataprep','zipCommand'))
FIPS_METADATA_URL=config.get('dataprep','fipsMetadataUrl')
FIPS_ZIPFILE_ROOT=config.get('dataprep','fipsZipfileRoot')
EXCLUDE_PUERTO_RICO=config.getboolean('dataprep','excludePuertoRico')
TIGER_SANDBOX=config.get('dataprep','tigerSandbox')

# dev config values
DEBUG=config.getboolean('dev','debug')

if TIGER_SANDBOX=='None': # it's a config string, not the None value 
    print "Please specify where to store generated files by setting"
    print "tigerSandbox in graphs\\overrides.ini"
    sys.exit(0)

DATA_DIR=os.path.join(TIGER_SANDBOX,'data')
IMAGES_DIR=os.path.join(TIGER_SANDBOX,'images')


class G(object):
    '''A class that exists only to hold global variables.

It holds only class variables.  All other classes should add their variables
as they become available.'''
    statesPerRow=6
    countiesPerRow=2


class FipsMetadataParser(object):
    '''TODO'''
    def __init__(self):
        print "\n====[ FipsMetadataParser ]===="

        # FIPS dictionary: Key is state code, value is a list holding the
        # the state abbreviation, and a list of two-tuples of county code and
        # county name.
        # >>> fd
        # {'46': ['SD', [('003', 'Aurora'), ('005', 'Beadle')]]}
        self.FIPS_D={}

        # FIPS codes list.  First use is for random selection.
        self.FIPS_L=[]

        # Regular expressions.
        self.spaces=" {5}?"
        self.state_code="^\d{2}?"
        self.state_name="[A-Z][A-Z]$"
        self.county_code="\d{3}?"

	# county_name is hard to match since it is the most variable.  Instead
	# of getting all complicated, I'm going to make a couple simplifying
	# assumptions.  First, the county name must be surrounded by 5 spaces
	# on either side that I'll strip off before storing or printing.
	# Second, all these characters are legal: (alpha), (hyphen), (dot),
	# (apostrophe), (whitespace).
        #
        # This one works fine everywhere except for Puerto Rico.
        #county_name = "[A-Z][A-Za-z\-\.\'\s]*?"
        #
        # This one is looser, and seems to work okay everywhere.  I'll
        # probably pay for this later, when it's time to load up the database.
        self.county_name="[A-Z].*?"

	# Compile the regexes to match lines to include in the state/county
	# list.  They are ordered here to match the order they appear in the
	# FIPS file. 
        self.p1=re.compile(self.state_code)
        self.p2=re.compile(self.county_code)
        self.p3=re.compile(self.spaces + self.county_name + self.spaces)
        self.p4=re.compile(self.state_name)
        #p5 = re.compile(spaces)

    # I should think about using this ...
    def greet(self):
        print "This is the FIPS county data download tool."
        print
        print "This utility will help you choose a county on which to run"
        print "the simulation.  It will then download the data from the U.S."
        print "Census bureau, and prepare it for import into a SQL database."
        print "Then you can run NAME_HERE, a database creation and import"
        print "tool to finish the data preparation steps."
        print
        print "Now we need to choose a state then county from a list, make a"
        print "random selection (good for demomode), or quit."
        print

    def fetch(self,FIPS_METADATA_URL=None):
        '''TODO'''
        # Pass in a URL for the real deal, or use the file for debugging.
        if FIPS_METADATA_URL:
            print 'Downloading and processing %s' % FIPS_METADATA_URL
            self.fips=urllib.urlopen(FIPS_METADATA_URL)
        else:
            localFipsFile='AL_AK_AZ_FIPS.txt'
            print 'Processing local file %s' % localFipsFile
            self.fips=file(localFipsFile)

    def clean(self):
        '''TODO'''
        # Scan the contents of the file coming over the network, save it to
        # memory, and strip out all non-data content.  It's doing more work
        # than absolutely necessary, but it's safe and simple.
        self.eligible=[]
        p_eligible=re.compile(self.state_code + self.spaces)

	# There are several graves, accents, and other things in county names
	# in Puerto Rico that may cause problems when it comes time to parse
	# the strings for import into the database.  Set EXCLUDE_PUERTO_RICO
	# to True in overrides.ini to strip it out from consideration when
	# generating the set of states and counties for either random sets or
	# for populating the chooser.
        if EXCLUDE_PUERTO_RICO is True:
            PuertoRicoRE=re.compile("PR$")
            for line in self.fips:
                if PuertoRicoRE.search(line):
                    pass
                elif p_eligible.match(line):
                    self.eligible.append(line)
        else:
            for line in self.fips:
                if p_eligible.match(line):
                    self.eligible.append(line)

    def parseAll(self):
        '''TODO'''
        for line in self.eligible:
            # find state code, county code, county name, state abbr.
            iterator1,iterator2,iterator3,iterator4= \
                self.p1.finditer(line),self.p2.finditer(line), \
                self.p3.finditer(line),self.p4.finditer(line)

            # These for loops extract the MatchObject instances from the
            # iterators.  They depend on the format of the data file.
            #
            for SC in iterator1:
                pass
            for CC in iterator2:
                pass
            for CN in iterator3:
                pass
            for SN in iterator4:
                pass

            # Add all lines to the FIPS_D dictionary.  (It's not much, but it
            # took me a while to figure this out!)
            self.next_state=SC.group()
            if not self.FIPS_D.get(self.next_state):
                self.FIPS_D[self.next_state]=([SN.group(),[(CC.group(), 
                        CN.group().strip())]])
            else:
                self.FIPS_D[self.next_state][1].append((CC.group(),
                        CN.group().strip()))
#        What to do here?  Should I return this to the caller, or keep it for
#        private use only?
#        return self.FIPS_D

    def parseCodes(self):
        '''Parse and return the state and county codes (concatenated).'''
        for line in self.eligible:
            # find state code and county code
            iterator1, iterator2= \
                self.p1.finditer(line), self.p2.finditer(line)
            for SC in iterator1:
                pass
            for CC in iterator2:
                pass
            self.FIPS_L.append(SC.group()+CC.group())
#        What to do here?  Should I return this to the caller, or keep it for
#        private use only?
#        return self.FIPS_L

    # This returns a list of tuples.  Think about how to make this easier to
    # work with.  It's at least consistent with getCounties.
    def getStates(self):
        '''Return the list of states and territories in FIPS_METADATA_URL.'''
        states=[]
        for key, value in self.FIPS_D.items():
            states.append((key, value[0]))
        return states

    def getStateAbbr(self, stateCode):
        '''Returns the state abbreviation (SA) or None.'''
        return self.FIPS_D.get(stateCode)

    # Currently unused?
    def getStateCode(self, stateAbbr):
        '''Returns the state code (SC) or None.'''
        for key, val in self.FIPS_D.iteritems():
            if stateAbbr==val[0]:
                return key
        return None

    def isState(self,stateCode):
        '''Returns True if stateCode is a valid FIPS state code.'''
        return self.FIPS_D.get(stateCode) is not None

    # This returns a list of tuples.  Think about how to make this easier to
    # work with.  It's at least consistent with getStates.
    #
    # Currently unused?
    def getCounties(self, stateAbbr):
        '''Return the counties in the given two-letter state abbreviation or None.'''
        try:
            return self.FIPS_D.get(stateAbbr)[1]
        except TypeError:  # bogus state
            return None

    # NOTE: getCountyName is a convenience method, and not required.  Also, I
    # don't see a need for a getCountyCode() method.  That's provided by the
    # user.
    #
    # Late note: I want this for random county selection.  I want a
    # getCountyAndStateFromCode() method that returns a ("county", "state")
    # two-tuple.  The first step is to get the county name from the code.
    #
    # Late note 2: Do this later.  For now, just get the random zip file.
    def getCountyName(self,countyCode):
        '''Returns the county name (CN) or None.'''
        pass

    def isCounty(self,stateCode,countyCode):
        '''Returns True if stateCode+countyCode is a valid FIPS county code.'''
        return stateCode+countyCode in self.FIPS_L

    def getRandomCounty(self):
        try:
            return random.choice(self.FIPS_L)
        except IndexError:
            print "Nothing to choose from.  Have you run parseCodes?"

    def showAll(self):
        '''Show the contents of the FIPS data file in a batch.'''
        for key, value in self.FIPS_D.items():
            print "(%s, %s)" % (key, value[0])
            for tup in value[1]:
                print "    ", tup

    # This method breaks the rules slightly, by printing to STDOUT.  I tend to
    # avoid that whenever possible, but in this case, it's purely read-only.
    #
    # Currently unused.
    def showStates(self):
        states=self.FIPS_D.keys()
        count=0
        for state in states:
	    if count==G.statesPerRow:
                print
                count=0
            print "  %s" % (state),
            count+=1

    # Like showStates, this method breaks the rules slightly, by printing to
    # STDOUT.
    def showCounties(self,stateCode):
        try:
            counties=self.FIPS_D.get(stateCode)[1]
            count=0
            for county in counties:
		if count==G.countiesPerRow:
                    print
                    count=0
                out="%s %s" % (county[0], county[1])
                print repr(out).ljust(32),
                count+=1
        except TypeError:  # bogus state
            print "%s is an invalid state abbreviation." % (stateAbbr)
            print "(How did you get here?)"


class UserInput(object):
    def __init__(self):
        print "\n====[ UserInput ]===="

    # TODO Maybe allow the user to pass in the range or exact values that are
    # accepted by the caller, and validate the input before returning it?
    def getDigit(self,min=None,max=None,msg=None):
        '''Helper method for collecting user input.'''
        while True:
            userIn=raw_input(msg)
            try:
                int(userIn)
            except ValueError:
                print "Please enter a digit"
                continue

            if not min<=len(userIn)<=max:
                if min==max:
                    print "Enter a %s-digit number" % (min)
                else:
                    print "Please enter a %s- to %s-digit number" % \
                            (min,max)
                continue
            break
        return userIn

    def getDbEngine(self):
        '''Query the user for which database engine to use'''
        engineCode=None
	dbEngineNames=[(1,'SQLite'),(2,'MySQL')]

        print "Enter the database engine: "
	while True:
	    engineCode=self.getDigit(1,1,"(1) SQLite (2) MySQL (3) quit: ")
            if engineCode=='1' or engineCode=='2':
                G.dbEngineName=dbEngineNames[int(engineCode)-1][1]
		if DEBUG:
		    print '[DEBUG] G.dbEngineName: %s' % G.dbEngineName
		break
            elif engineCode=='3':
                print "Exiting."
                sys.exit(0)
            else:
                continue # this is redundant but explicit

	# If using SQLite, need a path to the database file.  If using MySQL,
	# need username and password.
        if G.dbEngineName=='MySQL':
            self.__MySQLCreds()
	    self.__MySQLUri()
        elif G.dbEngineName=='SQLite':
            self.__SQLiteUri()
        else:
            print "Something's broken in UserInput.getDbEngine()!"
	    sys.exit(0)

    def __MySQLCreds(self):
            # TODO Be sneaky and use some cloaking method for password (later)
            G.dbUsername=raw_input('Enter the username: ')
	    G.dbPassword=raw_input('Enter the password: ')
	    G.dbHostname=raw_input('Enter the hostname (or localhost): ')
	    # TODO make it clear that this is the name of the database (ie:
	    # agents), not the name of the engine (ie: MySQL).
	    G.dbName=raw_input('Enter the database name: ')

    def __MySQLUri(self):
	    # Assemble the uri for SQLAlchemy.  Looks like
	    #mysql://username:password@host/database_name
	    G.dbUri='mysql://%s:%s@%s/%s' % (G.dbUsername,
			    G.dbPassword,
			    G.dbHostname,
			    G.dbName)
            if DEBUG:
                print '[DEBUG] G.dbUri: %s' % G.dbUri

    def __SQLiteUri(self):
            G.dbName='TGR'+G.stateCountyCode+'.db'	
	    # Assemble the uri for SQLAlchemy.  Looks like
	    # sqlite:///tgr53033.db.
	    G.sqlPath=DATA_DIR+'/'+G.stateAbbr+'/TGR'+ \
			    G.stateCountyCode+'/sql/'
            if DEBUG:
                print 'G.sqlPath: %s'%G.sqlPath
	    if not os.path.exists(G.sqlPath):
	        os.mkdir(G.sqlPath)
            G.dbUri=G.dbEngineName.lower()+':///'+G.sqlPath+G.dbName
            if DEBUG:
                print '[DEBUG] G.dbUri: %s' % G.dbUri


class GetFips(object):
    '''Help the user choose a county FIPS file.

Make it as simple as possible to choose a county FIPS zip file for
use in the graphs software.  Prompt the user for a state and
county by name, fetch the file and unzip it.  Then clean up extra
files that are not used by the graphs programs.'''

    def __init__(self):
        print "\n====[ GetFips ]===="
        # Maybe we shouldn't go to all this trouble up front.  The user may
        # want to exit without running the program.  But doing it here is
        # simpler.
        self.fp=FipsMetadataParser()
        self.fp.fetch(FIPS_METADATA_URL)
        self.fp.clean()
        self.fp.parseCodes()
        self.fp.parseAll()
        self.states=self.fp.getStates()
        self.states.sort()
	self.u=UserInput()

    def getSelection(self):
        # get the user's initial choice
        while True:
            print 'See the list of states, make a random selection, or quit?'
            userIn=self.u.getDigit(1,1,"(1) list (2) random (3) quit: ")
            if userIn=='1':
                print "Here are the states and territories you can choose from:"
                # Print the list of states, G.statesPerRow to a row
                states=self.fp.getStates()
                states.sort()
                count=0
                for abbr,code in states:
		    if count==G.statesPerRow:
                        print
                        count=0
                    print "    %s %s" % (code,abbr),
                    count+=1
                print

                # have the user choose a state by code, then display the
                # counties
                while True:
                    G.stateCode=self.u.getDigit(
				    min=2,max=2,msg="Enter the state code: ")
                    if self.fp.isState(G.stateCode):
                        G.stateAbbr=self.fp.getStateAbbr(
					G.stateCode)[0].upper()
                        print
			"Here are the counties in %s you can choose from:" % \
                                (G.stateAbbr)
                        self.fp.showCounties(G.stateCode)
                        break

                # choose a county by code
                while True:
                   print
		   # TODO am I even using this?  the syntax is outdated
                   G.countyCode=self.u.getDigit(
				   min=3,max=3,msg="Enter the county code: ")
                   if self.fp.isCounty(G.stateCode,G.countyCode):
                       # we've got a valid FIPS code, time to fetch the ZIP
                       # file
                       break

            elif userIn=='2':
                randomCounty=self.fp.getRandomCounty()
                G.stateCode=randomCounty[:2]
                G.stateAbbr=self.fp.getStateAbbr(G.stateCode)[0].upper()
                G.countyCode=randomCounty[2:]
                print "randomCounty: %s" % (randomCounty)
                break

            elif userIn=='3':
                print "Exiting."
                sys.exit(0)

            else:
                continue # this is redundant but explicit

            # Break out of the outermost while loop.  We don't get here unless
            # a state and county are chosen explicitly.
            break

    def getFipsZipFile(self):
        while True:
            confirm=self.u.getDigit(min=1,max=1,
                            msg="(1) download ZIP file (2) quit: ")
            if confirm=='1':
                # Some assembly required
                #
		# Note: zipFileUrl depends on FIPS_ZIPFILE_ROOT having a
		# trailing slash in the config file like it's supposed to.
		# I'll add a check later.
                #
#                if not FIPS_ZIPFILE_ROOT.endswith('/'):
#                    FIPS_ZIPFILE_ROOT+='/'
                G.stateCountyCode=G.stateCode+G.countyCode
                zipFileName='TGR%s.ZIP' % (G.stateCountyCode)
                zipFileUrl=FIPS_ZIPFILE_ROOT+G.stateAbbr+r'/'+zipFileName
                G.srcPath=os.path.normpath(os.path.join(DATA_DIR,
                        G.stateAbbr,'TGR'+G.stateCountyCode,'src'))
		if DEBUG:
		    print '[DEBUG] G.srcPath: %s' % G.srcPath
                print "Downloading %s from %s" % (zipFileName,
                     FIPS_ZIPFILE_ROOT+G.stateAbbr)

                # Prepare for the little bundle of joy (county zip file)
                if not os.path.exists(G.srcPath):
                    print "Creating %s" % G.srcPath
                    os.makedirs(G.srcPath)
                else:
                    print "Found directory %s" % G.srcPath

                zipFilePath=G.srcPath+r'/'+zipFileName
		if os.path.exists(zipFilePath):
                   print "Found %s.  Skipping download." % zipFilePath
                else:
                    # C:\Source\hg\graphs\bin>wget -O C:\Source\TIGER\sandbox\src
                    #    http://www2.census.gov/geo/tiger/tiger2006se/WA/TGR53033.ZIP
		    cmd="%s --output-document=%s %s" % \
                                (FETCH_COMMAND,zipFilePath,zipFileUrl)
                    status=os.system(cmd)
                    if DEBUG:
                        print '[DEBUG] cmd: %s' % cmd
                        print "Status: ", status
                break

            elif confirm=='2':
                print "Exiting."
                sys.exit(0)


class ProcessFipsFiles(object):
    '''Prepare FIPS files for processing by the mungers.

Find and unzip all TGRxxxxx.ZIP files in-place to a temporary
location (G.srcPath).  Copy the RT1 and RT2 files to a staging
area to make them available for later.  Remove the files in the
temporary location.'''

    def __init__(self):
        print "\n====[ ProcessFipsFiles ]===="

        # should look like G.srcPath
        G.rawPath=os.path.normpath(os.path.join(DATA_DIR,G.stateAbbr,
                'TGR'+G.stateCountyCode,'raw'))
        if not os.path.exists(G.rawPath):
            print "Creating %s" % G.rawPath
            os.makedirs(G.rawPath)
        else:
            print "Found %s" % G.rawPath

    def unzip(self):
        '''Unzip the chosen FIPS (TIGER) ZIP file.

The files are extracted to the same directory (from G.srcPath to
G.srcPath).'''
	# There should be only one ZIP file in G.srcPath for a particular
	# county.  One county, one zip file.
	for candidate_file in os.listdir(G.srcPath):
            # Note 1: no space between -o and the output directory!
	    # Note 2: -y is assume Yes on all queries.  This means previously
	    #     unzipped files will be overwritten.  That's fine.
#	    if zipfile.is_zipfile(candidate_file):
            if candidate_file.upper().endswith('ZIP'):
                cmd=ZIP_COMMAND+' '+os.path.join(G.srcPath,candidate_file)+ \
				' -o'+G.srcPath+' -y'
                status=os.system(cmd)
                if DEBUG:
	            print '[DEBUG] cmd: %s' % cmd
                    print "Status: ", status
            else:
		if DEBUG:
                    print '[DEBUG] Skipping %s (not a ZIP file)' % \
				    candidate_file

    def export(self):
        '''Copy RT1 and RT2 files from G.srcPath to G.rawPath.'''
        for rtfile in os.listdir(G.srcPath):
            tmp=rtfile.upper()
	    if DEBUG:
	        print '[DEBUG] checking %s for RT1 or RT2 extension' % tmp
            if tmp.endswith('RT1') or tmp.endswith('RT2'):
                cmd=shutil.copy2(os.path.join(G.srcPath,tmp),G.rawPath)
		if DEBUG:
		    print '[DEBUG] cmd: %s' % cmd
            else:
                if DEBUG:
                    print '[DEBUG] Skipping %s' % rtfile

    def cleanup(self):
        '''Remove all extracted files from G.srcPath.

Do not delete the ZIP file.'''
        for rtfile in os.listdir(G.srcPath):
            tmp=rtfile.upper()
	    if tmp.endswith('ZIP'):
	        if DEBUG:
                    print '[DEBUG] Skipping file %s' % tmp
            else:
                if DEBUG:
	            print '[DEBUG] Deleting file %s' % tmp
                os.unlink(os.path.join(G.srcPath,tmp))


class RunMungers(object):
    '''Process the raw data in the RT1 and RT2 files.

This class exposes a single method, process() that transforms the
record type data and generates recordsets suitable for import into
a SQL database.'''
    def __init__(self):
        print "\n====[ RunMungers ]===="

        # should look like G.srcPath
	G.mungedPath=os.path.normpath(os.path.join(DATA_DIR,G.stateAbbr,
		'TGR'+G.stateCountyCode,'munged'))

        if not os.path.exists(G.mungedPath):
            print "Creating %s" % G.mungedPath
            os.makedirs(G.mungedPath)
        else:
            print "Found %s" % G.mungedPath

    def process(self):
        '''Generate a SQL recordset from TIGER RT files.

This loop processes all the RT1 and RT2 files, and generates a SQL
(SQLAlchemy?) recordset from them.  If other files are present in
the source directory, they are skipped with a message to STDOUT.
'''
	for rtfile in os.listdir(G.rawPath):
            # TODO move this print statement into MungeRTx()
            #print "  Reading file %s" % rtfile
            before=os.path.join(G.rawPath,rtfile)
            after=os.path.join(G.mungedPath,rtfile)+'m'
            if rtfile.endswith('RT1'):
                #m=mungeRT1.MungeRT1()
                #m=MungeRT1()
                #m=MungeRT1(before,after)
                m=MungeRT1(before)
            elif rtfile.endswith('RT2'):
                #m=mungeRT2.MungeRT2()
                #m=MungeRT2()
                #m=MungeRT2(before,after)
                m=MungeRT2(before)
            else:
                print "* Skipping unknown file %s" % rtfile
                continue
            # a list
            recordset=m.munge(before)
            outfile=open(after,'w')
            #print "Writing recordset to %s" % (after)
            print "Writing munged data to %s" % (after)
            for record in recordset:
#            print record
                outfile.write(record+'\n')



class CreateDatabase(object):
    '''Generate the schemas and create the TIGER database.

This class has no methods.  Everything happens in __init__.'''
    def __init__(self):
        print "\n====[ CreateDatabase ]===="
        G.engine=sqlalchemy.create_engine(G.dbUri,echo=False)
	meta=MetaData()

        # describe table 'tiger01_table', query the database for its columns
        G.tiger01_Table=Table('tiger_01',meta,
        		Column('id',Integer,nullable=False,primary_key=True),
        		Column('rt',String(2),nullable=False,default=0),
        		Column('version',Integer,nullable=False,default=0),
        		Column('tlid',Integer,nullable=False,default=0),
        		Column('fedirp',String(3),nullable=True),
                        Column('fename',String(31),nullable=True),
        		Column('fetype',String(5),nullable=True),
        		Column('fedirs',String(3),nullable=True),
        		Column('fraddl',String(12),nullable=True),
        		Column('toaddl',String(12),nullable=True),
        		Column('fraddr',String(12),nullable=True),
        		Column('toaddr',String(12),nullable=True),
        		Column('zipl',Integer,nullable=True),
        		Column('zipr',Integer,nullable=True),
#        		Column('frlong',String(10,6),nullable=False,default=0),
#        		Column('frlat',String(10,6),nullable=False,default=0),
#        		Column('tolong',String(10,6),nullable=False,default=0),
#        		Column('tolat',String(10,6),nullable=False,default=0)
        		Column('frlong',String(10),nullable=False,default=0),
        		Column('frlat',String(10),nullable=False,default=0),
        		Column('tolong',String(10),nullable=False,default=0),
        		Column('tolat',String(10),nullable=False,default=0)
        		)
#        		Column('frlong',Numeric(10,6),nullable=False,default=0),
#        		Column('frlat',Numeric(10,6),nullable=False,default=0),
#        		Column('tolong',Numeric(10,6),nullable=False,default=0),
#        		Column('tolat',Numeric(10,6),nullable=False,default=0)

	# drop existing Table (if present) before creating and loading
	try:
	    G.tiger01_Table.drop(bind=G.engine,checkfirst=True)
	except sqlalchemy.exceptions.OperationalError:
            # no database, pass
            pass

	                # TODO foreign_key=tiger01_Table._id?
        		#Column('_rtid',Integer,nullable=False,primary_key=True),

#        tiger02_Table=Table('tiger_02',meta,
#        		Column('_rtid',Integer,nullable=False,primary_key=True),
#			Column('version',Integer,nullable=False,default=0),
#			Column('tlid',Integer,nullable=False,default=0),
#			Column('rtsq',Integer,nullable=False,default=0),
#			Column('long1',Numeric(11,6),nullable=False,default=0),
#			Column('lat1',Numeric(10,6),nullable=False,default=0),
#			Column('long2',Numeric(11,6),nullable=False,default=0),
#			Column('lat2',Numeric(10,6),nullable=False,default=0),
#			Column('long3',Numeric(11,6),nullable=False,default=0),
#			Column('lat3',Numeric(10,6),nullable=False,default=0),
#			Column('long4',Numeric(11,6),nullable=False,default=0),
#			Column('lat4',Numeric(10,6),nullable=False,default=0),
#			Column('long5',Numeric(11,6),nullable=False,default=0),
#			Column('lat5',Numeric(10,6),nullable=False,default=0),
#			Column('long6',Numeric(11,6),nullable=False,default=0),
#			Column('lat6',Numeric(10,6),nullable=False,default=0),
#			Column('long7',Numeric(11,6),nullable=False,default=0),
#			Column('lat7',Numeric(10,6),nullable=False,default=0),
#			Column('long8',Numeric(11,6),nullable=False,default=0),
#			Column('lat8',Numeric(10,6),nullable=False,default=0),
#			Column('long9',Numeric(11,6),nullable=False,default=0),
#			Column('lat9',Numeric(10,6),nullable=False,default=0),
#			Column('long10',Numeric(11,6),nullable=False,default=0),
#			Column('lat10',Numeric(10,6),nullable=False,default=0)
#			)

	# drop existing Table (if present) before creating and loading
#	tiger02_Table.drop(bind=engine,checkfirst=True)

        # issue CREATE statements for all tables
        meta.create_all(G.engine)

	# class RecordType1 goes here (if you're following the docs)

        mapper(RecordType1, G.tiger01_Table)
#        mapper(RecordType2, tiger02_Table)


class LoadDatabase(object):
    def __init__(self):
        print "\n====[ LoadDatabase ]===="

	# Create a Session object.  In SQLAlchemy terms, this is the ORM's
	# "handle" to the database.
        G.Session=sessionmaker(bind=G.engine,autoflush=True,transactional=True)

        for rtfile in os.listdir(G.mungedPath):
            if DEBUG:
	        print '[DEBUG] rtfile: %s' % rtfile
            rtfilePath=os.path.join(G.mungedPath,rtfile)
	    print 'Reading munged data from %s' % rtfilePath

            if rtfile.endswith('RT1m'):
		session=G.Session()
		recordTypesInRecordType1=[ 'rt','version','tlid','fedirp',
				'fename','fetype','fedirs','fraddl','toaddl',
				'fraddr','toaddr','zipl','zipr',
				'frlong','frlat','tolong','tolat' ]

		f=file(rtfilePath)
		for line in f:
                    # make a dictionary of parameter names to parameter values
                    # suitable for passing into RecordType1()
                    #
		    # remove '\n' from last field in each row with strip()
                    d=dict(zip(recordTypesInRecordType1,(line.strip().split(','))))

		    # convert record items with value 'NULL' to Python None
		    for k,v in d.items():
			# I put double quotes around several of the strings to
			# prevent them from being parsed as multiple tokens by
			# the munger.  Now it needs to go away.
                        d[k]=v.strip('"')
                        if 'NULL'==v:
                            d[k]=None
		    rt1obj=RecordType1(rt=d['rt'],version=d['version'],
				    tlid=d['tlid'],fedirp=d['fedirp'],
				    fename=d['fename'],fetype=d['fetype'],
				    fedirs=d['fetype'],fraddl=d['fraddl'],
				    toaddl=d['toaddl'],fraddr=d['fraddr'],
				    toaddr=d['toaddr'],zipl=d['zipl'],
				    zipr=d['zipr'],frlong=d['frlong'],
				    frlat=d['frlat'],tolong=d['tolong'],
				    tolat=d['tolat'])

		    session.save(rt1obj)
                session.commit()

            elif rtfile.endswith('RT2m'):
                # load up RecordType2 objects
		pass
            else:
                print "Something's broken in LoadDatabase.__init__()!"


class RecordType1(object):
    def __init__(self,rt=1,version=0,tlid=0,fedirp=None,fename=None,
		    fetype=None,fedirs=None,fraddl=None,toaddl=None,
		    fraddr=None,toaddr=None,zipl=None,zipr=None,
		    frlong=0,frlat=0,tolong=0,tolat=0):

	    #print "\n====[ RecordType1 ]===="
	    self.rt=rt
	    self.version=version
	    self.tlid=tlid
	    self.fedirp=fedirp
	    self.fename=fename
	    self.fetype=fetype
	    self.fedirs=fedirs
	    self.fraddl=fraddl
	    self.toaddl=toaddl

	    self.fraddr=fraddr
	    self.toaddr=toaddr
	    self.zipl=zipl
	    self.zipr=zipr
	    self.frlong=frlong
	    self.frlat=frlat
	    self.tolong=tolong
	    self.tolat=tolat

    def __repr__(self):
        return "<RecordType1('%s', '%s', '%s', '%s', '%s','%s', '%s', '%s', '%s', '%s','%s', '%s', '%s', '%s', '%s','%s','%s')>" % \
			(self.rt,self.version,self.tlid,self.fedirp,
					self.fename,self.fetype,self.fedirs,
					self.fraddl,self.toaddl,self.fraddr,
					self.toaddr,self.zipl,self.zipr,
					self.frlong,self.frlat,self.tolong,self.tolat)


class RecordType2(object):
    def __init__(self,rtid,version,tlid,rtsq,long1,lat1,long2,lat2,long3,lat3,
		    long4,lat4,long5,lat5,long6,lat6,long7,lat7,long8,lat8,
		    long9,lat9,long10,lat10):

	    #print "\n====[ RecordType2 ]===="
	    self.rtid=rtid
            self.version=version
            self.tlid=tlid
            self.rtsq=rtsq
            self.long1=long1
            self.lat1=lat1
            self.long2=long2
            self.lat2=lat2

            self.long3=long3
            self.lat3=lat3
            self.long4=long4
            self.lat4=lat4
            self.long5=long5
            self.lat5=lat5
            self.long6=long6
            self.lat6=lat6

            self.long7=long7
            self.lat7=lat7
            self.long8=long8
            self.lat8=lat8
            self.long9=long9
            self.lat9=lat9
            self.long10=long10
            self.lat10=lat10

    def __repr__(self):
        return "<RecordType2('%s','%s','%s','%s','%s','%s','%s','%s','%s','%s','%s','%s','%s','%s','%s','%s','%s','%s','%s','%s','%s','%s','%s','%s')>" % \
			(self.rtid,self.version,self.tlid,self.rtsq,
					self.long1,self.lat1,self.long2,self.lat2,
					self.long3,self.lat3,self.long4,self.lat4,
					self.long5,self.lat5,self.long6,self.lat6,
					self.long7,self.lat7,self.long8,self.lat8,
					self.long9,self.lat9,self.long10,self.lat10)


class MungeRT1(object):
    #def __init__(self,rawIn,mungedOut):
    def __init__(self,rawIn):
        print "\n====[ MungeRT1 ]===="
	print 'Reading raw data from %s' % rawIn

    def munge(self,infile):
        rtnum=1
        if not check_filename(infile,rtnum):
            print "Error: invalid input %s" % os.path.basename(infile)
            sys.exit(1)
        else:
            dat = file(infile, 'r')

        recordset=[]
        for line in dat:
            # TODO think about a field name to character range mapping for
	    # this.
            RT=field(line[0])
            VERSION=field(line[1:5])
            TLID=field(line[5:15])
            FEDIRP=field(line[17:19],True)
            FENAME=field(line[19:49],True)
            FETYPE=field(line[49:53],True)
            FEDIRS=field(line[53:55],True)
            FRADDL=field(line[58:69],True)
            TOADDL=field(line[69:80],True)
            FRADDR=field(line[80:91],True)
            TOADDR=field(line[91:102],True)
            ZIPL=field(line[106:111])
            ZIPR=field(line[111:116])
#            FRLONG=decimal2(field(line[190:200],True))
#            FRLAT=decimal2(field(line[200:209],True))
#            TOLONG=decimal2(field(line[209:219],True))
#            TOLAT=decimal2(field(line[219:228],True))

            FRLONG=int(line[190:200])
            FRLAT=int(line[200:209])
            TOLONG=int(line[209:219])
            TOLAT=int(line[219:228])

#            FRLONG=field(line[190:200],True)
#            FRLAT=field(line[200:209],True)
#            TOLONG=field(line[209:219],True)
#            TOLAT=field(line[219:228],True)

#            FRLONG=decimal2(field(line[190:200]))
#            FRLAT=decimal2(field(line[200:209]))
#            TOLONG=decimal2(field(line[209:219]))
#            TOLAT=decimal2(field(line[219:228]))

#            FRLONG=decimal(field(line[190:200]),True)
#            FRLAT=decimal(field(line[200:209]),True)
#            TOLONG=decimal(field(line[209:219]),True)
#            TOLAT=decimal(field(line[219:228]),True)

            # It is an error if any of RT, VERSION, TLID, FRLONG, FRLAT,
            # TOLONG or TOLAT are not present.
            if not check_required(
                    (RT, VERSION, TLID, FRLONG, FRLAT, TOLONG, TOLAT)):
                print "One of the required fields is missing!"
                sys.exit(1)

	    # TODO Ask the user what field separator they want (one space,
	    # vtab, comma), but default to comma.  Keep in sync with the other
	    # munge tool.
            #fieldsep='\t'
            fieldsep=','
            recordset.append(fieldsep.join((RT,VERSION,TLID,FEDIRP,
		    FENAME,FETYPE,FEDIRS,FRADDL,TOADDL,FRADDR,TOADDR,ZIPL,
		    ZIPR,str(FRLONG),str(FRLAT),str(TOLONG),str(TOLAT))))
        dat.close()
        return recordset


class MungeRT2(object):
    #def __init__(self,rawIn,mungedOut):
    def __init__(self,rawIn):
        print "\n====[ MungeRT2 ]===="
	print 'Reading raw data from %s' % rawIn


    def munge(self,infile):
        rtnum=2
        if not check_filename(infile,rtnum):
            print "Error: invalid input %s" % os.path.basename(infile)
            sys.exit(1)
        else:
            dat = file(infile, 'r')

        recordset=[]
        for line in dat:
            # TODO think about a field name to character range mapping for
	    # this.
            RT = field(line[0])
            VERSION = field(line[1:5])
            TLID = field(line[5:15])
            RTSQ = field(line[15:18])
            LONG1  = decimal(field(line[18:28]))
            LAT1   = decimal(field(line[28:37]))
            LONG2  = decimal(field(line[37:47]))
            LAT2   = decimal(field(line[47:56]))
            LONG3  = decimal(field(line[56:66]))
            LAT3   = decimal(field(line[66:75]))
            LONG4  = decimal(field(line[75:85]))
            LAT4   = decimal(field(line[85:94]))
            LONG5  = decimal(field(line[94:104]))
            LAT5   = decimal(field(line[104:113]))
            LONG6  = decimal(field(line[113:123]))
            LAT6   = decimal(field(line[123:132]))
            LONG7  = decimal(field(line[132:142]))
            LAT7   = decimal(field(line[142:151]))
            LONG8  = decimal(field(line[151:161]))
            LAT8   = decimal(field(line[161:170]))
            LONG9  = decimal(field(line[170:180]))
            LAT9   = decimal(field(line[180:189]))
            LONG10 = decimal(field(line[189:199]))
            LAT10  = decimal(field(line[199:208]))

            # It is an error if any of RT, VERSION, TLID, RTSQ, LONG1 or LAT1
            # are not present.
            #
            if not check_required((RT, VERSION, TLID, RTSQ, LONG1, LAT1)):
                print "One of the required fields is missing!"
                sys.exit(1)

	    # TODO Ask the user what field separator they want (one space,
	    # vtab, comma), but default to comma.  Keep in sync with the other
	    # munge tool.
            #fieldsep='\t'
            fieldsep=','
            recordset.append(fieldsep.join((RT,VERSION,TLID,RTSQ,
                str(LONG1),str(LAT1),str(LONG2),str(LAT2),
                str(LONG3),str(LAT3),str(LONG4),str(LAT4),
                str(LONG5),str(LAT5),str(LONG6),str(LAT6),
                str(LONG7),str(LAT7),str(LONG8),str(LAT8),
                str(LONG9),str(LAT9),str(LONG10),str(LAT10))))
        dat.close()
        return recordset


class QueryDatabase(object):
    '''Open a db connection with SQLAlchemy and fetch/return a ResultProxy.'''

    def __init__(self):
        print "\n====[ QueryDatabase ]===="
        self.session=G.Session()
#        print "record count:", self.getRecordCount()

    def getRecordCount(self):
        '''TODO'''
	try:
            return G.recordCount
        except: # AttributeError
            rp=self.session.execute(select([G.tiger01_Table.c.id]))
	    tmp=[]
	    for result in rp:
                tmp.append(result)
	    G.recordCount = len(tmp)
            return G.recordCount

    def getZipCodes(self):
        '''Collect the list of ZIP codes in this county.'''
        return self.session.execute(select([G.tiger01_Table.c.zipl]).distinct())

    def chooseGraphArea(self):
        '''Choose the geographical area for the simulation.

The most common choices are either the area of a ZIP code, or an
entire county.'''
        G.zipCodesResultProxy=self.getZipCodes()

        # TODO experiment with how to pair up numbers (01, 02, ...) with ZIP
	# codes before doing anything about the data structure
	zipList=[]
	zipDict={}
	zipCount=0
	for zip in G.zipCodesResultProxy:
            zipCount+=1
	    # append tuples to the list for display
            zipList.append((zipCount,zip[0]))
	    # add keys and values to the dict for selection
	    if zipCount<10:
                zipStr='0'+str(zipCount)
	    zipDict[zipStr]=zip[0]

        # the data is in zipList; present it to the user
        print 'Here are the county ZIP codes.  Choose one of them'
	print 'or (01 None) to use the whole county: '
	for number, zip in zipList:
            print '%02d %s' % (number, zip)
	self.u=UserInput()

	while True:
            userIn=self.u.getDigit(2,2,"Zip code: ")
	    if userIn in zipDict:
                # got valid input, now do something with it
		if zipDict[userIn]=='01':
                    # use the whole county
		    G.zipCode=None
                else:
                    G.zipCode=zipDict[userIn]
		return G.zipCode # gotta return, why not return something? :)
	    else:
                print "DEBUG %d not in zipDict?" % userIn

    # TODO I'd like to collapse __rpZip() and __rpAll() and any others into a
    # single method that takes variable arguments.  The problem I'm running
    # into is creating a callable string, and executing it.
    def __rpZip(self,zip):
        '''Fetch a SQLAlchemy ResultProxy based on a zip code query.'''
	return self.session.execute(select([
		G.tiger01_Table.c.tlid,
		G.tiger01_Table.c.frlong,
		G.tiger01_Table.c.frlat,
		G.tiger01_Table.c.tolong,
		G.tiger01_Table.c.tolat
		],G.tiger01_Table.c.zipl==zip).distinct())

    # TEMP -- make non-private if I need it outside this class
    def __rpZip(self,zip):
        '''Fetch a SQLAlchemy ResultProxy based on a zip code query.'''
	return self.session.execute(select([
		G.tiger01_Table.c.tlid,
		G.tiger01_Table.c.frlong,
		G.tiger01_Table.c.frlat,
		G.tiger01_Table.c.tolong,
		G.tiger01_Table.c.tolat
		],G.tiger01_Table.c.zipl==zip).distinct())

    def __rpAll(self):
        '''Fetch a SQLAlchemy ResultProxy for all records.'''
        return self.session.execute(select([G.tiger01_Table]).distinct())

    # TEMP -- make non-private if I need it outside this class
    def rpAll(self):
        '''Fetch a SQLAlchemy ResultProxy for all records.'''
        return self.session.execute(select([G.tiger01_Table]).distinct())

    # TODO I want to see some self.rt2.da_da_da in here!

    def tuptotup(self):
        '''TODO Describe (clearer than "Data format change method."'''
        if G.zipCode is None:
           rp=self.__rpAll()
        else:
           rp=self.__rpZip(G.zipCode)

        tuptotup={}
        for result in rp:
            frlong=float(result['frlong'])
            frlat=float(result['frlat'])
            tolong=float(result['tolong'])
            tolat=float(result['tolat'])
	    tuptotup[(frlong,frlat,tolong,tolat)]= \
			    [(frlong,frlat),(tolong,tolat)]
        return tuptotup

    # Here's an example in sqlite3
    #
    # C:\Source\hg\unified\generated\data\AK\TGR02068\sql>sqlite3 TGR02068.db
    # sqlite> SELECT id, frlong, frlat, tolong, tolat FROM tiger_01 ORDER BY random() LIMIT 1;
    # 438|-149.987164|64.144821|-150.002351|64.131662
    # sqlite> SELECT id, frlong, frlat, tolong, tolat FROM tiger_01 ORDER BY random() LIMIT 1;
    # 2164|-148.808454|63.56553|-148.811602|63.564018
    # sqlite>
    #
    # NOTE: this method is similar to agents.Agent mkcoords()
    def get_point(self):
        '''Fetch a SQLAlchemy ResultProxy for a random point on the graph.'''
	# TODO think about renaming this to get_vertices().  get_point() is
	# just plain wrong.  Update the docstring as well.
        randomRow=random.randint(1,self.getRecordCount())
	return self.session.execute(select([
		G.tiger01_Table.c.id,
		G.tiger01_Table.c.tlid,
		G.tiger01_Table.c.frlong,
		G.tiger01_Table.c.frlat,
		G.tiger01_Table.c.tolong,
		G.tiger01_Table.c.tolat
		],G.tiger01_Table.c.id==randomRow)).fetchone()


# BIG CAUTION: G in MakeGraph is a NetworkX Graph object.  G in the tigerutils
# module is a global class for holding variables.
class MakeGraph(object):
    def __init__(self):
        print "\n====[ MakeGraph ]===="
        self.q=QueryDatabase()

    def makeGraph(self):
        self.uniqlist=[]
	# TODO name the graph according to the county code and zipcode if
	# used.  The generated graphic should be named the same way.
        self.G=networkx.XGraph(name="please work ...")
        self.G.pos={}

#        for k,v in self.f.tuptotup(zipcode).items():
#        for k,v in self.f.tuptotup(G.zipCode).items():
        for k,v in self.q.tuptotup().items():

	    # NOTE: it is an error (currently unhandled) if the zipcode is not
	    # found in the database
            fr=(int(v[0][0]),int(v[0][1]))
            to=(int(v[1][0]),int(v[1][1]))
#            fr=(v[0][0],v[0][1])
#            to=(v[1][0],v[1][1])

            if fr not in self.uniqlist:
                self.uniqlist.append(fr)
                self.G.add_node(fr)
                self.G.pos[fr]=fr
            if to not in self.uniqlist:
                self.uniqlist.append(to)
                self.G.add_node(to)
                self.G.pos[to]=to

            self.G.add_edge(fr,to)
            self.G.pos[(fr,to)]=(fr,to)
	    # TODO: if DEBUG?
#            print "self.G.neighbors(fr) => %s" % self.G.neighbors(fr)
#            print "self.G.neighbors(to) => %s" % self.G.neighbors(to)
#            print
        self.G.info()
        # colors: b=blue, w=white, m=magenta, c=cyan, r=red, ...
        networkx.draw_networkx_nodes(self.G,self.G.pos,node_size=2,
			node_color='c')
        networkx.draw_networkx_edges(self.G,self.G.pos,width=0.3,
			edge_color='r')
        # Don't get cute here.  Just give me a file name.
	if G.zipCode is None:
            pngname="TGR%s.png" % G.stateCountyCode
	else:
            pngname="TGR%s_ZIP%s.png" % (G.stateCountyCode, G.zipCode)

	# TODO Where to write the file to?  It's going to the working dir
	# right now.
	if not os.path.exists(IMAGES_DIR):
            print 'Making images dir %s' % IMAGES_DIR
            os.mkdir(IMAGES_DIR)
	print 'Writing %s ...' % os.path.join(IMAGES_DIR, pngname),
        pylab.savefig(os.path.join(IMAGES_DIR, pngname))
	print 'done\n'

    # DO NOT USE THIS.  It's fundamentally broken.  You can't add an edge with
    # only one coordinate.  It's kind of like an x without a y;  or in this
    # case, a tlid without a ...
    def makeGraphFromTLID(self):
        self.uniqlist=[]
	# TODO name the graph according to the county code and zipcode if
	# used.  The generated graphic should be named the same way.
        self.G=networkx.XGraph(name="please work ...")
        self.G.pos={}

#        for k,v in self.f.tuptotup(zipcode).items():
#        for k,v in self.f.tuptotup(G.zipCode).items():
#        for k,v in self.q.tuptotup().items():

        # get the data directly
	self.query=QueryDatabase()
        if G.zipCode is None:
           rp=self.query.rpAll()
        else:
           rp=self.query.__rpZip(G.zipCode)

        for result in rp:
            tlid=float(result['tlid'])
            if tlid not in self.uniqlist:
                self.uniqlist.append(tlid)
                self.G.add_node(tlid)
                self.G.pos[tlid]=tlid

#            frlong=float(result['frlong'])
#            frlat=float(result['frlat'])
#            tolong=float(result['tolong'])
#            tolat=float(result['tolat'])
#	    tuptotup[(frlong,frlat,tolong,tolat)]= \
#			    [(frlong,frlat),(tolong,tolat)]
#        return tuptotup
#
#	    # NOTE: it is an error (currently unhandled) if the zipcode is not
#	    # found in the database
#            fr=(v[0][0],v[0][1])
#            to=(v[1][0],v[1][1])
#
#            if fr not in self.uniqlist:
#                self.uniqlist.append(fr)
#                self.G.add_node(fr)
#                self.G.pos[fr]=fr
#            if to not in self.uniqlist:
#                self.uniqlist.append(to)
#                self.G.add_node(to)
#                self.G.pos[to]=to

            self.G.add_edge(tlid)
            self.G.pos[(tlid)]=(tlid)
            print "self.G.neighbors(tlid) => %s" % self.G.neighbors(tlid)
            print
#            self.G.add_edge(fr,to)
#            self.G.pos[(fr,to)]=(fr,to)
#            print "self.G.neighbors(fr) => %s" % self.G.neighbors(fr)
#            print "self.G.neighbors(to) => %s" % self.G.neighbors(to)
#            print
        self.G.info()
        # colors: b=blue, w=white, m=magenta, c=cyan, r=red, ...
        networkx.draw_networkx_nodes(self.G,self.G.pos,node_size=2,
			node_color='c')
        networkx.draw_networkx_edges(self.G,self.G.pos,width=0.3,
			edge_color='r')
        # Don't get cute here.  Just give me a file name.
	if G.zipCode is None:
            pngname="TGR%s.png" % G.stateCountyCode
	else:
            pngname="TGR%s_ZIP%s.png" % (G.stateCountyCode, G.zipCode)

	# TODO Where to write the file to?  It's going to the working dir
	# right now.
	if not os.path.exists(IMAGES_DIR):
            print 'Making images dir %s' % IMAGES_DIR
            os.mkdir(IMAGES_DIR)
	print 'Writing %s ...' % os.path.join(IMAGES_DIR, pngname),
        pylab.savefig(os.path.join(IMAGES_DIR, pngname))
	print 'done\n'

    def shortest_path(self,point1,point2):
        # TODO get the points from the ...
        #
        # TODO arrange to choose source and target randomly
        # ZIP 99744
        #latlong1=(-149.198816, 64.347768)
        #latlong2=(-149.197317, 64.350731)
        #
        # ZIP 98121
#        latlong1=(-122.349738, 47.616520)
#        latlong2=(-122.352438, 47.617020)
#	print("NP.shortest_path: %s" % NP.shortest_path(G,latlong1,latlong2))

        # ugly temporary hack
#        point1=list(point1)
#        point2=list(point2)
#        point1[0]=point1[0].lstrip('+')
#        point1[1]=point1[1].lstrip('+')
#        point2[0]=point2[0].lstrip('+')
#        point2[1]=point2[1].lstrip('+')
#	point1=tuple(point1)
#	point2=tuple(point2)

        point1=list(point1)
        point2=list(point2)
        point1[0],point1[1]=int(point1[0]),int(point1[1])
        point2[0],point2[1]=int(point2[0]),int(point2[1])
#        point1[1]=int(point1[1])
#        point2[0]=int(point2[0])
#        point2[1]=int(point2[1])
	point1=tuple(point1)
	point2=tuple(point2)
#	print("networkx.path.shortest_path: %s" %
#			networkx.path.shortest_path(self.G,point1,point2))
        return networkx.path.shortest_path(self.G,point1,point2)

    def get_connected(self):
	print "connected components:"
	i=0
	j=0
	for comp in networkx.component.connected_components(self.G):
            print i, len(comp), comp
	    j+=len(comp)
	    i+=1
	print "total number of components: %d" % j

#
# Most of these are from the original mungeutils.py (now rolled into
# tigerutils.py).
#

def check_filename(filename,rt):
    '''Simple check for invalid RecordType file input'''
    rtnum=('RT'+str(rt)).lower()

    # Holdovers from the past: RT1u, RT2u.  In an earlier version of this
    # project, there were duplicate entries in the RT files, and they were
    # causing havoc with ... something.  The solution was to rip out all the
    # duplicates, and drop the remaining records into a new RT munged file
    # called RT1u and RT2u.  The u was for unique.
    extensions=['RT1','RT1m','RT1u','RT2','RT2m','RT2u']
    for x in extensions:
        if filename.endswith(x):
            return True
    return False

def check_required(n):
    '''Check for required fields in the RecordType file.'''
    fields=list(n)
    for field in fields:
        if field=='\N':
            return False
    return True

def decimal(field):
    '''Normalize all decimal data to the same precision.'''
    decimal_places=10**6
    return float(field)/decimal_places

def decimal2(field):
    '''Normalize all decimal data to the same precision.'''
    decimal_places=10**6
    return Decimal(field)/decimal_places

# Since I started this project, Python has added module decimal to the
# standard library.  I don't yet understand it enough to like it, but I need
# it.  [ http://www.python.org/doc/2.4.4/whatsnew/node9.html ]
#
# Converting from floating-point numbers poses a bit of a problem: should the
# FP number representing 1.1 turn into the decimal number for exactly 1.1, or
# for 1.1 plus whatever inaccuracies are introduced? The decision was to dodge
# the issue and leave such a conversion out of the API. Instead, you should
# convert the floating-point number into a string using the desired precision
# and pass the string to the Decimal constructor:
#
#>>> f = 1.1
#>>> decimal.Decimal(str(f))
#Decimal("1.1")
#>>> decimal.Decimal('%.12f' % f)
#Decimal("1.100000000000")
#
#def decimal2(field):
#    if not type(field)==type(float):
#        print 'float required'
#	sys.exit(0)
#    else:
#        print '[DEBUG] got a float'
#
#    decimal_places=6
#    # magic number -----------\/
#    print decimal.Decimal('%.6f' % (field))
##    return decimal.Decimal('%.6f' % (field))

def field(n,wrap=False):
    '''Clean certain data for safe insertion to database.'''
    x=n.strip()
    if x=='':
        return 'NULL'
    else:
        if wrap:
            # wrap in double quotes to prevent problems with names like
            # O'Brian (in FEDIRS mostly)
            return '"'+x+'"'
        else:
            return x

# vim: tw=78
