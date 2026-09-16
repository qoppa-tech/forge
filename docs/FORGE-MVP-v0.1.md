# FORGE MVP v0.1 — Bank-controlled lending on Solana

Prepared: 16 September 2026. Status: proposed implementation specification; no application or deployed program is delivered by this document.

## 1. Product decision

FORGE helps traditional banks integrate onchain financial operations with their existing systems. The first deliverable is a developer sandbox for a bank-controlled vault and a bilateral business loan on Solana.

The user has selected traditional banks as the primary customer, Solana as the first network, and proprietary source code initially. Hyperliquid and investment-market products belong to a later phase. The technical and product choices below are proposed defaults for the hackathon.

The first validation question: can a bank engineer integrate one controlled lending workflow, understand its permissions, and reconcile its results using FORGE's SDK and API?

## 2. The smallest complete demonstration

Use one fictional bank, two distinct bank approvers, one borrower, one vault, and a clearly labeled FORGE_TEST_USD token. Use a local validator first and Solana Devnet for the shared demonstration. The token has six decimals and no redemption promise.

| Step | Observable result |
| --- | --- |
| Treasury funds the vault with 10,000 test tokens | Vault balance is 10,000; funding transaction is recorded. |
| Bank proposes a 5,000-token loan | Borrower, amount, term and repayment terms become immutable. |
| Approver A approves | One approval appears; an attempted draw is rejected. |
| Approver B approves | The loan is eligible for the borrower to accept and draw. |
| Borrower accepts and draws | Borrower receives 5,000; vault balance becomes 5,000; principal receivable is 5,000. |
| Borrower repays 5,100 | Loan becomes repaid; vault balance becomes 10,100; principal receivable becomes zero. |
| Bank exports the statement | Each posting links to the finalized transaction and business reference. |

The example uses 2% interest for the entire agreed term, explicitly not an annual rate or APY. The repayment amount is fixed when the offer is created. Full early repayment is allowed for that same agreed amount in this demo model. The standard fixture uses a 30-day term; a separate short-term fixture demonstrates an overdue loan.

Seed the borrower with 100 test tokens before the loan so the demonstration can repay interest. Disclose this fixture funding. Repayment does not create interest tokens automatically.

The bank's core-system endpoint, customer checks and credit assessment are simulated. Token transfers, signatures, approval enforcement and loan state transitions must execute in the actual Solana program. This is an unsecured bilateral demonstration: overdue status records missed repayment; recovery remains a bank process. A prototype loan account records obligations; it is not presented as a legally certified or transferable security.

## 3. Stack and boundaries

| Layer | Proposed choice | Purpose |
| --- | --- | --- |
| Solana program | Rust + Anchor | Vault authority, approvals, loan state and token transfers. |
| SDK | TypeScript | Typed reads and transaction builders generated around the program IDL. |
| API and reconciler | One TypeScript application, Fastify, PostgreSQL | Bank integration, transaction tracking, statements and webhook delivery. |
| Operator interface | React + Vite | Vault overview, loan approval/draw/repayment and statement views. |
| Local development | Anchor local validator; Docker Compose for PostgreSQL | Repeatable development and demonstration fixtures. |
| Shared demonstration | Solana Devnet | Verifiable transactions using test assets. |

Keep the initial API in TypeScript so it can share the Anchor client with the SDK. A bank can consume REST from Go, Java or another language without depending on this implementation choice.

Anchor's current documentation uses @anchor-lang/core and specifies compatibility with @solana/web3.js v1. Record and pin the installed Anchor CLI, Rust dependencies and matching client packages together. Check the current generated template before adding dependencies. [Anchor TypeScript client](https://www.anchor-lang.com/docs/clients/typescript).

Start with the legacy SPL Token program and one explicitly configured test mint. Transfer restrictions on newly issued financial instruments can be a separate Token-2022 milestone. The vault rules in this MVP govern FORGE disbursements; they do not restrict every subsequent transfer of the underlying token.

Suggested repository layout:

