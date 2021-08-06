#!/usr/bin/env python
# -*- coding: utf-8 -*-
#
# Code released in the Public Domain. You can do whatever you want with this package.
# Though I'm learning Python, I've tried to use the best practices in XO development.
# Look at NOTES file to see how to adapt this program.
# Originally written by Pierre Métras <pierre@alterna.tv> for the OLPC XO laptop.


"""Learning time.
==============
The XO is missing a simple clock for kids to learn how to
read time, but more importantly to know what time is is. When you
don't own a clock, the XO can be used to display the time to
arrive in time at school...
A clock can also be used to learn how to count and read numbers.

Display and behavior can be changed with the buttons in the toolbar:
- A simple clock with hours figures to learn to tell the time.
- A nice clock face, without hours numbers.
- A digital clock with a time scale.
Also, the clock can print the current time in full letters. Or speak it aloud.

To help learning the time, all the clocks displays use a consistent color code:
- Hours         blue: #005FE4
- Minutes       green: #00B20D
- Seconds       red: #E6000A
- Days          dark red: #B20008
- Months        purple: #5E008C
- Years         brown: #9A5200


An analog clock is also very helpfull to determine where the North is when you
don't have a compass!
Check http://www.wikihow.com/Find-True-North-Without-a-Compass
And knowing where the True North is, you can build a Sun Clock!

Author: Pierre Metras <pierre@alterna.tv>
Based on work from Davyd Madeley, Lawrence Oluyede <l.oluyede@gmail.com>
SVG background adapted from Open ClipArt: http://openclipart.org/people/rihard/rihard_Clock_Calendar_2.svg

More about clocks and time in the World
---------------------------------------
- Clock face: http://en.wikipedia.org/wiki/Clock_face
- 12 hours clock: http://en.wikipedia.org/wiki/12-hour_clock
- 24 hours clock: http://en.wikipedia.org/wiki/24-hour_clock
- Thai 6 hours clock: http://en.wikipedia.org/wiki/Thai_six-hour_clock
- Time and date in the World: http://en.wikipedia.org/wiki/Date_and_time_notation_by_country
"""

# We initialize threading in gobject. As we will detach another thread to translate the
# time to text, this other thread will eventually update the display with idle_add()
# calls, because it is not running in the main event thread. But idle_add() puts a
# callback in the message queue with the lowest priority. When the nice clock is
# displayed, it can spend a few seconds (20 to 30 is common) before the GTK loop will
# process this low priority message. When we enable the threads, the processing is
# almost instantaneous.
import gobject
gobject.threads_init()

import pygtk
import gtk
from gtk import gdk
import pango

import math
from datetime import datetime
import threading
import gc
import re

from pgettext import pgettext as _p

from sugar.activity import activity
from sugar.graphics.toggletoolbutton import ToggleToolButton
from sugar.graphics.radiotoolbutton import RadioToolButton

from speaker import Speaker
from timewriter import TimeWriter


# The display modes of the clock
_MODE_SIMPLE_CLOCK = 0
_MODE_NICE_CLOCK = 1
_MODE_DIGITAL_CLOCK = 2


