class State(object):
  """state of joint"""
  # offer_al does not implictly return stationary state
  def __init__(self, states, initial, offer_stationary = False, offer_all = False):
    super(State, self).__init__()
    self.states = states
    self.initial = initial
    self.offer_stationary = offer_stationary
    self.offer_all = offer_all

  def reset():
    self.curr = self.initial

  def get_value():
    return self.states[self.curr]

  def get():
    return self.curr

  def set(state_index):
    self.curr = state_index

  def possible():
    states = []
    if offer_stationary:
      states = [self.curr]
    if offer_all:
      states += range(0, self.curr) + range(self.curr + 1, len(self.states))
    elif self.curr == 0:
      states += [1]
    elif self.curr == (len(self.curr) - 1):
      states += [self.curr - 1]
    else:
      states += [self.curr - 1, self.curr + 1]
    return states

class StateConfig(object):
  """config for state type"""
  def __init__(self, degrees, initial = 0, offer_stationary = False, offer_all = False):
    super(StateConfig, self).__init__()
    self.degrees = degrees
    self.initial = initial
    self.offer_stationary = offer_stationary
    self.offer_all = offer_all

  def create_state():
    return State(self.degrees, self.initial, offer_stationary=self.offer_stationary, offer_all=self.offer_all)
                                  
