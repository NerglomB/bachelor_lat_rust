import sympy
from sympy.parsing.sympy_parser import parse_expr

sympy.core.cache.clear_cache()

# Numerische Berechnungen
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

# Variablen zusammenfassen
expr = parse_expr("x+x+2*x")
print(expr)

expr = parse_expr("2*x-x-x+y")
print(expr)

expr = parse_expr("x+x+a*x")
print(expr)

expr = parse_expr("a*x-x-x+y")
print(expr)

expr = parse_expr("x*0")
print(expr)

expr = parse_expr("x*x")
print(expr)

expr = parse_expr("x**2*x")
print(expr)

expr = parse_expr("x**a*x")
print(expr)

expr = parse_expr("x/x")
print(expr)

expr = parse_expr("x**2/x")
print(expr)

expr = parse_expr("x**a/x")
print(expr)

expr = parse_expr("a*x+b*x")
print(expr)

expr = parse_expr("a**x*b**x")
print(expr)

expr = parse_expr("x**a*x**b")
print(expr)

# Terme mit Variablen substituieren
expr = parse_expr("2+x")
print(expr.subs("x", 3))

expr = parse_expr("2+x")
print(expr.subs("x", -3))

expr = parse_expr("2-x")
print(expr.subs("x", -3))

expr = parse_expr("2+x")
print(expr.subs("x", 0.5))

expr = parse_expr("2**x + 2**x")
print(expr.subs("x", 2))

expr = parse_expr("2**x + 2**x")
print(expr.subs("x", -1))

expr = parse_expr("sqrt(x)")
print(expr.subs("x", 3))

expr = parse_expr("sqrt(x)")
print(expr.subs("x", 4))

expr = parse_expr("sqrt(x)")
print(expr.subs("x", 16))

expr = parse_expr("sqrt(x)")
print(expr.subs("x", 17))

# expand
expr = parse_expr("(x + 1)*(x - 2) - (x - 1)*x")
print(sympy.expand(expr))

expr = parse_expr("(a+b)*(c+d)*e")
print(sympy.expand(expr))

expr = parse_expr("(a+b)**3")
print(sympy.expand(expr))

expr = parse_expr("x**(a+b)")
print(sympy.expand(expr))

expr = parse_expr("x**(a*b)")
print(sympy.expand(expr))

expr = parse_expr("(x+y)**a")
print(sympy.expand(expr, power_base=True, force=True))

expr = parse_expr("(x*y)**a")
print(sympy.expand(expr, power_base=True, force=True))

expr = parse_expr("log(x**2*y)")
print(sympy.expand(expr, log=True, force=True))

expr = parse_expr("log(x**a*y)")
print(sympy.expand(expr, log=True, force=True))