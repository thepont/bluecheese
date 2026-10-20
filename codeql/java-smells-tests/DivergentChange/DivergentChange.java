class DivergentSwitches {
  int first(int code) {
    switch (code) {
      case 1: return 1;
      default: return 0;
    }
  }

  int second(int code) {
    switch (code) {
      case 2: return 2;
      default: return 0;
    }
  }

  int third(int code) {
    switch (code) {
      case 3: return 3;
      default: return 0;
    }
  }
}

class TwoSwitches {
  int first(int code) {
    switch (code) { case 1: return 1; default: return 0; }
  }

  int second(int code) {
    switch (code) { case 2: return 2; default: return 0; }
  }
}
