import { ActorSubclass } from "@dfinity/agent";

import { CandidCanister } from "@bundly/ares-core";

import { idlFactory } from "../declarations/backend";
import { _SERVICE } from "../declarations/backend/backend.did";

export type BackendActor = ActorSubclass<_SERVICE>;

export const backend: CandidCanister = {
  idlFactory,
  actorConfig: {
    canisterId: process.env.BACKEND_CANISTER_ID
  }
};
