def pesquisa_binaria (lista, item): 
  baixo = 0
  alto = len(lista) - 1
  lista.sort()
  print('list sort', lista)
  while baixo <= alto:
    meio = (baixo + alto) // 2
    chute = lista[meio]
    if chute == item:
      return meio
    if chute > item:
      alto = meio - 1
    else:
      baixo = meio + 1
  return None


minha_lista = [1, 3 ,3 ,4 ,5, 2, 9]
print('minha lista', minha_lista)
print(pesquisa_binaria(minha_lista, 9))
