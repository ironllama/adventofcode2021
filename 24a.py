# Magic constants derived from the ALU code
# As = [12,10,10,-6,11,-12,11,12,12,-2,-5,-4,-4,-12]
# Bs = [ 6, 2,13, 8,13,  8, 3,11,10, 8,14, 6, 8,  2]
# Cs = [ 1, 1, 1,26, 1, 26, 1, 1, 1,26,26,26,26, 26]

As = [11, 13, 15, -8, 13, 15, -11, -4, -15, 14, 14, -1, -8, -14]
Bs = [6, 14, 14, 10, 9, 12, 8, 13, 12, 6, 9, 15, 4, 10]
Cs = [1, 1, 1, 26, 1, 1, 26, 26, 26, 1, 1, 26, 26, 26]

# to get these, I split the input by "inp w" lines
# and diffed the blocks: the A is the varying x addend,
# the B the varying y addend and the C the varying z divisor
# (apparently always 1 or 26 depending on the sign of A)

import itertools as it

def backward(A, B, C, z2, w):
    """The possible values of z before a single block
    if the final value of z is z2 and w is w"""
    zs = []

        # if ((old_z % 26) + ops[1][digit_pos]) != w {  // x = 1  As
        # if (old_z % 26) + A != w
        #     new_z = 26 * (old_z / 1) + (w + ops[2][digit_pos]);  Bs
            # new_z = 26 * old_z + w + B
            # new_z - w - B = 26 * old_z
            # (new_z - w - B) // 26 = old_z


        # } else {  // x = 0  w = (old_z % 26) + A    OR   if ((old_z % 26) + ops[1][digit_pos]) == w
        #                 w - A = old_z % 26
        # (w - A) >= 0 && (w - A) < 26 = old_z
        #     new_z = old_z / 26;
        #     new_z * 26 = old_z


    # temp_two = (z2 * 26)
    # if (temp_two % 26) + A == w:
    #     zs.append(temp_two)
    
    # temp_one = (z2 - w - B) // 26
    # if (temp_one % 26):
    #     zs.append(temp_one)


    if C == 26:
        zs.append((w - A) + (26 * z2))
    else:
        # zs.append((z2 - w - B) // 26)
        # zs.append(z2 // 26)
        # res = (z2 - w - B)
        # if res % 26 == 0:
        #     zs.append(res//26)
        res = (z2 - w - B) // 26
        print("RES:", res)
        if res % 26 == 0:
            zs.append(res)


    # x = z2 - w - B   # x is the previous?current value of z --- Extract remainder before popping?
    # if x % 26 == 0:  # Make sure the number is a multiple of 26??
    #     zs.append(x//26)  # Previous push? Popping from stack.

    # # if 0 <= w-A < 26:
    # # if (w - A) > 0:
    # if C == 26:
    #     zs.append((w - A) + (26 * z2))  # Previous pop? Pushing back into stack.

    return zs

def solve(part,As=As,Bs=Bs,Cs=Cs):
    zs = {0}
    result = {}
    if part == 1:
        ws = range(1,10)
    else:
        ws = range(9,0,-1)
    for A,B,C in zip(As[::-1],Bs[::-1],Cs[::-1]):
        # print("POSSIBLES:", zs)
        print(len(zs))
        newzs = set()
        for w,z in it.product(ws,zs):
            z0s = backward(A,B,C,z,w)
            for z0 in z0s:
                newzs.add(z0)
                result[z0] = (w,) + result.get(z, ())
        zs = newzs
    return ''.join(str(digit) for digit in result[0])

print(solve(1))
# print(solve(2))