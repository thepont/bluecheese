package edu.northeastern;

import spoon.Launcher;
import spoon.reflect.code.CtBlock;
import spoon.reflect.code.CtStatement;
import spoon.reflect.declaration.CtMethod;
import spoon.reflect.declaration.CtType;
import spoon.reflect.visitor.filter.TypeFilter;

/** Debug probe: list the CtStatement elements Spoon counts for LLOC. */
public class ProbeStatements {
    public static void main(String[] a) {
        Launcher l = new Launcher();
        l.addInputResource(a[0]);
        l.getEnvironment().setComplianceLevel(17);
        l.getEnvironment().setNoClasspath(true);
        l.getEnvironment().setCommentEnabled(true);
        l.buildModel();
        String want = a.length > 1 ? a[1] : null;
        for (CtType<?> t : l.getModel().getAllTypes()) {
            if (!t.getPosition().isValidPosition()) continue;
            for (CtMethod<?> m : t.getMethods()) {
                if (m.getBody() == null) continue;
                if (want != null && !m.getSimpleName().equals(want)) continue;
                System.out.println("METHOD " + t.getSimpleName() + "." + m.getSimpleName()
                        + " lloc=" + edu.northeastern.utils.Metrics.calculateLLOC(m)
                        + " cc=" + edu.northeastern.utils.Metrics.calculateCyclomaticComplexity(m));
                for (CtStatement s : m.getBody().getElements(new TypeFilter<>(CtStatement.class))) {
                    if (s.isImplicit() || s instanceof CtBlock) continue;
                    String txt = s.toString().replace("\n", " ");
                    if (txt.length() > 70) txt = txt.substring(0, 70);
                    System.out.println("   " + s.getClass().getSimpleName()
                            + " @" + s.getPosition().getLine() + " :: " + txt);
                }
            }
        }
    }
}
