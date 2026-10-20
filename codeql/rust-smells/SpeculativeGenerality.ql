/**
 * @name Speculative generality
 * @description Reports Rust traits without source implementations.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/rust/speculative-generality
 * @precision medium
 * @tags maintainability
 */
import codeql.rust.elements.Impl
import codeql.rust.elements.Trait

from Trait type
where
  not exists(Impl implementation | implementation.getTrait() = type)
select type, "This trait has no source implementation and may be speculative."
