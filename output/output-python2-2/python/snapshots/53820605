# -*- coding: utf-8 -*-
# -*- tab-width: 4; use-tabs: 1 -*-
# vim:tabstop=4:noexpandtab:
"""
Extends rcon for some Nexuiz-specific functionality.

That is, it's more helpful/automagical.

Note: If you actually care about the results of commands, it is encouraged that 
you send them one at a time. Commands with large output may cause strange 
issues.
"""
from __future__ import division, absolute_import, with_statement
from .rcon import Rcon
from . import utils
__all__ = 'Commands', 'NexRcon'

class _Commands(object):
	"""
	Does all the handling of command aliases, so that one can chain together 
	commands more easily.
	
	Each method represents an actual command.
	"""
	# Because it would be rediculous to implement every command, we do some hackery
	# When a NexRcon is init'd, it requests the current list of commands before 
	# start_stream() is called. This allows it to fully parse the long list.
	__commands = None
	__cont = ''
	def __init__(self):
		self.__commands = {}
		self.__cont = ''
	
	def _init_commands(self, text):
		"""
		Internal.
		Used by NexRcon to fill in details about commands.
		"""
		lines = (self.__cont+text).split('\n')
		for line in lines[:-1]:
			name, doc = line.split(':')
			name = name.strip()
			doc = doc.strip()
			self.__commands[name] = doc
			meth = getattr(self, name) # Cause the method to be created
		self.__cont = lines[-1]
		if self.__cont == '':
			return None
		return text
	
	def _get_method(self, name):
		"""
		Internal.
		Creates a method to parse a command.
		"""
		def generic_command(*pargs):
			return (name,)+pargs
		generic_command.__name__ = name
		if name in self.__commands:
			generic_command.__doc__ = self.__commands[doc]
		return generic_command
	
	def __getattr__(self, name): 
		"""Commands.__getattr__(n) <==> getattr(Commands, n)
		
		Returns methods for commands which haven't been created yet.
		"""
		# We depend on the behavior that if the method already exists, this isn't called
		generic_command = self._get_method(name)
		setattr(self, name, generic_command)
		return generic_command
	
	# These commands require special handling to be Pythonic.
	def say(self, text):
		"send a chat message to everyone on the server"
		return 'say '+utils.quote(text, say=True)
	
	def say_team(self, text):
		"send a chat message to your team on the server"
		return 'say_team '+utils.quote(text, say=True)
	
	def echo(self, text):
		"print a message to the console (useful in scripts)"
		return 'echo '+utils.quote(text, say=True)
	
	def tell(self, user, text):
		"send a chat message to only one person on the server"
		# Really, really weird
		return 'tell '+utils.quote("#%i %s"% (user, text), say=True)
	
	# All other commands work right.

Commands = _Commands()

class NexRcon(Rcon):
	"""
	Like Rcon, with the addition of handling Commands.
	"""
	# Uses some internal methods of Rcon
	_init = True
	def __init__(self, *pargs):
		"""
		See Rcon.__init__()
		"""
		Rcon.__init__(self, *pargs)
		self._init = True
		# Catches multiple packets because of the command list's size
		# Does some magical method swapping
		self._backup_textReceived = self.textReceived
		self.textReceived = self._textReceived
		# Grab the list of commands
		d = self.cmd('cmdlist')
		d.addCallback(Commands._init_commands)
		d.addCallback(self._finished_huh)
		# TODO: Do we wait for _init to be cleared?
	
	def _finished_huh(self, text):
		"""
		Internal.
		Does work dealing with wrapping up initialization.
		"""
		if text is None:
			self._init = False
			self._textReceived(None) # To clean up
		return text
	
	def _textReceived(self, text):
		"""
		Internal.
		Catches loose packets and wraps up init.
		"""
		if self._init:
			self._finished_huh(Commands._init_commands(text))
		if not self._init: # Make sure we swap methods back
			self.textReceived = self._backup_textReceived
	
	def send(self, *cmds):
		"""nr.send(cmd, ...) -> Deferred
		Sends 1 or more commands.
		
		Example:
		>>> NexRcon('127.0.0.1', 26000, 'spam&eggs').send(Commands.say("Hello!"))
		"""
		cmd_strings = []
		for cmd in cmds:
			if isinstance(cmd, basestring):
				# Pre-quoted
				cmd_strings.append(cmd)
			else:
				# Just a collection of arguments
				cmd_strings.append(self.format_cmd(*cmd))
		return self._sends(cmd_strings)