| Path | Content |
| --- | --- |
| programs/forge/ | Anchor program and instruction handlers |
| packages/sdk/ | Transaction builders, typed reads, IDL and error decoding |
| apps/api/ | HTTP routes, reconciler and transactional webhook outbox |
| apps/dashboard/ | Operator and borrower demonstration interface |
| examples/bank-core/ | Minimal independent REST consumer and webhook receiver |
| tests/ | Program integration tests and projection recovery tests |
| scripts/ | Local seed/reset and Devnet deployment helpers |
| docs/ | Setup, assumptions, API examples and demonstration script |

## 4. Program model

Use a program-derived address (PDA) as the authority of the vault's token account. The SPL Token program owns that token account; the FORGE program controls its authority through PDA signing. [Solana PDAs](https://solana.com/docs/core/pda), [Anchor token transfers](https://www.anchor-lang.com/docs/tokens/basics/transfer-tokens).

### Vault

Store the creating treasury authority, unique vault ID, mint, token-account address, two distinct approver public keys, fixed treasury withdrawal destination, per-loan principal limit, total outstanding-principal limit, outstanding principal, and a disbursement-paused flag.

Vault seeds include the creating bank authority and a fixed-size unique vault ID. Approvers and limits are immutable for this version; changing them requires a new vault. Creation by an address does not certify that address as a real bank.

The bank is the sole capital provider in this MVP. There are no depositor shares or third-party withdrawal claims to account for. Multiple vaults must remain isolated even though the main demo uses one.

### Loan

Store vault, unique loan ID, borrower public key and destination token account, principal, term_rate_bps, fixed interest, total repayment, term_seconds, offer expiry, two approval flags, stored state, disbursement time and repayment time.

Loan seeds include the vault address and fixed-size loan ID. Keep completed loan accounts for the prototype so a previously used business identifier cannot be reused to disburse again. Customer names, documents and other personal data remain offchain.

Stored states are Proposed, Approved, Active and Repaid. Expired and Overdue are computed views: an undrawn offer expires at its deadline; an unpaid active loan becomes overdue after its due timestamp. Time passing does not automatically execute an onchain instruction.

### Instructions

| Instruction | Required behavior |
| --- | --- |
| create_vault | Treasury signs; validate two distinct approvers, exact mint/program, destinations and limits. |
| fund_vault | Treasury authorizes a checked transfer of the configured token into its vault. |
| propose_loan | A configured approver creates immutable terms and the designated borrower destination. |
| approve_loan | Record one configured approver's signature once; both distinct approvals are necessary. |
| draw_loan | Borrower signs acceptance; verify both approvals, offer expiry, pause state, liquidity and outstanding limits; transfer and state update atomically. |
| repay_loan | Borrower signs; collect the exact fixed payoff into the configured vault; update state and outstanding principal atomically. |
| withdraw_available | Both bank approvers sign; send only available vault tokens to the immutable treasury destination. |
| set_disbursement_paused | Both approvers sign; stop new draws while allowing repayment and treasury funding. |

Approving an offer does not reserve cash. Every draw rechecks available liquidity and portfolio limits inside the program. Use Solana's Clock for expiry and disbursement timestamps. The due date is the actual disbursement time plus term_seconds. All credit approval checks must hold for direct program calls as well as API calls.

Ordinary token transfers into the vault do not repay a loan. Repayment requires the repay_loan instruction so the debt and transfer update together. Unexpected incoming transfers become reconciliation exceptions rather than automatically allocated loan payments.

The API never holds borrower or bank approval private keys. A local seed script may generate clearly identified demonstration wallets. Each role signs independently through a wallet or local CLI. The prototype's upgrade authority must be documented; do not claim the deployer has no control over an upgradeable program.

## 5. Arithmetic and operational records

Use integer base units throughout. Represent amounts as decimal strings in JSON, BigInt/BN in TypeScript as required by the client, u64 onchain, and checked u128 intermediate multiplication.

Interest = floor(principal_base_units × term_rate_bps / 10,000).

