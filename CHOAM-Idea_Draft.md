# CHOAM
Codex for Housing, Off-Grid Architecture & Mobility

Shelter is freedom.  
Homes must be open, modular, and enduring.  
No patent. No gatekeeper. No nation owns this.

We are building a Codex:  
Housing without borders, architecture without decay, mobility without dependence.  
It is the standard the world deserves—alive, shared, unbreakable.

**Version 1.1** (v1.0 citation widgets stripped; numbers still illustrative)  
**Date: September 3, 2025 · repaired August 2026**  
**Authors: Conceptualized with Grok, maintained by Dastille.**

## Table of Contents

1. Executive Summary
2. Introduction and Purpose
3. System Requirements
4. Architecture and Design Overview
5. Detailed Component Designs
6. Open-Source Strategy
7. Implementation Roadmap
8. Governance and Operations
9. Risk Management and Mitigations
10. Economic Analysis
11. Testing, Validation, and Evaluation
12. Deployment and Scaling Strategies
13. Maintenance, Sustainability, and Evolution
14. Appendices

## 1. Executive Summary

CHOAM is a flexible, open-source framework designed to guarantee baseline living standards through government-provisioned modular tiny homes equipped with solar energy, broadband internet, and distributed computing capabilities. By integrating universal basic services (UBS) principles, CHOAM addresses housing shortages, poverty, and sustainability challenges while generating revenue to aim at fiscal neutrality.

Hardware designs are licensed under the CERN Open Hardware Licence Strongly Reciprocal v2 (CERN OHL-S v2), ensuring mandatory sharing of modifications. Software components in this repository use AGPL-3.0; the original draft also allowed GPL-3.0 for non-networked tools.

This document is a blueprint for implementation, emphasizing adaptability across jurisdictions without fixed timelines. Precedents include CERN open hardware, UNICEF open-source tools, WikiHouse, BOINC, and Canadian net metering. With safeguards, CHOAM *aims* to reduce construction timelines (modular factory literature often cites up to ~50%) and lower costs on the order of 20–30%. Treat those percentages as design targets, not measured results.

## 2. Introduction and Purpose

CHOAM aims to establish a floor for living standards by provisioning essentials directly, evolving from UBS theories and UBI pilots. It is written against Canada's housing shortage and is portable. The purpose of this document is to guide adopters (governments, non-profits, cooperatives) in deploying CHOAM with openness, sustainability, and resistance to procurement capture.

## 3. System Requirements

### 3.1 Functional Requirements

- **Housing**: Produce and distribute modular tiny homes (200–400 sq ft) with customizable utility interfaces.
- **Energy**: Integrate 5–10 kW solar arrays per unit, enabling net metering for excess power.
- **Connectivity and computing**: Provide broadband and opt-in distributed computing nodes for research tasks.
- **Service extensions**: Food-security vouchers and health ties, adapted locally.
- **Open-source compliance**: All designs must allow modification and redistribution under the stated licenses.
- **Accessibility**: Ramps, 36-inch clearances, reachable controls, and layouts that work with mobility devices. This is a requirement, not a variant.

### 3.2 Non-Functional Requirements

- **Scalability**: Pilots of ~100 units to national scale; factory sketches of 1,000–2,000 units/year.
- **Security**: PIPEDA (and GDPR where applicable); encrypted data on compute nodes; opt-in only.
- **Sustainability**: Zero-waste design target; recyclable components.
- **Cost**: $50,000–$80,000 per unit, with a 5–7 year ROI sketch via revenue.

## 4. Architecture and Design Overview

Modular and decentralized. Central repositories host designs; factories produce components; units connect to grids and networks for revenue.

Flow: Design → Manufacture → Deploy → Operate → Monetize.

Interfaces should be standardized (ICC 1220-class, or the local equivalent) so a solar string, a compute node, and a kitchen wet-core can be swapped without a custom carpenter.

## 5. Detailed Component Designs

### 5.1 Modular Housing Framework

Stackable units with open-source blueprints (CNC-cut frames in the WikiHouse tradition). Plug-and-play utility modules. Cost literature for factory modular often lands far above stick-build per square foot until volume; the $500–600/sq ft figure in v1.0 is a **warning**, not a target. CHOAM's $50–80k envelope at 200–400 sq ft implies $125–400/sq ft — ambitious, and it only works with volunteer/public land, simple envelopes, and no luxury fit-out.

