/** Test the Rust no-implementation subset of Speculative Generality. */
import codeql.rust.elements.Impl
import codeql.rust.elements.Trait

from Trait type
where
  type.getFile().getRelativePath() = "SpeculativeGenerality.rs" and
  not exists(Impl implementation | implementation.getTrait() = type)
select type, "This trait has no source implementation and may be speculative."
