// SPDX-License-Identifier: AGPL-3.0-or-later

pragma solidity ^0.6.0;

import "@openzeppelin/contracts/proxy/Proxy.sol";
import "@openzeppelin/contracts/utils/Address.sol";

/**
 * @dev This contract implements an non-upgradeable/static proxy.
 * It is non-upgradeable because the implementation address where calls are
 * delegated to cannot be changed after construction.
 * This address is stored in storage in the location specified by
 * https://eips.ethereum.org/EIPS/eip-1967[EIP1967], so that it doesn't
 * conflict with the storage layout of the implementation behind the proxy.
 *
 * A non-upgradeble proxy is still useful as a user deployed contract;
 * it helps minimise gas costs of deploying a new contract, makes it easier to
 * verify the contracts abi when called from an external contract, among others.
 */
contract StaticProxy is Proxy {
    /**
     * @dev Initializes the upgradeable proxy with an initial implementation
     * specified by `_logic`.
     *
     * If `_data` is nonempty, it's used as data in a delegate call to `_logic`.
     * This will typically be an encoded function call, and allows
     * initializing the storage of the proxy like a Solidity constructor.
     */
    constructor(address _logic, bytes memory _data) public payable {
        assert(
            _IMPLEMENTATION_SLOT ==
                bytes32(uint256(keccak256("eip1967.proxy.implementation")) - 1)
        );
        _setImplementation(_logic);
        if (_data.length > 0) {
            // solhint-disable-next-line avoid-low-level-calls
            (bool success, ) = _logic.delegatecall(_data);
            require(
                success,
                "StaticProxy: delegatecall with data failed while initializing"
            );
        }
    }

    /**
     * @dev Storage slot with the address of the current implementation.
     * This is the keccak-256 hash of "eip1967.proxy.implementation" subtracted by 1, and is
     * validated in the constructor.
     */
    bytes32
        private constant _IMPLEMENTATION_SLOT = 0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc;

    /**
     * @dev Returns the current implementation address.
     */
    function _implementation() internal view override returns (address impl) {
        bytes32 slot = _IMPLEMENTATION_SLOT;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            impl := sload(slot)
        }
    }

    /**
     * @dev Stores a new address in the EIP1967 implementation slot.
     */
    function _setImplementation(address newImplementation) private {
        require(
            Address.isContract(newImplementation),
            "StaticProxy: new implementation is not a contract"
        );

        bytes32 slot = _IMPLEMENTATION_SLOT;

        // solhint-disable-next-line no-inline-assembly
        assembly {
            sstore(slot, newImplementation)
        }
    }
}