class ClockActivity(activity.Activity):
    """The clock activity displays a simple clock widget.
    """

    def __init__(self, handle):
        """Create and initialize the clock activity.
        """
        super(ClockActivity, self).__init__(handle)

        # TRANS: Title of the activity
        self.set_title(_p("Activity", "What Time Is It?"))

        # TRANS: The format used when writing the time in full letters.
        # You must take care to use a font size large enough so that kids can read it easily,
        # but also small enough so that all times combination fit on the screen, even
        # when the screen is rotated.
        # Pango markup: http://www.pygtk.org/docs/pygtk/pango-markup-language.html
        self._TIME_LETTERS_FORMAT = _p("Write Time", """<markup><span lang="en" font_desc="Sans 20">%s</span></markup>""")

        # TRANS: The format used to display the weekday and date (example: Tuesday 10/21/2008)
        # We recommend to use the same font size as for the time display.
        # See http://docs.python.org/lib/module-time.html for
        # available strftime formats.
        # xgettext:no-python-format
        self._DATE_SHORT_FORMAT = _p("Write Date", """<markup><span lang="en" font_desc="Sans 20"><span foreground="#B20008">%A</span>, <span foreground="#5E008C">%m</span>/<span foreground="#B20008">%d</span>/<span foreground="#9A5200">%Y</span></span></markup>""")

        # Should we write the time in full letters?
        self._write_time = False
        self._time_writer = None
        self._time_in_letters = self.get_title()
        
        # The optional labels to display the date, the day of week or time.
        self._time_letters = None
        self._date = None

        # Should we talk?
        self._speak_time = False
        self._time_speaker = None

        toolbox = self._make_toolbars()
        self._make_display()

        # Show the activity on the screen
        self.show_all()

        # Hide the tools we don't use from the activity toolbar
        toolbox.get_activity_toolbar().share.hide()
        toolbox.get_activity_toolbar().keep.hide()

        # We want to be notified when the minutes change
        self._clock.connect("time_minute", self._minutes_changed_cb)

        # We want also to be notified when the activity gets the focus or loses it.
        # When it is not active, we don't need to update the clock.
        self.connect("notify::active", self._notify_active_cb)


    def _make_toolbars(self):
        """Prepare and set the toolbars of the activity.
        Load and show icons. Associate them to the call back methods.
        """
        # Default toolbar
        toolbox = activity.ActivityToolbox(self)
        self.set_toolbox(toolbox)

        # In the activity toolbar, we find first, the name of the activity field, a
        # spring separator, the share combobox, the keep and quit buttons.
        #activity_toolbar = toolbox.get_activity_toolbar()
        # Hide the tools we don't use
        #activity_toolbar.share.set_no_show_all(True)
        #activity_toolbar.keep.set_no_show_all(True)


        # Create the display tool bar
        display_toolbar = gtk.Toolbar()
        
        # First group of radio button to select the type of clock to display
        button1 = RadioToolButton(named_icon = "simple-clock")
        button1.set_tooltip(_p("Toolbar", "Simple Clock"))
        button1.connect("toggled", self._display_mode_changed_cb, _MODE_SIMPLE_CLOCK)
        display_toolbar.insert(button1, -1)
        button2 = RadioToolButton(named_icon = "nice-clock", group = button1)
        button2.set_tooltip(_p("Toolbar", "Nice Clock"))
        button2.connect("toggled", self._display_mode_changed_cb, _MODE_NICE_CLOCK)
        display_toolbar.insert(button2, -1)
        button3 = RadioToolButton(named_icon = "digital-clock", group = button1)
        button3.set_tooltip(_p("Toolbar", "Digital Clock"))
        button3.connect("toggled", self._display_mode_changed_cb, _MODE_DIGITAL_CLOCK)
        display_toolbar.insert(button3, -1)
        
        # A separator between the two groups of buttons
        separator = gtk.SeparatorToolItem()
        separator.set_draw(True)
        display_toolbar.insert(separator, -1)
        
        # Now the options buttons to display other elements: date, day of week...
        # A button in the toolbar to write the time in full letters
        button = ToggleToolButton("write-time")
        button.set_tooltip(_p("Toolbar", "Display time in full letters"))
        button.connect("toggled", self._write_time_clicked_cb)
        display_toolbar.insert(button, -1)

        # The button to display the weekday and date
        button = ToggleToolButton("write-date")
        button.set_tooltip(_p("Toolbar", "Display weekday and date"))
        button.connect("toggled", self._write_date_clicked_cb)
        display_toolbar.insert(button, -1)

        # A separator between the two groups of buttons
        separator = gtk.SeparatorToolItem()
        separator.set_draw(True)
        display_toolbar.insert(separator, -1)
        
        # Another button to speak aloud the time
        button = ToggleToolButton("speak-time")
        button.set_tooltip(_p("Toolbar", "Talking clock"))
        button.connect("toggled", self._speak_time_clicked_cb)
        display_toolbar.insert(button, -1)

        # Add the toolbar to the activity menu
        toolbox.add_toolbar(_p("Toolbar", "Clock"), display_toolbar)
        toolbox.set_current_toolbar(1)

        # Return the toolbox (this is necessary because there is no activity.get_toolbox()
        # method).
        return toolbox


    def _make_display(self):
        """Prepare the display of the clock.
        The display has two parts: the clock face at the top, and the time in full letters
        at the bottom, when the user selects to show it.
        """
        # The clock face
        self._clock = ClockFace()
        
        # The label to print the time in full letters
        self._time_letters = gtk.Label()
        self._time_letters.set_no_show_all(True)
        # Following line in ineffective!
        #self._time_letters.set_line_wrap(True)
        # Resize the invisible label so that gtk will know in advance the height when
        # we show it.
        self._time_letters.set_markup(self._TIME_LETTERS_FORMAT % self._time_in_letters)

        # The label to write the date
        self._date = gtk.Label()
        self._date.set_no_show_all(True)
        self._date.set_markup(self._clock.get_time().strftime(self._DATE_SHORT_FORMAT))
        
        # Put all these widgets in a vertical box
        vbox = gtk.VBox(False)
        vbox.pack_start(self._clock, True)
        vbox.pack_start(self._time_letters, False)
        vbox.pack_start(self._date, False)

        # Attach the display to the activity
        self.set_canvas(vbox)


    def _write_date_clicked_cb(self, button):
        """The user clicked on the "write date" button to display the current weekday and
        date.
        """
        if button.get_active():
            gobject.idle_add(self._date.show)
        else:
            gobject.idle_add(self._date.hide)


    def _display_mode_changed_cb(self, radiobutton, display_mode):
        """The user selected a clock display mode (simple clock, nice or digital).
        """
        self._clock.set_display_mode(display_mode)


    def _write_time_clicked_cb(self, button):
        """The user clicked on the "write time" button to print the current time.
        """
        self._write_time = button.get_active()
        if self._write_time:
            gobject.idle_add(self._time_letters.show)
            self._write_and_speak(False)
        else:
            gobject.idle_add(self._time_letters.hide)


    def _speak_time_clicked_cb(self, button):
        """The user clicked on the "speak time" button to hear the talking clock.
        """
        self._speak_time = button.get_active()
        if self._speak_time:
            self._write_and_speak(True)


    def _minutes_changed_cb(self, clock):
        """Minutes have changed on the clock face: we have to update the display
        of the time in full letters if the user has chosen to have it and eventually
        croak the time.
        """
        # Change time display and talk, if necessary
        self._write_and_speak(True)

        # Update the weekday and date in case it was midnight
        gobject.idle_add(self._date.set_markup, \
                clock.get_time().strftime(self._DATE_SHORT_FORMAT))


    def _notify_active_cb(self, widget, event):
        """Sugar notify us that the activity is becoming active or inactive.
        When we are inactive, we change the activity status of the clock face widget,
        so that it can stop updating every seconds.
        """
        self._clock.active = self.props.active


    def _write_and_speak(self, speak):
        """
        Write and speak the time (called in another thread not to block the clock).
        """
        # A helper function for the running thread
        def thread_write_and_speak():
            # Only update the time in full letters when necessary
            if self._write_time or self._speak_time:
                self._do_write_time()

            # And if requested, say it aloud
            if self._speak_time and speak:
                self._do_speak_time()

        # Now detach a thread to do the big job
        thread = threading.Thread(target = thread_write_and_speak)
        thread.start()


    def _do_write_time(self):
        """Translate the time to full letters.
        """
        if self._time_writer is None:
            self._time_writer = TimeWriter()
        self._time_in_letters = self._time_writer.write_time(self._clock.get_time().hour, \
                self._clock.get_time().minute)
        gobject.idle_add(self._time_letters.set_markup, \
                self._TIME_LETTERS_FORMAT % self._time_in_letters)


    def _do_speak_time(self):
        """Speak aloud the current time.
        """
        if self._time_speaker is None:
            self._time_speaker = Speaker()
        self._time_speaker.speak(self._untag(self._time_in_letters))


    def _untag(self, text):
        """Remove all the tags (pango markup) from a text.
        """
        if text == False or "<" not in text:
            return text
        else:
            result = ""
            for s in re.findall(r"(<.*?>)|([^<>]+)", text):
                result += s[1]
            return result


