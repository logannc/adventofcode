import sys, re

lines = [re.split("[\\s=;,]+", x) for x in sys.stdin.read().splitlines()]
print(lines[0])
G = {x[1]: set(x[10:]) for x in lines}
F = {x[1]: int(x[5]) for x in lines if int(x[5]) != 0}
I = {x: 1 << i for i, x in enumerate(F)}
T = {x: {y: 1 if y in G[x] else float("+inf") for y in G} for x in G}
for k in T:
    for i in T:
        for j in T:
            T[i][j] = min(T[i][j], T[i][k] + T[k][j])

global c
c = 0
global d
d = 0


def visit(v, budget, state, flow, answer):
    global c
    global d
    c += 1
    if state in answer:
        d += 1
    answer[state] = max(answer.get(state, 0), flow)
    for u in F:
        newbudget = budget - T[v][u] - 1
        if I[u] & state or newbudget <= 0:
            continue
        visit(u, newbudget, state | I[u], flow + newbudget * F[u], answer)
    return answer


# total1 = max(visit("AA", 30, 0, 0, {}).values())
visited2 = visit("AA", 26, 0, 0, {})
print(len(visited2))
mv, mk = 0, None
for k1, v1 in visited2.items():
    for k2, v2 in visited2.items():
        if not k1 & k2:
            if v1 + v2 > mv:
                mv = v1 + v2
                mk = (k1, k2)
print(len(F))
print(c, d)
print(bin(mk[0]), len(bin(mk[0])[2:]))
print(bin(mk[1]), len(bin(mk[1])[2:]))
total2 = max(
    v1 + v2 for k1, v1 in visited2.items() for k2, v2 in visited2.items() if not k1 & k2
)
print(total2)
