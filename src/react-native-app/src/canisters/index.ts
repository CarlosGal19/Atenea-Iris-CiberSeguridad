import { CandidCanister } from "@bundly/ares-core";

import { BackendActor, backend } from "./backend";

export type CandidActors = {
  backend: BackendActor;
};

export const candidCanisters: Record<keyof CandidActors, CandidCanister> = {
  backend,
};
