package edu.northeastern;

import spoon.Launcher;
import spoon.reflect.declaration.CtMethod;
import spoon.reflect.declaration.CtType;
import edu.northeastern.utils.Metrics;

/**
 * Debug probe: dump per-method LLOC and cyclomatic complexity exactly as the
 * reference Metrics class computes them, so the Rust port can be diffed
 * against it. Not part of the shipped tool.
 */
public class Probe {
    public static void main(String[] args) {
        Launcher launcher = new Launcher();
        launcher.addInputResource(args[0]);
        launcher.getEnvironment().setComplianceLevel(17);
        launcher.getEnvironment().setNoClasspath(true);
        boolean comments = args.length > 1 && args[1].equals("comments");
        // Leave Spoon's default untouched unless explicitly asked.
        if (args.length > 1) {
            launcher.getEnvironment().setCommentEnabled(comments);
        }
        launcher.buildModel();

        for (CtType<?> type : launcher.getModel().getAllTypes()) {
            if (!type.getPosition().isValidPosition()) continue;
            String file = type.getPosition().getFile() != null
                    ? type.getPosition().getFile().getPath() : "?";
            for (CtMethod<?> m : type.getMethods()) {
                if (m.getBody() == null) continue;
                System.out.println(String.join("\t",
                        file,
                        type.getSimpleName(),
                        m.getSimpleName(),
                        String.valueOf(m.getPosition().getLine()),
                        String.valueOf(m.getBody().getPosition().getLine()),
                        String.valueOf(Metrics.calculateLLOC(m)),
                        String.valueOf(Metrics.calculateCyclomaticComplexity(m))));
            }
        }
    }
}
