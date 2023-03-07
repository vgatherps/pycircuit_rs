RUST_NAME_SEP = "__sep__"


def separated_names(*args):
    return RUST_NAME_SEP.join(args)