class ClockFace(gtk.DrawingArea):
    """The Pango widget of the clock.
    This widget draws a simple analog clock, with 3 hands (hours, minutes and seconds)
    or a digital clock. Depending on the display mode, different information is displayed.
    """

    def __init__(self):
        """Initialize the clock widget.
        The mode defaults to the basic analog clock, with no hours mark or date.
        """
        super(ClockFace, self).__init__()

        # The time on the clock face
        self._time = datetime.now()
        self._old_minute = self._time.minute

        # Update the clock only when the widget is active to save resource
        self._active = False

        # The display mode of the clock
        self._mode = _MODE_SIMPLE_CLOCK

        # SVG Background cache
        self._cache_pixbuf = None
        self._radius = -1

        # The graphic context used for drawings
        self._gc = None
        self._line_width = 2

        # Color codes (approved colors for XO screen:
        # http://wiki.laptop.org/go/XO_colors)
        colormap = self.get_colormap()
        self._COLOR_HOURS = colormap.alloc_color("#005FE4")      # XO Medium Blue
        self._COLOR_MINUTES = colormap.alloc_color("#00B20D")    # XO Medium Green
        self._COLOR_SECONDS = colormap.alloc_color("#E6000A")    # XO Medium Red
        self._COLOR_WHITE = colormap.alloc_color("#FFFFFF")      # White
        self._COLOR_BLACK = colormap.alloc_color("#000000")      # Black

        # gtk.Widget signals
        self.connect("expose-event", self._expose_cb)
        self.connect("size-allocate", self._size_allocate_cb)

        # The masks to capture the events we are interested in
        self.add_events(gdk.EXPOSURE_MASK | gdk.VISIBILITY_NOTIFY_MASK)

        # Define a new signal to notify the application when minutes change.
        # If the user wants to display the time in full letters, the method of the
        # activity will be called back to refresh the display.
        gobject.signal_new("time_minute", ClockFace, gobject.SIGNAL_RUN_LAST, \
                gobject.TYPE_NONE, [])


    def set_display_mode(self, mode):
        """Set the type of clock to display (simple, nice, digital).
        'mode' is one of MODE_XXX_CLOCK constants.
        """
        self._mode = mode


    def _size_allocate_cb(self, widget, allocation):
        """We know the size of the widget on the screen, so we keep the parameters
        which are important for our rendering (center of the clock, radius).
        """
        if widget.window:
            # Store the measures of the clock face widget
            self._center_x = int(allocation.x + allocation.width / 2.0)
            self._center_y = int(allocation.y + allocation.height / 2.0)
            self._radius = max(min(int(allocation.width / 2.0), \
                    int(allocation.height / 2.0)) - 20, 0)
            self._width = allocation.width
            self._height = allocation.height
            self._line_width = int(self._radius / 150)

            # Reload the cached pixbuf
            self._cache_pixbuf = gdk.pixbuf_new_from_file_at_size("clock.svg", 2 * self._radius, 2 * self._radius)
            gc.collect() # Reclaim memory from old pixbuf


    def _expose_cb(self, widget, event):
        """The widget is exposed and must draw itself on the graphic context.
        In GTK+, widgets are double-buffered. It means that an off-screen buffer is
        automatically created to draw on it before the expose event is called and
        it prevents the screen from flickering.
        """
        if self._active:
            self._gc = self.window.new_gc()

            if self._mode == _MODE_NICE_CLOCK:
                self._draw_nice_clock()
            elif self._mode == _MODE_SIMPLE_CLOCK:
                self._draw_simple_clock()
            elif self._mode == _MODE_DIGITAL_CLOCK:
                self._draw_digital_clock()
            else:
                raise ValueError, "Unknown display mode: %d." % self._mode

        return False


    def _draw_markup(self, x, y, markup):
        """Write the markup text given as parameter, centered on (x, y) coordinates.
        The markup must follow Pango markup syntax
        See http://www.pygtk.org/pygtk2reference/pango-markup-language.html
        It allows to specify the fonts, colors and styles and to display rich
        text fully localizable.
        """
        pango_context = self.get_pango_context()
        layout = pango.Layout(pango_context)

        layout.set_markup(markup)
        layout.set_alignment(pango.ALIGN_CENTER)

        x_bearing, y_bearing, width, height = layout.get_pixel_extents()[1][:4]
        self.window.draw_layout(self._gc, int(x - width / 2 - x_bearing), int(y - height / 2 - y_bearing), layout)


    def _draw_digital_clock(self):
        """Draw the digital clock.
        """
        self._draw_time_scale()
        self._draw_time()


    def _draw_time_scale(self):
        """Draw a time scale for digital clock.
        """
        # Draw scales of hours, minutes and seconds, to give the children
        # an appreciation of the time flowing...
        hours_length = 2 * self._radius / 24 * self._time.hour
        minutes_length = 2 * self._radius / 60 * self._time.minute
        seconds_length = 2 * self._radius / 60 * self._time.second

        # Fill background
        self._gc.set_line_attributes(self._line_width, gdk.LINE_SOLID, \
                gdk.CAP_BUTT, gdk.JOIN_BEVEL)
        self._gc.set_foreground(self._COLOR_WHITE)
        self.window.draw_rectangle(self._gc, True, \
                int(self._center_x - 1.1 * self._radius), \
                int(self._center_y - 0.8 * self._radius), \
                int(2.2 * self._radius), \
                int(0.55 * self._radius))

        h = int(0.15 * self._radius)
        x = int(self._center_x - self._radius)

        # Hours scale
        self._gc.set_foreground(self._COLOR_HOURS)
        y = int(self._center_y - 0.75 * self._radius)
        self.window.draw_rectangle(self._gc, True, x, y, hours_length, h)

        # Minutes scale
        self._gc.set_foreground(self._COLOR_MINUTES)
        y = int(self._center_y - 0.60 * self._radius)
        self.window.draw_rectangle(self._gc, True, x, y, minutes_length, h)
        
        # Seconds scale
        self._gc.set_foreground(self._COLOR_SECONDS)
        y = int(self._center_y - 0.45 * self._radius)
        self.window.draw_rectangle(self._gc, True, x, y, seconds_length, h)


    def _draw_time(self):
        """Draw the time in colors (digital display).
        """
        # TRANS: The format used to display the time for digital clock
        # You can add AM/PM indicator or use 12/24 format, for example "%I:%M:%S %p".
        # See http://docs.python.org/lib/module-time.html for
        # available strftime formats
        # If the display of the time is moving horizontally, it means that the glyphs
        # of the digits used in the font don't have the same width. Try to use a
        # Monospace font.
        # xgettext:no-python-format
        markup = _p("Digital Clock", """<markup><span lang="en" font_desc="Sans,Monospace Bold 48"><span foreground="#005FE4">%I</span>:<span foreground="#00B20D">%M</span>:<span foreground="#E6000A">%S</span>%p</span></markup>""")
        # BUG: The following line kills Python 2.5 but is valid in 2.4
        markup_time = self._time.strftime(markup)
        #markup_time = time.strftime(markup)

        self._gc.set_foreground(self._COLOR_BLACK)
        self._draw_markup(self._center_x, int(self._center_y + 0.3 * self._radius), markup_time)



    def _draw_simple_clock(self):
        """Draw the simple clock variants.
        """
        self._draw_simple_background()
        self._draw_numbers()
        self._draw_hands()


    def _draw_simple_background(self):
        """Draw the background of the simple clock.
        The simple clock background is a white disk, with hours and minutes
        ticks, and the hour numbers.
        """
        # Simple clock background
        self._gc.set_foreground(self._COLOR_WHITE)
        self.window.draw_arc(self._gc, True, self._center_x - self._radius, self._center_y - self._radius, 2 * self._radius, 2 * self._radius, 0, 360 * 64)
        self._gc.set_foreground(self.get_style().fg[gtk.STATE_NORMAL])
        self._gc.set_line_attributes(4 * self._line_width, gdk.LINE_SOLID, gdk.CAP_ROUND, gdk.JOIN_ROUND)
        self.window.draw_arc(self._gc, False, self._center_x - self._radius, self._center_y - self._radius, 2 * self._radius, 2 * self._radius, 0, 360 * 64)

        # Clock ticks
        self._gc.set_line_attributes(4 * self._line_width, gdk.LINE_SOLID, gdk.CAP_ROUND, gdk.JOIN_ROUND)
        for i in xrange(60):
            if i % 15 == 0:
                inset = 0.175 * self._radius
            elif i % 5 == 0:
                inset = 0.1 * self._radius
            else:
                inset = 0.05 * self._radius

            self.window.draw_line(self._gc, \
                    int(self._center_x + (self._radius - inset) * math.cos(i * math.pi / 30.0)), \
                    int(self._center_y + (self._radius - inset) * math.sin(i * math.pi / 30.0)), \
                    int(self._center_x + self._radius * math.cos(i * math.pi / 30.0)), \
                    int(self._center_y + self._radius * math.sin(i * math.pi / 30.0)))


    def _draw_nice_background(self):
        """Draw the nice clock background.
        The background has been loaded from the clock.svg file to a pixbuf, and we just
        draw this pixbuf onto the pixmap where we will be drawing the hands.
        """
        # We draw the background from the SVG pixbuf
        self.window.draw_pixbuf(None, self._cache_pixbuf, 0, 0, self._center_x - self._radius, self._center_y - self._radius)


    def _draw_nice_clock(self):
        """Draw the nice clock.
        """
        self._draw_nice_background()
        self._draw_hands()


    def _draw_hands(self):
        """Draw the hands of the analog clocks.
        """
        hours = self._time.hour
        minutes = self._time.minute
        seconds = self._time.second

        # Hour hand:
        # The hour hand is rotated 30 degrees (pi/6 r) per hour +
        # 1/2 a degree (pi/360) per minute
        self._gc.set_foreground(self._COLOR_HOURS)
        self._gc.set_line_attributes(8 * self._line_width, gdk.LINE_SOLID, gdk.CAP_ROUND, gdk.JOIN_ROUND)
        self.window.draw_line(self._gc, self._center_x, self._center_y, \
                int(self._center_x + self._radius * 0.5 * \
                math.sin(math.pi / 6 * hours + math.pi / 360 * minutes)), \
                int(self._center_y + self._radius * 0.5 * \
                - math.cos(math.pi / 6 * hours + math.pi / 360 * minutes)))
    
        # Minute hand:
        # The minute hand is rotated 6 degrees (pi/30 r) per minute
        self._gc.set_foreground(self._COLOR_MINUTES)
        self._gc.set_line_attributes(6 * self._line_width, gdk.LINE_SOLID, gdk.CAP_ROUND, gdk.JOIN_ROUND)
        self.window.draw_line(self._gc, self._center_x, self._center_y, \
                int(self._center_x + self._radius * 0.8 * \
                math.sin(math.pi / 30 * minutes)), \
                int(self._center_y + self._radius * 0.8 * \
                - math.cos(math.pi / 30 * minutes)))
    
        # Seconds hand:
        # Operates identically to the minute hand
        self._gc.set_foreground(self._COLOR_SECONDS)
        self._gc.set_line_attributes(2 * self._line_width, gdk.LINE_SOLID, gdk.CAP_ROUND, gdk.JOIN_ROUND)
        self.window.draw_line(self._gc, self._center_x, self._center_y, \
                int(self._center_x + self._radius * 0.7 * \
                math.sin(math.pi / 30 * seconds)), \
                int(self._center_y + self._radius * 0.7 * \
                - math.cos(math.pi / 30 * seconds)))


    def _draw_numbers(self):
        """Draw the numbers of the hours.
        """
        self._gc.set_foreground(self._COLOR_HOURS)

        for i in xrange(12):
            # TRANS: The format of the font used to print hour numbers, from 1 to 12.
            hour_number = _p("Hour Number", """<markup><span lang="en" font_desc="Sans Bold 20">%d</span></markup>""") % (i + 1)
            self._draw_markup(self._center_x + 0.75 * self._radius * math.cos((i - 2) * math.pi / 6.0), \
                    self._center_y + 0.75 * self._radius * math.sin((i - 2) * math.pi / 6.0), \
                    hour_number)


    def _redraw_canvas(self):
        """Force a redraw of the clock on the screen.
        """
        # If we are attached to a window, redraw ourself.
        if self.window:
            self.queue_draw()
            self.window.process_updates(True)


    def _update_cb(self):
        """Called every seconds to update the time value.
        """
        # update the time and force a redraw of the clock
        self._time = datetime.now()

        gobject.idle_add(self._redraw_canvas)

        # When the minutes change, we raise the 'time_minute' signal. We can't test on
        # 'self._time.second == 0' for instance because gtk timer does not guarantee
        # to call us every seconds.
        if self._old_minute != self._time.minute:
            self.emit("time_minute")
            self._old_minute = self._time.minute

        # Keep running this timer as long as the clock is active (ie. visible)
        return self._active


    def get_time(self):
        """Public access to the time member of the clock face.
        """
        return self._time


    def _get_active(self):
        """Get the activity status of the clock. When active, the clock face redraws
        itself. When inactive, we do nothing to save resources.
        """
        return self._active


    def _set_active(self, active):
        """Set the activity state of the clock face. When Sugar reactivates the clock,
        we start a timer to be called every seconds and update the clock.
        """
        self._active = active

        if active:
            # We must redraw the clock...
            self._update_cb()

            # And update again the clock every seconds.
            gobject.timeout_add(1000, self._update_cb)


    active = property(_get_active, _set_active)


