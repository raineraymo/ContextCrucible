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
