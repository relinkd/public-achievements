# Achievement Module

This module provides functionality for managing achievements within a system. It includes logic for checking achievement eligibility, generating hashes, and managing achievement statuses.

## Modules

- `access`: Provides access control functions for checking if the caller is a controller and for retrieving the caller's principal ID.
- `ecdsa`: Provides ECDSA-related functionality, including public key retrieval, signing, and signature verification.
- `logic`: Contains the logic for checking achievement eligibility and managing achievements.
- `state`: Manages the state of the achievement system, including metadata, hashes, and achievement statuses.
- `storable`: Defines storable types and their implementations for use with stable structures.

## Access Module

The `access` module provides functions to check if the caller is a controller and to retrieve the caller's principal ID.

## ECDSA Module

The `ecdsa` module provides functions for ECDSA-related operations, including retrieving public keys, signing messages, and verifying signatures.

## Logic Module

The `logic` module contains the core logic for checking achievement eligibility and managing achievements. It includes functions to generate hashes, receive achievements, and verify signatures.

## State Module

The `state` module manages the state of the achievement system. It includes functions to update and retrieve metadata, hashes, and achievement statuses.

## Storable Module

The `storable` module defines types that can be stored in stable structures. It includes types for achievements, signatures, and principal-related data.
