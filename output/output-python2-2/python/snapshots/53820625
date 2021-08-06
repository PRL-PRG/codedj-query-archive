# -*- coding: utf-8 -*-
# -*- tab-width: 4; use-tabs: 1 -*-
# vim:tabstop=4:noexpandtab:
"""
Extends rcon for some Nexuiz-specific functionality.

That is, it's more helpful/automagical.
"""
from __future__ import division, absolute_import, with_statement
from .rcon import Rcon
__all__ = 'Commands',

class _Commands(object):
	"""
	Does all the handling of command aliases, so that one can chain together 
	commands more easily.
	
	TODO: Add some callback-like things?
	"""
	def getvar(self, name, value):
		"""Commands.setvar(string) -> string
		Gets a variable.
		"""
	
	def setvar(self, name, value):
		"""Commands.setvar(string, string) -> None
		Sets a variable.
		"""
	
	def say(self, text):
		"""Commands.say(string) -> None
		Causes the server to say something in chat.
		"""
		# NOTE: say has different escaping rules.

Commands = _Commands()

"""
Core C Commands:
cmd.c: alias, cmd, cmdlist, cprint, cvar_lockdefaults, cvar_resettodefaults_all, cvar_resettodefaults_nosaveonly, cvar_resettodefaults_saveonly, cvarlist, defer, echo, exec, set, seta, stuffcmds, toggle, wait

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
"""

"""
QC Commands:
begin : signon 3 (client asks server to start sending entities, and will go to signon 4 (playing) when the first entity update is received)
bottomcolor : QW command to set bottom color without changing top color
changelevel : change to another level, bringing along all connected clients
cl_cmd : calls the client QC function GameCommand with the supplied string as argument
clear : clear console history
color : change your player shirt and pants colors
condump : output console history to a file (see also log_file)
connect : connect to a server by IP address or hostname
curl : download data from an URL and add to search path
demos : restart looping demos defined by the last startdemos command
dir : list files in searchpath matching an * filename pattern, one per line
download : downloads a specified file from the server
fixtrans : change alpha-zero pixels in an image file to sensible values, and write out a new TGA (warning: SLOW)
fly : fly mode (flight)
fs_rescan : rescans filesystem for new pack archives and any other changes
fullinfo : allows client to modify their userinfo
fullserverinfo : internal use only, sent by server to client to update client's local copy of serverinfo string
gamedir : changes active gamedir list (can take multiple arguments), not including base directory (example usage: gamedir ctf)
give : alter inventory
god : god mode (invulnerability)
heartbeat : send a heartbeat to the master server (updates your server information)
kick : kick a player off the server by number or name
kill : die instantly
load : load a saved game file
loadconfig : reset everything and reload configs
ls : list files in searchpath matching an * filename pattern, multiple per line
map : kick everyone off the server and start a new level
maps : list information about available maps
maxplayers : sets limit on how many players (or bots) may be connected to the server at once
memlist : prints memory pool information (or if used as memlist 5 lists individual allocations of 5K or larger, 0 lists all allocations)
memstats : prints memory system statistics
menu_cmd : calls the menu QC function GameCommand with the supplied string as argument
messagemode : input a chat message to say to everyone
messagemode2 : input a chat message to say to only your team
modellist : prints a list of loaded models
modelprecache : load a model
name : change your player name
net_refresh : query dp master servers and refresh all server information
net_slist : query dp master servers and print all server information
net_slistqw : query qw master servers and print all server information
net_stats : print network statistics
noclip : noclip mode (flight without collisions, move through walls)
notarget : notarget mode (monsters do not see you)
packet : send a packet to the specified address:port containing a text string
path : print searchpath (game directories and archives)
pause : pause the game (if the server allows pausing)
ping : print ping times of all players on the server
pingplreport : command sent by server containing client ping and packet loss values for scoreboard, triggered by pings command from client (not used by QW servers)
pings : command sent by clients to request updated ping and packetloss of players on scoreboard (originally from QW, but also used on NQ servers)
playermodel : change your player model
playerskin : change your player skin number
prespawn : signon 1 (client acknowledges that server information has been received)
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
quit : quit the game
rate : change your network connection speed
rcon : sends a command to the server console (if your rcon_password matches the server's rcon_password), or to the address specified by rcon_address when not connected (again rcon_password must match the server's)
reconnect : reconnect to the last server you were on, or resets a quakeworld connection (do not use if currently playing on a netquake server)
restart : restart current level
save : save the game to a file
saveconfig : save settings to config.cfg (or a specified filename) immediately (also automatic when quitting)
say : send a chat message to everyone on the server
say_team : send a chat message to your team on the server
sendcvar : sends the value of a cvar to the server as a sentcvar command, for use by QuakeC
setinfo : modifies your userinfo
spawn : signon 2 (client has sent player information, and is asking server to send scoreboard rankings)
startdemos : start playing back the selected demos sequentially (used at end of startup script)
status : print server status information
stopdemo : stop playing or recording demo (like stop command) and return to looping demos
sv_areastats : prints statistics on entity culling during collision traces
sv_cmd : calls the server QC function GameCommand with the supplied string as argument
sv_saveentfile : save map entities to .ent file (to allow external editing)
sv_startdownload : begins sending a file to the client (network protocol use only)
tell : send a chat message to only one person on the server
toggleconsole : opens or closes the console
topcolor : QW command to set top color without changing bottom color
user : prints additional information about a player number or name on the scoreboard
users : prints additional information about all players on the scoreboard
version : print engine version
viewframe : change animation frame of viewthing entity in current level
viewmodel : change model of viewthing entity in current level
viewnext : change to next animation frame of viewthing entity in current level
viewprev : change to previous animation frame of viewthing entity in current level
"""

if __name__ == '__main__':
	pass
