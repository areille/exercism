class Luhn {
  List<int> toValidIntList(String chain) {
    final list = chain
        .replaceAll(' ', '')
        .split('')
        .map(int.tryParse)
        .toList(growable: false);
    if (list.length <= 1 || list.contains(null)) {
      throw Exception('Bad formatting');
    }
    return list;
  }

  bool valid(String sin) {
    try {
      final list = toValidIntList(sin);
      int sum = list.last;
      for (int i = 0; i < list.length - 1; i++) {
        int value = list[i];
        if (i % 2 == list.length % 2) {
          value *= 2;
          if (value > 9) {
            value -= 9;
          }
        }
        sum += value;
      }
      return sum % 10 == 0;
    } catch (e) {
      return false;
    }
  }
}
