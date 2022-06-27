import sympy
from sympy.parsing.sympy_parser import parse_expr

# sympy.core.cache.clear_cache()

# Numerische Berechnungen
expr = parse_expr("1+2+3+4-5")

expr = parse_expr("1*2*3*4")

expr = parse_expr("1*2*3*4*0")

expr = parse_expr("1*2*3*4*(-1)")

expr = parse_expr("2+3*4+5")

expr = parse_expr("(2+3)*(4+5)")

expr = parse_expr("2**(3**4)")

expr = parse_expr("0.1+0.2")

expr = parse_expr("3**-1 + 3**-1")

expr = parse_expr("3**-1 * 3**-1")

expr = parse_expr("sqrt(2)")

expr = parse_expr("sqrt(2) + sqrt(2)")

expr = parse_expr("sqrt(2) * sqrt(2)")

# Variablen zusammenfassen
expr = parse_expr("x+x+2*x")

expr = parse_expr("2*x-x-x+y")

expr = parse_expr("x+x+a*x")

expr = parse_expr("a*x-x-x+y")

expr = parse_expr("x*0")

expr = parse_expr("x*x")

expr = parse_expr("x**2*x")

expr = parse_expr("x**a*x")

expr = parse_expr("x/x")

expr = parse_expr("x**2/x")

expr = parse_expr("x**a/x")

expr = parse_expr("a*x+b*x")

expr = parse_expr("a**x*b**x")

expr = parse_expr("x**a*x**b")

# Terme mit Variablen substituieren
expr = parse_expr("2+x")
expr.subs("x", 3)

expr = parse_expr("2+x")
expr.subs("x", -3)

expr = parse_expr("2-x")
expr.subs("x", -3)

expr = parse_expr("2+x")
expr.subs("x", 0.5)

expr = parse_expr("2**x + 2**x")
expr.subs("x", 2)

expr = parse_expr("2**x + 2**x")
expr.subs("x", -1)

expr = parse_expr("sqrt(x)")
expr.subs("x", 3)

expr = parse_expr("sqrt(x)")
expr.subs("x", 4)

expr = parse_expr("sqrt(x)")
expr.subs("x", 16)

expr = parse_expr("sqrt(x)")
expr.subs("x", 17)

# expand
expr = parse_expr("(x + 1)*(x - 2) - (x - 1)*x")
sympy.expand(expr)

expr = parse_expr("(a+b)*(c+d)*e")
sympy.expand(expr)

expr = parse_expr("(a+b)**3")
sympy.expand(expr)

expr = parse_expr("x**(a+b)")
sympy.expand(expr)

expr = parse_expr("x**(a*b)")
sympy.expand(expr)

expr = parse_expr("(x+y)**a")
sympy.expand(expr, power_base=True, force=True)

expr = parse_expr("(x*y)**a")
sympy.expand(expr, power_base=True, force=True)

expr = parse_expr("log(x**2*y)")
sympy.expand(expr, log=True, force=True)

expr = parse_expr("log(x**a*y)")
sympy.expand(expr, log=True, force=True)

expr = parse_expr("(x**2 + x)/x")
sympy.simplify(expr)

expr = parse_expr("(x**2 + x)*(y**2 + y)/x/y")
sympy.simplify(expr)

expr = parse_expr("(x**2 + x)*(y**2 + y)/x/a")
sympy.simplify(expr)

expr = parse_expr("(x**3 + x**2 + x)*(x**2 + x)/x")
sympy.simplify(expr)