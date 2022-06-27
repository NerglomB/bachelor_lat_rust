import sympy

# sympy.core.cache.clear_cache()

a, b, c = sympy.symbols('a b c')
expr = a + b + c
expr_list = []

for i_a in range(0, 50):
  for i_b in range(0, 50):
    for i_c in range(0, 50):
      expr_list.append(expr.subs([(a, i_a), (b, i_b), (c, i_c)]))