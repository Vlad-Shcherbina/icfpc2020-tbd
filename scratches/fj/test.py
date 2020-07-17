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

print(parse_str('110110000111011111100001001111110101000000'))
print(parse_str('110110000111011111100001001111110100110000'))

n1 = 0b00010011111101010000
n2 = 0b00010011111101001100
print(n1 - n2)
print(n2 * 10 / 4 / 60 / 60)

print((86400 - 81744) / 60)

