## Why

`Retainer` is documented as "an opaque token proving authority to settle a claim,"
but it is declared `Retainer(pub Uuid)` — a bare public field that any code can
construct, mutate, and read. The prose overclaims and the code under-delivers: the
representation is exposed and the "opaque" word is not honoured.

To be precise (correcting an earlier overstatement): this is **not** a security
hole. Authority is enforced by the registry validating the retainer against the
claim it issued — a forged identifier simply does not match. The real defect is API
hygiene: a bare public field contradicts the token's documented role and leaks its
representation. This change makes the code match the contract.

## What Changes

- Make `Retainer`'s identifier a private field.
- Add `Retainer::new(Uuid)` (registries mint tokens through it) and
  `Retainer::id(&self) -> Uuid` (registries read identity to validate).
- Correct the doc to state that authority is registry-validated, not proven by the
  type system.
- Update the three construction sites (kernel test, driver test, example) to use
  `Retainer::new`.

Non-goals: NOT encapsulating `Pact`/`Claim` yet, and NOT adding the hunyi
no-bare-pub anti-regression ratchet — that rule is module-scoped and would require
encapsulating the whole contract first (a separate, broader change). The private
field here is enforced by the compiler.

## Capabilities

### Modified Capabilities
- `domain-language`: add a requirement that `Retainer` encapsulates its identifier
  and that its authority is registry-validated.

## Impact

- **Code**: `pacta-contract` `Retainer` API (breaking: `.0` field access removed;
  pre-release, `publish = false`, no external users). Tests and the example updated.
- **Dependencies / governance**: none — no new dependency; Tianheng and the async
  ratchet are unaffected.
- **Enforcement**: the encapsulation is compiler-enforced (private field). The
  hunyi no-bare-pub ratchet is deferred to a future whole-contract encapsulation.