Total repayment = principal_base_units + interest_base_units, using checked addition. Validate a positive principal and term, a bounded term rate, future offer expiry and all bank limits before storing an offer. The UI must name the rate as a whole-term rate and show the resulting payoff before signing.

The demonstration amount vector is principal 5,000,000,000 base units, term_rate_bps 200, interest 100,000,000 and total repayment 5,100,000,000. After funding and repayment, vault cash is 10,100,000,000 base units.

Build a small balanced operational subledger. The initial treasury opening balance is a fixture. Proposed postings:

| Event | Debit | Credit |
| --- | --- | --- |
| Fund vault | Vault token asset 10,000 | Treasury token asset 10,000 |
| Disburse | Principal receivable 5,000 | Vault token asset 5,000 |
| Repay | Vault token asset 5,100 | Principal receivable 5,000 and demo interest income 100 |

These postings demonstrate reconciliation and use a simplified cash-basis interest treatment. They are not a bank's regulatory accounting policy. Preserve the external business reference, chain, program ID, signature and slot for every posting.

## 6. API, signing and settlement

Expose transaction builders and read endpoints first. The names below describe a proposed interface, not an existing published SDK.

| Interface | Output |
| --- | --- |
| POST /v1/vaults | Creation transaction plan and required signer |
| POST /v1/vaults/{id}/fund | Funding transaction plan |
| POST /v1/vaults/{id}/loans | Immutable offer transaction plan |
| POST /v1/loans/{id}/approve | Transaction plan for the caller's approver key |
| POST /v1/loans/{id}/draw | Borrower acceptance and draw transaction plan |
| POST /v1/loans/{id}/repay | Full payoff transaction plan |
| POST /v1/transactions | Submit signed bytes; return operation ID and signature |
| GET /v1/operations/{id} | Submission and settlement status |
| GET /v1/loans/{id} | Terms, approvals, stored state and computed expiry/overdue status |
| GET /v1/vaults/{id}/statement | Finalized postings and reconciliation exceptions |

SDK builders should expose the decoded intent, required signers and serialized transaction. The caller signs after reviewing the actual destination, mint, amount, network and program. Authentication to the API controls institution-scoped metadata access; onchain signatures authorize financial operations.

