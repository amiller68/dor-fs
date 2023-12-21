// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import {RootCid} from "../src/root_cid/RootCid.sol";

contract CounterTest is Test {
    bytes32 foo = "foo";
    bytes32 bar = "bar";
    bytes32[2] init = [foo,bar];
    bytes32[2] update = [bar,foo];
    RootCid public root_cid;

    function setUp() public {
        root_cid = new RootCid(init);
    }

    function testRead() public {
        bytes32[2] memory cid = root_cid.read();
        assertEq(cid[0], init[0]);
        assertEq(cid[1], init[1]);
    }
    function testUpdate() public {
        root_cid.update(init, update);
        bytes32[2] memory cid = root_cid.read();
        assertEq(cid[0], update[0]);
        assertEq(cid[1], update[1]);
    }

    function testGrantWriter() public {
        root_cid.grantWriter(address(this));
        root_cid.update(init, update);
        bytes32[2] memory cid = root_cid.read();
        assertEq(cid[0], update[0]);
        assertEq(cid[1], update[1]);
    }

    function testFail_OutOfOrderUpdate() public {
        root_cid.update(update, init);
    }

    function testFail_UpdateAsNonWriter() public {
        vm.prank(address(0));
        root_cid.update(init, update);
    }
}
