#!/usr/bin/env python3


import logging
import sys
from argparse import ArgumentParser

LOG_FORMAT = (
    '%(asctime)s'
    '\t%(levelname)s'
    '\t%(funcName)s'
    '\t%(message)s'
)

def initialize_logger(logger):
    # the root logger defaults to the WARNING log level.
    # this isn't acceptable as when starting up as debug, all debug messages
    # would be dropped until the root logger is configured. Setting to loglevel
    # to NOTSET causes all messages to be logged.
    logger.setLevel(logging.NOTSET)

    formatter = logging.Formatter(fmt=LOG_FORMAT, datefmt='%H:%M:%S')
    handler = logging.StreamHandler(stream=sys.stderr)
    handler.setFormatter(formatter)
    logger.addHandler(handler)


def setup_logger(name):
    # loggers are organized as a tree, setup handlers on the root
    root_logger = logging.getLogger()
    if not root_logger.hasHandlers():
        initialize_logger(root_logger)

    return logging.getLogger(name)

logger = setup_logger("galaxy")


class Symbol:
    """ :42 """
    __slots__ = ("name",)

    def __init__(self, name):
        self.name = name

    def __repr__(self):
        return f":{repr(self.name)}"

    def __hash__(self):
        return hash((Symbol, self.name))


class VariableRef:
    scope = {}

    __slots__ = ("name",)

    def __init__(self, name):
        self.name = name

    @classmethod
    def set_value(cls, name, value):
        cls.scope[name] = value


nil = tuple()


def car(val):
    assert isinstance(val, tuple)
    return val[0]


def cdr(val):
    assert isinstance(val, tuple)
    return val[1]


class Callable:
    __slots__ = ("func",)

    def __init__(self, func):
        self.func = func

    def __repr__(self):
        return f"<Callable {repr(self.func)}>"

    def __call__(self, value):
        return self.func(value)

    @classmethod
    def wrap(cls, f):
        return cls(f)

def galaxy_callable(e):
    return isinstance(e, Callable)



@Callable.wrap
def apply(f):
    @Callable.wrap
    def apply_val(x):
        return f(x)

    @Callable.wrap
    def apply_tuple_val(x):
        return apply(apply(x)(car(f)))(cdr(f))

    if f is tuple:
        return apply_tuple_val
    return apply_val


def cons(val):
    @Callable.wrap
    def cons_list(l):
        return (val, l)
    return cons_list


keywords = {
    "cons": cons,
    "vec": cons,
    "ap": apply,
    "nil": nil,
}


class ASSIGN_OP():
    pass


def galaxy_lex_word(word):
    if word == '=':
        return ASSIGN_OP

    if word.isnumeric():
        return int(word)

    if word[0] == ":":
        return Symbol(int(word[1:]))

    kw = keywords.get(word, None)
    if kw is not None:
        return kw

    if word.isalpha():
        return Symbol(word)

    raise RuntimeError("invalid token", word)


def galaxy_eval(tokens):
    stack = []
    for token in reversed(tokens):
        if not galaxy_callable(token):
            logger.debug("pushing %s", token)
            stack.append(token)
            continue

        logger.debug("starting to apply %s", token)
        res = token
        while stack:
            arg = stack.pop()
            res = res(arg)
            if not galaxy_callable(res):
                logger.debug("result not galaxy_callable, stopping: %s", res)
                break
        else:
            logger.debug("stopped because of an empty stack, leaving curried: %s", res)

        stack.append(res)

    if len(stack) != 1:
        raise RuntimeError("invalid exit stack state", stack)

    return stack[0]


def galaxy_repl(line_stream):
    for line in line_stream:
        tokens = list(map(galaxy_lex_word, line.split()))
        print(tokens)

        # check and parse assignments
        var_name = None
        if len(tokens) >= 2 and tokens[1] is ASSIGN_OP:
            var_name = tokens[0]
            tokens = tokens[2:]

        eval_result = galaxy_eval(tokens)
        logger.info("line evaluated to %s", eval_result)

        if var_name is not None:
            VariableRef.set_value(var_name, eval_result)
        logger.info("stored into %s", var_name)


def main(args):
    parser = ArgumentParser(description='Evaluate a galaxy script')
    parser.add_argument('script', nargs='?', default="-", help='script to run')

    options = parser.parse_args(args=args)
    if options.script == "-":
        galaxy_repl(sys.stdin)
    else:
        with open(options.script) as fp:
            galaxy_repl(fp)


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
