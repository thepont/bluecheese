package switchstmts.case3;

import java.util.*;

class CosmeticSampleGood {
  private final String sampleId;
  private final switchstmts.case3.TestTypeGood testType;
  private final Map<String, Double> results = new LinkedHashMap<>();

  public CosmeticSampleGood(String sampleId, switchstmts.case3.TestTypeGood testType) {
    this.sampleId = sampleId;
    this.testType = testType;
  }

  public String getSampleId() {
    return sampleId;
  }

  public switchstmts.case3.TestTypeGood getTestType() {
    return testType;
  }

  public void recordResult(String metric, double value) {
    results.put(metric, value);
  }

  public Map<String, Double> getResults() {
    return Collections.unmodifiableMap(results);
  }
}

class CosmeticLabGood {
  private final boolean isCertified;
  private final Map<switchstmts.case3.TestTypeGood, TestExecutor> executors = new EnumMap<>(switchstmts.case3.TestTypeGood.class);

  public CosmeticLabGood(boolean isCertified) {
    this.isCertified = isCertified;
    executors.put(switchstmts.case3.TestTypeGood.PH_BALANCE, new PhBalanceExecutor());
    executors.put(switchstmts.case3.TestTypeGood.OIL_CONTENT, new OilContentExecutor());
    executors.put(switchstmts.case3.TestTypeGood.MOISTURE_LEVEL, new MoistureLevelExecutor());
    executors.put(switchstmts.case3.TestTypeGood.UV_PROTECTION, new UvProtectionExecutor());
    executors.put(switchstmts.case3.TestTypeGood.STABILITY_UNDER_HEAT, new HeatStabilityExecutor());
    executors.put(switchstmts.case3.TestTypeGood.FRAGRANCE_INTENSITY, new FragranceIntensityExecutor());
    executors.put(switchstmts.case3.TestTypeGood.COLOR_UNIFORMITY, new ColorUniformityExecutor());
    executors.put(switchstmts.case3.TestTypeGood.PARTICLE_SIZE_DISTRIBUTION, new ParticleSizeDistributionExecutor());
  }

  public void performTest(switchstmts.case3.CosmeticSampleGood sample) {
    if (!isCertified) {
      System.out.println("Lab is not certified to run tests.");
      return;
    }

    System.out.println("Running " + sample.getTestType()
            + " on sample " + sample.getSampleId());

    TestExecutor executor = executors.get(sample.getTestType());
    if (executor != null) {
      executor.execute(sample);
    } else {
      System.out.println("Unknown test type: " + sample.getTestType());
    }

    System.out.println("Results: " + sample.getResults());
  }
}

interface TestExecutor {
  void execute(switchstmts.case3.CosmeticSampleGood sample);
}

class PhBalanceExecutor implements TestExecutor {
  @Override
  public void execute(switchstmts.case3.CosmeticSampleGood sample) {
    Random rand = new Random(sample.getSampleId().hashCode() ^ switchstmts.case3.TestConstantsGood.PH_SEED_OFFSET);
    double reading = switchstmts.case3.TestConstantsGood.PH_MIN + rand.nextDouble() * switchstmts.case3.TestConstantsGood.PH_RANGE;
    double value = Math.round(reading * switchstmts.case3.TestConstantsGood.PH_PRECISION) / switchstmts.case3.TestConstantsGood.PH_PRECISION;
    sample.recordResult("pH", value);
  }
}

class OilContentExecutor implements TestExecutor {
  @Override
  public void execute(switchstmts.case3.CosmeticSampleGood sample) {
    Random rand = new Random(sample.getSampleId().hashCode() ^ switchstmts.case3.TestConstantsGood.OIL_SEED_OFFSET);
    double perc = switchstmts.case3.TestConstantsGood.OIL_MIN + rand.nextDouble() * switchstmts.case3.TestConstantsGood.OIL_RANGE;
    double value = Math.round(perc * switchstmts.case3.TestConstantsGood.OIL_PRECISION) / switchstmts.case3.TestConstantsGood.OIL_PRECISION;
    sample.recordResult("Oil %", value);
  }
}

class MoistureLevelExecutor implements TestExecutor {
  @Override
  public void execute(switchstmts.case3.CosmeticSampleGood sample) {
    Random rand = new Random(sample.getSampleId().hashCode() ^ switchstmts.case3.TestConstantsGood.MOISTURE_SEED_OFFSET);
    double moisture = switchstmts.case3.TestConstantsGood.MOISTURE_MIN + rand.nextDouble() * switchstmts.case3.TestConstantsGood.MOISTURE_RANGE;
    double value = Math.round(moisture * switchstmts.case3.TestConstantsGood.MOISTURE_PRECISION) / switchstmts.case3.TestConstantsGood.MOISTURE_PRECISION;
    sample.recordResult("Moisture %", value);
  }
}

class UvProtectionExecutor implements TestExecutor {
  @Override
  public void execute(switchstmts.case3.CosmeticSampleGood sample) {
    Random rand = new Random(sample.getSampleId().hashCode() ^ switchstmts.case3.TestConstantsGood.UV_SEED_OFFSET);
    int spf = switchstmts.case3.TestConstantsGood.SPF_MIN + rand.nextInt(switchstmts.case3.TestConstantsGood.SPF_RANGE);
    sample.recordResult("SPF Value", spf);
  }
}

