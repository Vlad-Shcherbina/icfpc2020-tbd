def str2bits(s):
    for c in s:
        assert c in '01'
        yield int(c)


def parse_cons(it):
    return (parse(it), parse(it))


def parse_number(it):
    length = 0
    while next(it):
        length += 4
    bits = [next(it) for _ in range(length)]
    return ''.join(str(b) for b in bits)


def parse(it):
    b0b1 = next(it), next(it)
    if b0b1 == (0, 0):
        return None
    elif b0b1 == (1, 1):
        return parse_cons(it)
    elif b0b1 == (0, 1):
        return parse_number(it)
    else:
        return '-' + parse_number(it)

def parse_str(s):
    it = str2bits(s)
    res = parse(it)
    assert next(it, None) is None
    return res

s = '''122 410
203 203
77 329
 456 201
 384 192
 34 160
 437 497
 428 107'''

def b8(x):
    s = '{:09b}'.format(int(x))
    print(s[0:3], s[3:6], s[6:9], sep='\n')
    print()

for line in s.split('\n'):
    a, b = line.split()
    b8(a)
    b8(b)
    print('---------')