### 5.2 Renewable Energy Integration

Rooftop solar with batteries. Excess sold via net metering (illustrative $200–500/unit/year depending on tariff — check the actual IESO/utility rider before using this in a budget). Open-source inverters; aggregate into virtual power plants. Grid constraints are mitigated with storage and policy, not wishful interconnection.

### 5.3 Internet and Distributed Computing Network

Broadband nodes running copyleft software for opt-in tasks (BOINC-class). Revenue from research partnerships. Excess solar powers the node so the household is not billed to donate cycles. PIPEDA: no ambient audio, no facial capture, no off-box identity.

### 5.4 Additional Baseline Services

Vouchers for food; ties to healthcare. Open-source apps for household management — never a mandatory telemetry agent.

## 6. Open-Source Strategy

### 6.1 Licensing and Governance

- **Hardware**: CERN OHL-S v2, including full documentation for derivatives.
- **Software**: AGPL-3.0 for anything networked; GPL-3.0 acceptable for offline tools.
- **Governance**: Steering committee with public minutes; OSHWA certification as a credibility check.

### 6.2 Repository and Collaboration

GitHub is fine; Codeberg/GitLab mirrors are welcome. Required: CONTRIBUTING.md, issue templates, public bill-of-materials.

## 7. Implementation Roadmap

- **Pilot**: 100–500 units in volunteer jurisdictions.
- **Scale-up**: Factories only after the pilot's cost, accessibility, and energy numbers are real.
- **Optimization**: Export of parts, not of a gated franchise.

Resource sketch: $1–2B for factories if a nation actually does this. Timelines for modular can be shorter than site-built (literature: on the order of 16 weeks/unit once the line exists). Adapt per adopter.

## 8. Governance and Operations

Central body (federal or a co-op federation) sets the spec. Provincial/municipal partners site the units. Dashboards for cost, vacancy, energy export, compute hours. Annual public audit.

## 9. Risk Management and Mitigations

| Class | Failure | Mitigation |
| --- | --- | --- |
| Economic | Cost overrun, market distortion | PPPs with open books, unit-cost caps |
| Social / political | Inequity, NIMBY | Community siting, disability and income audits |
| Technical | Interface drift | Frozen spec + DIN/ICC-class standards |
| Environmental | Construction waste | Lifecycle assessment, take-back |
| Legal / security | Privacy abuse on compute nodes | PIPEDA, no default cameras, deletion logs |
| Corruption | Procurement graft | Open contracting, beneficial-ownership registries, whistleblower protection (OECD / UNCAC posture) |

## 10. Economic Analysis

### 10.1 Cost projections

Factories $1–2B initial; units $50k–80k. Both are **order-of-magnitude**.

### 10.2 Revenue models

Solar aggregate and compute partnerships are the two intended streams. v1.0's "$1–3B solar / $1B compute" lines are national-scale sketches, not a pro forma.

### 10.3 Benefit-cost

Health and poverty-cost offsets are the public-finance case. ROI 5–7 years is a **target**, gated on tariffs and occupancy.

## 11. Testing, Validation, and Evaluation

Pilot metrics: delivered cost, days-to-occupy, accessibility audit pass rate, kWh exported, compute hours opted-in, user satisfaction, privacy incidents (must be zero). Third-party audit before scale-up.

## 12. Deployment and Scaling Strategies

Start small. Do not announce 100,000 units from a markdown file. Scale factories only on measured unit cost. Parts may export; the spec stays public.

## 13. Maintenance, Sustainability, and Evolution

Spec lives in this repo. Community PRs. Circular-economy take-back for panels, batteries, and CNC waste.

## 14. Appendices

### A. License details

- CERN OHL-S v2 — hardware documentation, strongly reciprocal.
- AGPL-3.0 — this repository and networked software.
- GPL-3.0 — optional for offline tools.

### B. Case studies and precedents

WikiHouse; Terner Center modular multifamily (timeline cuts); ARENA-style VPPs; Folding@Home / BOINC; Canadian municipal open-contracting practice.

### C. Glossary

- **UBS**: Universal Basic Services
- **Copyleft**: License requiring openness in derivatives
- **VPP**: Virtual power plant
- **Unimproved land value**: Value of the site excluding buildings — relevant to the companion tax-reform repo
