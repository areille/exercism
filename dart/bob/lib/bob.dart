class Bob {
  String response(String arg) {
    final isQuestion = arg.trim().endsWith('?');
    final containsCharacter = RegExp(r'[a-zA-Z]+').hasMatch(arg);
    final isUpper = arg.toUpperCase() == arg;
    if (isQuestion) {
      if (isUpper && containsCharacter) {
        return 'Calm down, I know what I\'m doing!';
      }
      return 'Sure.';
    }
    if (isUpper && containsCharacter) return 'Whoa, chill out!';
    if (arg.trim() == '') return 'Fine. Be that way!';
    return 'Whatever.';
  }
}
