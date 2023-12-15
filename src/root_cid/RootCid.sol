// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8;

import "@openzeppelin/contracts/access/AccessControl.sol";

/// @title Root CID Contract
/// @author Alex Miller
/// @notice This contract is a simple pointer to a metadata file on IPFS

contract RootCid is AccessControl {
  bytes32 public cid;
  bytes32 public constant WRITER_ROLE = keccak256("WRITER_ROLE");

  constructor(bytes32 _cid) {
    _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
    _grantRole(WRITER_ROLE, msg.sender);
    cid = _cid;
  }
  
  event updated(bytes32 cid);

  /* Permissions */

  function grantWriter(address account) public {
    require(hasRole(DEFAULT_ADMIN_ROLE, msg.sender));
    _grantRole(WRITER_ROLE, account);
  }

  /* CRUD ops */

  function read() public view returns (bytes32) {
    return cid;
  }

  // Set the CID of the blog - restricted to owner
  function update(bytes32 _cid) public { 
    require(hasRole(WRITER_ROLE, msg.sender));
    cid = _cid;
    emit updated(cid);
  }
}
