import sympy
from sympy.parsing.sympy_parser import parse_expr

sympy.core.cache.clear_cache()

expr = parse_expr("1+2+3+4-5")
print(expr)

expr = parse_expr("1*2*3*4")
print(expr)

expr = parse_expr("1*2*3*4*0")
print(expr)

expr = parse_expr("1*2*3*4*(-1)")
print(expr)

expr = parse_expr("2+3*4+5")
print(expr)

expr = parse_expr("(2+3)*(4+5)")
print(expr)

expr = parse_expr("2**(3**4)")
print(expr)

expr = parse_expr("0.1+0.2")
print(expr)

expr = parse_expr("3**-1 + 3**-1")
print(expr)

expr = parse_expr("3**-1 * 3**-1")
print(expr)

expr = parse_expr("sqrt(2)")
print(expr)

expr = parse_expr("sqrt(2) + sqrt(2)")
print(expr)

expr = parse_expr("sqrt(2) * sqrt(2)")
print(expr)