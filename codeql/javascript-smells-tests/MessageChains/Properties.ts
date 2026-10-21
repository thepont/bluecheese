function property(root) { return root.a.b.c; }
function optional(root) { return root?.a?.b?.c; }
function mixed(root) { return root.getA().b.c; }
function short(root) { return root.a.b; }
function fluent(root) { return root.a().b().c(); }
function interrupted(root) { return root.getA().save().b.c; }
