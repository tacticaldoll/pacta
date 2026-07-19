## 1. Contract API Migration

- [x] 1.1 Rename `pacta-contract` public storage trait from `Store` to `Registry`.
- [x] 1.2 Rename public pact grouping and business data fields from `lane`/`payload` to `docket`/`clause`.
- [x] 1.3 Rename `ReservationReceipt` and `Reservation` to `Retainer` and `Claim`.
- [x] 1.4 Rename public lifecycle methods from `reserve`/`ack`/`nack` to `claim`/`fulfill`/`breach`, preserving storage-purity semantics.
- [x] 1.5 Review whether `heartbeat` should remain as a mechanical retainer extension verb or be renamed during this change.

## 2. Documentation And Roadmap

- [x] 2.1 Add Pacta's canonical domain-language glossary to project documentation.
- [x] 2.2 Update `PROJECT.md` terminology and core contract sections to use contract/arbitration vocabulary.
- [x] 2.3 Update `README.md` to explain the renamed public lifecycle without weakening the middleware/storage separation.
- [x] 2.4 Update `BACKLOG.md` roadmap and deferred-work language for Executor, Registry, Docket, and Tribunal terminology.
- [x] 2.5 Add an ADR recording the contract-domain language decision and the public/private naming boundary.

## 3. Verification

- [x] 3.1 Search the repository for old public terms (`Store`, `Reservation`, `reserve`, `ack`, `nack`, `lane`, `payload`, `Handler`, `Driver`) and verify remaining uses are historical, comparative, or private-mechanical.
- [x] 3.2 Run the documented Definition of Done commands from the workspace root.
