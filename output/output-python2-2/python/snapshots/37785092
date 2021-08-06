from Tokens import *
from Scanners import CDirectoryScanner

from TreeNode import *



class CParser(object):
    def __init__(self, scanner, firsts):
        self.scanner = scanner
        self.grammar = {}
        self.stack = ['']
        self.terminals = []
        self.firsts = {}
        self.rules = {}
        self.actions = {}
        
        self.firsts = firsts
        
        
    def AddAction(self, neterminal, sthing):
        pass
        
    def AddTerminal(self, terminal):
        self.terminals.append(terminal)
        
    def AddRule(self, topstack, first, follow):
        self.rules[(topstack, first)] = [i for i in reversed(follow)]
        
    def __CreateNode(self, pointer, symbol, lexem):
        type = self.actions[symbol].GetSymbol()[0]
        if type == 'element':
            node = CElementNode(self.actions[symbol], lexem)
        elif type == 'property':
            node = CPropertyNode(self.actions[symbol], lexem)
        elif type == 'attribute':
            node = CAttributeNode(self.actions[symbol], lexem)
        elif type == 'connection':
            node = CConnectionNode(self.actions[symbol], lexem)
        else:
            node = CNode(self.actions[symbol], lexem)
        node.AddValue(lexem)
        pointer.AddChild(node)
        pointer = node
        return pointer
        
    def Execute(self, root):
        pointer = root
        token, lexem, pos = self.scanner.token(-1, self.firsts[''])
        i = 0
        while True:
            if self.stack[-1] is None:
                i -= 1
                self.stack.pop()
                pointer = pointer.GetParent()
                if self.stack[-1] is not None:
                    token, lexem, pos = self.scanner.token(-1, self.firsts[self.stack[-1]])
            elif self.stack[-1] in self.terminals:
                if self.stack[-1] == token:
                    top = self.stack.pop()
                    if not self.stack:
                        break;
                    if self.stack[-1] is None:
                        continue
                    token, lexem, pos = self.scanner.token(-1, self.firsts[self.stack[-1]])
                    pointer.AddValue(lexem)
                else:
                    return
            else:
                if (self.stack[-1], token) in self.rules:
                    top = self.stack.pop()
                    if top in self.actions and token != 'epsilon':
                        #~ self.stack.extend([None]+self.rules[(top, token)])
                        if self.actions[top].IsLoop():
                            self.stack.extend(self.rules[(top, token)][0:1]+[None]+self.rules[(top, token)][1:])
                        else:
                            self.stack.extend([None]+self.rules[(top, token)])
                        i = i+1
                        pointer = self.__CreateNode(pointer, top, lexem)
                    else:
                        self.stack.extend(self.rules[(top, token)])
                else:
                    return
        return root
        
def build_parser(language, dir, node):
    tokens = [CEodToken(), CEofToken(), CWhitespaceToken(), CEpsilonToken()]
    ignore = [CWhitespaceToken()]
    rules = {}
    actions = {}
    
    if language.GetIndents():
        tokens.extend((CBrToken(True), CIndentToken(), CDedentToken()))
    else:
        tokens.extend((CBrToken(True), ))
        ignore.extend((CBrToken(True), ))
    for terminal, regexp, type in language.GetTokens():
        if type == "text":
            tokens.append(CStringToken(terminal, regexp))
        elif type == "file":
            tokens.append(CFileToken(terminal, regexp))
        elif type == "dir":
            tokens.append(CDirToken(terminal, regexp))
    
    for element in language.GetWalk():
        for nt, action in element.GetAction():
            actions[nt] = action
            
            
    scanner = CDirectoryScanner(tokens, [token.GetTerminal() for token in ignore], dir)
    terminals = [token.GetTerminal() for token in tokens] + ['epsilon']
    rules = {}
    for symbol, rule in language.GetRules():
        if symbol not in rules:
            rules[symbol] = []
        if rule == []:
            rule = ['epsilon']
        rules[symbol].append(rule)    
    
    LL = {}
    
    def FIRST(symbol, LL):
        result = set()
        for rule in rules[symbol]:
            tmp = set()
            while rule:
                if rule[0] in terminals:
                    tmp.add(rule[0])
                    LL[(symbol, rule[0])] = rule 
                else:
                    for frs in FIRST(rule[0], LL):
                        if frs != 'epsilon':
                            LL[(symbol, frs)] = rule
                        tmp.add(frs)
                    
                rule = rule[1:]
                if 'epsilon' in tmp:
                    if rule:
                        tmp.discard('epsilon')
                else:
                    break
            result |= tmp
        return result
    firsts = {}
    for symbol, rls in sorted(rules.items()):
        frsts = FIRST(symbol, LL)
        firsts[symbol] = frsts
    for t in terminals:
        firsts[t] = set([t])

    parser = CParser(scanner, firsts)
    parser.actions = actions
    for term in terminals:
        parser.AddTerminal(term)
    for (top, first), rule in sorted(LL.items()):
        parser.AddRule(top, first, rule)
    
    root = CNode(None, None)
    root.SetProjectNode(node)
    root = parser.Execute(root)
    
    return root
        