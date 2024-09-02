# Reputation Module

This module provides functionality for managing achievements and reputation within a system. It includes logic for issuing achievements, managing metadata, and handling permissions.

## Modules

- `access`: Provides access control functions for checking if the caller is a controller and for retrieving the caller's principal ID.
- `icrc_7`: Contains types and logic related to the ICRC-7 standard.
- `logic`: Contains the logic for issuing achievements and managing reputation.
- `state`: Manages the state of the reputation module, including metadata, achievements, and permissions.
- `storable`: Defines storable types and their implementations for use with stable structures.
- `types`: Defines the types used in the reputation module.
- `utils`: Provides utility functions for the reputation module.

## Access Module

The `access` module provides functions to check if the caller is a controller and to retrieve the caller's principal ID.

## ICRC-7 Module

The `icrc_7` module contains types and logic related to the ICRC-7 standard.

## Logic Module

The `logic` module contains the core logic for issuing achievements and managing reputation. It includes functions to issue achievements, check permissions, and manage metadata.

## State Module

The `state` module manages the state of the reputation module. It includes functions to update and retrieve metadata, achievements, and permissions.

## Storable Module

The `storable` module defines types that can be stored in stable structures. It includes types for achievements, permissions, and principal-related data.

## Types Module

The `types` module defines the types used in the reputation module. It includes types for achievement metadata.

## Utils Module

The `utils` module provides utility functions for the reputation module. It includes functions to build principal sums and other helper functions.
