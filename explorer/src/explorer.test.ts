/**
 * Tests for the contextcrucible explorer, run with the built-in `node:test`
 * runner so no third-party test framework is required.
 */

import { test } from "node:test";
import assert from "node:assert/strict";
import {
  allocationByDirectory,
  allocationByLanguage,
  bar,
  includedFiles,
  parseManifest,
  renderReport,
  type Manifest,
} from "./explorer.js";

function sampleManifest(): Manifest {
  return {
    summary: {
      tool: "contextcrucible",
      version: "0.4.0",
      label: "test",
      query: "budget solver",
      budget_tokens: 2000,
      tokens_used: 1000,
      budget_utilization: 0.5,
      captured_value: 100.0,
      min_score: 0,
      solver_method: "dp",
      counts: {
        included: 3,
        excluded_secret: 1,
