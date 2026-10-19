package parallelhierarchy.case2;

import java.util.Objects;

import java.util.EnumMap;
import java.util.Map;
import java.util.function.Consumer;

enum OperatingSystemGood {
  IOS, ANDROID
}

class PlatformDetectorGood {
  public static parallelhierarchy.case2.OperatingSystemGood detectPlatform() {
    String os = System.getProperty("os.name", "").toLowerCase();
    if (os.contains("mac") || os.contains("ios")) {
      return parallelhierarchy.case2.OperatingSystemGood.IOS;
    } else {
      return parallelhierarchy.case2.OperatingSystemGood.ANDROID;
    }
  }

  public static parallelhierarchy.case2.UIFactoryGood getFactory() {
    return new parallelhierarchy.case2.PlatformUIFactoryGood();
  }
}

abstract class AbstractAlertButtonGood {
  abstract void render();
}

class PlatformAlertButton extends parallelhierarchy.case2.AbstractAlertButtonGood {
  private final parallelhierarchy.case2.OperatingSystemGood operatingSystem;

  private static final Map<parallelhierarchy.case2.OperatingSystemGood, Runnable> RENDERERS = new EnumMap<>(parallelhierarchy.case2.OperatingSystemGood.class);
  static {
    RENDERERS.put(parallelhierarchy.case2.OperatingSystemGood.IOS, () -> System.out.println("🐦 Rendering iOS‑style button"));
    RENDERERS.put(parallelhierarchy.case2.OperatingSystemGood.ANDROID, () -> System.out.println("🤖 Rendering Android‑style button"));
  }

  public PlatformAlertButton() {
    this(parallelhierarchy.case2.PlatformDetectorGood.detectPlatform());
  }

  public PlatformAlertButton(parallelhierarchy.case2.OperatingSystemGood operatingSystem) {
    this.operatingSystem = Objects.requireNonNull(operatingSystem);
  }

  @Override
  void render() {
    Runnable renderer = RENDERERS.get(operatingSystem);
    if (renderer == null) {
      throw new IllegalStateException("Unknown operatingSystem: " + operatingSystem);
    }
    renderer.run();
  }
}

abstract class AbstractAlertDialogGood {
  abstract void show(String message);
}

class PlatformAlertDialogGood extends parallelhierarchy.case2.AbstractAlertDialogGood {
  private final parallelhierarchy.case2.OperatingSystemGood operatingSystem;

  private static final Map<parallelhierarchy.case2.OperatingSystemGood, Consumer<String>> DISPLAYERS = new EnumMap<>(parallelhierarchy.case2.OperatingSystemGood.class);
  static {
    DISPLAYERS.put(parallelhierarchy.case2.OperatingSystemGood.IOS, message -> System.out.println("🐦 iOS Alert: " + message));
    DISPLAYERS.put(parallelhierarchy.case2.OperatingSystemGood.ANDROID, message -> System.out.println("🤖 Android Alert: " + message));
  }

  public PlatformAlertDialogGood() {
    this(parallelhierarchy.case2.PlatformDetectorGood.detectPlatform());
  }

  public PlatformAlertDialogGood(parallelhierarchy.case2.OperatingSystemGood operatingSystem) {
    this.operatingSystem = Objects.requireNonNull(operatingSystem);
  }

  @Override
  void show(String message) {
    Consumer<String> displayer = DISPLAYERS.get(operatingSystem);
    if (displayer == null) {
      throw new IllegalStateException("Unknown operatingSystem: " + operatingSystem);
    }
    displayer.accept(message);
  }
}

interface UIFactoryGood {
  parallelhierarchy.case2.AbstractAlertButtonGood createButton();
  parallelhierarchy.case2.AbstractAlertDialogGood createAlertDialog();
}

class PlatformUIFactoryGood implements parallelhierarchy.case2.UIFactoryGood {
  private final parallelhierarchy.case2.OperatingSystemGood operatingSystem;

  public PlatformUIFactoryGood() {
    this(parallelhierarchy.case2.PlatformDetectorGood.detectPlatform());
  }

  public PlatformUIFactoryGood(parallelhierarchy.case2.OperatingSystemGood operatingSystem) {
    this.operatingSystem = Objects.requireNonNull(operatingSystem);
  }

  @Override
  public parallelhierarchy.case2.AbstractAlertButtonGood createButton() {
    return new PlatformAlertButton(operatingSystem);
  }

  @Override
  public parallelhierarchy.case2.AbstractAlertDialogGood createAlertDialog() {
    return new parallelhierarchy.case2.PlatformAlertDialogGood(operatingSystem);
  }
}