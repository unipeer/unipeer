// SPDX-License-Identifier: AGPL-3.0-or-later

pragma solidity ^0.6.0;

import "@openzeppelin/contracts/access/Ownable.sol";

import "./StatusEnum.sol";

/**
 * An extended version of the standard `Pausable` contract, with more possible statuses:
 *  * STOPPED: all swap actions cannot be executed until the status is changed,
 *  * RETURN_ONLY: the existing swaps can only be returned, no new swaps can be created;
 *  * FINALIZE_ONLY: the existing swaps can be released or returned, no new swaps can be created;
 *  * ACTIVE: all swap actions can be executed.
 *
 * @dev the status enum is strictly monotonic, and the default 0 is mapped to STOPPED for safety.
 */
contract WithStatus is Ownable {
    event StatusChanged(Status oldStatus, Status newStatus);

    Status public status = Status.ACTIVE;

    function setStatus(Status _status) external onlyOwner {
        emit StatusChanged(status, _status);
        status = _status;
    }

    modifier statusAtLeast(Status _status) {
        require(status >= _status, "invalid contract status");
        _;
    }
}
