// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import {RootCid} from "../src/root_cid/RootCid.sol";

contract CounterTest is Test {
    bytes32 init = "foo";
    bytes32 update = "bar";
    RootCid public root_cid;

    function setUp() public {
        root_cid = new RootCid(init);
    }

    function testRead() public {
        bytes32 cid = root_cid.read();
        assertEq(cid, init);
    }

    function testUpdate() public {
        root_cid.update(init, update);
        bytes32 cid = root_cid.read();
        assertEq(cid, update);
    }

    function testGrantWriter() public {
        root_cid.grantWriter(address(this));
        root_cid.update(init, update);
        bytes32 cid = root_cid.read();
        assertEq(cid, update);
    }

    function testFail_OutOfOrderUpdate() public {
        root_cid.update(update, init);
    }

    function testFail_UpdateAsNonWriter() public {
        vm.prank(address(0));
        root_cid.update(init, update);
    }
}