Use separate transaction states: prepared, submitted, confirmed, finalized, failed, and expired. A submission response does not mean settlement. Show a confirmed transaction promptly, but create final statement postings and settlement webhooks only after successful finalization. [Signature status](https://solana.com/docs/rpc/http/getsignaturestatuses), [transaction retrieval](https://solana.com/docs/rpc/http/gettransaction).

Persist an idempotency key and payload hash per institution and API action. Reusing a key with a different request returns a conflict. Retransmit the same signed bytes when appropriate; never blindly create a fresh funding transaction after an uncertain result. If expiry leaves a funding result unresolved, surface that uncertainty for reconciliation before requesting another signature. Loan-state guards independently prevent a second draw or a second completed repayment.

Start with one polling reconciler, a persisted cursor and a database transaction that records the chain event, balanced posting and webhook outbox item together. Deduplicate by network, signature and instruction/event position. Replaying finalized events after a crash must not duplicate postings. Webhooks are at least once; publish a stable event ID that the bank-core example deduplicates.

## 7. Build order and exit criteria

Treat the dates as a proposed allocation up to the 12 October submission deadline, not an effort guarantee. Start with the CLI workflow; build screens after the program proves the transaction rules.

| Milestone | Target window | Exit criterion |
| --- | --- | --- |
| 1. Vault | 16–18 Sep | Local validator runs; create/fund vault; unauthorized direct withdrawal fails. |
| 2. Complete loan | 19–25 Sep | Two approvals, acceptance, disbursement, full repayment and overdue view work in the CLI. |
| 3. SDK and bank adapter | 26–30 Sep | Independent example uses the SDK/API; transaction intents and final statements reconcile. |
| 4. Operator demonstration | 1–5 Oct | Small UI covers the full flow; shared Devnet deployment is reproducible. |
| 5. External evaluation | 6–8 Oct | A bank engineer or relevant technical evaluator attempts integration; findings are recorded accurately. |
| 6. Submission | 9–12 Oct | Fix remaining flow failures, record demonstration and prepare a clear business pitch. |

Today's first implementation ticket: initialize the private Anchor workspace; record toolchain versions; implement create_vault and fund_vault with an explicit mint and PDA authority; seed two approvers and a treasury wallet; verify funded balance and rejection of an unauthorized transfer. That executable boundary is the foundation for the lending flow.

Useful initial commands after installing the documented toolchain: anchor init forge, anchor build, and anchor test. Follow the current [local development guide](https://www.anchor-lang.com/docs/quickstart/local); pin the successful toolchain rather than assuming an unverified version.

## 8. Essential acceptance tests

1. A caller bypassing the API cannot draw without both distinct configured approvers and the borrower signature.
2. One key cannot count as both approvers; a repeated approval does not advance the count.
3. Approved terms cannot change, including borrower, destination, amount, mint, rate and expiry.
4. Accounts from another vault, a different mint or an unexpected token program are rejected.
5. A second draw and a second repayment cannot transfer funds again, including with a new transaction signature.
6. Concurrent approved offers cannot breach available cash or the outstanding-principal cap.
7. Pausing blocks new disbursements while repayment remains possible; expired offers cannot draw.
8. Arithmetic rejects overflow and invalid values; rounding matches the documented fixed payoff.
9. Withdrawals require both bank signatures and the fixed treasury destination; receivables do not count as cash.
10. Failed or unfinalized transactions do not create final postings; replay after a worker crash creates each posting once.
11. A direct token donation triggers an exception rather than silently changing a loan balance.
12. One institution's API credentials cannot read another institution's offchain records.

## 9. Product validation and next modules

The first useful evidence is one technical evaluator completing the integration and identifying an internal bank workflow that FORGE could improve. Ask which system would integrate, which controls are missing, who would sponsor a sandbox pilot, and whether the time saved justifies paying for the integration. Record measured integration time and failures; present interest as interest unless a pilot or commercial commitment actually exists.

After the complete demonstration, prioritize borrower/issuer attestations, partial repayments and accrual policies, tokenized instruments with appropriate transfer rules, and production custody integration according to bank feedback. Hyperliquid belongs to the subsequent markets module. Keep common institution identity, approvals and statements reusable without building a universal chain abstraction in v0.1.

## 10. Handoff to Pi or another coding agent

Implement milestone 1 of this specification in a private repository. First read any repository instructions and check the current Anchor toolchain. Use one program, one configured six-decimal test mint, a PDA-authorized vault, immutable bank approver keys, and independent local test wallets. Keep bank and borrower signing keys out of the API. Provide a reproducible local fixture and meaningful direct-program authorization tests. Report exactly which commands pass and whether any transaction was deployed beyond the local validator. Continue to the loan lifecycle only after vault accounting and authority checks pass. Use this document as the product specification and preserve the stated Solana-first traditional-bank scope.

## Primary references

- [Anchor framework](https://www.anchor-lang.com/docs) — program development framework.
- [Anchor TypeScript client](https://www.anchor-lang.com/docs/clients/typescript) — current client package and compatibility constraint.
- [Anchor local development](https://www.anchor-lang.com/docs/quickstart/local) — setup, builds and local execution.
- [Solana PDAs](https://solana.com/docs/core/pda) — program signing and deterministic account addresses.
- [Anchor token transfers](https://www.anchor-lang.com/docs/tokens/basics/transfer-tokens) — checked transfers and PDA authority.
- [Solana clusters](https://solana.com/docs/references/clusters) — Devnet for application development with test assets.
- [Signature status](https://solana.com/docs/rpc/http/getsignaturestatuses) and [transaction retrieval](https://solana.com/docs/rpc/http/gettransaction) — reconciliation inputs.
- [Colosseum event](https://colosseum.com/worldsfair) — submission deadline; check for subsequent changes.