class HeatStabilityExecutor implements TestExecutor {
  @Override
  public void execute(switchstmts.case3.CosmeticSampleGood sample) {
    Random rand = new Random(sample.getSampleId().hashCode() ^ switchstmts.case3.TestConstantsGood.HEAT_SEED_OFFSET);
    double degradation = rand.nextDouble() * switchstmts.case3.TestConstantsGood.HEAT_MAX_DEGRADATION;
    double value = Math.round(degradation * switchstmts.case3.TestConstantsGood.HEAT_PRECISION) / switchstmts.case3.TestConstantsGood.HEAT_PRECISION;
    sample.recordResult("Degradation %", value);
  }
}

class FragranceIntensityExecutor implements TestExecutor {
  @Override
  public void execute(switchstmts.case3.CosmeticSampleGood sample) {
    Random rand = new Random(sample.getSampleId().hashCode() ^ switchstmts.case3.TestConstantsGood.FRAGRANCE_SEED_OFFSET);
    double score = switchstmts.case3.TestConstantsGood.FRAGRANCE_MIN + rand.nextDouble() * switchstmts.case3.TestConstantsGood.FRAGRANCE_RANGE;
    double value = Math.round(score * switchstmts.case3.TestConstantsGood.FRAGRANCE_PRECISION) / switchstmts.case3.TestConstantsGood.FRAGRANCE_PRECISION;
    sample.recordResult("Scent Score", value);
  }
}

class ColorUniformityExecutor implements TestExecutor {
  @Override
  public void execute(switchstmts.case3.CosmeticSampleGood sample) {
    Random rand = new Random(sample.getSampleId().hashCode() ^ switchstmts.case3.TestConstantsGood.COLOR_SEED_OFFSET);
    double deltaE = switchstmts.case3.TestConstantsGood.COLOR_MIN + rand.nextDouble() * switchstmts.case3.TestConstantsGood.COLOR_RANGE;
    double value = Math.round(deltaE * switchstmts.case3.TestConstantsGood.COLOR_PRECISION) / switchstmts.case3.TestConstantsGood.COLOR_PRECISION;
    sample.recordResult("ΔE", value);
  }
}

class ParticleSizeDistributionExecutor implements TestExecutor {
  @Override
  public void execute(switchstmts.case3.CosmeticSampleGood sample) {
    Random rand = new Random(sample.getSampleId().hashCode() ^ switchstmts.case3.TestConstantsGood.PARTICLE_SEED_OFFSET);
    double size = switchstmts.case3.TestConstantsGood.PARTICLE_MIN + rand.nextDouble() * switchstmts.case3.TestConstantsGood.PARTICLE_RANGE;
    double value = Math.round(size * switchstmts.case3.TestConstantsGood.PARTICLE_PRECISION) / switchstmts.case3.TestConstantsGood.PARTICLE_PRECISION;
    sample.recordResult("Avg Particle Size (µm)", value);
  }
}

enum TestTypeGood {
  PH_BALANCE,
  OIL_CONTENT,
  MOISTURE_LEVEL,
  UV_PROTECTION,
  STABILITY_UNDER_HEAT,
  FRAGRANCE_INTENSITY,
  COLOR_UNIFORMITY,
  PARTICLE_SIZE_DISTRIBUTION
}

final class TestConstantsGood {
  private TestConstantsGood() {}
  static final long PH_SEED_OFFSET = 0L;
  static final double PH_MIN = 4.5;
  static final double PH_RANGE = 3.5;
  static final double PH_PRECISION = 100.0;

  static final long OIL_SEED_OFFSET = 0xDEADBEEFL;
  static final double OIL_MIN = 5.0;
  static final double OIL_RANGE = 15.0;
  static final double OIL_PRECISION = 10.0;

  static final long MOISTURE_SEED_OFFSET = 0xCAFEBABEL;
  static final double MOISTURE_MIN = 3.0;
  static final double MOISTURE_RANGE = 9.0;
  static final double MOISTURE_PRECISION = 10.0;

  static final long UV_SEED_OFFSET = 0x12345678L;
  static final int SPF_MIN = 15;
  static final int SPF_RANGE = 36;

  static final long HEAT_SEED_OFFSET = 0x87654321L;
  static final double HEAT_MAX_DEGRADATION = 5.0;
  static final double HEAT_PRECISION = 100.0;

  static final long FRAGRANCE_SEED_OFFSET = 0xABCDEF01L;
  static final double FRAGRANCE_MIN = 1.0;
  static final double FRAGRANCE_RANGE = 9.0;
  static final double FRAGRANCE_PRECISION = 10.0;

  static final long COLOR_SEED_OFFSET = 0x10FEDCBAL;
  static final double COLOR_MIN = 0.5;
  static final double COLOR_RANGE = 2.5;
  static final double COLOR_PRECISION = 100.0;

  static final long PARTICLE_SEED_OFFSET = 0x0FEDCBA1L;
  static final double PARTICLE_MIN = 0.1;
  static final double PARTICLE_RANGE = 4.9;
  static final double PARTICLE_PRECISION = 100.0;
}