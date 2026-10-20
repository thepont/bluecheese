class Remote {
  int first;
  int second;
  int third;
}

class FeatureEnvy {
  int usesRemoteFields(Remote remote) {
    return remote.first + remote.second + remote.third;
  }

  int usesOnlyOneRemoteField(Remote remote) {
    return remote.first + remote.first;
  }
}
