import { register, Counter, Gauge } from "prom-client";

export const attestationsTotal = new Counter({
  name: "trustlink_attestations_total",
  help: "Total number of attestations created",
});

export const revocationsTotal = new Counter({
  name: "trustlink_revocations_total",
  help: "Total number of attestations revoked",
});

export const issuersTotal = new Gauge({
  name: "trustlink_issuers_total",
  help: "Current number of registered issuers",
});

export const eventsProcessedTotal = new Counter({
  name: "trustlink_events_processed_total",
  help: "Total number of events processed",
});

export const indexerLagLedgers = new Gauge({
  name: "trustlink_indexer_lag_ledgers",
  help: "Number of ledgers behind the tip",
});

export async function getMetrics(): Promise<string> {
  return register.metrics();
}