"""
Core C Commands:
cmd.c:
alias : create a script function (parameters are passed in as $1 through $9, and $* for all parameters)
cmd : send a console commandline to the server (used by some mods)
cmdlist : lists all console commands beginning with the specified prefix
cprint : print something at the screen center
cvar_lockdefaults : stores the current values of all cvars into their default values, only used once during startup after parsing default.cfg
cvar_resettodefaults_all : sets all cvars to their locked default values
cvar_resettodefaults_nosaveonly : sets all non-saved cvars to their locked default values (variables that will not be saved to config.cfg)
cvar_resettodefaults_saveonly : sets all saved cvars to their locked default values (variables that will be saved to config.cfg)
cvarlist : lists all console variables beginning with the specified prefix
defer : execute a command in the future
echo : print a message to the console (useful in scripts)
exec : execute a script file
set : create or change the value of a console variable
seta : create or change the value of a console variable that will be saved to config.cfg
toggle : toggles a console variable's values (use for more info)
stuffcmds : execute commandline parameters (must be present in quake.rc script)
wait : make script execution wait for next rendered frame

host_cmd.c:
begin : signon 3 (client asks server to start sending entities, and will go to signon 4 (playing) when the first entity update is received)
bottomcolor : QW command to set bottom color without changing top color
changelevel : change to another level, bringing along all connected clients
color : change your player shirt and pants colors
connect : connect to a server by IP address or hostname
demos : restart looping demos defined by the last startdemos command
fixtrans : change alpha-zero pixels in an image file to sensible values, and write out a new TGA (warning: SLOW)
fly : fly mode (flight)
fullinfo : allows client to modify their userinfo
fullserverinfo : internal use only, sent by server to client to update client's local copy of serverinfo string
give : alter inventory
god : god mode (invulnerability)
kick : kick a player off the server by number or name
kill : die instantly
load : load a saved game file
map : kick everyone off the server and start a new level
maxplayers : sets limit on how many players (or bots) may be connected to the server at once
name : change your player name
noclip : noclip mode (flight without collisions, move through walls)
notarget : notarget mode (monsters do not see you)
packet : send a packet to the specified address:port containing a text string
pause : pause the game (if the server allows pausing)
pingplreport : command sent by server containing client ping and packet loss values for scoreboard, triggered by pings command from client (not used by QW servers)
pings : command sent by clients to request updated ping and packetloss of players on scoreboard (originally from QW, but also used on NQ servers)
playermodel : change your player model
playerskin : change your player skin number
prespawn : signon 1 (client acknowledges that server information has been received)
quit : quit the game
rate : change your network connection speed
rcon : sends a command to the server console (if your rcon_password matches the server's rcon_password), or to the address specified by rcon_address when not connected (again rcon_password must match the server's)
reconnect : reconnect to the last server you were on, or resets a quakeworld connection (do not use if currently playing on a netquake server)
restart : restart current level
save : save the game to a file
say : send a chat message to everyone on the server
say_team : send a chat message to your team on the server
sendcvar : sends the value of a cvar to the server as a sentcvar command, for use by QuakeC
setinfo : modifies your userinfo
spawn : signon 2 (client has sent player information, and is asking server to send scoreboard rankings)
startdemos : start playing back the selected demos sequentially (used at end of startup script)
status : print server status information
stopdemo : stop playing or recording demo (like stop command) and return to looping demos
tell : send a chat message to only one person on the server
topcolor : QW command to set top color without changing bottom color
user : prints additional information about a player number or name on the scoreboard
users : prints additional information about all players on the scoreboard
version : print engine version
viewframe : change animation frame of viewthing entity in current level
viewmodel : change model of viewthing entity in current level
viewnext : change to next animation frame of viewthing entity in current level
viewprev : change to previous animation frame of viewthing entity in current level
"""

"""
Unsorted Commands:
cl_cmd : calls the client QC function GameCommand with the supplied string as argument
clear : clear console history
condump : output console history to a file (see also log_file)
curl : download data from an URL and add to search path
dir : list files in searchpath matching an * filename pattern, one per line
download : downloads a specified file from the server
fs_rescan : rescans filesystem for new pack archives and any other changes
gamedir : changes active gamedir list (can take multiple arguments), not including base directory (example usage: gamedir ctf)
heartbeat : send a heartbeat to the master server (updates your server information)
loadconfig : reset everything and reload configs
ls : list files in searchpath matching an * filename pattern, multiple per line
maps : list information about available maps
memlist : prints memory pool information (or if used as memlist 5 lists individual allocations of 5K or larger, 0 lists all allocations)
memstats : prints memory system statistics
menu_cmd : calls the menu QC function GameCommand with the supplied string as argument
messagemode : input a chat message to say to everyone
messagemode2 : input a chat message to say to only your team
modellist : prints a list of loaded models
modelprecache : load a model
net_refresh : query dp master servers and refresh all server information
net_slist : query dp master servers and print all server information
net_slistqw : query qw master servers and print all server information
net_stats : print network statistics
path : print searchpath (game directories and archives)
ping : print ping times of all players on the server
prvm_callprofile : prints execution statistics about the most time consuming QuakeC calls from the engine in the selected VM (server, client, menu)
prvm_edict : print all data about an entity number in the selected VM (server, client, menu)
prvm_edictcount : prints number of active entities in the selected VM (server, client, menu)
prvm_edicts : prints all data about all entities in the selected VM (server, client, menu)
prvm_edictset : changes value of a specified property of a specified entity in the selected VM (server, client, menu)
prvm_fields : prints usage statistics on properties (how many entities have non-zero values) in the selected VM (server, client, menu)
prvm_global : prints value of a specified global variable in the selected VM (server, client, menu)
prvm_globals : prints all global variables in the selected VM (server, client, menu)
prvm_globalset : sets value of a specified global variable in the selected VM (server, client, menu)
prvm_printfunction : prints a disassembly (QuakeC instructions) of the specified function in the selected VM (server, client, menu)
prvm_profile : prints execution statistics about the most used QuakeC functions in the selected VM (server, client, menu)
saveconfig : save settings to config.cfg (or a specified filename) immediately (also automatic when quitting)
sv_areastats : prints statistics on entity culling during collision traces
sv_cmd : calls the server QC function GameCommand with the supplied string as argument
sv_saveentfile : save map entities to .ent file (to allow external editing)
sv_startdownload : begins sending a file to the client (network protocol use only)
toggleconsole : opens or closes the console
"""
"""
There are no QC commands.
"""
"""
*_cmd: qcsrc/common/gamecommand.qc:47
cl_cmd: ?
menu_cmd: qcsrc/menu/gamecommand.qc:34
sv_cmd: qcsrc/server/gamecommand.qc:18
"""

if __name__ == '__main__':
	pass
